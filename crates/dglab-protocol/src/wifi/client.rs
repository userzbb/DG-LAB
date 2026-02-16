//! WebSocket 客户端实现

use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as TungsteniteMessage};
use tracing::{debug, error, info, warn};
use url::Url;

use super::*;

/// WebSocket 客户端内部状态
#[derive(Default)]
struct ClientState {
    /// 本端 client_id
    client_id: Option<String>,
    /// 绑定的目标 client_id
    target_id: Option<String>,
    /// 是否已连接
    connected: bool,
}

/// 可克隆的 WsClient 句柄
#[derive(Clone)]
pub struct WsClientHandle {
    /// 发送消息的通道
    tx: mpsc::Sender<TungsteniteMessage>,
    /// 客户端状态
    state: Arc<Mutex<ClientState>>,
    /// 服务器 URL
    server_url: String,
}

/// WebSocket 客户端
///
/// 用于与 DG-LAB APP 通过 WebSocket 进行通信。
///
/// # 示例
///
/// ```no_run
/// use dglab_protocol::wifi::{WsClient, WsEvent, StrengthOperation, Channel};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut client = WsClient::connect_official().await?;
///
/// // 等待获取 clientId
/// let client_id = loop {
///     if let Some(event) = client.recv_event().await? {
///         match event {
///             WsEvent::ClientId(id) => break id,
///             _ => continue,
///         }
///     }
/// };
///
/// println!("Client ID: {}", client_id);
/// println!("QR URL: {}", client.qr_url().await.unwrap());
///
/// // 等待绑定
/// loop {
///     if let Some(event) = client.recv_event().await? {
///         match event {
///             WsEvent::Bound(_) => break,
///             _ => continue,
///         }
///     }
/// }
///
/// // 发送强度增加指令
/// client.send_strength_operation(StrengthOperation::increase(Channel::A, 5)).await?;
///
/// # Ok(())
/// # }
/// ```
pub struct WsClient {
    /// 可克隆的句柄
    handle: WsClientHandle,
    /// 接收事件的通道
    rx: mpsc::Receiver<WsEvent>,
}

impl Clone for WsClient {
    fn clone(&self) -> Self {
        // clone 时创建一个新的 dummy receiver
        let (_, rx) = mpsc::channel(32);
        Self {
            handle: self.handle.clone(),
            rx,
        }
    }
}

impl WsClient {
    /// 连接到指定的 WebSocket 服务器
    ///
    /// # 参数
    /// - `server_url`: WebSocket 服务器 URL，例如 "wss://ws.dungeon-lab.cn"
    pub async fn connect(server_url: &str) -> WsResult<Self> {
        let url = Url::parse(server_url)?;

        debug!("Connecting to WebSocket server: {}", url);

        let (ws_stream, response) = connect_async(url).await?;
        debug!("WebSocket connected: {:?}", response.status());

        let (mut write, mut read) = ws_stream.split();

        let (tx, mut internal_rx) = mpsc::channel(32);
        let (event_tx, event_rx) = mpsc::channel(32);

        let state = Arc::new(Mutex::new(ClientState {
            client_id: None,
            target_id: None,
            connected: true,
        }));

        let state_clone = state.clone();

        // 发送任务
        tokio::spawn(async move {
            while let Some(msg) = internal_rx.recv().await {
                if let Err(e) = write.send(msg).await {
                    error!("Failed to send message: {}", e);
                    break;
                }
            }
            let _ = write.close().await;
        });

        // 接收任务
        tokio::spawn(async move {
            while let Some(msg_result) = read.next().await {
                match msg_result {
                    Ok(msg) => {
                        if let TungsteniteMessage::Text(text) = msg {
                            debug!("Received message: {}", text);
                            match serde_json::from_str::<WsMessage>(&text) {
                                Ok(ws_msg) => {
                                    let event = WsEvent::from_message(&ws_msg);

                                    // 更新状态
                                    let mut state = state_clone.lock().await;
                                    match &event {
                                        WsEvent::ClientId(id) => {
                                            state.client_id = Some(id.clone());
                                        }
                                        WsEvent::Bound(target_id) => {
                                            state.target_id = Some(target_id.clone());
                                        }
                                        _ => {}
                                    }

                                    if let Err(e) = event_tx.send(event).await {
                                        warn!("Failed to send event: {}", e);
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to parse message: {}", e);
                                }
                            }
                        } else if let TungsteniteMessage::Close(_) = msg {
                            info!("Received close frame");
                            break;
                        }
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                }
            }

            let mut state = state_clone.lock().await;
            state.connected = false;
        });

        let handle = WsClientHandle {
            tx,
            state,
            server_url: server_url.to_string(),
        };

        Ok(Self {
            handle,
            rx: event_rx,
        })
    }

    /// 连接到官方 WebSocket 服务器
    pub async fn connect_official() -> WsResult<Self> {
        Self::connect(OFFICIAL_SERVER).await
    }

    /// 获取可克隆的句柄
    pub fn handle(&self) -> WsClientHandle {
        self.handle.clone()
    }

    /// 获取当前 client_id
    pub async fn client_id(&self) -> Option<String> {
        self.handle.state.lock().await.client_id.clone()
    }

    /// 获取已绑定的 target_id
    pub async fn target_id(&self) -> Option<String> {
        self.handle.state.lock().await.target_id.clone()
    }

    /// 检查是否已连接
    pub async fn is_connected(&self) -> bool {
        self.handle.state.lock().await.connected
    }

    /// 检查是否已绑定到目标
    pub async fn is_bound(&self) -> bool {
        self.handle.state.lock().await.target_id.is_some()
    }

    /// 获取二维码 URL
    pub async fn qr_url(&self) -> Option<String> {
        let client_id = self.handle.state.lock().await.client_id.clone()?;
        Some(qr::generate_url(&self.handle.server_url, &client_id))
    }

    /// 获取官方服务器二维码 URL
    pub async fn official_qr_url(&self) -> Option<String> {
        let client_id = self.handle.state.lock().await.client_id.clone()?;
        Some(qr::generate_official_url(&client_id))
    }

    /// 发送原始 WebSocket 消息
    pub async fn send_raw(&self, msg: TungsteniteMessage) -> WsResult<()> {
        self.handle
            .tx
            .send(msg)
            .await
            .map_err(|e| WsError::Send(e.to_string()))
    }

    /// 发送 WsMessage
    pub async fn send(&self, msg: &WsMessage) -> WsResult<()> {
        let text = serde_json::to_string(msg)?;
        self.send_raw(TungsteniteMessage::Text(text)).await
    }

    /// 发送心跳包
    pub async fn send_heartbeat(&self) -> WsResult<()> {
        let state = self.handle.state.lock().await;
        let client_id = state.client_id.clone().unwrap_or_default();
        let target_id = state.target_id.clone().unwrap_or_default();

        let msg = WsMessage::new(
            MessageType::Heartbeat,
            client_id,
            target_id,
            "200".to_string(),
        );
        self.send(&msg).await
    }

    /// 等待绑定成功（带超时）
    pub async fn wait_for_bind(&mut self, timeout_secs: u64) -> WsResult<bool> {
        use tokio::time::{timeout, Duration};

        let start = std::time::Instant::now();
        let timeout_duration = Duration::from_secs(timeout_secs);

        loop {
            // 检查是否已绑定
            if self.is_bound().await {
                return Ok(true);
            }

            // 检查是否超时
            if start.elapsed() > timeout_duration {
                return Ok(false);
            }

            // 等待事件
            match timeout(Duration::from_millis(500), self.recv_event()).await {
                Ok(Ok(Some(event))) => {
                    match event {
                        WsEvent::Bound(_) => return Ok(true),
                        WsEvent::Error(_) => return Ok(false),
                        WsEvent::BindTimeout => return Ok(false),
                        WsEvent::Closed => return Ok(false),
                        _ => continue, // 其他事件继续等待
                    }
                }
                Ok(Ok(None)) => {
                    // 通道关闭
                    return Ok(false);
                }
                Ok(Err(e)) => {
                    // 接收错误
                    return Err(e);
                }
                Err(_) => {
                    // 超时，继续循环
                    continue;
                }
            }
        }
    }

    /// 发送强度操作
    pub async fn send_strength_operation(&self, op: StrengthOperation) -> WsResult<()> {
        let state = self.handle.state.lock().await;
        let client_id = state.client_id.clone().ok_or(WsError::NotConnected)?;
        let target_id = state.target_id.clone().ok_or(WsError::NotBound)?;
        drop(state);

        let msg = WsMessage::new(MessageType::Msg, client_id, target_id, op.to_message());
        self.send(&msg).await
    }

    /// 发送波形数据
    pub async fn send_pulse(&self, pulse: PulseData) -> WsResult<()> {
        let state = self.handle.state.lock().await;
        let client_id = state.client_id.clone().ok_or(WsError::NotConnected)?;
        let target_id = state.target_id.clone().ok_or(WsError::NotBound)?;
        drop(state);

        let message = pulse.to_message();
        if message.len() > 1950 {
            return Err(WsError::Protocol("Message too long".to_string()));
        }

        let msg = WsMessage::new(MessageType::Msg, client_id, target_id, message);
        self.send(&msg).await
    }

    /// 发送清空队列操作
    pub async fn send_clear(&self, channel: Channel) -> WsResult<()> {
        let state = self.handle.state.lock().await;
        let client_id = state.client_id.clone().ok_or(WsError::NotConnected)?;
        let target_id = state.target_id.clone().ok_or(WsError::NotBound)?;
        drop(state);

        let op = ClearOperation::new(channel);
        let msg = WsMessage::new(MessageType::Msg, client_id, target_id, op.to_message());
        self.send(&msg).await
    }

    /// 接收原始消息
    pub async fn recv(&mut self) -> WsResult<Option<WsEvent>> {
        Ok(self.rx.recv().await)
    }

    /// 接收事件（同 recv）
    pub async fn recv_event(&mut self) -> WsResult<Option<WsEvent>> {
        self.recv().await
    }

    /// 启动自动心跳任务
    ///
    /// 每分钟发送一次心跳包。
    ///
    /// # 参数
    /// - `interval_secs`: 心跳间隔（秒），默认 60 秒
    pub async fn start_heartbeat(&self, interval_secs: Option<u64>) {
        let interval = std::time::Duration::from_secs(interval_secs.unwrap_or(60));
        let tx = self.handle.tx.clone();
        let state = self.handle.state.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval);
            loop {
                interval.tick().await;

                let state_guard = state.lock().await;
                if !state_guard.connected {
                    break;
                }

                let client_id = state_guard.client_id.clone().unwrap_or_default();
                let target_id = state_guard.target_id.clone().unwrap_or_default();
                drop(state_guard);

                let ws_msg = WsMessage::new(MessageType::Heartbeat, client_id, target_id, "");
                if let Ok(text) = serde_json::to_string(&ws_msg) {
                    if tx.send(TungsteniteMessage::Text(text)).await.is_err() {
                        break;
                    }
                }
            }
        });
    }

    /// 关闭连接
    pub async fn close(&self) -> WsResult<()> {
        self.send_raw(TungsteniteMessage::Close(None)).await?;
        let mut state = self.handle.state.lock().await;
        state.connected = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_state_default() {
        let state = ClientState::default();
        assert!(state.client_id.is_none());
        assert!(state.target_id.is_none());
        assert!(!state.connected);
    }
}
