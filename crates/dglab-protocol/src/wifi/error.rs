//! WiFi WebSocket 错误类型

use thiserror::Error;

/// WebSocket 错误类型
#[derive(Error, Debug)]
pub enum WsError {
    /// 连接错误
    #[error("Connection error: {0}")]
    Connection(String),

    /// 协议错误
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// JSON 序列化/反序列化错误
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL 解析错误
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// WebSocket 协议错误
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// 发送错误
    #[error("Send error: {0}")]
    Send(String),

    /// 接收错误
    #[error("Receive error: {0}")]
    Receive(String),

    /// 未连接
    #[error("Not connected")]
    NotConnected,

    /// 已连接
    #[error("Already connected")]
    AlreadyConnected,

    /// 未绑定
    #[error("Not bound to target")]
    NotBound,

    /// 超时
    #[error("Timeout")]
    Timeout,

    /// 无效消息
    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    /// 其他错误
    #[error("Other error: {0}")]
    Other(String),
}

/// WebSocket Result 类型
pub type WsResult<T> = std::result::Result<T, WsError>;
