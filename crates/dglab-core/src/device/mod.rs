//! 设备模块
//!
//! 提供设备抽象 trait 和具体实现。

pub mod bridge;
pub mod coyote;
pub mod traits;

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tracing::debug;

pub use bridge::BleWsBridgeDevice;
pub use coyote::{CoyoteDevice, WsCoyoteDevice};
pub use traits::{Device, DeviceConfig};

/// 设备状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceState {
    /// 已断开
    Disconnected,
    /// 连接中
    Connecting,
    /// 已连接
    Connected,
    /// 运行中
    Running,
    /// 错误
    Error,
}

/// 设备事件
#[derive(Debug, Clone)]
pub enum DeviceEvent {
    /// 状态变更
    StateChanged(DeviceState),
    /// 强度变更 (通道 A, 通道 B)
    PowerChanged(u8, u8),
    /// 设备信息更新
    InfoUpdated(crate::device::traits::DeviceInfo),
    /// 电池电量更新
    BatteryUpdated(u8),
    /// 错误
    Error(String),
}

/// 基础设备实现
pub struct BaseDevice {
    /// 设备 ID
    id: String,
    /// 设备名称
    name: String,
    /// 设备状态
    state: DeviceState,
    /// 通道 A 强度
    power_a: u8,
    /// 通道 B 强度
    power_b: u8,
    /// 通道 A 最大强度
    max_power_a: u8,
    /// 通道 B 最大强度
    max_power_b: u8,
    /// 事件发送器
    event_tx: broadcast::Sender<DeviceEvent>,
}

impl BaseDevice {
    /// 创建新的基础设备
    pub fn new(id: String, name: String) -> Self {
        let (event_tx, _) = broadcast::channel(32);

        Self {
            id,
            name,
            state: DeviceState::Disconnected,
            power_a: 0,
            power_b: 0,
            max_power_a: 100,
            max_power_b: 100,
            event_tx,
        }
    }

    /// 获取设备 ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 获取设备名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取设备状态
    pub fn state(&self) -> DeviceState {
        self.state
    }

    /// 设置设备状态
    pub fn set_state(&mut self, state: DeviceState) {
        if self.state != state {
            debug!(
                "Device {} state changed: {:?} -> {:?}",
                self.id, self.state, state
            );
            self.state = state;
            let _ = self.event_tx.send(DeviceEvent::StateChanged(state));
        }
    }

    /// 获取通道 A 强度
    pub fn power_a(&self) -> u8 {
        self.power_a
    }

    /// 获取通道 B 强度
    pub fn power_b(&self) -> u8 {
        self.power_b
    }

    /// 设置通道强度
    pub fn set_power(&mut self, channel: u8, power: u8) -> crate::Result<()> {
        let max_power = match channel {
            0 => self.max_power_a,
            1 => self.max_power_b,
            _ => {
                return Err(crate::CoreError::InvalidParameter(
                    "Invalid channel".to_string(),
                ))
            }
        };

        if power > max_power {
            return Err(crate::CoreError::PowerOutOfRange(power, max_power));
        }

        match channel {
            0 => self.power_a = power,
            1 => self.power_b = power,
            _ => {}
        }

        let _ = self
            .event_tx
            .send(DeviceEvent::PowerChanged(self.power_a, self.power_b));
        Ok(())
    }

    /// 获取事件接收器
    pub fn subscribe_events(&self) -> broadcast::Receiver<DeviceEvent> {
        self.event_tx.subscribe()
    }

    /// 发送事件
    pub fn send_event(&self, event: DeviceEvent) {
        let _ = self.event_tx.send(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === DeviceState 测试 ===

    #[test]
    fn test_device_state_equality() {
        assert_eq!(DeviceState::Disconnected, DeviceState::Disconnected);
        assert_ne!(DeviceState::Disconnected, DeviceState::Connected);
    }

    #[test]
    fn test_device_state_clone() {
        let state = DeviceState::Running;
        let cloned = state;
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_device_state_debug() {
        let s = format!("{:?}", DeviceState::Connecting);
        assert_eq!(s, "Connecting");
    }

    // === DeviceEvent 测试 ===

    #[test]
    fn test_device_event_state_changed() {
        let event = DeviceEvent::StateChanged(DeviceState::Connected);
        if let DeviceEvent::StateChanged(state) = event {
            assert_eq!(state, DeviceState::Connected);
        } else {
            panic!("Expected StateChanged");
        }
    }

    #[test]
    fn test_device_event_power_changed() {
        let event = DeviceEvent::PowerChanged(50, 60);
        if let DeviceEvent::PowerChanged(a, b) = event {
            assert_eq!(a, 50);
            assert_eq!(b, 60);
        } else {
            panic!("Expected PowerChanged");
        }
    }

    #[test]
    fn test_device_event_battery_updated() {
        let event = DeviceEvent::BatteryUpdated(85);
        if let DeviceEvent::BatteryUpdated(level) = event {
            assert_eq!(level, 85);
        } else {
            panic!("Expected BatteryUpdated");
        }
    }

    #[test]
    fn test_device_event_error() {
        let event = DeviceEvent::Error("test error".to_string());
        if let DeviceEvent::Error(msg) = event {
            assert_eq!(msg, "test error");
        } else {
            panic!("Expected Error");
        }
    }

    #[test]
    fn test_device_event_clone() {
        let event = DeviceEvent::PowerChanged(10, 20);
        let cloned = event.clone();
        if let DeviceEvent::PowerChanged(a, b) = cloned {
            assert_eq!(a, 10);
            assert_eq!(b, 20);
        } else {
            panic!("Expected PowerChanged");
        }
    }

    // === BaseDevice 测试 ===

    #[test]
    fn test_base_device_new() {
        let dev = BaseDevice::new("dev-1".to_string(), "Test Device".to_string());
        assert_eq!(dev.id(), "dev-1");
        assert_eq!(dev.name(), "Test Device");
        assert_eq!(dev.state(), DeviceState::Disconnected);
        assert_eq!(dev.power_a(), 0);
        assert_eq!(dev.power_b(), 0);
    }

    #[test]
    fn test_base_device_set_state() {
        let mut dev = BaseDevice::new("dev-1".to_string(), "Test".to_string());
        dev.set_state(DeviceState::Connected);
        assert_eq!(dev.state(), DeviceState::Connected);
    }

    #[test]
    fn test_base_device_set_state_emits_event() {
        let mut dev = BaseDevice::new("dev-1".to_string(), "Test".to_string());
        let mut rx = dev.subscribe_events();

        dev.set_state(DeviceState::Connected);

        let event = rx.try_recv().unwrap();
        if let DeviceEvent::StateChanged(state) = event {
            assert_eq!(state, DeviceState::Connected);
        } else {
            panic!("Expected StateChanged");
        }
    }

    #[test]
    fn test_base_device_set_state_same_no_event() {
        let mut dev = BaseDevice::new("dev-1".to_string(), "Test".to_string());
        // 初始状态是 Disconnected，再次设置相同状态不应触发事件
        let mut rx = dev.subscribe_events();
        dev.set_state(DeviceState::Disconnected);
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn test_base_device_set_power_a() {
        let mut dev = BaseDevice::new("dev-1".to_string(), "Test".to_string());
        dev.set_power(0, 50).unwrap();
        assert_eq!(dev.power_a(), 50);
        assert_eq!(dev.power_b(), 0);
    }

    #[test]
    fn test_base_device_set_power_b() {
        let mut dev = BaseDevice::new("dev-1".to_string(), "Test".to_string());
        dev.set_power(1, 75).unwrap();
        assert_eq!(dev.power_a(), 0);
        assert_eq!(dev.power_b(), 75);
    }

    #[test]
    fn test_base_device_set_power_invalid_channel() {
        let mut dev = BaseDevice::new("dev-1".to_string(), "Test".to_string());
        let result = dev.set_power(2, 50);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid channel"));
    }

    #[test]
    fn test_base_device_set_power_exceeds_max() {
        let mut dev = BaseDevice::new("dev-1".to_string(), "Test".to_string());
        // max_power_a 默认 100
        let result = dev.set_power(0, 101);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("out of range"));
    }

    #[test]
    fn test_base_device_set_power_at_max() {
        let mut dev = BaseDevice::new("dev-1".to_string(), "Test".to_string());
        dev.set_power(0, 100).unwrap();
        assert_eq!(dev.power_a(), 100);
    }

    #[test]
    fn test_base_device_set_power_emits_event() {
        let mut dev = BaseDevice::new("dev-1".to_string(), "Test".to_string());
        let mut rx = dev.subscribe_events();

        dev.set_power(0, 30).unwrap();

        let event = rx.try_recv().unwrap();
        if let DeviceEvent::PowerChanged(a, b) = event {
            assert_eq!(a, 30);
            assert_eq!(b, 0);
        } else {
            panic!("Expected PowerChanged");
        }
    }

    #[test]
    fn test_base_device_send_event() {
        let dev = BaseDevice::new("dev-1".to_string(), "Test".to_string());
        let mut rx = dev.subscribe_events();

        dev.send_event(DeviceEvent::BatteryUpdated(42));

        let event = rx.try_recv().unwrap();
        if let DeviceEvent::BatteryUpdated(level) = event {
            assert_eq!(level, 42);
        } else {
            panic!("Expected BatteryUpdated");
        }
    }

    #[test]
    fn test_base_device_multiple_subscribers() {
        let mut dev = BaseDevice::new("dev-1".to_string(), "Test".to_string());
        let mut rx1 = dev.subscribe_events();
        let mut rx2 = dev.subscribe_events();

        dev.set_state(DeviceState::Running);

        assert!(rx1.try_recv().is_ok());
        assert!(rx2.try_recv().is_ok());
    }
}
