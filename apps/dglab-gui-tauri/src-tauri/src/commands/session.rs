//! 会话管理相关命令

use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::debug;

use crate::state::AppState;

/// 会话信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// 会话 ID
    pub id: String,
    /// 会话创建时间
    pub created_at: String,
    /// 活动设备数量
    pub active_devices: usize,
    /// 总设备数量
    pub total_devices: usize,
}

/// 获取会话信息
#[tauri::command]
pub async fn get_session_info(state: State<'_, AppState>) -> Result<SessionInfo, String> {
    debug!("Getting session info");

    let manager = state.session_manager.read().await;
    let info = manager.session_info().await;

    Ok(SessionInfo {
        id: info.id,
        created_at: info.created_at.to_rfc3339(),
        active_devices: info.active_devices,
        total_devices: info.total_devices,
    })
}

/// 列出所有设备 ID
#[tauri::command]
pub async fn list_devices(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    debug!("Listing all devices");

    let manager = state.session_manager.read().await;
    let device_ids = manager.list_devices().await;

    Ok(device_ids)
}
