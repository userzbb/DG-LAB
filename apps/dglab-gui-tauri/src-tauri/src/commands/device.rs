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

    let manager = BleManager::new().await.map_err(|e| {
        let error_msg = format!("创建蓝牙管理器失败: {}. 请检查蓝牙是否已启用", e);
        tracing::error!("{}", error_msg);
        error_msg
    })?;

    manager.start_scan().await.map_err(|e| {
        let error_msg = format!("启动扫描失败: {}. 请检查蓝牙权限", e);
        tracing::error!("{}", error_msg);
        error_msg
    })?;

    // Wait for scan duration
    let timeout = std::time::Duration::from_secs(timeout_secs.unwrap_or(10));
    tokio::time::sleep(timeout).await;

    let results = manager.get_scan_results().await.map_err(|e| {
        let error_msg = format!("获取扫描结果失败: {}", e);
        tracing::error!("{}", error_msg);
        error_msg
    })?;

    manager.stop_scan().await.map_err(|e| {
        let error_msg = format!("停止扫描失败: {}", e);
        tracing::error!("{}", error_msg);
        error_msg
    })?;

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
    use std::sync::Arc;

    info!("Connecting to BLE device: {} ({})", device_name, device_id);

    // 创建 BLE manager
    let ble_manager = Arc::new(BleManager::new().await.map_err(|e| {
        let error_msg = format!("创建蓝牙管理器失败: {}. 请检查蓝牙是否已启用", e);
        tracing::error!("{}", error_msg);
        error_msg
    })?);

    // 连接到 BLE 设备
    let ble_device = ble_manager.connect(&device_id).await.map_err(|e| {
        let error_msg = format!("连接蓝牙设备失败: {}. 请确保设备已开启且在范围内", e);
        tracing::error!("{}", error_msg);
        error_msg
    })?;

    info!("BLE device connected successfully");

    // 创建 CoyoteDevice（使用带 manager 的构造函数）
    let mut coyote =
        CoyoteDevice::with_manager(device_id.clone(), device_name.clone(), ble_manager.clone());

    // 设置协议设备
    coyote.set_protocol_device(ble_device);

    // 连接并初始化设备（发送 BF 配置等）
    coyote.connect().await.map_err(|e| {
        let error_msg = format!("设备初始化失败: {}. 请重试或重启设备", e);
        tracing::error!("{}", error_msg);
        error_msg
    })?;

    info!("Device initialized successfully");

    let info = coyote.info();

    // 保存 BLE manager，防止连接被丢弃
    {
        let mut managers = state.ble_managers.write().await;
        managers.insert(device_id.clone(), ble_manager.clone());
    }

    // 添加到会话管理器
    {
        let manager = state.session_manager.write().await;
        manager.add_device(Box::new(coyote)).await.map_err(|e| {
            let error_msg = format!("添加设备到会话失败: {}", e);
            tracing::error!("{}", error_msg);
            error_msg
        })?;
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
        .ok_or_else(|| format!("设备未找到: {}", device_id))?;

    let mut dev = device.write().await;
    dev.disconnect().await.map_err(|e| {
        let error_msg = format!("断开连接失败: {}", e);
        tracing::error!("{}", error_msg);
        error_msg
    })?;

    // 清理 BLE manager
    {
        let mut managers = state.ble_managers.write().await;
        managers.remove(&device_id);
    }

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
