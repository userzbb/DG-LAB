//! CLI 错误类型

use thiserror::Error;

/// CLI 错误类型
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum CliError {
    /// 核心库错误
    #[error("Core error: {0}")]
    CoreError(#[from] dglab_core::error::CoreError),

    /// 协议库错误
    #[error("Protocol error: {0}")]
    ProtocolError(#[from] dglab_protocol::error::ProtocolError),

    /// IO 错误
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// 整数解析错误
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    /// 无效输入
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// 无设备连接
    #[error("No device connected")]
    NoDevice,

    /// 设备未找到
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    /// 其他错误
    #[error("Other error: {0}")]
    Other(String),
}

/// CLI Result 类型
pub type Result<T> = std::result::Result<T, CliError>;
