//! 设备事件定义

use serde::{Deserialize, Serialize};

use dglab_core::device::traits::DeviceInfo;
use dglab_core::device::DeviceState;

/// 设备状态变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStateChangedEvent {
    /// 设备 ID
    pub device_id: String,
    /// 新状态
    pub state: DeviceState,
}

/// 设备功率变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicePowerChangedEvent {
    /// 设备 ID
    pub device_id: String,
    /// 通道 A 功率
    pub power_a: u8,
    /// 通道 B 功率
    pub power_b: u8,
}

/// 设备信息更新事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DeviceInfoUpdatedEvent {
    /// 设备 ID
    pub device_id: String,
    /// 设备信息
    pub info: DeviceInfo,
}

/// 设备电池电量更新事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DeviceBatteryUpdatedEvent {
    /// 设备 ID
    pub device_id: String,
    /// 电池电量 (0-100)
    pub battery: u8,
}

/// 设备错误事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DeviceErrorEvent {
    /// 设备 ID
    pub device_id: String,
    /// 错误信息
    pub error: String,
}

/// 事件名称常量
#[allow(dead_code)]
pub mod event_names {
    /// 设备状态变更
    pub const DEVICE_STATE_CHANGED: &str = "device:state_changed";
    /// 设备功率变更
    pub const DEVICE_POWER_CHANGED: &str = "device:power_changed";
    /// 设备信息更新
    pub const DEVICE_INFO_UPDATED: &str = "device:info_updated";
    /// 设备电池电量更新
    pub const DEVICE_BATTERY_UPDATED: &str = "device:battery_updated";
    /// 设备错误
    pub const DEVICE_ERROR: &str = "device:error";
}
