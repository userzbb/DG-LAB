//! BLE + WebSocket 桥接设备
//!
//! 充当 DG-LAB APP 的替代品，允许第三方控制器通过 WebSocket 服务器远程控制设备

use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{broadcast, Mutex};
use tracing::{debug, error, info, warn};

use dglab_protocol::wifi::{WsClient, WsEvent};

use super::traits::{Device, DeviceInfo, WaveformConfig};
use super::{BaseDevice, DeviceEvent, DeviceState};
use crate::error::{CoreError, Result};

use super::CoyoteDevice;

/// BLE + WebSocket 桥接设备内部状态
struct BridgeInner {
    /// BLE 设备
    ble_device: Mutex<CoyoteDevice>,
    /// WebSocket 客户端
    ws_client: Mutex<Option<WsClient>>,
    /// 服务器 URL
    server_url: String,
}

/// BLE + WebSocket 桥接设备
///
/// 该设备充当 DG-LAB APP 的角色：
/// - 通过 BLE 直接连接主机
/// - 通过 WebSocket 连接到服务器
/// - 将服务器收到的指令转发给 BLE 主机
/// - 将主机状态同步到服务器
///
/// # 使用场景
///
/// ```text
/// 第三方控制器 → WebSocket → 服务器 ← WebSocket ← BridgeDevice ← BLE ← 主机
/// ```
pub struct BleWsBridgeDevice {
    /// 基础设备信息
    base: BaseDevice,
    /// 内部状态
    inner: Arc<BridgeInner>,
    /// WebSocket 接收任务
    ws_receive_task: Option<tokio::task::JoinHandle<()>>,
    /// 状态同步任务
    sync_task: Option<tokio::task::JoinHandle<()>>,
}

impl BleWsBridgeDevice {
    /// 创建新的桥接设备（使用官方服务器）
    pub fn new(id: String, name: String, ble_device_id: String, ble_device_name: String) -> Self {
        Self::with_server(
            id,
            name,
            ble_device_id,
            ble_device_name,
            dglab_protocol::wifi::OFFICIAL_SERVER.to_string(),
        )
    }

    /// 创建新的桥接设备（使用自定义服务器）
    pub fn with_server(
        id: String,
        name: String,
        ble_device_id: String,
        ble_device_name: String,
        server_url: String,
    ) -> Self {
        let base = BaseDevice::new(id, name);
        let ble_device = CoyoteDevice::new(ble_device_id, ble_device_name);

        let inner = Arc::new(BridgeInner {
            ble_device: Mutex::new(ble_device),
            ws_client: Mutex::new(None),
            server_url,
        });

        Self {
            base,
            inner,
            ws_receive_task: None,
            sync_task: None,
        }
    }

    /// 连接 BLE 设备
    pub async fn connect_ble(&self, protocol_device: dglab_protocol::ble::BleDevice) -> Result<()> {
        info!("Connecting to BLE device");

        let mut ble_dev = self.inner.ble_device.lock().await;
        ble_dev.set_protocol_device(protocol_device);
        ble_dev.connect().await?;

        info!("BLE device connected");
        Ok(())
    }

    /// 获取二维码 URL（连接 WebSocket 后可用）
    pub async fn qr_url(&self) -> Option<String> {
        let client = self.inner.ws_client.lock().await;
        if let Some(c) = client.as_ref() {
            c.official_qr_url().await
        } else {
            None
        }
    }

    /// 检查是否已绑定到控制器
    pub async fn is_bound(&self) -> bool {
        let client = self.inner.ws_client.lock().await;
        if let Some(c) = client.as_ref() {
            c.is_bound().await
        } else {
            false
        }
    }

    /// 启动 WebSocket 消息接收任务
    fn start_ws_receive_task(&mut self) {
        let inner = self.inner.clone();

        let handle = tokio::spawn(async move {
            loop {
                let mut client = inner.ws_client.lock().await;
                let Some(c) = client.as_mut() else {
                    break;
                };

                match c.recv_event().await {
                    Ok(Some(event)) => {
                        Self::handle_ws_event(&inner, event).await;
                    }
                    Ok(None) => {
                        debug!("WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        error!("WebSocket receive error: {}", e);
                        break;
                    }
                }
            }
        });

        self.ws_receive_task = Some(handle);
    }

    /// 停止 WebSocket 接收任务
    fn stop_ws_receive_task(&mut self) {
        if let Some(handle) = self.ws_receive_task.take() {
            handle.abort();
        }
    }

    /// 启动 BLE → WebSocket 状态同步任务
    fn start_sync_task(&mut self) {
        let inner = self.inner.clone();

        let handle = tokio::spawn(async move {
            let ble_dev = inner.ble_device.lock().await;
            let mut event_rx = ble_dev.subscribe_events();
            drop(ble_dev);

            loop {
                match event_rx.recv().await {
                    Ok(event) => {
                        Self::handle_ble_event(&inner, event).await;
                    }
                    Err(e) => {
                        debug!("BLE event channel closed: {}", e);
                        break;
                    }
                }
            }
        });

        self.sync_task = Some(handle);
    }

    /// 停止状态同步任务
    fn stop_sync_task(&mut self) {
        if let Some(handle) = self.sync_task.take() {
            handle.abort();
        }
    }

    /// 处理 WebSocket 事件（从服务器接收的控制指令）
    async fn handle_ws_event(inner: &Arc<BridgeInner>, event: WsEvent) {
        match event {
            WsEvent::ClientId(id) => {
                debug!("Received WebSocket client ID: {}", id);
            }
            WsEvent::Bound(target_id) => {
                info!("Bound to controller: {}", target_id);
            }
            WsEvent::Heartbeat => {
                debug!("Received heartbeat");
            }
            WsEvent::Strength(strength_data) => {
                // 这是从对方收到的强度数据，我们作为 APP 端接收
                debug!("Received strength data: {:?}", strength_data);
            }
            WsEvent::Feedback(button) => {
                info!("Received feedback button: {:?}", button);
                // 反馈按钮 - 暂不处理
            }
            WsEvent::PeerDisconnected => {
                info!("Controller disconnected");
            }
            WsEvent::Error(code) => {
                warn!("WebSocket error: {:?}", code);
            }
            WsEvent::Other(msg) => {
                debug!("Received message: {}", msg.message);
                // 解析控制指令
                Self::handle_control_message(inner, &msg.message).await;
            }
            WsEvent::BindTimeout => {
                warn!("WebSocket bind timeout");
            }
            WsEvent::Closed => {
                info!("WebSocket connection closed");
            }
        }
    }

    /// 处理控制消息
    async fn handle_control_message(inner: &Arc<BridgeInner>, message: &str) {
        // 强度操作: strength-{channel}+{mode}+{value}
        // channel: 1=A, 2=B
        // mode: 0=减少, 1=增加, 2=设置
        if message.starts_with("strength-") {
            Self::parse_and_apply_strength(inner, message).await;
        }
        // 波形数据: pulse-{channel}:[...]
        else if message.starts_with("pulse-") {
            Self::parse_and_apply_pulse(inner, message).await;
        }
        // 清空: clear-{channel}
        else if message.starts_with("clear-") {
            Self::parse_and_apply_clear(inner, message).await;
        } else {
            debug!("Unknown control message: {}", message);
        }
    }

    /// 解析并应用强度操作
    async fn parse_and_apply_strength(inner: &Arc<BridgeInner>, message: &str) {
        let parts: Vec<&str> = message.trim_start_matches("strength-").split('+').collect();

        if parts.len() != 3 {
            warn!("Invalid strength message format: {}", message);
            return;
        }

        let channel = match parts[0] {
            "1" => 0u8, // A 通道
            "2" => 1u8, // B 通道
            _ => {
                warn!("Invalid channel: {}", parts[0]);
                return;
            }
        };

        let mode: u8 = match parts[1].parse() {
            Ok(m) => m,
            Err(_) => {
                warn!("Invalid mode: {}", parts[1]);
                return;
            }
        };

        let value: u8 = match parts[2].parse() {
            Ok(v) => v,
            Err(_) => {
                warn!("Invalid value: {}", parts[2]);
                return;
            }
        };

        let mut ble_dev = inner.ble_device.lock().await;
        let current_power = ble_dev.get_power(channel);

        let new_power = match mode {
            0 => current_power.saturating_sub(value),          // 减少
            1 => current_power.saturating_add(value).min(200), // 增加
            2 => value.min(200),                               // 设置
            _ => {
                warn!("Unknown strength mode: {}", mode);
                return;
            }
        };

        if let Err(e) = ble_dev.set_power(channel, new_power).await {
            error!("Failed to set power on channel {}: {}", channel, e);
        } else {
            debug!(
                "Applied power {} on channel {} (was {})",
                new_power, channel, current_power
            );
        }
    }

    /// 解析并应用波形数据
    async fn parse_and_apply_pulse(_inner: &Arc<BridgeInner>, message: &str) {
        // TODO: 实现波形数据解析和应用
        warn!("Pulse data parsing not yet implemented: {}", message);
    }

    /// 解析并应用清空操作
    async fn parse_and_apply_clear(inner: &Arc<BridgeInner>, message: &str) {
        let channel_str = message.trim_start_matches("clear-");
        let channel = match channel_str {
            "1" | "A" => 0u8,
            "2" | "B" => 1u8,
            _ => {
                warn!("Invalid clear channel: {}", channel_str);
                return;
            }
        };

        let mut ble_dev = inner.ble_device.lock().await;
        if let Err(e) = ble_dev.set_power(channel, 0).await {
            error!("Failed to clear channel {}: {}", channel, e);
        } else {
            debug!("Cleared channel {}", channel);
        }
    }

    /// 处理 BLE 设备事件（同步状态到 WebSocket）
    async fn handle_ble_event(inner: &Arc<BridgeInner>, event: DeviceEvent) {
        match event {
            DeviceEvent::StatusReport { power_a, power_b } => {
                debug!("BLE power status: A={}, B={}", power_a, power_b);
                // 同步强度到 WebSocket
                Self::sync_strength_to_ws(inner, power_a, power_b).await;
            }
            DeviceEvent::StateChanged(state) => {
                debug!("BLE state changed: {:?}", state);
            }
            DeviceEvent::BatteryUpdated(level) => {
                debug!("BLE battery updated: {}%", level);
            }
            _ => {}
        }
    }

    /// 同步强度到 WebSocket
    async fn sync_strength_to_ws(inner: &Arc<BridgeInner>, power_a: u8, power_b: u8) {
        let client = inner.ws_client.lock().await;
        if let Some(c) = client.as_ref() {
            // 构造状态消息并发送
            use dglab_protocol::wifi::{MessageType, WsMessage};

            // 获取 client_id 和 target_id
            if let (Some(client_id), Some(target_id)) = (c.client_id().await, c.target_id().await) {
                // 从 BLE 设备获取实际的强度上限
                let (max_a, max_b) = {
                    let ble_device = inner.ble_device.lock().await;
                    let info = ble_device.info();
                    (info.max_power_a, info.max_power_b)
                };

                // 发送当前强度状态
                // 格式: "strength-{A}+{B}+{maxA}+{maxB}"
                let message = format!("strength-{}+{}+{}+{}", power_a, power_b, max_a, max_b);
                let ws_msg = WsMessage::new(MessageType::Msg, client_id, target_id, message);

                if let Err(e) = c.send(&ws_msg).await {
                    warn!("Failed to sync strength to WebSocket: {}", e);
                }
            }
        }
    }
}

#[async_trait]
impl Device for BleWsBridgeDevice {
    fn id(&self) -> &str {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn state(&self) -> DeviceState {
        self.base.state()
    }

    fn info(&self) -> DeviceInfo {
        // 由于 info() 不是异步方法，我们无法获取锁
        // 使用默认值，实际强度上限会在 sync_strength_to_ws 中正确获取
        DeviceInfo {
            id: self.base.id().to_string(),
            name: self.base.name().to_string(),
            device_type: "Coyote-BLE-WS-Bridge".to_string(),
            firmware_version: String::new(),
            hardware_version: String::new(),
            battery_level: 100,
            power_a: self.base.power_a(),
            power_b: self.base.power_b(),
            max_power_a: 200, // 默认值，实际值在 sync_strength_to_ws 中获取
            max_power_b: 200, // 默认值，实际值在 sync_strength_to_ws 中获取
        }
    }

    async fn connect(&mut self) -> Result<()> {
        info!("Connecting BLE-WS Bridge device");

        if self.base.state() == DeviceState::Connected {
            return Ok(());
        }

        self.base.set_state(DeviceState::Connecting);

        // 1. 连接 WebSocket
        let mut client = WsClient::connect(&self.inner.server_url)
            .await
            .map_err(|e| CoreError::Other(format!("WebSocket connect error: {}", e)))?;

        // 2. 等待绑定（参考 hyperzlib 项目，超时 20 秒）
        info!("Waiting for WebSocket binding...");
        let bind_timeout_secs = 20;

        match client.wait_for_bind(bind_timeout_secs).await {
            Ok(true) => {
                info!("WebSocket binding successful");
            }
            Ok(false) => {
                let err_msg = format!(
                    "WebSocket binding timeout after {} seconds",
                    bind_timeout_secs
                );
                error!("{}", err_msg);
                return Err(CoreError::Other(err_msg));
            }
            Err(e) => {
                let err_msg = format!("WebSocket binding error: {}", e);
                error!("{}", err_msg);
                return Err(CoreError::Other(err_msg));
            }
        }

        {
            let mut ws_client = self.inner.ws_client.lock().await;
            *ws_client = Some(client);
        }

        // 3. 启动任务
        self.start_ws_receive_task();
        self.start_sync_task();

        self.base.set_state(DeviceState::Connected);

        info!("BLE-WS Bridge device connected and bound");
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        info!("Disconnecting BLE-WS Bridge device");

        if self.base.state() == DeviceState::Disconnected {
            return Ok(());
        }

        // 停止任务
        self.stop_ws_receive_task();
        self.stop_sync_task();

        // 断开 BLE
        let mut ble_dev = self.inner.ble_device.lock().await;
        let _ = ble_dev.disconnect().await; // 忽略错误

        // 关闭 WebSocket
        let mut ws_client = self.inner.ws_client.lock().await;
        *ws_client = None;

        self.base.set_state(DeviceState::Disconnected);

        info!("BLE-WS Bridge device disconnected");
        Ok(())
    }

    async fn start(&mut self) -> Result<()> {
        info!("Starting BLE-WS Bridge device");

        if self.base.state() != DeviceState::Connected {
            return Err(CoreError::DeviceNotConnected);
        }

        // 启动 BLE 设备
        let mut ble_dev = self.inner.ble_device.lock().await;
        ble_dev.start().await?;

        self.base.set_state(DeviceState::Running);

        info!("BLE-WS Bridge device started");
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping BLE-WS Bridge device");

        if self.base.state() == DeviceState::Disconnected {
            return Ok(());
        }

        // 停止 BLE 设备
        let mut ble_dev = self.inner.ble_device.lock().await;
        ble_dev.stop().await?;

        self.base.set_state(DeviceState::Connected);

        info!("BLE-WS Bridge device stopped");
        Ok(())
    }

    async fn set_power(&mut self, channel: u8, power: u8) -> Result<()> {
        // 直接操作 BLE 设备
        let mut ble_dev = self.inner.ble_device.lock().await;
        ble_dev.set_power(channel, power).await?;

        // 更新 base 状态
        self.base.set_power(channel, power)?;

        Ok(())
    }

    fn get_power(&self, channel: u8) -> u8 {
        match channel {
            0 => self.base.power_a(),
            1 => self.base.power_b(),
            _ => 0,
        }
    }

    async fn set_waveform(&mut self, channel: u8, config: WaveformConfig) -> Result<()> {
        // 直接操作 BLE 设备
        let mut ble_dev = self.inner.ble_device.lock().await;
        ble_dev.set_waveform(channel, config).await
    }

    async fn heartbeat(&mut self) -> Result<()> {
        // BLE 设备自己会处理心跳
        let mut ble_dev = self.inner.ble_device.lock().await;
        ble_dev.heartbeat().await?;

        // WebSocket 心跳
        let client = self.inner.ws_client.lock().await;
        if let Some(c) = client.as_ref() {
            c.send_heartbeat()
                .await
                .map_err(|e| CoreError::Other(format!("WebSocket heartbeat error: {}", e)))?;
        }

        Ok(())
    }

    fn subscribe_events(&self) -> broadcast::Receiver<DeviceEvent> {
        self.base.subscribe_events()
    }
}
