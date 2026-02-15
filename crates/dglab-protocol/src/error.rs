//! 错误类型定义

use thiserror::Error;

/// 协议库错误类型
#[derive(Error, Debug)]
pub enum ProtocolError {
    /// BLE 相关错误
    #[error("BLE error: {0}")]
    BleError(String),

    /// WiFi 相关错误
    #[error("WiFi error: {0}")]
    WifiError(String),

    /// 数据包编码错误
    #[error("Packet encoding error: {0}")]
    EncodeError(String),

    /// 数据包解码错误
    #[error("Packet decoding error: {0}")]
    DecodeError(String),

    /// 设备未找到
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    /// 连接错误
    #[error("Connection error: {0}")]
    ConnectionError(String),

    /// 超时错误
    #[error("Timeout error")]
    Timeout,

    /// IO 错误
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// 其他错误
    #[error("Other error: {0}")]
    Other(String),
}

/// 协议库 Result 类型
pub type Result<T> = std::result::Result<T, ProtocolError>;
