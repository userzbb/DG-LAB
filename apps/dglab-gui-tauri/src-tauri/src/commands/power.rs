//! 功率控制相关命令

use tauri::{AppHandle, Emitter, State};
use tracing::{debug, info};

use dglab_core::device::DeviceState;

use crate::events::{event_names, DevicePowerChangedEvent, DeviceStateChangedEvent};
use crate::state::AppState;

/// 设置设备功率
#[tauri::command]
pub async fn set_power(
    app: AppHandle,
    state: State<'_, AppState>,
    device_id: String,
    channel: u8,
    power: u8,
) -> Result<(), String> {
    info!(
        "Setting power for device {}, channel {}: {}",
        device_id, channel, power
    );

    let manager = state.session_manager.read().await;
    let device = manager
        .get_device(&device_id)
        .await
        .ok_or_else(|| format!("Device not found: {}", device_id))?;

    let mut dev = device.write().await;
    dev.set_power(channel, power)
        .await
        .map_err(|e| format!("Failed to set power: {}", e))?;

    // 获取当前功率状态并发送事件
    let info = dev.info();
    let _ = app.emit(
        event_names::DEVICE_POWER_CHANGED,
        DevicePowerChangedEvent {
            device_id: device_id.clone(),
            power_a: info.power_a,
            power_b: info.power_b,
        },
    );

    Ok(())
}

/// 开始设备输出
#[tauri::command]
pub async fn start_device(
    app: AppHandle,
    state: State<'_, AppState>,
    device_id: String,
) -> Result<(), String> {
    info!("Starting device: {}", device_id);

    let manager = state.session_manager.read().await;
    let device = manager
        .get_device(&device_id)
        .await
        .ok_or_else(|| format!("Device not found: {}", device_id))?;

    let mut dev = device.write().await;
    dev.start()
        .await
        .map_err(|e| format!("Failed to start device: {}", e))?;

    // 发送状态变更事件
    let _ = app.emit(
        event_names::DEVICE_STATE_CHANGED,
        DeviceStateChangedEvent {
            device_id: device_id.clone(),
            state: DeviceState::Running,
        },
    );

    Ok(())
}

/// 停止设备输出
#[tauri::command]
pub async fn stop_device(
    app: AppHandle,
    state: State<'_, AppState>,
    device_id: String,
) -> Result<(), String> {
    info!("Stopping device: {}", device_id);

    let manager = state.session_manager.read().await;
    let device = manager
        .get_device(&device_id)
        .await
        .ok_or_else(|| format!("Device not found: {}", device_id))?;

    let mut dev = device.write().await;
    dev.stop()
        .await
        .map_err(|e| format!("Failed to stop device: {}", e))?;

    // 发送状态变更事件
    let _ = app.emit(
        event_names::DEVICE_STATE_CHANGED,
        DeviceStateChangedEvent {
            device_id: device_id.clone(),
            state: DeviceState::Connected,
        },
    );

    Ok(())
}

/// 紧急停止（设置所有通道功率为 0 并停止）
#[tauri::command]
pub async fn emergency_stop(
    app: AppHandle,
    state: State<'_, AppState>,
    device_id: String,
) -> Result<(), String> {
    info!("Emergency stop for device: {}", device_id);

    let manager = state.session_manager.read().await;
    let device = manager
        .get_device(&device_id)
        .await
        .ok_or_else(|| format!("Device not found: {}", device_id))?;

    let mut dev = device.write().await;

    // 设置所有通道为 0
    if let Err(e) = dev.set_power(0, 0).await {
        debug!("Failed to set channel A to 0: {}", e);
    }
    if let Err(e) = dev.set_power(1, 0).await {
        debug!("Failed to set channel B to 0: {}", e);
    }

    // 停止设备
    dev.stop()
        .await
        .map_err(|e| format!("Failed to stop device: {}", e))?;

    // 发送功率变更事件
    let _ = app.emit(
        event_names::DEVICE_POWER_CHANGED,
        DevicePowerChangedEvent {
            device_id: device_id.clone(),
            power_a: 0,
            power_b: 0,
        },
    );

    // 发送状态变更事件
    let _ = app.emit(
        event_names::DEVICE_STATE_CHANGED,
        DeviceStateChangedEvent {
            device_id: device_id.clone(),
            state: DeviceState::Connected,
        },
    );

    Ok(())
}
