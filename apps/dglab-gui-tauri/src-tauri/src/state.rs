//! 应用状态管理

use std::sync::Arc;
use tokio::sync::RwLock;

use dglab_core::session::SessionManager;

/// 应用状态
pub struct AppState {
    /// 会话管理器
    pub session_manager: Arc<RwLock<SessionManager>>,
}

impl AppState {
    /// 创建新的应用状态
    pub fn new() -> Self {
        Self {
            session_manager: Arc::new(RwLock::new(SessionManager::new())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
