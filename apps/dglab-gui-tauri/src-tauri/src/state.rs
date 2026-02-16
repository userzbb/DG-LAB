//! 应用状态管理

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use dglab_core::session::SessionManager;
use dglab_protocol::ble::BleManager;

/// 应用状态
pub struct AppState {
    /// 会话管理器
    pub session_manager: Arc<RwLock<SessionManager>>,
    /// BLE 管理器（保持连接）
    pub ble_managers: Arc<RwLock<HashMap<String, BleManager>>>,
}

impl AppState {
    /// 创建新的应用状态
    pub fn new() -> Self {
        Self {
            session_manager: Arc::new(RwLock::new(SessionManager::new())),
            ble_managers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
