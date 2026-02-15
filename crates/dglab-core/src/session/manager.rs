//! 会话管理器

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info, warn};

use crate::device::{Device, DeviceEvent, DeviceState};
use crate::error::{CoreError, Result};

/// 设备包装类型
type DeviceBox = Box<dyn Device>;
/// 设备映射
type DeviceMap = HashMap<String, Arc<RwLock<DeviceBox>>>;

/// 会话事件
#[derive(Debug, Clone)]
pub enum SessionEvent {
    /// 设备已添加
    DeviceAdded(String),
    /// 设备已移除
    DeviceRemoved(String),
    /// 设备连接状态变更
    DeviceStateChanged(String, DeviceState),
    /// 会话错误
    Error(String),
}

/// 会话信息
#[derive(Debug, Clone)]
pub struct SessionInfo {
    /// 会话 ID
    pub id: String,
    /// 会话创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 活动设备数量
    pub active_devices: usize,
    /// 总设备数量
    pub total_devices: usize,
}

/// 会话管理器
pub struct SessionManager {
    /// 会话 ID
    session_id: String,
    /// 设备集合
    devices: Arc<RwLock<DeviceMap>>,
    /// 事件发送器
    event_tx: broadcast::Sender<SessionEvent>,
    /// 创建时间
    created_at: chrono::DateTime<chrono::Utc>,
}

impl SessionManager {
    /// 创建新的会话管理器
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(32);

        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            devices: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            created_at: chrono::Utc::now(),
        }
    }

    /// 获取会话 ID
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// 获取会话信息
    pub async fn session_info(&self) -> SessionInfo {
        let devices = self.devices.read().await;
        let total_devices = devices.len();
        let mut active_devices = 0;

        for device in devices.values() {
            let dev = device.read().await;
            if dev.state() == DeviceState::Connected || dev.state() == DeviceState::Running {
                active_devices += 1;
            }
        }

        SessionInfo {
            id: self.session_id.clone(),
            created_at: self.created_at,
            active_devices,
            total_devices,
        }
    }

    /// 添加设备
    pub async fn add_device(&self, device: DeviceBox) -> Result<()> {
        let device_id = device.id().to_string();
        info!("Adding device: {}", device_id);

        let mut devices = self.devices.write().await;

        if devices.contains_key(&device_id) {
            return Err(CoreError::DeviceAlreadyExists(device_id));
        }

        // 订阅设备事件
        let mut events = device.subscribe_events();
        let event_tx = self.event_tx.clone();
        let device_id_clone = device_id.clone();

        tokio::spawn(async move {
            while let Ok(event) = events.recv().await {
                if let DeviceEvent::StateChanged(state) = event {
                    let _ = event_tx.send(SessionEvent::DeviceStateChanged(
                        device_id_clone.clone(),
                        state,
                    ));
                }
            }
        });

        devices.insert(device_id.clone(), Arc::new(RwLock::new(device)));
        let _ = self.event_tx.send(SessionEvent::DeviceAdded(device_id));

        Ok(())
    }

    /// 移除设备
    pub async fn remove_device(&self, device_id: &str) -> Result<()> {
        info!("Removing device: {}", device_id);

        let mut devices = self.devices.write().await;

        if let Some(device) = devices.remove(device_id) {
            let mut dev = device.write().await;
            let _ = dev.disconnect().await;
        }

        let _ = self
            .event_tx
            .send(SessionEvent::DeviceRemoved(device_id.to_string()));

        Ok(())
    }

    /// 获取设备
    pub async fn get_device(&self, device_id: &str) -> Option<Arc<RwLock<DeviceBox>>> {
        let devices = self.devices.read().await;
        devices.get(device_id).cloned()
    }

    /// 获取所有设备 ID
    pub async fn list_devices(&self) -> Vec<String> {
        let devices = self.devices.read().await;
        devices.keys().cloned().collect()
    }

    /// 连接所有设备
    pub async fn connect_all(&self) -> Result<()> {
        info!("Connecting all devices");

        let devices = self.devices.read().await;

        for (id, device) in devices.iter() {
            debug!("Connecting device: {}", id);
            let mut dev = device.write().await;
            if let Err(e) = dev.connect().await {
                warn!("Failed to connect device {}: {}", id, e);
            }
        }

        Ok(())
    }

    /// 断开所有设备
    pub async fn disconnect_all(&self) -> Result<()> {
        info!("Disconnecting all devices");

        let devices = self.devices.read().await;

        for (id, device) in devices.iter() {
            debug!("Disconnecting device: {}", id);
            let mut dev = device.write().await;
            if let Err(e) = dev.disconnect().await {
                warn!("Failed to disconnect device {}: {}", id, e);
            }
        }

        Ok(())
    }

    /// 启动所有设备
    pub async fn start_all(&self) -> Result<()> {
        info!("Starting all devices");

        let devices = self.devices.read().await;

        for (id, device) in devices.iter() {
            debug!("Starting device: {}", id);
            let mut dev = device.write().await;
            if let Err(e) = dev.start().await {
                warn!("Failed to start device {}: {}", id, e);
            }
        }

        Ok(())
    }

    /// 停止所有设备
    pub async fn stop_all(&self) -> Result<()> {
        info!("Stopping all devices");

        let devices = self.devices.read().await;

        for (id, device) in devices.iter() {
            debug!("Stopping device: {}", id);
            let mut dev = device.write().await;
            if let Err(e) = dev.stop().await {
                warn!("Failed to stop device {}: {}", id, e);
            }
        }

        Ok(())
    }

    /// 订阅会话事件
    pub fn subscribe_events(&self) -> broadcast::Receiver<SessionEvent> {
        self.event_tx.subscribe()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::traits::{DeviceInfo, WaveformConfig};

    /// 用于测试的 Mock 设备
    struct MockDevice {
        id: String,
        name: String,
        state: DeviceState,
        power_a: u8,
        power_b: u8,
        event_tx: broadcast::Sender<DeviceEvent>,
    }

    impl MockDevice {
        fn new(id: &str, name: &str) -> Self {
            let (event_tx, _) = broadcast::channel(32);
            Self {
                id: id.to_string(),
                name: name.to_string(),
                state: DeviceState::Disconnected,
                power_a: 0,
                power_b: 0,
                event_tx,
            }
        }
    }

    #[async_trait::async_trait]
    impl Device for MockDevice {
        fn id(&self) -> &str {
            &self.id
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn state(&self) -> DeviceState {
            self.state
        }

        fn info(&self) -> DeviceInfo {
            DeviceInfo {
                id: self.id.clone(),
                name: self.name.clone(),
                device_type: "mock".to_string(),
                firmware_version: "1.0".to_string(),
                hardware_version: "1.0".to_string(),
                battery_level: 100,
                power_a: self.power_a,
                power_b: self.power_b,
                max_power_a: 100,
                max_power_b: 100,
            }
        }

        async fn connect(&mut self) -> Result<()> {
            self.state = DeviceState::Connected;
            let _ = self
                .event_tx
                .send(DeviceEvent::StateChanged(DeviceState::Connected));
            Ok(())
        }

        async fn disconnect(&mut self) -> Result<()> {
            self.state = DeviceState::Disconnected;
            let _ = self
                .event_tx
                .send(DeviceEvent::StateChanged(DeviceState::Disconnected));
            Ok(())
        }

        async fn start(&mut self) -> Result<()> {
            self.state = DeviceState::Running;
            let _ = self
                .event_tx
                .send(DeviceEvent::StateChanged(DeviceState::Running));
            Ok(())
        }

        async fn stop(&mut self) -> Result<()> {
            self.state = DeviceState::Connected;
            let _ = self
                .event_tx
                .send(DeviceEvent::StateChanged(DeviceState::Connected));
            Ok(())
        }

        async fn set_power(&mut self, channel: u8, power: u8) -> Result<()> {
            match channel {
                0 => self.power_a = power,
                1 => self.power_b = power,
                _ => return Err(CoreError::InvalidParameter("Invalid channel".to_string())),
            }
            Ok(())
        }

        fn get_power(&self, channel: u8) -> u8 {
            match channel {
                0 => self.power_a,
                1 => self.power_b,
                _ => 0,
            }
        }

        async fn set_waveform(&mut self, _channel: u8, _waveform: WaveformConfig) -> Result<()> {
            Ok(())
        }

        async fn heartbeat(&mut self) -> Result<()> {
            Ok(())
        }

        fn subscribe_events(&self) -> broadcast::Receiver<DeviceEvent> {
            self.event_tx.subscribe()
        }
    }

    // === SessionManager 测试 ===

    #[test]
    fn test_session_manager_new() {
        let manager = SessionManager::new();
        assert!(!manager.session_id().is_empty());
    }

    #[test]
    fn test_session_manager_unique_ids() {
        let m1 = SessionManager::new();
        let m2 = SessionManager::new();
        assert_ne!(m1.session_id(), m2.session_id());
    }

    #[test]
    fn test_session_manager_default() {
        let manager = SessionManager::default();
        assert!(!manager.session_id().is_empty());
    }

    #[tokio::test]
    async fn test_add_device() {
        let manager = SessionManager::new();
        let device = Box::new(MockDevice::new("dev-1", "Test Device"));

        manager.add_device(device).await.unwrap();

        let devices = manager.list_devices().await;
        assert_eq!(devices.len(), 1);
        assert!(devices.contains(&"dev-1".to_string()));
    }

    #[tokio::test]
    async fn test_add_duplicate_device_fails() {
        let manager = SessionManager::new();
        let d1 = Box::new(MockDevice::new("dev-1", "Device 1"));
        let d2 = Box::new(MockDevice::new("dev-1", "Device 1 Copy"));

        manager.add_device(d1).await.unwrap();
        let result = manager.add_device(d2).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_device_emits_event() {
        let manager = SessionManager::new();
        let mut rx = manager.subscribe_events();
        let device = Box::new(MockDevice::new("dev-1", "Test"));

        manager.add_device(device).await.unwrap();

        let event = rx.try_recv().unwrap();
        if let SessionEvent::DeviceAdded(id) = event {
            assert_eq!(id, "dev-1");
        } else {
            panic!("Expected DeviceAdded event");
        }
    }

    #[tokio::test]
    async fn test_remove_device() {
        let manager = SessionManager::new();
        let device = Box::new(MockDevice::new("dev-1", "Test"));
        manager.add_device(device).await.unwrap();

        manager.remove_device("dev-1").await.unwrap();

        let devices = manager.list_devices().await;
        assert!(devices.is_empty());
    }

    #[tokio::test]
    async fn test_remove_device_emits_event() {
        let manager = SessionManager::new();
        let device = Box::new(MockDevice::new("dev-1", "Test"));
        manager.add_device(device).await.unwrap();

        let mut rx = manager.subscribe_events();
        manager.remove_device("dev-1").await.unwrap();

        let event = rx.try_recv().unwrap();
        if let SessionEvent::DeviceRemoved(id) = event {
            assert_eq!(id, "dev-1");
        } else {
            panic!("Expected DeviceRemoved event");
        }
    }

    #[tokio::test]
    async fn test_get_device() {
        let manager = SessionManager::new();
        let device = Box::new(MockDevice::new("dev-1", "Test"));
        manager.add_device(device).await.unwrap();

        let dev = manager.get_device("dev-1").await;
        assert!(dev.is_some());

        let dev = manager.get_device("nonexistent").await;
        assert!(dev.is_none());
    }

    #[tokio::test]
    async fn test_list_devices_multiple() {
        let manager = SessionManager::new();
        manager
            .add_device(Box::new(MockDevice::new("dev-1", "D1")))
            .await
            .unwrap();
        manager
            .add_device(Box::new(MockDevice::new("dev-2", "D2")))
            .await
            .unwrap();
        manager
            .add_device(Box::new(MockDevice::new("dev-3", "D3")))
            .await
            .unwrap();

        let devices = manager.list_devices().await;
        assert_eq!(devices.len(), 3);
    }

    #[tokio::test]
    async fn test_session_info_empty() {
        let manager = SessionManager::new();
        let info = manager.session_info().await;
        assert_eq!(info.total_devices, 0);
        assert_eq!(info.active_devices, 0);
        assert_eq!(info.id, manager.session_id());
    }

    #[tokio::test]
    async fn test_session_info_with_devices() {
        let manager = SessionManager::new();
        manager
            .add_device(Box::new(MockDevice::new("dev-1", "D1")))
            .await
            .unwrap();
        manager
            .add_device(Box::new(MockDevice::new("dev-2", "D2")))
            .await
            .unwrap();

        let info = manager.session_info().await;
        assert_eq!(info.total_devices, 2);
        // 未连接，所以 active 为 0
        assert_eq!(info.active_devices, 0);
    }

    #[tokio::test]
    async fn test_connect_all() {
        let manager = SessionManager::new();
        manager
            .add_device(Box::new(MockDevice::new("dev-1", "D1")))
            .await
            .unwrap();
        manager
            .add_device(Box::new(MockDevice::new("dev-2", "D2")))
            .await
            .unwrap();

        manager.connect_all().await.unwrap();

        // 验证设备已连接
        let dev = manager.get_device("dev-1").await.unwrap();
        let d = dev.read().await;
        assert_eq!(d.state(), DeviceState::Connected);
    }

    #[tokio::test]
    async fn test_disconnect_all() {
        let manager = SessionManager::new();
        manager
            .add_device(Box::new(MockDevice::new("dev-1", "D1")))
            .await
            .unwrap();

        manager.connect_all().await.unwrap();
        manager.disconnect_all().await.unwrap();

        let dev = manager.get_device("dev-1").await.unwrap();
        let d = dev.read().await;
        assert_eq!(d.state(), DeviceState::Disconnected);
    }

    #[tokio::test]
    async fn test_start_all() {
        let manager = SessionManager::new();
        manager
            .add_device(Box::new(MockDevice::new("dev-1", "D1")))
            .await
            .unwrap();

        manager.start_all().await.unwrap();

        let dev = manager.get_device("dev-1").await.unwrap();
        let d = dev.read().await;
        assert_eq!(d.state(), DeviceState::Running);
    }

    #[tokio::test]
    async fn test_stop_all() {
        let manager = SessionManager::new();
        manager
            .add_device(Box::new(MockDevice::new("dev-1", "D1")))
            .await
            .unwrap();

        manager.start_all().await.unwrap();
        manager.stop_all().await.unwrap();

        let dev = manager.get_device("dev-1").await.unwrap();
        let d = dev.read().await;
        assert_eq!(d.state(), DeviceState::Connected);
    }

    // === SessionEvent 测试 ===

    #[test]
    fn test_session_event_debug() {
        let event = SessionEvent::DeviceAdded("dev-1".to_string());
        let s = format!("{:?}", event);
        assert!(s.contains("DeviceAdded"));
    }

    #[test]
    fn test_session_event_clone() {
        let event = SessionEvent::Error("test".to_string());
        let cloned = event.clone();
        if let SessionEvent::Error(msg) = cloned {
            assert_eq!(msg, "test");
        } else {
            panic!("Expected Error");
        }
    }

    // === SessionInfo 测试 ===

    #[test]
    fn test_session_info_debug() {
        let info = SessionInfo {
            id: "test-id".to_string(),
            created_at: chrono::Utc::now(),
            active_devices: 1,
            total_devices: 2,
        };
        let s = format!("{:?}", info);
        assert!(s.contains("test-id"));
    }
}
