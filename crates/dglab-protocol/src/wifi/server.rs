//! WebSocket 服务器实现
//!
//! 根据 hyperzlib/DG-Lab-Coyote-Game-Hub 项目的实现逻辑，提供 WebSocket 服务器功能。
//!
//! # 架构
//!
//! ```text
//! 网页前端 → WebSocket → 服务器 ← WebSocket ← DG-LAB APP ← BLE ← 主机
//! ```
//!
//! # 连接 URL 格式
//!
//! - DG-LAB APP: `ws://server:port/dglab/{clientId}`
//! - 网页前端: `ws://server:port/web/{clientId}`
//!
//! # 示例
//!
//! ```no_run
//! use dglab_protocol::wifi::WsServer;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let server = WsServer::new("127.0.0.1:8080".to_string());
//! server.start().await?;
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, RwLock};
use tokio_tungstenite::{accept_async, tungstenite::Message as TungsteniteMessage};
use tracing::{debug, error, info, warn};

use super::*;

/// WebSocket 服务器
pub struct WsServer {
    /// 监听地址
    bind_addr: String,
    /// 客户端管理器
    clients: Arc<RwLock<HashMap<String, Arc<WsClientConnection>>>>,
    /// 事件广播
    event_tx: broadcast::Sender<ServerEvent>,
}

/// 服务器事件
#[derive(Debug, Clone)]
pub enum ServerEvent {
    /// 客户端已连接
    ClientConnected(String),
    /// 客户端已断开
    ClientDisconnected(String),
    /// 客户端已绑定
    ClientBound {
        /// 客户端 ID
        client_id: String,
        /// 目标 ID
        target_id: String,
    },
    /// 收到消息
    MessageReceived {
        /// 发送方
        from: String,
        /// 接收方
        to: String,
        /// 消息内容
        message: String,
    },
}

/// 客户端连接
pub struct WsClientConnection {
    /// 客户端 ID
    #[allow(dead_code)]
    client_id: String,
    /// 绑定的目标 ID
    target_id: Arc<RwLock<Option<String>>>,
    /// 消息发送通道
    tx: tokio::sync::mpsc::Sender<TungsteniteMessage>,
}

impl WsServer {
    /// 创建新的服务器
    pub fn new(bind_addr: String) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        Self {
            bind_addr,
            clients: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
        }
    }

    /// 订阅服务器事件
    pub fn subscribe_events(&self) -> broadcast::Receiver<ServerEvent> {
        self.event_tx.subscribe()
    }

    /// 启动服务器
    pub async fn start(&self) -> WsResult<()> {
        let listener = TcpListener::bind(&self.bind_addr)
            .await
            .map_err(|e| WsError::Connection(e.to_string()))?;

        info!("WebSocket server listening on {}", self.bind_addr);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    debug!("New connection from {}", addr);
                    let clients = self.clients.clone();
                    let event_tx = self.event_tx.clone();

                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, clients, event_tx).await {
                            error!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    /// 处理新连接
    async fn handle_connection(
        stream: TcpStream,
        clients: Arc<RwLock<HashMap<String, Arc<WsClientConnection>>>>,
        event_tx: broadcast::Sender<ServerEvent>,
    ) -> WsResult<()> {
        let ws_stream = accept_async(stream)
            .await
            .map_err(|e| WsError::Connection(e.to_string()))?;

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        use futures_util::sink::SinkExt;
        use futures_util::stream::StreamExt;

        // 等待客户端发送第一条消息（应该包含 clientId）
        let first_msg = ws_receiver
            .next()
            .await
            .ok_or_else(|| WsError::Connection("Connection closed".to_string()))?
            .map_err(|e| WsError::Connection(e.to_string()))?;

        let client_id = match first_msg {
            TungsteniteMessage::Text(text) => {
                // 尝试从 URL 路径中提取 clientId
                // 或者从消息中解析
                // 这里简化处理，假设第一条消息就是 clientId
                text.trim().to_string()
            }
            _ => {
                return Err(WsError::InvalidMessage(
                    "Expected text message with clientId".to_string(),
                ));
            }
        };

        // 验证 clientId
        if client_id.is_empty() {
            let error_msg = WsMessage::new(
                MessageType::Error,
                "",
                "",
                RetCode::InvalidClientId.as_str(),
            );
            let _ = ws_sender
                .send(TungsteniteMessage::Text(
                    serde_json::to_string(&error_msg).unwrap(),
                ))
                .await;
            return Err(WsError::InvalidMessage("Invalid client ID".to_string()));
        }

        // 检查 ID 是否已被占用
        {
            let clients_read = clients.read().await;
            if clients_read.contains_key(&client_id) {
                let error_msg = WsMessage::new(
                    MessageType::Error,
                    &client_id,
                    "",
                    RetCode::IdAlreadyBound.as_str(),
                );
                let _ = ws_sender
                    .send(TungsteniteMessage::Text(
                        serde_json::to_string(&error_msg).unwrap(),
                    ))
                    .await;
                return Err(WsError::Other("ID already bound".to_string()));
            }
        }

        info!("Client connected: {}", client_id);

        // 创建消息发送通道
        let (tx, mut rx) = tokio::sync::mpsc::channel::<TungsteniteMessage>(100);

        // 创建客户端连接对象
        let client_conn = Arc::new(WsClientConnection {
            client_id: client_id.clone(),
            target_id: Arc::new(RwLock::new(None)),
            tx,
        });

        // 注册客户端
        {
            let mut clients_write = clients.write().await;
            clients_write.insert(client_id.clone(), client_conn.clone());
        }

        // 触发连接事件
        let _ = event_tx.send(ServerEvent::ClientConnected(client_id.clone()));

        // 发送 BIND 请求，要求客户端提供 targetId
        let bind_msg = WsMessage::new(
            MessageType::Bind,
            "",
            "",
            MessageDataHead::TargetId.as_str(),
        );
        let _ = ws_sender
            .send(TungsteniteMessage::Text(
                serde_json::to_string(&bind_msg).unwrap(),
            ))
            .await;

        // 启动发送任务
        let client_id_clone = client_id.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = ws_sender.send(msg).await {
                    error!("Failed to send message to {}: {}", client_id_clone, e);
                    break;
                }
            }
        });

        // 启动接收任务
        let client_id_for_recv = client_id.clone();
        let clients_for_recv = clients.clone();
        let event_tx_for_recv = event_tx.clone();
        let client_conn_for_recv = client_conn.clone();

        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(TungsteniteMessage::Text(text)) => {
                    if let Err(e) = Self::handle_message(
                        &text,
                        &client_id_for_recv,
                        &clients_for_recv,
                        &event_tx_for_recv,
                        &client_conn_for_recv,
                    )
                    .await
                    {
                        error!("Failed to handle message: {}", e);
                    }
                }
                Ok(TungsteniteMessage::Close(_)) => {
                    info!("Client {} closed connection", client_id_for_recv);
                    break;
                }
                Err(e) => {
                    error!("WebSocket error for {}: {}", client_id_for_recv, e);
                    break;
                }
                _ => {}
            }
        }

        // 清理客户端
        {
            let mut clients_write = clients.write().await;
            clients_write.remove(&client_id);
        }

        // 触发断开事件
        let _ = event_tx.send(ServerEvent::ClientDisconnected(client_id));

        Ok(())
    }

    /// 处理客户端消息
    async fn handle_message(
        text: &str,
        client_id: &str,
        clients: &Arc<RwLock<HashMap<String, Arc<WsClientConnection>>>>,
        event_tx: &broadcast::Sender<ServerEvent>,
        client_conn: &Arc<WsClientConnection>,
    ) -> WsResult<()> {
        let msg: WsMessage = serde_json::from_str(text)
            .map_err(|e| WsError::InvalidMessage(format!("JSON parse error: {}", e)))?;

        debug!(
            "Received message from {}: type={}, target={}, message={}",
            client_id, msg.msg_type, msg.target_id, msg.message
        );

        match msg.message_type() {
            MessageType::Bind => {
                // 客户端响应绑定请求
                if msg.message == MessageDataHead::DgLab.as_str() {
                    // 客户端确认绑定
                    let mut target_id_write = client_conn.target_id.write().await;
                    *target_id_write = Some(msg.target_id.clone());

                    info!("Client {} bound to {}", client_id, msg.target_id);

                    // 触发绑定事件
                    let _ = event_tx.send(ServerEvent::ClientBound {
                        client_id: client_id.to_string(),
                        target_id: msg.target_id.clone(),
                    });

                    // 发送绑定成功响应
                    let response =
                        WsMessage::new(MessageType::Bind, "", "", RetCode::Success.as_str());
                    let _ = client_conn
                        .tx
                        .send(TungsteniteMessage::Text(
                            serde_json::to_string(&response).unwrap(),
                        ))
                        .await;
                }
            }
            MessageType::Heartbeat => {
                // 响应心跳
                let response = WsMessage::new(
                    MessageType::Heartbeat,
                    "",
                    "",
                    MessageDataHead::DgLab.as_str(),
                );
                let _ = client_conn
                    .tx
                    .send(TungsteniteMessage::Text(
                        serde_json::to_string(&response).unwrap(),
                    ))
                    .await;
            }
            MessageType::Msg => {
                // 转发消息到目标客户端
                let target_id = &msg.target_id;
                if target_id.is_empty() {
                    warn!("Message from {} has no target", client_id);
                    return Ok(());
                }

                let clients_read = clients.read().await;
                if let Some(target_conn) = clients_read.get(target_id) {
                    let _ = target_conn
                        .tx
                        .send(TungsteniteMessage::Text(text.to_string()))
                        .await;

                    // 触发消息事件
                    let _ = event_tx.send(ServerEvent::MessageReceived {
                        from: client_id.to_string(),
                        to: target_id.clone(),
                        message: msg.message.clone(),
                    });
                } else {
                    warn!("Target client {} not found", target_id);
                }
            }
            MessageType::Break => {
                info!("Client {} requested disconnect", client_id);
            }
            _ => {
                debug!("Unhandled message type: {:?}", msg.message_type());
            }
        }

        Ok(())
    }
}
