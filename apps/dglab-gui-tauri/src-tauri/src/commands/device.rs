//! 设备相关命令

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};
use tracing::{debug, info};

use dglab_core::device::traits::DeviceInfo;
use dglab_core::device::DeviceState;
use dglab_protocol::ble::BleManager;

use crate::events::{event_names, DeviceStateChangedEvent};
use crate::state::AppState;

/// 扫描到的设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedDevice {
    /// 设备 ID
    pub id: String,
    /// 设备名称
    pub name: String,
    /// 信号强度 (RSSI)
    pub rssi: Option<i16>,
    /// 设备地址
    pub address: String,
}

/// 扫描 BLE 设备
#[tauri::command]
pub async fn scan_ble_devices(timeout_secs: Option<u64>) -> Result<Vec<ScannedDevice>, String> {
    info!("Starting BLE device scan, timeout: {:?}", timeout_secs);

    let manager = BleManager::new()
        .await
        .map_err(|e| format!("Failed to create BLE manager: {}", e))?;

    manager
        .start_scan()
        .await
        .map_err(|e| format!("Failed to start scan: {}", e))?;

    // Wait for scan duration
    let timeout = std::time::Duration::from_secs(timeout_secs.unwrap_or(10));
    tokio::time::sleep(timeout).await;

    let results = manager
        .get_scan_results()
        .await
        .map_err(|e| format!("Failed to get scan results: {}", e))?;

    manager
        .stop_scan()
        .await
        .map_err(|e| format!("Failed to stop scan: {}", e))?;

    let scanned: Vec<ScannedDevice> = results
        .into_iter()
        .map(|r| ScannedDevice {
            id: r.id,
            name: r.name,
            rssi: r.rssi,
            address: r.address,
        })
        .collect();

    info!("Found {} devices", scanned.len());
    Ok(scanned)
}

/// 连接 BLE 设备（从扫描结果）
#[tauri::command]
pub async fn connect_ble_device(
    app: AppHandle,
    state: State<'_, AppState>,
    device_id: String,
    device_name: String,
) -> Result<DeviceInfo, String> {
    use dglab_core::device::{CoyoteDevice, Device};

    info!("Connecting to BLE device: {} ({})", device_name, device_id);

    // 创建 BLE manager 并连接设备
    let ble_manager = BleManager::new()
        .await
        .map_err(|e| format!("Failed to create BLE manager: {}", e))?;

    let ble_device = ble_manager
        .connect(&device_id)
        .await
        .map_err(|e| format!("Failed to connect to BLE device: {}", e))?;

    // 创建 CoyoteDevice 并设置协议设备
    let mut coyote = CoyoteDevice::new(device_id.clone(), device_name.clone());
    coyote.set_protocol_device(ble_device);
    coyote
        .connect()
        .await
        .map_err(|e| format!("Failed to connect device: {}", e))?;

    let info = coyote.info();

    // 添加到会话管理器
    {
        let mut manager = state.session_manager.write().await;
        manager
            .add_device(Box::new(coyote))
            .await
            .map_err(|e| format!("Failed to add device to session: {}", e))?;
    }

    // 发送状态变更事件
    let _ = app.emit(
        event_names::DEVICE_STATE_CHANGED,
        DeviceStateChangedEvent {
            device_id: device_id.clone(),
            state: DeviceState::Connected,
        },
    );

    info!("Successfully connected to device: {}", device_id);
    Ok(info)
}

/// 连接设备（已存在于 session manager 中的设备）
#[tauri::command]
pub async fn connect_device(
    app: AppHandle,
    state: State<'_, AppState>,
    device_id: String,
) -> Result<DeviceInfo, String> {
    info!("Connecting to device: {}", device_id);

    let manager = state.session_manager.read().await;
    let device = manager
        .get_device(&device_id)
        .await
        .ok_or_else(|| format!("Device not found: {}", device_id))?;

    let mut dev = device.write().await;
    dev.connect()
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    let info = dev.info();

    // 发送状态变更事件
    let _ = app.emit(
        event_names::DEVICE_STATE_CHANGED,
        DeviceStateChangedEvent {
            device_id: device_id.clone(),
            state: DeviceState::Connected,
        },
    );

    info!("Connected to device: {}", device_id);
    Ok(info)
}

/// 断开设备连接
#[tauri::command]
pub async fn disconnect_device(
    app: AppHandle,
    state: State<'_, AppState>,
    device_id: String,
) -> Result<(), String> {
    info!("Disconnecting device: {}", device_id);

    let manager = state.session_manager.read().await;
    let device = manager
        .get_device(&device_id)
        .await
        .ok_or_else(|| format!("Device not found: {}", device_id))?;

    let mut dev = device.write().await;
    dev.disconnect()
        .await
        .map_err(|e| format!("Disconnect failed: {}", e))?;

    // 发送状态变更事件
    let _ = app.emit(
        event_names::DEVICE_STATE_CHANGED,
        DeviceStateChangedEvent {
            device_id: device_id.clone(),
            state: DeviceState::Disconnected,
        },
    );

    info!("Disconnected device: {}", device_id);
    Ok(())
}

/// 获取设备信息
#[tauri::command]
pub async fn get_device_info(
    state: State<'_, AppState>,
    device_id: String,
) -> Result<DeviceInfo, String> {
    debug!("Getting device info: {}", device_id);

    let manager = state.session_manager.read().await;
    let device = manager
        .get_device(&device_id)
        .await
        .ok_or_else(|| format!("Device not found: {}", device_id))?;

    let dev = device.read().await;
    Ok(dev.info())
}

/// 获取设备状态
#[tauri::command]
pub async fn get_device_state(
    state: State<'_, AppState>,
    device_id: String,
) -> Result<DeviceState, String> {
    debug!("Getting device state: {}", device_id);

    let manager = state.session_manager.read().await;
    let device = manager
        .get_device(&device_id)
        .await
        .ok_or_else(|| format!("Device not found: {}", device_id))?;

    let dev = device.read().await;
    Ok(dev.state())
}
