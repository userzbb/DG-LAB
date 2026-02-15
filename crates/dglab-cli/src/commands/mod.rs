//! CLI 命令实现

use std::sync::Arc;

use crate::error::Result;
use dglab_core::preset::PresetManager;
use dglab_core::session::SessionManager;
use dglab_protocol::ble::BleManager;

pub mod connect;
pub mod control;
pub mod preset;
pub mod scan;
pub mod script;
pub mod wifi;

pub use connect::ConnectArgs;
pub use control::ControlArgs;
pub use preset::PresetArgs;
pub use scan::ScanArgs;
pub use script::ScriptArgs;
pub use wifi::WifiArgs;

/// CLI 应用
pub struct DglabCli {
    /// BLE 管理器
    ble_manager: Arc<BleManager>,
    /// 会话管理器
    session_manager: SessionManager,
    /// 预设管理器
    preset_manager: PresetManager,
}

impl DglabCli {
    /// 创建新的 CLI 应用
    pub async fn new() -> Result<Self> {
        let ble_manager = Arc::new(BleManager::new().await?);
        let session_manager = SessionManager::new();
        let mut preset_manager = PresetManager::default_dir()?;
        preset_manager.initialize().await?;

        Ok(Self {
            ble_manager,
            session_manager,
            preset_manager,
        })
    }

    /// 扫描设备
    pub async fn scan(&mut self, args: ScanArgs) -> Result<()> {
        scan::execute(self, args).await
    }

    /// 连接设备
    pub async fn connect(&mut self, args: ConnectArgs) -> Result<()> {
        connect::execute(self, args).await
    }

    /// 控制设备
    pub async fn control(&mut self, args: ControlArgs) -> Result<()> {
        control::execute(self, args).await
    }

    /// 预设管理
    pub async fn preset(&mut self, args: PresetArgs) -> Result<()> {
        preset::execute(self, args).await
    }

    /// 运行脚本
    pub async fn script(&mut self, args: ScriptArgs) -> Result<()> {
        script::execute(self, args).await
    }

    /// 运行 TUI
    pub async fn run_tui(&mut self) -> Result<()> {
        crate::tui::run(self).await
    }

    /// WiFi 命令
    pub async fn wifi(&mut self, args: WifiArgs) -> Result<()> {
        wifi::execute(self, args).await
    }

    /// 获取 BLE 管理器
    pub fn ble_manager(&self) -> &Arc<BleManager> {
        &self.ble_manager
    }

    /// 获取会话管理器
    pub fn session_manager(&self) -> &SessionManager {
        &self.session_manager
    }

    /// 获取预设管理器
    pub fn preset_manager(&self) -> &PresetManager {
        &self.preset_manager
    }

    /// 获取可变的预设管理器
    pub fn preset_manager_mut(&mut self) -> &mut PresetManager {
        &mut self.preset_manager
    }
}
