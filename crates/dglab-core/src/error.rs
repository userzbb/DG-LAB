//! 错误类型定义

use thiserror::Error;

/// 核心库错误类型
#[derive(Error, Debug)]
pub enum CoreError {
    /// 协议错误
    #[error("Protocol error: {0}")]
    ProtocolError(#[from] dglab_protocol::error::ProtocolError),

    /// 设备未连接
    #[error("Device not connected")]
    DeviceNotConnected,

    /// 设备已存在
    #[error("Device already exists: {0}")]
    DeviceAlreadyExists(String),

    /// 设备不存在
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    /// 无效参数
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// 强度超出范围
    #[error("Power out of range: {0}, max: {1}")]
    PowerOutOfRange(u8, u8),

    /// 预设不存在
    #[error("Preset not found: {0}")]
    PresetNotFound(String),

    /// 预设已存在
    #[error("Preset already exists: {0}")]
    PresetAlreadyExists(String),

    /// 脚本错误
    #[error("Script error: {0}")]
    ScriptError(String),

    /// IO 错误
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// 序列化错误
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// 其他错误
    #[error("Other error: {0}")]
    Other(String),
}

/// 核心库 Result 类型
pub type Result<T> = std::result::Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_not_connected() {
        let err = CoreError::DeviceNotConnected;
        assert_eq!(err.to_string(), "Device not connected");
    }

    #[test]
    fn test_device_already_exists() {
        let err = CoreError::DeviceAlreadyExists("dev-1".to_string());
        assert!(err.to_string().contains("dev-1"));
    }

    #[test]
    fn test_device_not_found() {
        let err = CoreError::DeviceNotFound("dev-2".to_string());
        assert!(err.to_string().contains("dev-2"));
    }

    #[test]
    fn test_invalid_parameter() {
        let err = CoreError::InvalidParameter("bad param".to_string());
        assert!(err.to_string().contains("bad param"));
    }

    #[test]
    fn test_power_out_of_range() {
        let err = CoreError::PowerOutOfRange(150, 100);
        let msg = err.to_string();
        assert!(msg.contains("150"));
        assert!(msg.contains("100"));
    }

    #[test]
    fn test_preset_not_found() {
        let err = CoreError::PresetNotFound("preset-1".to_string());
        assert!(err.to_string().contains("preset-1"));
    }

    #[test]
    fn test_preset_already_exists() {
        let err = CoreError::PresetAlreadyExists("preset-1".to_string());
        assert!(err.to_string().contains("preset-1"));
    }

    #[test]
    fn test_script_error() {
        let err = CoreError::ScriptError("script failed".to_string());
        assert!(err.to_string().contains("script failed"));
    }

    #[test]
    fn test_other_error() {
        let err = CoreError::Other("something".to_string());
        assert!(err.to_string().contains("something"));
    }

    #[test]
    fn test_io_error_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = CoreError::from(io_err);
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn test_serde_error_from() {
        let serde_err = serde_json::from_str::<String>("not json").unwrap_err();
        let err = CoreError::from(serde_err);
        assert!(err.to_string().contains("Serialization error"));
    }

    #[test]
    fn test_error_debug() {
        let err = CoreError::DeviceNotConnected;
        let debug = format!("{:?}", err);
        assert!(debug.contains("DeviceNotConnected"));
    }
}
