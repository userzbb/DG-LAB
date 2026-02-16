//! 模拟设备实现，用于测试和开发

use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info};

use super::traits::{Device, DeviceInfo, WaveformConfig};
use super::{DeviceEvent, DeviceState};
use crate::error::{CoreError, Result};

/// 模拟设备
///
/// 用于在没有真实硬件的情况下测试和开发
pub struct MockDevice {
    /// 设备 ID
    id: String,
    /// 设备名称
    name: String,
    /// 设备状态
    state: Arc<RwLock<DeviceState>>,
    /// 设备信息
    info: Arc<RwLock<DeviceInfo>>,
    /// 事件广播通道
    event_tx: broadcast::Sender<DeviceEvent>,
}

impl MockDevice {
    /// 创建新的模拟设备
    pub fn new(id: String, name: String) -> Self {
        let (event_tx, _) = broadcast::channel(100);

        let info = DeviceInfo {
            id: id.clone(),
            name: name.clone(),
            device_type: "mock".to_string(),
            firmware_version: "1.0.0".to_string(),
            hardware_version: "1.0.0".to_string(),
            battery_level: 100,
            power_a: 0,
            power_b: 0,
            max_power_a: 100,
            max_power_b: 100,
        };

        Self {
            id,
            name,
            state: Arc::new(RwLock::new(DeviceState::Disconnected)),
            info: Arc::new(RwLock::new(info)),
            event_tx,
        }
    }

    /// 模拟电池消耗
    async fn simulate_battery_drain(&self) {
        let mut info = self.info.write().await;
        if info.battery_level > 0 {
            info.battery_level = info.battery_level.saturating_sub(1);
            debug!("模拟设备电池: {}%", info.battery_level);
        }
    }

    /// 发送事件
    fn send_event(&self, event: DeviceEvent) {
        let _ = self.event_tx.send(event);
    }
}

#[async_trait]
impl Device for MockDevice {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn state(&self) -> DeviceState {
        // 这里使用 blocking，因为 trait 方法不是 async
        // 在实际使用中，外部应该持有 Arc<RwLock<Device>>
        futures::executor::block_on(async { *self.state.read().await })
    }

    fn info(&self) -> DeviceInfo {
        futures::executor::block_on(async { self.info.read().await.clone() })
    }

    async fn connect(&mut self) -> Result<()> {
        info!("模拟设备连接: {}", self.name);

        let mut state = self.state.write().await;
        *state = DeviceState::Connecting;
        self.send_event(DeviceEvent::StateChanged(DeviceState::Connecting));

        // 模拟连接延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        *state = DeviceState::Connected;
        self.send_event(DeviceEvent::StateChanged(DeviceState::Connected));

        info!("模拟设备已连接: {}", self.name);
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        info!("模拟设备断开: {}", self.name);

        let mut state = self.state.write().await;
        *state = DeviceState::Disconnected;
        self.send_event(DeviceEvent::StateChanged(DeviceState::Disconnected));

        Ok(())
    }

    async fn start(&mut self) -> Result<()> {
        let state = self.state.read().await;
        if *state != DeviceState::Connected {
            return Err(CoreError::DeviceNotConnected);
        }
        drop(state);

        info!("模拟设备开始输出: {}", self.name);
        self.send_event(DeviceEvent::Started);

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        let state = self.state.read().await;
        if *state != DeviceState::Connected {
            return Err(CoreError::DeviceNotConnected);
        }
        drop(state);

        info!("模拟设备停止输出: {}", self.name);
        self.send_event(DeviceEvent::Stopped);

        // 停止时重置强度
        let mut info = self.info.write().await;
        info.power_a = 0;
        info.power_b = 0;

        Ok(())
    }

    async fn set_power(&mut self, channel: u8, power: u8) -> Result<()> {
        let state = self.state.read().await;
        if *state != DeviceState::Connected {
            return Err(CoreError::DeviceNotConnected);
        }
        drop(state);

        let mut info = self.info.write().await;

        let max_power = match channel {
            0 => info.max_power_a,
            1 => info.max_power_b,
            _ => return Err(CoreError::InvalidChannel(channel)),
        };

        let clamped_power = power.min(max_power);

        match channel {
            0 => {
                debug!(
                    "模拟设备设置通道 A 强度: {} -> {}",
                    info.power_a, clamped_power
                );
                info.power_a = clamped_power;
            }
            1 => {
                debug!(
                    "模拟设备设置通道 B 强度: {} -> {}",
                    info.power_b, clamped_power
                );
                info.power_b = clamped_power;
            }
            _ => unreachable!(),
        }

        self.send_event(DeviceEvent::PowerChanged {
            channel,
            power: clamped_power,
        });

        // 模拟电池消耗
        drop(info);
        self.simulate_battery_drain().await;

        Ok(())
    }

    fn get_power(&self, channel: u8) -> u8 {
        let info = futures::executor::block_on(async { self.info.read().await.clone() });
        match channel {
            0 => info.power_a,
            1 => info.power_b,
            _ => 0,
        }
    }

    async fn set_waveform(&mut self, channel: u8, waveform: WaveformConfig) -> Result<()> {
        let state = self.state.read().await;
        if *state != DeviceState::Connected {
            return Err(CoreError::DeviceNotConnected);
        }
        drop(state);

        info!(
            "模拟设备设置通道 {} 波形: {:?}",
            channel, waveform.waveform_type
        );

        self.send_event(DeviceEvent::WaveformChanged { channel });

        Ok(())
    }

    async fn heartbeat(&mut self) -> Result<()> {
        let state = self.state.read().await;
        if *state != DeviceState::Connected {
            return Err(CoreError::DeviceNotConnected);
        }
        drop(state);

        debug!("模拟设备心跳: {}", self.name);
        self.send_event(DeviceEvent::Heartbeat);

        Ok(())
    }

    fn subscribe_events(&self) -> broadcast::Receiver<DeviceEvent> {
        self.event_tx.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_device_creation() {
        let device = MockDevice::new("mock-001".to_string(), "Test Device".to_string());

        assert_eq!(device.id(), "mock-001");
        assert_eq!(device.name(), "Test Device");
        assert_eq!(device.state(), DeviceState::Disconnected);

        let info = device.info();
        assert_eq!(info.device_type, "mock");
        assert_eq!(info.battery_level, 100);
        assert_eq!(info.power_a, 0);
        assert_eq!(info.power_b, 0);
    }

    #[tokio::test]
    async fn test_mock_device_connect_disconnect() {
        let mut device = MockDevice::new("mock-001".to_string(), "Test Device".to_string());

        // 初始状态
        assert_eq!(device.state(), DeviceState::Disconnected);

        // 连接
        device.connect().await.unwrap();
        assert_eq!(device.state(), DeviceState::Connected);

        // 断开
        device.disconnect().await.unwrap();
        assert_eq!(device.state(), DeviceState::Disconnected);
    }

    #[tokio::test]
    async fn test_mock_device_power_control() {
        let mut device = MockDevice::new("mock-001".to_string(), "Test Device".to_string());

        // 未连接时设置强度应该失败
        let result = device.set_power(0, 50).await;
        assert!(result.is_err());

        // 连接后设置强度
        device.connect().await.unwrap();
        device.set_power(0, 50).await.unwrap();
        assert_eq!(device.get_power(0), 50);

        device.set_power(1, 75).await.unwrap();
        assert_eq!(device.get_power(1), 75);

        // 超过最大值应该被限制
        device.set_power(0, 150).await.unwrap();
        assert_eq!(device.get_power(0), 100);
    }

    #[tokio::test]
    async fn test_mock_device_start_stop() {
        let mut device = MockDevice::new("mock-001".to_string(), "Test Device".to_string());

        // 未连接时启动应该失败
        let result = device.start().await;
        assert!(result.is_err());

        // 连接后启动
        device.connect().await.unwrap();
        device.set_power(0, 50).await.unwrap();

        device.start().await.unwrap();
        assert_eq!(device.get_power(0), 50);

        // 停止后强度应该归零
        device.stop().await.unwrap();
        assert_eq!(device.get_power(0), 0);
        assert_eq!(device.get_power(1), 0);
    }

    #[tokio::test]
    async fn test_mock_device_events() {
        let mut device = MockDevice::new("mock-001".to_string(), "Test Device".to_string());
        let mut rx = device.subscribe_events();

        // 连接事件
        device.connect().await.unwrap();

        let event = rx.recv().await.unwrap();
        assert!(matches!(
            event,
            DeviceEvent::StateChanged(DeviceState::Connecting)
        ));

        let event = rx.recv().await.unwrap();
        assert!(matches!(
            event,
            DeviceEvent::StateChanged(DeviceState::Connected)
        ));

        // 强度变化事件
        device.set_power(0, 50).await.unwrap();
        let event = rx.recv().await.unwrap();
        assert!(matches!(
            event,
            DeviceEvent::PowerChanged {
                channel: 0,
                power: 50
            }
        ));

        // 启动事件
        device.start().await.unwrap();
        let event = rx.recv().await.unwrap();
        assert!(matches!(event, DeviceEvent::Started));

        // 停止事件
        device.stop().await.unwrap();
        let event = rx.recv().await.unwrap();
        assert!(matches!(event, DeviceEvent::Stopped));
    }

    #[tokio::test]
    async fn test_mock_device_heartbeat() {
        let mut device = MockDevice::new("mock-001".to_string(), "Test Device".to_string());
        let mut rx = device.subscribe_events();

        device.connect().await.unwrap();
        // 跳过连接事件
        let _ = rx.recv().await;
        let _ = rx.recv().await;

        device.heartbeat().await.unwrap();
        let event = rx.recv().await.unwrap();
        assert!(matches!(event, DeviceEvent::Heartbeat));
    }

    #[tokio::test]
    async fn test_mock_device_invalid_channel() {
        let mut device = MockDevice::new("mock-001".to_string(), "Test Device".to_string());
        device.connect().await.unwrap();

        let result = device.set_power(2, 50).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CoreError::InvalidChannel(2)));
    }

    #[tokio::test]
    async fn test_mock_device_waveform() {
        let mut device = MockDevice::new("mock-001".to_string(), "Test Device".to_string());
        let mut rx = device.subscribe_events();

        device.connect().await.unwrap();
        // 跳过连接事件
        let _ = rx.recv().await;
        let _ = rx.recv().await;

        let waveform = WaveformConfig::default();
        device.set_waveform(0, waveform).await.unwrap();

        let event = rx.recv().await.unwrap();
        assert!(matches!(event, DeviceEvent::WaveformChanged { channel: 0 }));
    }
}
