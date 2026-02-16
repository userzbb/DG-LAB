//! WiFi 相关命令

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};
use tracing::{debug, info, warn};

use dglab_core::device::{Device, WsCoyoteDevice};

use crate::events::{event_names, DeviceStateChangedEvent};
use crate::state::AppState;

/// WiFi 连接请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiConnectRequest {
    /// 自定义服务器地址（可选，默认使用官方服务器）
    pub server_url: Option<String>,
}

/// WiFi 连接响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiConnectResponse {
    /// 设备 ID
    pub device_id: String,
    /// 设备名称
    pub device_name: String,
    /// 二维码 URL
    pub qr_url: String,
}

/// 连接 WiFi 设备
///
/// 创建 WiFi 设备并返回二维码 URL，用户需要用 DG-LAB APP 扫描二维码进行绑定
#[tauri::command]
pub async fn wifi_connect(
    app: AppHandle,
    state: State<'_, AppState>,
    request: WifiConnectRequest,
) -> Result<WifiConnectResponse, String> {
    info!("Starting WiFi connection");

    let device_id = uuid::Uuid::new_v4().to_string();
    let device_name = "WiFi-Coyote".to_string();

    // 创建 WiFi 设备
    let mut wifi_device = if let Some(server) = request.server_url {
        info!("Using custom server: {}", server);
        WsCoyoteDevice::with_server(device_id.clone(), device_name.clone(), server)
    } else {
        info!("Using official server");
        WsCoyoteDevice::new(device_id.clone(), device_name.clone())
    };

    // 连接到服务器
    wifi_device
        .connect()
        .await
        .map_err(|e| format!("Failed to connect to WiFi server: {}", e))?;

    // 等待获取 clientId
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 获取二维码 URL
    let qr_url = wifi_device
        .qr_url()
        .await
        .ok_or_else(|| "Failed to get QR code URL".to_string())?;

    info!("WiFi device created with QR URL: {}", qr_url);

    // 添加到会话管理器
    let manager = state.session_manager.write().await;
    manager
        .add_device(Box::new(wifi_device))
        .await
        .map_err(|e| format!("Failed to add device to session: {}", e))?;

    // 发送设备添加事件
    let _ = app.emit(
        event_names::DEVICE_STATE_CHANGED,
        DeviceStateChangedEvent {
            device_id: device_id.clone(),
            state: dglab_core::device::DeviceState::Connecting,
        },
    );

    Ok(WifiConnectResponse {
        device_id,
        device_name,
        qr_url,
    })
}

/// 检查 WiFi 设备绑定状态
#[tauri::command]
pub async fn wifi_check_binding(
    state: State<'_, AppState>,
    device_id: String,
) -> Result<bool, String> {
    debug!("Checking WiFi binding status for: {}", device_id);

    let manager = state.session_manager.read().await;
    let device = manager
        .get_device(&device_id)
        .await
        .ok_or_else(|| format!("Device not found: {}", device_id))?;

    let dev = device.read().await;
    let state = dev.state();

    // 如果已连接说明绑定成功
    Ok(matches!(
        state,
        dglab_core::device::DeviceState::Connected | dglab_core::device::DeviceState::Running
    ))
}

/// 取消 WiFi 连接（断开并移除设备）
#[tauri::command]
pub async fn wifi_cancel(
    app: AppHandle,
    state: State<'_, AppState>,
    device_id: String,
) -> Result<(), String> {
    info!("Cancelling WiFi connection: {}", device_id);

    let manager = state.session_manager.write().await;

    // 断开设备
    if let Some(device) = manager.get_device(&device_id).await {
        let mut dev = device.write().await;
        if let Err(e) = dev.disconnect().await {
            warn!("Failed to disconnect device: {}", e);
        }
    }

    // 从会话中移除
    manager
        .remove_device(&device_id)
        .await
        .map_err(|e| format!("Failed to remove device: {}", e))?;

    // 发送设备移除事件
    let _ = app.emit(
        event_names::DEVICE_STATE_CHANGED,
        DeviceStateChangedEvent {
            device_id: device_id.clone(),
            state: dglab_core::device::DeviceState::Disconnected,
        },
    );

    Ok(())
}
