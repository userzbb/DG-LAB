//! WiFi WebSocket 通信模块
//!
//! 提供与 DG-LAB APP 通过 WebSocket 进行通信的功能。
//!
//! # 示例
//!
//! ```no_run
//! use dglab_protocol::wifi::{WsClient, WsEvent, StrengthOperation, Channel};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut client = WsClient::connect_official().await?;
//!
//! // 等待获取 clientId
//! let client_id = loop {
//!     if let Some(event) = client.recv_event().await? {
//!         match event {
//!             WsEvent::ClientId(id) => break id,
//!             _ => continue,
//!         }
//!     }
//! };
//!
//! println!("Client ID: {}", client_id);
//! println!("QR URL: {}", client.official_qr_url().await.unwrap());
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};

pub use client::WsClient;
pub use error::{WsError, WsResult};
pub use server::{ServerEvent, WsServer};

mod client;
mod error;
mod server;

/// 官方 WebSocket 服务器地址
pub const OFFICIAL_SERVER: &str = "wss://ws.dungeon-lab.cn";

/// 心跳间隔（秒）- 根据 hyperzlib 项目实现
pub const HEARTBEAT_INTERVAL: u64 = 20;

/// 心跳超时（秒）- 根据 hyperzlib 项目实现
pub const HEARTBEAT_TIMEOUT: u64 = 20;

/// 返回码 (RetCode) - 根据 hyperzlib 项目实现
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetCode {
    /// 成功
    #[serde(rename = "200")]
    Success,
    /// 客户端断开连接
    #[serde(rename = "209")]
    ClientDisconnected,
    /// 无效的客户端ID
    #[serde(rename = "210")]
    InvalidClientId,
    /// 服务器延迟（超时）
    #[serde(rename = "211")]
    ServerDelay,
    /// ID 已被绑定
    #[serde(rename = "400")]
    IdAlreadyBound,
    /// 目标客户端未找到
    #[serde(rename = "401")]
    TargetClientNotFound,
    /// 不兼容的关系
    #[serde(rename = "402")]
    IncompatibleRelationship,
    /// 非 JSON 内容
    #[serde(rename = "403")]
    NonJsonContent,
    /// 接收者未找到
    #[serde(rename = "404")]
    RecipientNotFound,
    /// 消息过长
    #[serde(rename = "405")]
    MessageTooLong,
    /// 服务器内部错误
    #[serde(rename = "500")]
    ServerInternalError,
}

impl RetCode {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            RetCode::Success => "200",
            RetCode::ClientDisconnected => "209",
            RetCode::InvalidClientId => "210",
            RetCode::ServerDelay => "211",
            RetCode::IdAlreadyBound => "400",
            RetCode::TargetClientNotFound => "401",
            RetCode::IncompatibleRelationship => "402",
            RetCode::NonJsonContent => "403",
            RetCode::RecipientNotFound => "404",
            RetCode::MessageTooLong => "405",
            RetCode::ServerInternalError => "500",
        }
    }

    /// 从字符串解析（不推荐直接使用，请使用 `str::parse()`）
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}

impl std::str::FromStr for RetCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "200" => Ok(RetCode::Success),
            "209" => Ok(RetCode::ClientDisconnected),
            "210" => Ok(RetCode::InvalidClientId),
            "211" => Ok(RetCode::ServerDelay),
            "400" => Ok(RetCode::IdAlreadyBound),
            "401" => Ok(RetCode::TargetClientNotFound),
            "402" => Ok(RetCode::IncompatibleRelationship),
            "403" => Ok(RetCode::NonJsonContent),
            "404" => Ok(RetCode::RecipientNotFound),
            "405" => Ok(RetCode::MessageTooLong),
            "500" => Ok(RetCode::ServerInternalError),
            _ => Err(()),
        }
    }
}

/// 消息类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    /// 心跳包
    Heartbeat,
    /// 关系绑定
    Bind,
    /// 数据指令
    Msg,
    /// 连接断开
    Break,
    /// 服务错误
    Error,
    /// 未知类型
    Unknown(String),
}

impl From<&str> for MessageType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "heartbeat" => MessageType::Heartbeat,
            "bind" => MessageType::Bind,
            "msg" => MessageType::Msg,
            "break" => MessageType::Break,
            "error" => MessageType::Error,
            _ => MessageType::Unknown(s.to_string()),
        }
    }
}

impl From<MessageType> for String {
    fn from(t: MessageType) -> String {
        match t {
            MessageType::Heartbeat => "heartbeat".to_string(),
            MessageType::Bind => "bind".to_string(),
            MessageType::Msg => "msg".to_string(),
            MessageType::Break => "break".to_string(),
            MessageType::Error => "error".to_string(),
            MessageType::Unknown(s) => s,
        }
    }
}

/// 消息数据头 (MessageDataHead) - 根据 hyperzlib 项目实现
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageDataHead {
    /// 目标ID
    TargetId,
    /// DG-Lab 标识
    DgLab,
    /// 强度控制
    Strength,
    /// 波形数据
    Pulse,
    /// 清除波形
    Clear,
    /// 按钮反馈
    Feedback,
}

impl MessageDataHead {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageDataHead::TargetId => "targetId",
            MessageDataHead::DgLab => "DGLAB",
            MessageDataHead::Strength => "strength",
            MessageDataHead::Pulse => "pulse",
            MessageDataHead::Clear => "clear",
            MessageDataHead::Feedback => "feedback",
        }
    }

    /// 从字符串解析（不推荐直接使用，请使用 `str::parse()`）
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}

impl std::str::FromStr for MessageDataHead {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "targetId" => Ok(MessageDataHead::TargetId),
            "DGLAB" => Ok(MessageDataHead::DgLab),
            "strength" => Ok(MessageDataHead::Strength),
            "pulse" => Ok(MessageDataHead::Pulse),
            "clear" => Ok(MessageDataHead::Clear),
            "feedback" => Ok(MessageDataHead::Feedback),
            _ => Err(()),
        }
    }
}

impl From<MessageDataHead> for String {
    fn from(head: MessageDataHead) -> String {
        head.as_str().to_string()
    }
}

/// WebSocket 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    /// 指令类型
    #[serde(rename = "type")]
    pub msg_type: String,
    /// 发送方客户端 ID
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// 接收方客户端 ID
    #[serde(rename = "targetId")]
    pub target_id: String,
    /// 消息内容
    pub message: String,
}

impl WsMessage {
    /// 创建新消息
    pub fn new(
        msg_type: MessageType,
        client_id: impl Into<String>,
        target_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            msg_type: msg_type.into(),
            client_id: client_id.into(),
            target_id: target_id.into(),
            message: message.into(),
        }
    }

    /// 获取消息类型
    pub fn message_type(&self) -> MessageType {
        MessageType::from(self.msg_type.as_str())
    }

    /// 判断是否是心跳消息
    pub fn is_heartbeat(&self) -> bool {
        matches!(self.message_type(), MessageType::Heartbeat)
    }

    /// 判断是否是绑定消息
    pub fn is_bind(&self) -> bool {
        matches!(self.message_type(), MessageType::Bind)
    }

    /// 判断是否是数据消息
    pub fn is_msg(&self) -> bool {
        matches!(self.message_type(), MessageType::Msg)
    }

    /// 判断是否是断开消息
    pub fn is_break(&self) -> bool {
        matches!(self.message_type(), MessageType::Break)
    }

    /// 判断是否是错误消息
    pub fn is_error(&self) -> bool {
        matches!(self.message_type(), MessageType::Error)
    }
}

/// 强度数据（从 APP 接收）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrengthData {
    /// A 通道当前强度
    pub strength_a: u8,
    /// B 通道当前强度
    pub strength_b: u8,
    /// A 通道最大强度
    pub max_a: u8,
    /// B 通道最大强度
    pub max_b: u8,
}

impl StrengthData {
    /// 从消息字符串解析
    pub fn parse(message: &str) -> Option<Self> {
        if !message.starts_with("strength-") {
            return None;
        }

        let parts: Vec<&str> = message.trim_start_matches("strength-").split('+').collect();
        if parts.len() != 4 {
            return None;
        }

        Some(Self {
            strength_a: parts[0].parse().ok()?,
            strength_b: parts[1].parse().ok()?,
            max_a: parts[2].parse().ok()?,
            max_b: parts[3].parse().ok()?,
        })
    }
}

/// 通道选择
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// A 通道
    A,
    /// B 通道
    B,
}

/// 强度操作模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrengthMode {
    /// 减少
    Decrease,
    /// 增加
    Increase,
    /// 指定数值
    Set,
}

/// 强度操作（发送到 APP）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrengthOperation {
    /// 通道
    pub channel: Channel,
    /// 操作模式
    pub mode: StrengthMode,
    /// 数值
    pub value: u8,
}

impl StrengthOperation {
    /// 创建新的强度操作
    pub fn new(channel: Channel, mode: StrengthMode, value: u8) -> Self {
        Self {
            channel,
            mode,
            value,
        }
    }

    /// 增加强度
    pub fn increase(channel: Channel, value: u8) -> Self {
        Self::new(channel, StrengthMode::Increase, value)
    }

    /// 减少强度
    pub fn decrease(channel: Channel, value: u8) -> Self {
        Self::new(channel, StrengthMode::Decrease, value)
    }

    /// 设置强度
    pub fn set(channel: Channel, value: u8) -> Self {
        Self::new(channel, StrengthMode::Set, value)
    }

    /// 转换为消息字符串
    pub fn to_message(&self) -> String {
        let channel = match self.channel {
            Channel::A => 1,
            Channel::B => 2,
        };
        let mode = match self.mode {
            StrengthMode::Decrease => 0,
            StrengthMode::Increase => 1,
            StrengthMode::Set => 2,
        };
        format!("strength-{channel}+{mode}+{}", self.value)
    }
}

/// 波形数据
#[derive(Debug, Clone)]
pub struct PulseData {
    /// 通道
    pub channel: Channel,
    /// 波形数据数组（8字节HEX格式，每条100ms）
    pub pulses: Vec<String>,
}

impl PulseData {
    /// 创建新的波形数据
    pub fn new(channel: Channel, pulses: Vec<String>) -> Self {
        Self { channel, pulses }
    }

    /// 从强度值创建简单波形
    pub fn from_strength(
        channel: Channel,
        strength_a: u8,
        strength_b: u8,
        duration_ms: u32,
    ) -> Self {
        let count = (duration_ms / 100).clamp(1, 100) as usize;
        // 简单波形格式: [X, X, A, X, X, B, X, X]
        let pulse = format!(
            "0101{:02x}0101{:02x}0101",
            strength_a.min(100),
            strength_b.min(100)
        );
        Self {
            channel,
            pulses: vec![pulse; count],
        }
    }

    /// 转换为消息字符串
    ///
    /// 生成符合协议的波形指令格式：`pulse-A:["hex","hex",...]`
    /// 每条 hex 数据为 8 字节（16 个十六进制字符），代表 100ms 的波形数据。
    pub fn to_message(&self) -> String {
        let channel = match self.channel {
            Channel::A => "A",
            Channel::B => "B",
        };
        let quoted_pulses: Vec<String> = self.pulses.iter().map(|p| format!("\"{p}\"")).collect();
        let pulses = quoted_pulses.join(",");
        format!("pulse-{channel}:[{pulses}]")
    }
}

/// 清空队列操作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClearOperation {
    /// 通道
    pub channel: Channel,
}

impl ClearOperation {
    /// 创建新的清空操作
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    /// 转换为消息字符串
    pub fn to_message(&self) -> String {
        let channel = match self.channel {
            Channel::A => 1,
            Channel::B => 2,
        };
        format!("clear-{channel}")
    }
}

/// APP 反馈按钮
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedbackButton {
    /// A 通道按钮 0
    A0,
    /// A 通道按钮 1
    A1,
    /// A 通道按钮 2
    A2,
    /// A 通道按钮 3
    A3,
    /// A 通道按钮 4
    A4,
    /// B 通道按钮 0
    B0,
    /// B 通道按钮 1
    B1,
    /// B 通道按钮 2
    B2,
    /// B 通道按钮 3
    B3,
    /// B 通道按钮 4
    B4,
}

impl FeedbackButton {
    /// 从索引解析
    pub fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::A0),
            1 => Some(Self::A1),
            2 => Some(Self::A2),
            3 => Some(Self::A3),
            4 => Some(Self::A4),
            5 => Some(Self::B0),
            6 => Some(Self::B1),
            7 => Some(Self::B2),
            8 => Some(Self::B3),
            9 => Some(Self::B4),
            _ => None,
        }
    }

    /// 从消息字符串解析
    pub fn parse(message: &str) -> Option<Self> {
        if !message.starts_with("feedback-") {
            return None;
        }
        let index: u8 = message.trim_start_matches("feedback-").parse().ok()?;
        Self::from_index(index)
    }
}

/// 错误码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// 成功
    Success,
    /// 对方客户端已断开
    PeerDisconnected,
    /// 二维码中没有有效的 clientID
    InvalidQrClientId,
    /// 服务器迟迟不下发 app 端的 id
    NoAppId,
    /// 此 id 已被其他客户端绑定
    IdAlreadyBound,
    /// 要绑定的目标客户端不存在
    TargetNotFound,
    /// 收信方和寄信方不是绑定关系
    NotBound,
    /// 发送的内容不是标准 json 对象
    InvalidJson,
    /// 未找到收信人（离线）
    RecipientOffline,
    /// 下发的 message 长度大于 1950
    MessageTooLong,
    /// 服务器内部异常
    ServerError,
    /// 未知错误码
    Unknown(u16),
}

impl From<u16> for ErrorCode {
    fn from(code: u16) -> Self {
        match code {
            200 => Self::Success,
            209 => Self::PeerDisconnected,
            210 => Self::InvalidQrClientId,
            211 => Self::NoAppId,
            400 => Self::IdAlreadyBound,
            401 => Self::TargetNotFound,
            402 => Self::NotBound,
            403 => Self::InvalidJson,
            404 => Self::RecipientOffline,
            405 => Self::MessageTooLong,
            500 => Self::ServerError,
            _ => Self::Unknown(code),
        }
    }
}

impl ErrorCode {
    /// 从字符串解析
    pub fn parse(message: &str) -> Self {
        message.parse::<u16>().map_or(Self::Unknown(0), Self::from)
    }

    /// 获取错误描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::Success => "成功",
            Self::PeerDisconnected => "对方客户端已断开",
            Self::InvalidQrClientId => "二维码中没有有效的 clientID",
            Self::NoAppId => "服务器迟迟不下发 app 端的 id",
            Self::IdAlreadyBound => "此 id 已被其他客户端绑定",
            Self::TargetNotFound => "要绑定的目标客户端不存在",
            Self::NotBound => "收信方和寄信方不是绑定关系",
            Self::InvalidJson => "发送的内容不是标准 json 对象",
            Self::RecipientOffline => "未找到收信人（离线）",
            Self::MessageTooLong => "下发的 message 长度大于 1950",
            Self::ServerError => "服务器内部异常",
            Self::Unknown(_) => "未知错误",
        }
    }
}

/// 从 WsMessage 接收到的事件
#[derive(Debug, Clone)]
pub enum WsEvent {
    /// 收到心跳响应
    Heartbeat,
    /// 收到 clientId
    ClientId(String),
    /// 绑定成功
    Bound(String),
    /// 收到强度数据
    Strength(StrengthData),
    /// APP 按钮反馈
    Feedback(FeedbackButton),
    /// 对方断开连接
    PeerDisconnected,
    /// 收到错误
    Error(ErrorCode),
    /// 绑定超时
    BindTimeout,
    /// 连接关闭
    Closed,
    /// 其他消息
    Other(WsMessage),
}

impl WsEvent {
    /// 从 WsMessage 解析事件
    pub fn from_message(msg: &WsMessage) -> Self {
        match msg.message_type() {
            MessageType::Heartbeat => Self::Heartbeat,
            MessageType::Bind => {
                if msg.message == "200" {
                    Self::Bound(msg.target_id.clone())
                } else if msg.message == "targetId" {
                    Self::ClientId(msg.client_id.clone())
                } else {
                    Self::Other(msg.clone())
                }
            }
            MessageType::Msg => {
                if let Some(strength) = StrengthData::parse(&msg.message) {
                    Self::Strength(strength)
                } else if let Some(button) = FeedbackButton::parse(&msg.message) {
                    Self::Feedback(button)
                } else {
                    Self::Other(msg.clone())
                }
            }
            MessageType::Break => Self::PeerDisconnected,
            MessageType::Error => Self::Error(ErrorCode::parse(&msg.message)),
            MessageType::Unknown(_) => Self::Other(msg.clone()),
        }
    }
}

/// 二维码生成辅助
pub mod qr {
    use super::*;

    /// 生成二维码内容 URL
    pub fn generate_url(server_url: &str, client_id: &str) -> String {
        let ws_url = format!("{server_url}/{client_id}");
        format!("https://www.dungeon-lab.com/app-download.php#DGLAB-SOCKET#{ws_url}")
    }

    /// 使用官方服务器生成二维码内容 URL
    pub fn generate_official_url(client_id: &str) -> String {
        generate_url(OFFICIAL_SERVER, client_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type() {
        assert_eq!(MessageType::from("heartbeat"), MessageType::Heartbeat);
        assert_eq!(MessageType::from("bind"), MessageType::Bind);
        assert_eq!(MessageType::from("msg"), MessageType::Msg);
        assert_eq!(MessageType::from("break"), MessageType::Break);
        assert_eq!(MessageType::from("error"), MessageType::Error);
        assert!(matches!(
            MessageType::from("unknown"),
            MessageType::Unknown(_)
        ));
    }

    #[test]
    fn test_strength_data_parse() {
        let data = StrengthData::parse("strength-11+7+100+35").unwrap();
        assert_eq!(data.strength_a, 11);
        assert_eq!(data.strength_b, 7);
        assert_eq!(data.max_a, 100);
        assert_eq!(data.max_b, 35);

        assert!(StrengthData::parse("invalid").is_none());
        assert!(StrengthData::parse("strength-1+2").is_none());
    }

    #[test]
    fn test_strength_operation() {
        let op = StrengthOperation::increase(Channel::A, 5);
        assert_eq!(op.to_message(), "strength-1+1+5");

        let op = StrengthOperation::decrease(Channel::B, 3);
        assert_eq!(op.to_message(), "strength-2+0+3");

        let op = StrengthOperation::set(Channel::A, 0);
        assert_eq!(op.to_message(), "strength-1+2+0");
    }

    #[test]
    fn test_clear_operation() {
        let op = ClearOperation::new(Channel::A);
        assert_eq!(op.to_message(), "clear-1");

        let op = ClearOperation::new(Channel::B);
        assert_eq!(op.to_message(), "clear-2");
    }

    #[test]
    fn test_feedback_button() {
        assert_eq!(FeedbackButton::from_index(0), Some(FeedbackButton::A0));
        assert_eq!(FeedbackButton::from_index(4), Some(FeedbackButton::A4));
        assert_eq!(FeedbackButton::from_index(5), Some(FeedbackButton::B0));
        assert_eq!(FeedbackButton::from_index(9), Some(FeedbackButton::B4));
        assert!(FeedbackButton::from_index(10).is_none());

        assert_eq!(
            FeedbackButton::parse("feedback-0"),
            Some(FeedbackButton::A0)
        );
        assert_eq!(
            FeedbackButton::parse("feedback-5"),
            Some(FeedbackButton::B0)
        );
    }

    #[test]
    fn test_error_code() {
        assert_eq!(ErrorCode::from(200), ErrorCode::Success);
        assert_eq!(ErrorCode::from(209), ErrorCode::PeerDisconnected);
        assert_eq!(ErrorCode::from(400), ErrorCode::IdAlreadyBound);
        assert_eq!(ErrorCode::from(500), ErrorCode::ServerError);
        assert!(matches!(ErrorCode::from(999), ErrorCode::Unknown(_)));
    }

    #[test]
    fn test_qr_url() {
        let url = qr::generate_official_url("test-client-id");
        assert!(url.contains("test-client-id"));
        assert!(url.starts_with("https://www.dungeon-lab.com/"));
    }

    #[test]
    fn test_pulse_data() {
        let pulse = PulseData::from_strength(Channel::A, 50, 30, 1000);
        assert_eq!(pulse.pulses.len(), 10);
        let msg = pulse.to_message();
        assert!(msg.starts_with("pulse-A:["));
    }
}
