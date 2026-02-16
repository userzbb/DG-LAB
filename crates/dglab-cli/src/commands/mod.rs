//! CLI 命令实现

use std::sync::Arc;

use crate::error::Result;
use dglab_core::preset::PresetManager;
use dglab_core::session::SessionManager;
use dglab_protocol::ble::BleManager;

pub mod bridge;
pub mod connect;
pub mod control;
pub mod preset;
pub mod scan;
pub mod script;
pub mod wifi;

pub use bridge::BridgeArgs;
pub use connect::ConnectArgs;
pub use control::ControlArgs;
pub use preset::PresetArgs;
pub use scan::ScanArgs;
pub use script::ScriptArgs;
pub use wifi::WifiArgs;

/// CLI 应用
pub struct DglabCli {
    /// BLE 管理器（可选，延迟初始化）
    ble_manager: Option<Arc<BleManager>>,
    /// 会话管理器
    session_manager: SessionManager,
    /// 预设管理器
    preset_manager: PresetManager,
}

impl DglabCli {
    /// 创建新的 CLI 应用（不初始化 BLE）
    pub async fn new() -> Result<Self> {
        let session_manager = SessionManager::new();
        let mut preset_manager = PresetManager::default_dir()?;
        preset_manager.initialize().await?;

        Ok(Self {
            ble_manager: None,
            session_manager,
            preset_manager,
        })
    }

    /// 获取或初始化 BLE 管理器
    async fn get_or_init_ble(&mut self) -> Result<&Arc<BleManager>> {
        if self.ble_manager.is_none() {
            self.ble_manager = Some(Arc::new(BleManager::new().await?));
        }
        Ok(self.ble_manager.as_ref().unwrap())
    }

    /// 扫描设备
    pub async fn scan(&mut self, args: ScanArgs) -> Result<()> {
        // 延迟初始化 BLE
        self.get_or_init_ble().await?;
        scan::execute(self, args).await
    }

    /// 连接设备
    pub async fn connect(&mut self, args: ConnectArgs) -> Result<()> {
        // 延迟初始化 BLE
        self.get_or_init_ble().await?;
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

    /// 桥接模式
    pub async fn bridge(&mut self, args: BridgeArgs) -> Result<()> {
        // 延迟初始化 BLE
        self.get_or_init_ble().await?;
        bridge::execute(self, args).await
    }

    /// 获取 BLE 管理器
    pub fn ble_manager(&self) -> Option<&Arc<BleManager>> {
        self.ble_manager.as_ref()
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
