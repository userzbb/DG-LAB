//! 设备 trait 定义

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use super::{DeviceEvent, DeviceState};
use crate::error::Result;

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备 ID
    pub id: String,
    /// 设备名称
    pub name: String,
    /// 设备类型
    pub device_type: String,
    /// 固件版本
    pub firmware_version: String,
    /// 硬件版本
    pub hardware_version: String,
    /// 电池电量 (0-100)
    pub battery_level: u8,
    /// 通道 A 当前强度
    pub power_a: u8,
    /// 通道 B 当前强度
    pub power_b: u8,
    /// 通道 A 最大强度
    pub max_power_a: u8,
    /// 通道 B 最大强度
    pub max_power_b: u8,
}

/// 设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// 设备 ID
    pub id: String,
    /// 设备名称
    pub name: String,
    /// 连接类型 (ble/wifi)
    pub connection_type: String,
    /// 连接地址
    pub address: Option<String>,
    /// 自动重连
    pub auto_reconnect: bool,
    /// 安全限制（最大强度）
    pub safety_limit: Option<u8>,
}

/// 设备 trait
#[async_trait]
pub trait Device: Send + Sync {
    /// 获取设备 ID
    fn id(&self) -> &str;

    /// 获取设备名称
    fn name(&self) -> &str;

    /// 获取设备状态
    fn state(&self) -> DeviceState;

    /// 获取设备信息
    fn info(&self) -> DeviceInfo;

    /// 连接设备
    async fn connect(&mut self) -> Result<()>;

    /// 断开设备
    async fn disconnect(&mut self) -> Result<()>;

    /// 开始输出
    async fn start(&mut self) -> Result<()>;

    /// 停止输出
    async fn stop(&mut self) -> Result<()>;

    /// 设置通道强度
    async fn set_power(&mut self, channel: u8, power: u8) -> Result<()>;

    /// 获取通道强度
    fn get_power(&self, channel: u8) -> u8;

    /// 设置波形
    async fn set_waveform(&mut self, channel: u8, waveform: WaveformConfig) -> Result<()>;

    /// 发送心跳
    async fn heartbeat(&mut self) -> Result<()>;

    /// 订阅设备事件
    fn subscribe_events(&self) -> broadcast::Receiver<DeviceEvent>;
}

/// 波形配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveformConfig {
    /// 波形类型
    pub waveform_type: WaveformType,
    /// 频率 (Hz)
    pub frequency: u16,
    /// 脉宽 (微秒)
    pub pulse_width: u16,
    /// 强度 (0-100)
    pub intensity: u8,
    /// 自定义波形数据
    pub custom_data: Option<Vec<u8>>,
}

impl Default for WaveformConfig {
    fn default() -> Self {
        Self {
            waveform_type: WaveformType::Continuous,
            frequency: 100,
            pulse_width: 200,
            intensity: 50,
            custom_data: None,
        }
    }
}

/// 波形类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaveformType {
    /// 连续波
    Continuous,
    /// 脉冲波
    Pulse,
    /// 锯齿波
    Sawtooth,
    /// 正弦波
    Sine,
    /// 方波
    Square,
    /// 三角波
    Triangle,
    /// 自定义
    Custom,
}

#[cfg(test)]
mod tests {
    use super::*;

    // === DeviceInfo 测试 ===

    #[test]
    fn test_device_info_serde_roundtrip() {
        let info = DeviceInfo {
            id: "dev-1".to_string(),
            name: "Test Device".to_string(),
            device_type: "coyote_v3".to_string(),
            firmware_version: "1.2.3".to_string(),
            hardware_version: "2.0".to_string(),
            battery_level: 85,
            power_a: 30,
            power_b: 40,
            max_power_a: 100,
            max_power_b: 100,
        };

        let json = serde_json::to_string(&info).unwrap();
        let restored: DeviceInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.id, "dev-1");
        assert_eq!(restored.name, "Test Device");
        assert_eq!(restored.device_type, "coyote_v3");
        assert_eq!(restored.firmware_version, "1.2.3");
        assert_eq!(restored.hardware_version, "2.0");
        assert_eq!(restored.battery_level, 85);
        assert_eq!(restored.power_a, 30);
        assert_eq!(restored.power_b, 40);
    }

    #[test]
    fn test_device_info_clone() {
        let info = DeviceInfo {
            id: "dev-1".to_string(),
            name: "Test".to_string(),
            device_type: "ble".to_string(),
            firmware_version: "1.0".to_string(),
            hardware_version: "1.0".to_string(),
            battery_level: 50,
            power_a: 0,
            power_b: 0,
            max_power_a: 100,
            max_power_b: 100,
        };

        let cloned = info.clone();
        assert_eq!(cloned.id, info.id);
        assert_eq!(cloned.name, info.name);
    }

    // === DeviceConfig 测试 ===

    #[test]
    fn test_device_config_serde_roundtrip() {
        let config = DeviceConfig {
            id: "cfg-1".to_string(),
            name: "My Device".to_string(),
            connection_type: "ble".to_string(),
            address: Some("AA:BB:CC:DD:EE:FF".to_string()),
            auto_reconnect: true,
            safety_limit: Some(80),
        };

        let json = serde_json::to_string(&config).unwrap();
        let restored: DeviceConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.id, "cfg-1");
        assert_eq!(restored.connection_type, "ble");
        assert_eq!(restored.address, Some("AA:BB:CC:DD:EE:FF".to_string()));
        assert!(restored.auto_reconnect);
        assert_eq!(restored.safety_limit, Some(80));
    }

    #[test]
    fn test_device_config_optional_fields() {
        let config = DeviceConfig {
            id: "cfg-2".to_string(),
            name: "Wireless".to_string(),
            connection_type: "wifi".to_string(),
            address: None,
            auto_reconnect: false,
            safety_limit: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        let restored: DeviceConfig = serde_json::from_str(&json).unwrap();

        assert!(restored.address.is_none());
        assert!(restored.safety_limit.is_none());
        assert!(!restored.auto_reconnect);
    }

    // === WaveformConfig 测试 ===

    #[test]
    fn test_waveform_config_default() {
        let config = WaveformConfig::default();
        assert_eq!(config.waveform_type, WaveformType::Continuous);
        assert_eq!(config.frequency, 100);
        assert_eq!(config.pulse_width, 200);
        assert_eq!(config.intensity, 50);
        assert!(config.custom_data.is_none());
    }

    #[test]
    fn test_waveform_config_serde_roundtrip() {
        let config = WaveformConfig {
            waveform_type: WaveformType::Sine,
            frequency: 200,
            pulse_width: 150,
            intensity: 75,
            custom_data: Some(vec![1, 2, 3, 4]),
        };

        let json = serde_json::to_string(&config).unwrap();
        let restored: WaveformConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.waveform_type, WaveformType::Sine);
        assert_eq!(restored.frequency, 200);
        assert_eq!(restored.pulse_width, 150);
        assert_eq!(restored.intensity, 75);
        assert_eq!(restored.custom_data, Some(vec![1, 2, 3, 4]));
    }

    // === WaveformType 测试 ===

    #[test]
    fn test_waveform_type_equality() {
        assert_eq!(WaveformType::Pulse, WaveformType::Pulse);
        assert_ne!(WaveformType::Pulse, WaveformType::Sine);
    }

    #[test]
    fn test_waveform_type_all_variants() {
        let types = [
            WaveformType::Continuous,
            WaveformType::Pulse,
            WaveformType::Sawtooth,
            WaveformType::Sine,
            WaveformType::Square,
            WaveformType::Triangle,
            WaveformType::Custom,
        ];
        assert_eq!(types.len(), 7);
        // 确认每个变体可以序列化
        for wt in &types {
            let json = serde_json::to_string(wt).unwrap();
            let restored: WaveformType = serde_json::from_str(&json).unwrap();
            assert_eq!(*wt, restored);
        }
    }
}
