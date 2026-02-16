//! Coyote 设备实现
//!
//! BLE 设备使用 V3 协议（B0/BF/B1 指令），WiFi 设备使用 WebSocket JSON 协议。

use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::{broadcast, Mutex};
use tracing::{debug, error, info, warn};

use dglab_protocol::ble::{BleDevice as ProtocolBleDevice, BleManager};
use dglab_protocol::v3::{
    B0Command, B1Response, BFCommand, ChannelStrengthMode, NotifyMessage, StrengthMode,
    WaveformData, MAX_STRENGTH,
};

use crate::device::traits::{Device, DeviceInfo, WaveformConfig, WaveformType};
use crate::device::{BaseDevice, DeviceEvent, DeviceState};
use crate::error::{CoreError, Result};

// ============================================================================
// V3 BLE 输出状态（供 100ms 输出循环共享）
// ============================================================================

/// V3 协议共享输出状态
///
/// 由 CoyoteDevice 和后台输出任务共同访问。
struct V3OutputState {
    /// 目标 A 通道强度 (0~200)
    target_strength_a: AtomicU8,
    /// 目标 B 通道强度 (0~200)
    target_strength_b: AtomicU8,
    /// 是否需要发送 A 通道强度变更
    pending_strength_a: AtomicBool,
    /// 是否需要发送 B 通道强度变更
    pending_strength_b: AtomicBool,
    /// 序列号 (0~15)
    sequence: AtomicU8,
    /// 当前 A 通道波形
    waveform_a: Mutex<WaveformData>,
    /// 当前 B 通道波形
    waveform_b: Mutex<WaveformData>,
}

impl V3OutputState {
    fn new() -> Self {
        Self {
            target_strength_a: AtomicU8::new(0),
            target_strength_b: AtomicU8::new(0),
            pending_strength_a: AtomicBool::new(false),
            pending_strength_b: AtomicBool::new(false),
            sequence: AtomicU8::new(0),
            waveform_a: Mutex::new(WaveformData::silent()),
            waveform_b: Mutex::new(WaveformData::silent()),
        }
    }

    /// 获取并递增序列号 (0~15 循环)
    fn next_sequence(&self) -> u8 {
        let seq = self.sequence.fetch_add(1, Ordering::Relaxed);
        // 确保始终在 1~15 范围内（0 表示无需反馈）
        (seq % 15) + 1
    }

    /// 构建下一个 B0 指令
    async fn build_b0(&self) -> B0Command {
        let need_a = self.pending_strength_a.swap(false, Ordering::Relaxed);
        let need_b = self.pending_strength_b.swap(false, Ordering::Relaxed);

        let mode_a = if need_a {
            ChannelStrengthMode::Absolute
        } else {
            ChannelStrengthMode::NoChange
        };

        let mode_b = if need_b {
            ChannelStrengthMode::Absolute
        } else {
            ChannelStrengthMode::NoChange
        };

        let sequence = if need_a || need_b {
            self.next_sequence()
        } else {
            0
        };

        let waveform_a = *self.waveform_a.lock().await;
        let waveform_b = *self.waveform_b.lock().await;

        B0Command {
            sequence,
            strength_mode: StrengthMode::new(mode_a, mode_b),
            strength_a: self.target_strength_a.load(Ordering::Relaxed),
            strength_b: self.target_strength_b.load(Ordering::Relaxed),
            waveform_a,
            waveform_b,
        }
    }
}

// ============================================================================
// BLE Coyote 设备（V3 协议）
// ============================================================================

/// Coyote BLE 设备（V3 协议）
///
/// 使用 B0 指令每 100ms 发送强度和波形数据，
/// 使用 BF 指令设置软上限，接收 B1 强度反馈。
pub struct CoyoteDevice {
    /// 基础设备
    base: BaseDevice,
    /// BLE 管理器
    ble_manager: Option<Arc<BleManager>>,
    /// 协议设备
    protocol_device: Option<ProtocolBleDevice>,
    /// V3 协议共享输出状态
    output_state: Arc<V3OutputState>,
    /// 100ms 输出任务句柄
    output_task: Option<tokio::task::JoinHandle<()>>,
    /// 接收任务句柄
    receive_task: Option<tokio::task::JoinHandle<()>>,
}

impl CoyoteDevice {
    /// 创建新的 Coyote 设备
    pub fn new(id: String, name: String) -> Self {
        let base = BaseDevice::new(id, name);
        let output_state = Arc::new(V3OutputState::new());

        Self {
            base,
            ble_manager: None,
            protocol_device: None,
            output_state,
            output_task: None,
            receive_task: None,
        }
    }

    /// 使用 BLE 管理器创建设备
    pub fn with_manager(id: String, name: String, manager: Arc<BleManager>) -> Self {
        let mut device = Self::new(id, name);
        device.ble_manager = Some(manager);
        device
    }

    /// 设置协议设备
    pub fn set_protocol_device(&mut self, device: ProtocolBleDevice) {
        self.protocol_device = Some(device);
    }

    /// 发送 BF 配置指令
    ///
    /// 每次重连后必须重新发送 BF 指令设置软上限。
    async fn send_bf_config(&self, config: &BFCommand) -> Result<()> {
        let device = self
            .protocol_device
            .as_ref()
            .ok_or(CoreError::DeviceNotConnected)?;

        let data = config.encode();
        debug!("Sending BF config: {:02x?}", data);
        device.send(&data).await?;

        Ok(())
    }

    /// 启动 100ms B0 输出循环
    fn start_output_loop(&mut self) {
        if let Some(device) = self.protocol_device.clone() {
            let state = self.output_state.clone();
            let event_tx = self.base.event_tx.clone();

            let handle = tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_millis(100));

                loop {
                    interval.tick().await;

                    let cmd = state.build_b0().await;
                    let data = cmd.encode();

                    if let Err(e) = device.send(&data).await {
                        warn!("B0 send failed: {}", e);
                        let _ = event_tx.send(DeviceEvent::Error(format!("B0 send failed: {}", e)));
                        break;
                    }
                }
            });

            self.output_task = Some(handle);
        }
    }

    /// 停止输出循环
    fn stop_output_loop(&mut self) {
        if let Some(handle) = self.output_task.take() {
            handle.abort();
        }
    }

    /// 启动接收任务（监听 B1 强度反馈）
    fn start_receive_task(&mut self) {
        if let Some(device) = self.protocol_device.clone() {
            let event_tx = self.base.event_tx.clone();

            let handle = tokio::spawn(async move {
                loop {
                    match device.receive().await {
                        Ok(data) => {
                            debug!("Received notification: {:02x?}", data);
                            match NotifyMessage::parse(&data) {
                                NotifyMessage::Strength(b1) => {
                                    Self::handle_b1_response(&b1, &event_tx);
                                }
                                NotifyMessage::Unknown(data) => {
                                    debug!("Unknown notification: {:02x?}", data);
                                }
                            }
                        }
                        Err(e) => {
                            error!("BLE receive error: {}", e);
                            let _ = event_tx.send(DeviceEvent::Error(e.to_string()));
                            break;
                        }
                    }
                }
            });

            self.receive_task = Some(handle);
        }
    }

    /// 停止接收任务
    fn stop_receive_task(&mut self) {
        if let Some(handle) = self.receive_task.take() {
            handle.abort();
        }
    }

    /// 处理 B1 强度反馈
    fn handle_b1_response(response: &B1Response, event_tx: &broadcast::Sender<DeviceEvent>) {
        debug!(
            "B1 response: seq={}, strength_a={}, strength_b={}",
            response.sequence, response.strength_a, response.strength_b
        );
        let _ = event_tx.send(DeviceEvent::StatusReport {
            power_a: response.strength_a,
            power_b: response.strength_b,
        });
    }

    /// 将 WaveformConfig 转为 V3 WaveformData
    fn waveform_config_to_v3(config: &WaveformConfig) -> WaveformData {
        // V3 波形格式: 4 组 [频率, 强度]，每组 25ms
        // 简单映射: 将 WaveformConfig 的 frequency 压缩后作为频率，intensity 作为强度
        let freq = dglab_protocol::v3::compress_frequency(config.frequency);
        let intensity = config.intensity.min(100);

        match config.waveform_type {
            WaveformType::Continuous => {
                // 连续: 4 组相同
                WaveformData::uniform(freq, intensity)
            }
            WaveformType::Pulse => {
                // 脉冲: 前 2 组有输出，后 2 组静默
                WaveformData::new([freq, freq, freq, freq], [intensity, intensity, 0, 0])
            }
            WaveformType::Sawtooth => {
                // 锯齿: 强度递增
                let step = intensity / 4;
                WaveformData::new([freq; 4], [step, step * 2, step * 3, intensity])
            }
            WaveformType::Sine => {
                // 正弦近似: 0 -> peak -> 0 -> 0
                let half = intensity / 2;
                WaveformData::new([freq; 4], [half, intensity, half, 0])
            }
            WaveformType::Square => {
                // 方波: 全开或全关
                WaveformData::new([freq; 4], [intensity, intensity, 0, 0])
            }
            WaveformType::Triangle => {
                // 三角: 上升再下降
                let third = intensity / 3;
                WaveformData::new([freq; 4], [third, intensity, intensity, third])
            }
            WaveformType::Custom => {
                // 自定义: 如果有 custom_data 且足够长度则使用，否则默认均匀
                if let Some(ref data) = config.custom_data {
                    if data.len() >= 8 {
                        WaveformData::new(
                            [data[0], data[1], data[2], data[3]],
                            [data[4], data[5], data[6], data[7]],
                        )
                    } else {
                        WaveformData::uniform(freq, intensity)
                    }
                } else {
                    WaveformData::uniform(freq, intensity)
                }
            }
        }
    }
}

#[async_trait]
impl Device for CoyoteDevice {
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
        DeviceInfo {
            id: self.base.id().to_string(),
            name: self.base.name().to_string(),
            device_type: "Coyote V3".to_string(),
            firmware_version: String::new(),
            hardware_version: String::new(),
            battery_level: 0, // 通过 BLE 电池特征单独读取
            power_a: self.output_state.target_strength_a.load(Ordering::Relaxed),
            power_b: self.output_state.target_strength_b.load(Ordering::Relaxed),
            max_power_a: MAX_STRENGTH,
            max_power_b: MAX_STRENGTH,
        }
    }

    async fn connect(&mut self) -> Result<()> {
        info!("Connecting to Coyote V3 device: {}", self.base.id());

        if self.base.state() == DeviceState::Connected {
            return Ok(());
        }

        self.base.set_state(DeviceState::Connecting);

        // 如果还没有 protocol_device，且有 BLE 管理器，使用它连接
        if self.protocol_device.is_none() {
            if let Some(manager) = &self.ble_manager {
                let device = manager.connect(self.base.id()).await?;
                self.protocol_device = Some(device);
            } else {
                return Err(CoreError::DeviceNotConnected);
            }
        }

        // 连接后发送 BF 配置（设置软上限为最大值）
        let bf = BFCommand::default_config();
        self.send_bf_config(&bf).await?;

        self.base.set_state(DeviceState::Connected);

        // 启动接收任务
        self.start_receive_task();

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        info!("Disconnecting Coyote V3 device: {}", self.base.id());

        self.stop_output_loop();
        self.stop_receive_task();

        if let Some(device) = &self.protocol_device {
            device.disconnect().await?;
        }

        self.protocol_device = None;
        self.base.set_state(DeviceState::Disconnected);

        Ok(())
    }

    async fn start(&mut self) -> Result<()> {
        info!("Starting Coyote V3 output: {}", self.base.id());

        if self.base.state() != DeviceState::Connected {
            return Err(CoreError::DeviceNotConnected);
        }

        // 启动 100ms B0 输出循环
        self.start_output_loop();
        self.base.set_state(DeviceState::Running);

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping Coyote V3 output: {}", self.base.id());

        if self.base.state() != DeviceState::Running {
            return Ok(());
        }

        // 停止输出循环
        self.stop_output_loop();

        // 重置强度和波形
        self.output_state
            .target_strength_a
            .store(0, Ordering::Relaxed);
        self.output_state
            .target_strength_b
            .store(0, Ordering::Relaxed);
        *self.output_state.waveform_a.lock().await = WaveformData::silent();
        *self.output_state.waveform_b.lock().await = WaveformData::silent();

        self.base.set_state(DeviceState::Connected);

        Ok(())
    }

    async fn set_power(&mut self, channel: u8, power: u8) -> Result<()> {
        debug!("Setting V3 channel {} power to {}", channel, power);

        if power > MAX_STRENGTH {
            return Err(CoreError::PowerOutOfRange(power, MAX_STRENGTH));
        }

        match channel {
            0 => {
                self.output_state
                    .target_strength_a
                    .store(power, Ordering::Relaxed);
                self.output_state
                    .pending_strength_a
                    .store(true, Ordering::Relaxed);
            }
            1 => {
                self.output_state
                    .target_strength_b
                    .store(power, Ordering::Relaxed);
                self.output_state
                    .pending_strength_b
                    .store(true, Ordering::Relaxed);
            }
            _ => return Err(CoreError::InvalidParameter("Invalid channel".to_string())),
        }

        // 更新 BaseDevice 的强度值（用于事件通知）
        // 注意: V3 最大强度 200，但 BaseDevice 默认 max 100，需要兼容
        let _ = self
            .base
            .set_power(channel, power.min(self.base.power_a().max(power)));

        Ok(())
    }

    fn get_power(&self, channel: u8) -> u8 {
        match channel {
            0 => self.output_state.target_strength_a.load(Ordering::Relaxed),
            1 => self.output_state.target_strength_b.load(Ordering::Relaxed),
            _ => 0,
        }
    }

    async fn set_waveform(&mut self, channel: u8, config: WaveformConfig) -> Result<()> {
        debug!("Setting V3 channel {} waveform: {:?}", channel, config);

        let waveform = Self::waveform_config_to_v3(&config);

        match channel {
            0 => *self.output_state.waveform_a.lock().await = waveform,
            1 => *self.output_state.waveform_b.lock().await = waveform,
            _ => return Err(CoreError::InvalidParameter("Invalid channel".to_string())),
        }

        Ok(())
    }

    async fn heartbeat(&mut self) -> Result<()> {
        // V3 协议中，100ms B0 输出循环本身就是心跳
        // 如果未在运行状态，发送一个 NoChange 的 B0
        if self.base.state() == DeviceState::Connected {
            if let Some(device) = &self.protocol_device {
                let cmd = B0Command::waveform_only(WaveformData::silent(), WaveformData::silent());
                let data = cmd.encode();
                device.send(&data).await?;
            }
        }
        Ok(())
    }

    fn subscribe_events(&self) -> broadcast::Receiver<DeviceEvent> {
        self.base.subscribe_events()
    }
}

impl Drop for CoyoteDevice {
    fn drop(&mut self) {
        self.stop_output_loop();
        self.stop_receive_task();
    }
}

// ============================================================================
// WiFi WebSocket Coyote 设备实现
// ============================================================================

/// WiFi WebSocket Coyote 设备（共享状态）
struct WsCoyoteInner {
    /// WebSocket 客户端
    ws_client: Mutex<Option<dglab_protocol::wifi::WsClient>>,
    /// 服务器 URL
    server_url: String,
}

/// WiFi WebSocket Coyote 设备
pub struct WsCoyoteDevice {
    /// 基础设备
    base: BaseDevice,
    /// 内部状态（Arc 包装，可跨任务共享）
    inner: Arc<WsCoyoteInner>,
    /// 心跳任务句柄
    heartbeat_task: Option<tokio::task::JoinHandle<()>>,
    /// 接收任务句柄
    receive_task: Option<tokio::task::JoinHandle<()>>,
}

impl WsCoyoteDevice {
    /// 创建新的 WiFi 设备（使用官方服务器）
    pub fn new(id: String, name: String) -> Self {
        Self::with_server(id, name, dglab_protocol::wifi::OFFICIAL_SERVER.to_string())
    }

    /// 创建新的 WiFi 设备（使用自定义服务器）
    pub fn with_server(id: String, name: String, server_url: String) -> Self {
        let base = BaseDevice::new(id, name);
        let inner = Arc::new(WsCoyoteInner {
            ws_client: Mutex::new(None),
            server_url,
        });

        Self {
            base,
            inner,
            heartbeat_task: None,
            receive_task: None,
        }
    }

    /// 获取二维码 URL（连接后可用）
    pub async fn qr_url(&self) -> Option<String> {
        let client = self.inner.ws_client.lock().await;
        if let Some(c) = client.as_ref() {
            c.official_qr_url().await
        } else {
            None
        }
    }

    /// 检查是否已绑定到 APP
    pub async fn is_bound(&self) -> bool {
        let client = self.inner.ws_client.lock().await;
        if let Some(c) = client.as_ref() {
            c.is_bound().await
        } else {
            false
        }
    }

    /// 获取服务器 URL
    pub fn server_url(&self) -> &str {
        &self.inner.server_url
    }

    /// 启动心跳任务
    fn start_heartbeat(&mut self) {
        let inner = self.inner.clone();
        let event_tx = self.base.event_tx.clone();
        let state = self.base.state();

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));

            loop {
                interval.tick().await;

                if state != DeviceState::Connected && state != DeviceState::Running {
                    break;
                }

                let client = inner.ws_client.lock().await;
                if let Some(c) = client.as_ref() {
                    if let Err(e) = c.send_heartbeat().await {
                        warn!("WebSocket heartbeat failed: {}", e);
                        let _ =
                            event_tx.send(DeviceEvent::Error(format!("Heartbeat failed: {}", e)));
                    }
                }
            }
        });

        self.heartbeat_task = Some(handle);
    }

    /// 停止心跳任务
    fn stop_heartbeat(&mut self) {
        if let Some(handle) = self.heartbeat_task.take() {
            handle.abort();
        }
    }

    /// 启动接收任务
    fn start_receive_task(&mut self) {
        let inner = self.inner.clone();
        let event_tx = self.base.event_tx.clone();
        let mut power_a = self.base.power_a();
        let mut power_b = self.base.power_b();

        let handle = tokio::spawn(async move {
            loop {
                let mut client = inner.ws_client.lock().await;
                let Some(c) = client.as_mut() else {
                    break;
                };

                match c.recv_event().await {
                    Ok(Some(event)) => {
                        Self::handle_ws_event(event, &event_tx, &mut power_a, &mut power_b);
                    }
                    Ok(None) => {
                        debug!("WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        error!("WebSocket receive error: {}", e);
                        let _ = event_tx.send(DeviceEvent::Error(e.to_string()));
                        break;
                    }
                }
            }
        });

        self.receive_task = Some(handle);
    }

    /// 停止接收任务
    fn stop_receive_task(&mut self) {
        if let Some(handle) = self.receive_task.take() {
            handle.abort();
        }
    }

    /// 处理 WebSocket 事件
    fn handle_ws_event(
        event: dglab_protocol::wifi::WsEvent,
        event_tx: &broadcast::Sender<DeviceEvent>,
        power_a: &mut u8,
        power_b: &mut u8,
    ) {
        match event {
            dglab_protocol::wifi::WsEvent::ClientId(_) => {
                debug!("Received client ID");
            }
            dglab_protocol::wifi::WsEvent::Bound(target_id) => {
                info!("Bound to target: {}", target_id);
                let _ = event_tx.send(DeviceEvent::InfoUpdated(DeviceInfo {
                    id: String::new(),
                    name: String::new(),
                    device_type: "Coyote-WiFi".to_string(),
                    firmware_version: String::new(),
                    hardware_version: String::new(),
                    battery_level: 100,
                    power_a: *power_a,
                    power_b: *power_b,
                    max_power_a: 100,
                    max_power_b: 100,
                }));
            }
            dglab_protocol::wifi::WsEvent::Strength(data) => {
                *power_a = data.strength_a;
                *power_b = data.strength_b;
                let _ = event_tx.send(DeviceEvent::StatusReport {
                    power_a: *power_a,
                    power_b: *power_b,
                });
            }
            dglab_protocol::wifi::WsEvent::Feedback(button) => {
                debug!("Feedback button pressed: {:?}", button);
            }
            dglab_protocol::wifi::WsEvent::PeerDisconnected => {
                info!("Peer disconnected");
                let _ = event_tx.send(DeviceEvent::Error("Peer disconnected".to_string()));
            }
            dglab_protocol::wifi::WsEvent::Error(code) => {
                warn!("WebSocket error: {:?}", code);
                let _ = event_tx.send(DeviceEvent::Error(format!("{:?}", code)));
            }
            dglab_protocol::wifi::WsEvent::Heartbeat => {
                debug!("Heartbeat received");
            }
            dglab_protocol::wifi::WsEvent::Other(msg) => {
                debug!("Other message: {:?}", msg);
            }
            dglab_protocol::wifi::WsEvent::BindTimeout => {
                warn!("WebSocket bind timeout");
            }
            dglab_protocol::wifi::WsEvent::Closed => {
                info!("WebSocket connection closed");
            }
        }
    }

    /// 发送强度操作
    async fn send_strength_operation(
        &self,
        op: dglab_protocol::wifi::StrengthOperation,
    ) -> Result<()> {
        let client = self.inner.ws_client.lock().await;
        let c = client.as_ref().ok_or(CoreError::DeviceNotConnected)?;

        c.send_strength_operation(op)
            .await
            .map_err(|e| CoreError::Other(format!("WebSocket send error: {}", e)))?;

        Ok(())
    }
}

#[async_trait]
impl Device for WsCoyoteDevice {
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
        DeviceInfo {
            id: self.base.id().to_string(),
            name: self.base.name().to_string(),
            device_type: "Coyote-WiFi".to_string(),
            firmware_version: String::new(),
            hardware_version: String::new(),
            battery_level: 100,
            power_a: self.base.power_a(),
            power_b: self.base.power_b(),
            max_power_a: 100,
            max_power_b: 100,
        }
    }

    async fn connect(&mut self) -> Result<()> {
        info!("Connecting to WiFi server: {}", self.inner.server_url);

        if self.base.state() == DeviceState::Connected {
            return Ok(());
        }

        self.base.set_state(DeviceState::Connecting);

        // 连接 WebSocket
        let client = dglab_protocol::wifi::WsClient::connect(&self.inner.server_url)
            .await
            .map_err(|e| CoreError::Other(format!("WebSocket connect error: {}", e)))?;

        {
            let mut ws_client = self.inner.ws_client.lock().await;
            *ws_client = Some(client);
        }

        self.base.set_state(DeviceState::Connected);

        // 启动后台任务
        self.start_receive_task();
        self.start_heartbeat();

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        info!("Disconnecting WiFi device: {}", self.base.id());

        self.stop_heartbeat();
        self.stop_receive_task();

        {
            let client = self.inner.ws_client.lock().await;
            if let Some(c) = client.as_ref() {
                let _ = c.close().await;
            }
        }

        {
            let mut ws_client = self.inner.ws_client.lock().await;
            *ws_client = None;
        }

        self.base.set_state(DeviceState::Disconnected);

        Ok(())
    }

    async fn start(&mut self) -> Result<()> {
        info!("Starting WiFi device output: {}", self.base.id());

        if self.base.state() != DeviceState::Connected {
            return Err(CoreError::DeviceNotConnected);
        }

        // WiFi 模式下，start 不发送特殊指令，只是更新状态
        self.base.set_state(DeviceState::Running);

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping WiFi device output: {}", self.base.id());

        if self.base.state() != DeviceState::Running {
            return Ok(());
        }

        // 停止时将强度归零
        self.set_power(0, 0).await?;
        self.set_power(1, 0).await?;

        self.base.set_state(DeviceState::Connected);

        Ok(())
    }

    async fn set_power(&mut self, channel: u8, power: u8) -> Result<()> {
        debug!("Setting WiFi channel {} power to {}", channel, power);

        self.base.set_power(channel, power)?;

        let ws_channel = match channel {
            0 => dglab_protocol::wifi::Channel::A,
            1 => dglab_protocol::wifi::Channel::B,
            _ => return Err(CoreError::InvalidParameter("Invalid channel".to_string())),
        };

        let op = dglab_protocol::wifi::StrengthOperation::set(ws_channel, power);

        if self.base.state() == DeviceState::Connected || self.base.state() == DeviceState::Running
        {
            self.send_strength_operation(op).await?;
        }

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
        debug!("Setting WiFi channel {} waveform: {:?}", channel, config);

        // WiFi 模式通过 pulse 数据发送波形
        let ws_channel = match channel {
            0 => dglab_protocol::wifi::Channel::A,
            1 => dglab_protocol::wifi::Channel::B,
            _ => return Err(CoreError::InvalidParameter("Invalid channel".to_string())),
        };

        // 创建简单的脉冲数据
        let power_a = if channel == 0 {
            config.intensity
        } else {
            self.base.power_a()
        };
        let power_b = if channel == 1 {
            config.intensity
        } else {
            self.base.power_b()
        };
        let pulse =
            dglab_protocol::wifi::PulseData::from_strength(ws_channel, power_a, power_b, 1000);

        let client = self.inner.ws_client.lock().await;
        if let Some(c) = client.as_ref() {
            c.send_pulse(pulse)
                .await
                .map_err(|e| CoreError::Other(format!("WebSocket send pulse error: {}", e)))?;
        }

        Ok(())
    }

    async fn heartbeat(&mut self) -> Result<()> {
        let client = self.inner.ws_client.lock().await;
        if let Some(c) = client.as_ref() {
            c.send_heartbeat()
                .await
                .map_err(|e| CoreError::Other(format!("Heartbeat error: {}", e)))?;
        }
        Ok(())
    }

    fn subscribe_events(&self) -> broadcast::Receiver<DeviceEvent> {
        self.base.subscribe_events()
    }
}

impl Drop for WsCoyoteDevice {
    fn drop(&mut self) {
        self.stop_heartbeat();
        self.stop_receive_task();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === V3OutputState 测试 ===

    #[test]
    fn test_v3_output_state_new() {
        let state = V3OutputState::new();
        assert_eq!(state.target_strength_a.load(Ordering::Relaxed), 0);
        assert_eq!(state.target_strength_b.load(Ordering::Relaxed), 0);
        assert!(!state.pending_strength_a.load(Ordering::Relaxed));
        assert!(!state.pending_strength_b.load(Ordering::Relaxed));
    }

    #[test]
    fn test_v3_output_state_next_sequence() {
        let state = V3OutputState::new();
        let s1 = state.next_sequence();
        let s2 = state.next_sequence();
        let s3 = state.next_sequence();
        // 序列号应在 1~15 范围内
        assert!((1..=15).contains(&s1));
        assert!((1..=15).contains(&s2));
        assert!((1..=15).contains(&s3));
        // 应递增
        assert_ne!(s1, s2);
    }

    #[tokio::test]
    async fn test_v3_output_state_build_b0_no_change() {
        let state = V3OutputState::new();
        let cmd = state.build_b0().await;

        assert_eq!(cmd.sequence, 0); // 无强度变更，序列号为 0
        assert_eq!(cmd.strength_mode, StrengthMode::both_no_change());
    }

    #[tokio::test]
    async fn test_v3_output_state_build_b0_with_strength_change() {
        let state = V3OutputState::new();
        state.target_strength_a.store(50, Ordering::Relaxed);
        state.pending_strength_a.store(true, Ordering::Relaxed);

        let cmd = state.build_b0().await;

        assert_ne!(cmd.sequence, 0); // 有变更，应有序列号
        assert_eq!(cmd.strength_mode.channel_a, ChannelStrengthMode::Absolute);
        assert_eq!(cmd.strength_mode.channel_b, ChannelStrengthMode::NoChange);
        assert_eq!(cmd.strength_a, 50);

        // pending 应被消耗
        assert!(!state.pending_strength_a.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn test_v3_output_state_build_b0_both_channels() {
        let state = V3OutputState::new();
        state.target_strength_a.store(30, Ordering::Relaxed);
        state.target_strength_b.store(60, Ordering::Relaxed);
        state.pending_strength_a.store(true, Ordering::Relaxed);
        state.pending_strength_b.store(true, Ordering::Relaxed);

        let cmd = state.build_b0().await;

        assert_eq!(cmd.strength_mode.channel_a, ChannelStrengthMode::Absolute);
        assert_eq!(cmd.strength_mode.channel_b, ChannelStrengthMode::Absolute);
        assert_eq!(cmd.strength_a, 30);
        assert_eq!(cmd.strength_b, 60);
    }

    #[tokio::test]
    async fn test_v3_output_state_build_b0_with_waveform() {
        let state = V3OutputState::new();
        let waveform = WaveformData::uniform(50, 80);
        *state.waveform_a.lock().await = waveform;

        let cmd = state.build_b0().await;
        assert_eq!(cmd.waveform_a, waveform);
    }

    // === CoyoteDevice 测试 ===

    #[test]
    fn test_coyote_new() {
        let dev = CoyoteDevice::new("dev-1".to_string(), "Test Coyote".to_string());
        assert_eq!(dev.id(), "dev-1");
        assert_eq!(dev.name(), "Test Coyote");
        assert_eq!(dev.state(), DeviceState::Disconnected);
        assert_eq!(dev.get_power(0), 0);
        assert_eq!(dev.get_power(1), 0);
    }

    #[test]
    fn test_coyote_info() {
        let dev = CoyoteDevice::new("dev-1".to_string(), "Test".to_string());
        let info = dev.info();
        assert_eq!(info.id, "dev-1");
        assert_eq!(info.device_type, "Coyote V3");
        assert_eq!(info.max_power_a, MAX_STRENGTH);
        assert_eq!(info.max_power_b, MAX_STRENGTH);
    }

    #[tokio::test]
    async fn test_coyote_set_power() {
        let mut dev = CoyoteDevice::new("dev-1".to_string(), "Test".to_string());
        dev.set_power(0, 100).await.unwrap();
        assert_eq!(dev.get_power(0), 100);

        dev.set_power(1, 150).await.unwrap();
        assert_eq!(dev.get_power(1), 150);
    }

    #[tokio::test]
    async fn test_coyote_set_power_triggers_pending() {
        let mut dev = CoyoteDevice::new("dev-1".to_string(), "Test".to_string());
        dev.set_power(0, 50).await.unwrap();
        assert!(dev.output_state.pending_strength_a.load(Ordering::Relaxed));

        dev.set_power(1, 60).await.unwrap();
        assert!(dev.output_state.pending_strength_b.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn test_coyote_set_power_exceeds_max() {
        let mut dev = CoyoteDevice::new("dev-1".to_string(), "Test".to_string());
        let result = dev.set_power(0, 201).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_coyote_set_power_invalid_channel() {
        let mut dev = CoyoteDevice::new("dev-1".to_string(), "Test".to_string());
        let result = dev.set_power(2, 50).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_coyote_set_waveform() {
        let mut dev = CoyoteDevice::new("dev-1".to_string(), "Test".to_string());
        let config = WaveformConfig::default();
        dev.set_waveform(0, config).await.unwrap();

        let waveform = *dev.output_state.waveform_a.lock().await;
        // Continuous + default freq 100 → compress_frequency(100) = 100
        assert_eq!(waveform, WaveformData::uniform(100, 50));
    }

    #[tokio::test]
    async fn test_coyote_set_waveform_invalid_channel() {
        let mut dev = CoyoteDevice::new("dev-1".to_string(), "Test".to_string());
        let result = dev.set_waveform(2, WaveformConfig::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_coyote_start_without_connect_fails() {
        let mut dev = CoyoteDevice::new("dev-1".to_string(), "Test".to_string());
        let result = dev.start().await;
        assert!(result.is_err());
    }

    #[test]
    fn test_coyote_get_power_invalid_channel() {
        let dev = CoyoteDevice::new("dev-1".to_string(), "Test".to_string());
        assert_eq!(dev.get_power(2), 0);
    }

    // === WaveformConfig → V3 转换测试 ===

    #[test]
    fn test_waveform_config_to_v3_continuous() {
        let config = WaveformConfig {
            waveform_type: WaveformType::Continuous,
            frequency: 50,
            pulse_width: 200,
            intensity: 80,
            custom_data: None,
        };
        let v3 = CoyoteDevice::waveform_config_to_v3(&config);
        let freq = dglab_protocol::v3::compress_frequency(50);
        assert_eq!(v3, WaveformData::uniform(freq, 80));
    }

    #[test]
    fn test_waveform_config_to_v3_pulse() {
        let config = WaveformConfig {
            waveform_type: WaveformType::Pulse,
            frequency: 100,
            pulse_width: 200,
            intensity: 60,
            custom_data: None,
        };
        let v3 = CoyoteDevice::waveform_config_to_v3(&config);
        assert_eq!(v3.intensity[0], 60);
        assert_eq!(v3.intensity[1], 60);
        assert_eq!(v3.intensity[2], 0);
        assert_eq!(v3.intensity[3], 0);
    }

    #[test]
    fn test_waveform_config_to_v3_custom_with_data() {
        let config = WaveformConfig {
            waveform_type: WaveformType::Custom,
            frequency: 100,
            pulse_width: 200,
            intensity: 50,
            custom_data: Some(vec![20, 30, 40, 50, 10, 20, 30, 40]),
        };
        let v3 = CoyoteDevice::waveform_config_to_v3(&config);
        assert_eq!(v3.frequency, [20, 30, 40, 50]);
        assert_eq!(v3.intensity, [10, 20, 30, 40]);
    }

    #[test]
    fn test_waveform_config_to_v3_custom_no_data() {
        let config = WaveformConfig {
            waveform_type: WaveformType::Custom,
            frequency: 100,
            pulse_width: 200,
            intensity: 50,
            custom_data: None,
        };
        let v3 = CoyoteDevice::waveform_config_to_v3(&config);
        // 无自定义数据，fallback 到 uniform
        let freq = dglab_protocol::v3::compress_frequency(100);
        assert_eq!(v3, WaveformData::uniform(freq, 50));
    }

    // === WsCoyoteDevice 测试 ===

    #[test]
    fn test_ws_coyote_new() {
        let dev = WsCoyoteDevice::new("ws-1".to_string(), "WiFi Device".to_string());
        assert_eq!(dev.id(), "ws-1");
        assert_eq!(dev.name(), "WiFi Device");
        assert_eq!(dev.state(), DeviceState::Disconnected);
        assert_eq!(dev.server_url(), dglab_protocol::wifi::OFFICIAL_SERVER);
    }

    #[test]
    fn test_ws_coyote_with_server() {
        let dev = WsCoyoteDevice::with_server(
            "ws-2".to_string(),
            "Custom Server".to_string(),
            "ws://localhost:1234".to_string(),
        );
        assert_eq!(dev.server_url(), "ws://localhost:1234");
    }

    #[test]
    fn test_ws_coyote_info() {
        let dev = WsCoyoteDevice::new("ws-1".to_string(), "WiFi".to_string());
        let info = dev.info();
        assert_eq!(info.device_type, "Coyote-WiFi");
        assert_eq!(info.power_a, 0);
        assert_eq!(info.power_b, 0);
    }

    #[tokio::test]
    async fn test_ws_coyote_start_without_connect_fails() {
        let mut dev = WsCoyoteDevice::new("ws-1".to_string(), "WiFi".to_string());
        let result = dev.start().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ws_coyote_qr_url_not_connected() {
        let dev = WsCoyoteDevice::new("ws-1".to_string(), "WiFi".to_string());
        assert!(dev.qr_url().await.is_none());
    }

    #[tokio::test]
    async fn test_ws_coyote_is_bound_not_connected() {
        let dev = WsCoyoteDevice::new("ws-1".to_string(), "WiFi".to_string());
        assert!(!dev.is_bound().await);
    }
}
