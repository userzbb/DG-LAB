# DG-LAB 架构设计文档

## 目录

- [概述](#概述)
- [系统架构](#系统架构)
- [模块设计](#模块设计)
- [数据流](#数据流)
- [协议规范](#协议规范)
- [状态管理](#状态管理)
- [事件系统](#事件系统)
- [API 参考](#api-参考)

---

## 概述

DG-LAB 是一个用 Rust 构建的跨平台 DG-LAB 设备控制器,支持通过蓝牙低功耗(BLE)和 WiFi 与 DG-LAB Coyote 3.0 设备通信。项目采用分层架构设计,将协议实现、核心业务逻辑、用户界面分离,便于维护和扩展。

### 技术栈

**后端 (Rust)**
- **异步运行时**: tokio (完全特性支持)
- **BLE 通信**: btleplug
- **WebSocket**: tokio-tungstenite
- **序列化**: serde, serde_json, bincode
- **错误处理**: thiserror, anyhow
- **日志**: tracing, tracing-subscriber
- **GUI 框架**: Tauri 2.0

**前端 (TypeScript/React)**
- **框架**: React 19
- **构建工具**: Vite 5
- **语言**: TypeScript 5
- **样式**: Tailwind CSS v4
- **UI 组件**: shadcn-ui
- **状态管理**: Zustand
- **通知**: Sonner

---

## 系统架构

### 分层架构图

```
┌─────────────────────────────────────────────────────────────┐
│                     用户界面层 (UI Layer)                    │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────────┐  ┌──────────────────┐  ┌────────────┐ │
│  │   GUI (Tauri)    │  │   CLI (Clap)     │  │ TUI (Ratatui) │
│  │                  │  │                  │  │            │ │
│  │ React + Tailwind │  │ 命令行参数解析    │  │ 实时交互界面│ │
│  │ Zustand 状态管理 │  │ 简单命令执行      │  │ 键盘快捷键  │ │
│  └──────────────────┘  └──────────────────┘  └────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            ↕ Tauri Commands / Direct API
┌─────────────────────────────────────────────────────────────┐
│                   核心业务逻辑层 (Core Layer)                 │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────┐   │
│  │              SessionManager (会话管理器)               │   │
│  │  - 设备生命周期管理                                     │   │
│  │  - 多设备并发控制                                      │   │
│  │  - 事件分发与监听                                      │   │
│  └──────────────────────────────────────────────────────┘   │
│           ↕                    ↕                    ↕         │
│  ┌──────────────┐   ┌──────────────────┐   ┌──────────────┐ │
│  │    Device    │   │ WaveformGenerator│   │PresetStorage │ │
│  │   设备抽象    │   │    波形生成       │   │  预设管理    │ │
│  │              │   │                  │   │              │ │
│  │ - Trait API  │   │ - 内置波形模板    │   │ - 保存/加载  │ │
│  │ - 状态机     │   │ - 自定义波形      │   │ - 分类管理   │ │
│  └──────────────┘   └──────────────────┘   └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
                                ↕
┌─────────────────────────────────────────────────────────────┐
│                  协议层 (Protocol Layer)                     │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────────────┐       ┌─────────────────────────┐ │
│  │   BLE Protocol (V3)  │       │   WiFi Protocol (WS)    │ │
│  │                      │       │                         │ │
│  │ - 设备扫描与发现      │       │ - WebSocket 连接         │ │
│  │ - GATT 特征读写      │       │ - 消息序列化             │ │
│  │ - 数据包编解码       │       │ - 心跳保活               │ │
│  │ - 连接状态管理       │       │ - 断线重连               │ │
│  └──────────────────────┘       └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                                ↕
┌─────────────────────────────────────────────────────────────┐
│                   硬件抽象层 (HAL)                           │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐               ┌────────────────────┐   │
│  │   btleplug      │               │ tokio-tungstenite  │   │
│  │  (BLE 库)       │               │   (WebSocket 库)   │   │
│  └─────────────────┘               └────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### 工作空间结构

```
DG_LAB/
├── crates/                      # Rust 库
│   ├── dglab-protocol/          # 协议实现
│   ├── dglab-core/              # 核心业务逻辑
│   └── dglab-cli/               # 命令行工具
├── apps/                        # 应用程序
│   └── dglab-gui-tauri/         # Tauri + React GUI
│       ├── src/                 # React 前端源码
│       └── src-tauri/           # Rust 后端源码
├── docs/                        # 文档
├── examples/                    # 示例代码
└── scripts/                     # 构建脚本
```

---

## 模块设计

### 1. dglab-protocol (协议层)

**职责**: 实现与 DG-LAB 设备的底层通信协议

**模块组成**:

```rust
dglab-protocol/
├── src/
│   ├── lib.rs              // 库入口
│   ├── error.rs            // 错误定义
│   ├── ble/                // BLE 通信模块
│   │   ├── mod.rs          // BLE 模块入口
│   │   ├── scanner.rs      // 设备扫描
│   │   └── connection.rs   // 连接管理
│   ├── v3/                 // V3 协议实现 (推荐)
│   │   ├── mod.rs          // V3 模块入口
│   │   ├── device.rs       // V3 设备实现
│   │   ├── packet.rs       // 数据包编解码
│   │   └── constants.rs    // 协议常量
│   ├── wifi/               // WiFi WebSocket 协议
│   │   ├── mod.rs
│   │   ├── client.rs       // WebSocket 客户端
│   │   └── message.rs      // 消息格式
│   └── packet.rs           // 旧版协议 (已弃用)
```

**核心类型**:

```rust
// BLE 设备扫描器
pub struct BleScanner {
    adapter: Adapter,
    devices: Arc<RwLock<Vec<Peripheral>>>,
}

// V3 BLE 设备
pub struct V3BleDevice {
    peripheral: Peripheral,
    tx_char: Characteristic,
    rx_char: Characteristic,
    state: DeviceState,
}

// WiFi WebSocket 客户端
pub struct WifiClient {
    ws_stream: WebSocketStream<TcpStream>,
    device_id: String,
}
```

**关键 API**:

- `BleScanner::scan()` - 扫描附近的 BLE 设备
- `V3BleDevice::connect()` - 连接到设备
- `V3BleDevice::set_power()` - 设置通道功率
- `V3BleDevice::send_pulse()` - 发送脉冲数据

---

### 2. dglab-core (核心层)

**职责**: 提供设备抽象、会话管理、波形生成等高级功能

**模块组成**:

```rust
dglab-core/
├── src/
│   ├── lib.rs              // 库入口
│   ├── error.rs            // 错误定义
│   ├── device/             // 设备抽象
│   │   ├── mod.rs          // 设备模块入口
│   │   ├── traits.rs       // Device trait 定义
│   │   └── coyote.rs       // Coyote 3.0 设备实现
│   ├── session/            // 会话管理
│   │   ├── mod.rs
│   │   └── manager.rs      // SessionManager
│   ├── waveform/           // 波形生成
│   │   ├── mod.rs
│   │   └── generator.rs    // WaveformGenerator
│   ├── preset/             // 预设管理
│   │   ├── mod.rs
│   │   └── storage.rs      // PresetStorage
│   └── script/             // 脚本引擎 (计划中)
│       ├── mod.rs
│       └── engine.rs
```

**核心 Trait**:

```rust
/// 设备抽象 Trait
#[async_trait]
pub trait Device: Send + Sync {
    /// 连接设备
    async fn connect(&mut self) -> Result<()>;
    
    /// 断开连接
    async fn disconnect(&mut self) -> Result<()>;
    
    /// 设置通道功率 (0-200)
    async fn set_power(&mut self, channel_a: u8, channel_b: u8) -> Result<()>;
    
    /// 发送脉冲数据
    async fn send_pulse(&mut self, channel_a: Vec<u8>, channel_b: Vec<u8>) -> Result<()>;
    
    /// 获取设备信息
    fn info(&self) -> &DeviceInfo;
    
    /// 获取当前状态
    fn state(&self) -> DeviceState;
    
    /// 订阅设备事件
    fn subscribe(&self) -> broadcast::Receiver<DeviceEvent>;
}
```

**会话管理器**:

```rust
/// 会话管理器 - 统一管理所有设备
pub struct SessionManager {
    /// 设备映射表: device_id -> Device
    devices: Arc<RwLock<HashMap<String, Arc<RwLock<Box<dyn Device>>>>>>,
    
    /// 事件广播通道
    event_tx: broadcast::Sender<SessionEvent>,
    
    /// 会话 ID
    session_id: String,
}

impl SessionManager {
    /// 添加设备到会话
    pub async fn add_device(&self, device: Box<dyn Device>) -> Result<String>;
    
    /// 移除设备
    pub async fn remove_device(&self, device_id: &str) -> Result<()>;
    
    /// 获取设备列表
    pub async fn list_devices(&self) -> Vec<DeviceInfo>;
    
    /// 控制设备功率
    pub async fn control_device(&self, device_id: &str, power_a: u8, power_b: u8) -> Result<()>;
}
```

**波形生成器**:

```rust
/// 波形生成器
pub struct WaveformGenerator;

impl WaveformGenerator {
    /// 生成正弦波
    pub fn sine_wave(frequency: f32, amplitude: u8, duration_ms: u32) -> Vec<u8>;
    
    /// 生成方波
    pub fn square_wave(frequency: f32, amplitude: u8, duty_cycle: f32) -> Vec<u8>;
    
    /// 生成锯齿波
    pub fn sawtooth_wave(frequency: f32, amplitude: u8) -> Vec<u8>;
    
    /// 生成随机波形
    pub fn random_wave(min: u8, max: u8, length: usize) -> Vec<u8>;
}
```

---

### 3. dglab-cli (命令行工具)

**职责**: 提供 CLI 和 TUI 用户界面

**模块组成**:

```rust
dglab-cli/
├── src/
│   ├── main.rs             // CLI 入口
│   ├── commands/           // 子命令实现
│   │   ├── mod.rs
│   │   ├── scan.rs         // 扫描设备
│   │   ├── connect.rs      // 连接设备
│   │   ├── control.rs      // 控制设备
│   │   └── tui.rs          // 启动 TUI
│   └── tui/                // TUI 实现
│       ├── mod.rs
│       ├── app.rs          // TUI 应用
│       └── ui.rs           // UI 渲染
```

**CLI 命令**:

```bash
dglab scan                          # 扫描设备
dglab connect [DEVICE_ID]           # 连接设备
dglab control --power-a 50 --power-b 30  # 控制功率
dglab tui                           # 启动 TUI
```

---

### 4. dglab-gui-tauri (桌面 GUI)

**职责**: 提供跨平台桌面图形界面

**前端架构** (`src/`):

```
src/
├── main.tsx                // React 入口
├── App.tsx                 // 根组件
├── pages/                  // 页面组件
│   ├── Dashboard.tsx       // 仪表盘
│   ├── DeviceScanner.tsx   // 设备扫描
│   ├── PowerControl.tsx    // 功率控制
│   ├── WaveformGenerator.tsx  // 波形生成
│   └── PresetManager.tsx   // 预设管理
├── stores/                 // Zustand 状态管理
│   ├── appStore.ts         // 应用全局状态
│   ├── deviceStore.ts      // 设备状态
│   ├── waveformStore.ts    // 波形状态
│   └── presetStore.ts      // 预设状态
├── components/             // UI 组件
│   └── ui/                 // shadcn-ui 组件
└── lib/                    // 工具函数
    └── tauri.ts            // Tauri API 封装
```

**后端架构** (`src-tauri/src/`):

```rust
src-tauri/src/
├── lib.rs                  // Tauri 应用入口
├── main.rs                 // 主函数
├── state.rs                // 应用状态
├── events.rs               // 事件定义
└── commands/               // Tauri 命令
    ├── mod.rs
    ├── device.rs           // 设备操作命令
    ├── power.rs            // 功率控制命令
    └── session.rs          // 会话管理命令
```

**Tauri 命令清单**:

| 命令名称 | 功能 | 参数 | 返回值 |
|---------|------|------|--------|
| `scan_ble_devices` | 扫描 BLE 设备 | `timeout_secs: u64` | `Vec<DeviceInfo>` |
| `connect_device` | 连接设备 | `device_id: String` | `Result<()>` |
| `disconnect_device` | 断开设备 | `device_id: String` | `Result<()>` |
| `get_device_info` | 获取设备信息 | `device_id: String` | `DeviceInfo` |
| `get_device_state` | 获取设备状态 | `device_id: String` | `DeviceState` |
| `set_power` | 设置功率 | `device_id, power_a, power_b` | `Result<()>` |
| `start_device` | 启动设备 | `device_id: String` | `Result<()>` |
| `stop_device` | 停止设备 | `device_id: String` | `Result<()>` |
| `emergency_stop` | 紧急停止 | 无 | `Result<()>` |
| `get_session_info` | 获取会话信息 | 无 | `SessionInfo` |
| `list_devices` | 列出所有设备 | 无 | `Vec<DeviceInfo>` |

---

## 数据流

### 1. 设备连接流程

```
┌─────────────┐
│  用户点击连接 │
└──────┬──────┘
       │
       ↓
┌─────────────────────────┐
│ React: deviceStore      │
│ .connectDevice(id)      │
└──────┬──────────────────┘
       │ invoke("connect_device")
       ↓
┌─────────────────────────┐
│ Tauri Command:          │
│ connect_device()        │
└──────┬──────────────────┘
       │
       ↓
┌─────────────────────────┐
│ SessionManager:         │
│ .add_device()           │
└──────┬──────────────────┘
       │
       ↓
┌─────────────────────────┐
│ Device Trait:           │
│ .connect()              │
└──────┬──────────────────┘
       │
       ↓
┌─────────────────────────┐
│ V3BleDevice:            │
│ - GATT 连接             │
│ - 特征发现              │
│ - 订阅通知              │
└──────┬──────────────────┘
       │
       ↓ (成功)
┌─────────────────────────┐
│ 触发事件:                │
│ DeviceEvent::Connected  │
└──────┬──────────────────┘
       │
       ↓
┌─────────────────────────┐
│ Tauri Event Emit:       │
│ "device:state_changed"  │
└──────┬──────────────────┘
       │
       ↓
┌─────────────────────────┐
│ React: useEffect 监听    │
│ 更新 UI 状态             │
└─────────────────────────┘
```

### 2. 功率控制流程

```
┌───────────────┐
│ 用户拖动滑块   │
└──────┬────────┘
       │
       ↓
┌────────────────────────┐
│ React: PowerControl    │
│ 调用 setPower()         │
└──────┬─────────────────┘
       │ invoke("set_power", {device_id, power_a, power_b})
       ↓
┌────────────────────────┐
│ Tauri Command:         │
│ set_power()            │
└──────┬─────────────────┘
       │
       ↓
┌────────────────────────┐
│ SessionManager:        │
│ .control_device()      │
└──────┬─────────────────┘
       │
       ↓
┌────────────────────────┐
│ Device Trait:          │
│ .set_power(a, b)       │
└──────┬─────────────────┘
       │
       ↓
┌────────────────────────┐
│ V3BleDevice:           │
│ - 构造 V3 数据包        │
│ - 写入 TX 特征          │
└──────┬─────────────────┘
       │
       ↓ (通过 BLE)
┌────────────────────────┐
│ DG-LAB 硬件设备         │
│ - 接收数据包            │
│ - 调整输出功率          │
└────────────────────────┘
```

### 3. 事件传播流程

```
┌─────────────────────┐
│  设备硬件状态变化    │
│  (电池/功率/错误)    │
└──────┬──────────────┘
       │ (BLE 通知)
       ↓
┌─────────────────────┐
│ V3BleDevice:        │
│ 接收 RX 特征通知     │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│ Device Trait:       │
│ 广播 DeviceEvent    │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│ SessionManager:     │
│ 监听并转发事件       │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│ Tauri:              │
│ emit() 到前端        │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│ React:              │
│ listen() 接收事件    │
│ 更新 Zustand store  │
│ 触发 UI 重新渲染     │
└─────────────────────┘
```

---

## 协议规范

### V3 BLE 协议

**GATT 服务**: `0000fff0-0000-1000-8000-00805f9b34fb`

**特征**:
- **TX (写入)**: `0000fff2-0000-1000-8000-00805f9b34fb`
- **RX (通知)**: `0000fff1-0000-1000-8000-00805f9b34fb`

**数据包格式**:

```
┌────┬────┬────┬────┬────┬──────┬────┐
│ 头  │类型│长度│序号│数据│ CRC  │尾  │
├────┼────┼────┼────┼────┼──────┼────┤
│0x55│0x0X│ N  │SEQ │... │CRC16 │0xAA│
└────┴────┴────┴────┴────┴──────┴────┘
```

**主要命令类型**:

| 类型码 | 名称 | 功能 | 数据格式 |
|-------|------|------|---------|
| `0x01` | 设置功率 | 设置 A/B 通道功率 | `[power_a, power_b]` |
| `0x02` | 发送脉冲 | 发送波形数据 | `[channel, ...pulse_data]` |
| `0x03` | 查询状态 | 获取设备状态 | 无 |
| `0x04` | 设置模式 | 切换工作模式 | `[mode]` |

**CRC-16 校验**: 使用 CRC-16/MODBUS 算法

### WiFi WebSocket 协议

**连接地址**: `ws://<device_ip>:8080/ws`

**消息格式** (JSON):

```json
{
  "type": "command",
  "command": "set_power",
  "data": {
    "channel_a": 50,
    "channel_b": 30
  },
  "timestamp": 1234567890
}
```

**消息类型**:
- `command` - 控制命令
- `query` - 查询请求
- `response` - 响应消息
- `event` - 事件通知

---

## 状态管理

### 设备状态机

```
       ┌─────────────┐
       │ Disconnected│
       └──────┬──────┘
              │ connect()
              ↓
       ┌─────────────┐
       │ Connecting  │
       └──────┬──────┘
              │ (成功)
              ↓
       ┌─────────────┐
   ┌───│  Connected  │───┐
   │   └──────┬──────┘   │
   │          │          │
   │ start()  │  stop()  │ disconnect()
   │          ↓          │
   │   ┌─────────────┐   │
   └──→│   Running   │───┘
       └──────┬──────┘
              │ error
              ↓
       ┌─────────────┐
       │    Error    │
       └─────────────┘
```

### Zustand Store 结构

**appStore** (应用全局状态):

```typescript
interface AppState {
  currentPage: string;           // 当前页面
  sidebarCollapsed: boolean;     // 侧边栏折叠状态
  theme: 'light' | 'dark';       // 主题
  setCurrentPage: (page: string) => void;
  toggleSidebar: () => void;
  setTheme: (theme: string) => void;
}
```

**deviceStore** (设备状态):

```typescript
interface DeviceState {
  devices: Map<string, DeviceInfo>;     // 设备列表
  selectedDeviceId: string | null;      // 当前选中设备
  scanningDevices: DeviceInfo[];        // 扫描到的设备
  isScanning: boolean;                  // 是否正在扫描
  
  // Actions
  scanDevices: (timeout: number) => Promise<void>;
  connectDevice: (id: string) => Promise<void>;
  disconnectDevice: (id: string) => Promise<void>;
  setSelectedDevice: (id: string) => void;
}
```

**waveformStore** (波形状态):

```typescript
interface WaveformState {
  waveformType: 'sine' | 'square' | 'sawtooth' | 'random';
  frequency: number;             // 频率 (Hz)
  amplitude: number;             // 振幅 (0-200)
  dutyCycle: number;             // 占空比 (0-1)
  
  // Actions
  setWaveformType: (type: string) => void;
  setFrequency: (freq: number) => void;
  generateWaveform: () => Promise<number[]>;
}
```

**presetStore** (预设状态):

```typescript
interface PresetState {
  presets: Preset[];             // 预设列表
  currentPreset: Preset | null;  // 当前预设
  
  // Actions
  loadPresets: () => Promise<void>;
  savePreset: (preset: Preset) => Promise<void>;
  deletePreset: (id: string) => Promise<void>;
  applyPreset: (id: string) => Promise<void>;
}
```

---

## 事件系统

### 后端事件定义

```rust
/// 设备事件
#[derive(Debug, Clone)]
pub enum DeviceEvent {
    /// 已连接
    Connected,
    /// 已断开
    Disconnected,
    /// 功率变更
    PowerChanged { power_a: u8, power_b: u8 },
    /// 电池更新
    BatteryUpdated(u8),
    /// 错误
    Error(String),
}

/// 会话事件
#[derive(Debug, Clone)]
pub enum SessionEvent {
    /// 设备已添加
    DeviceAdded(String),
    /// 设备已移除
    DeviceRemoved(String),
    /// 设备状态变更
    DeviceStateChanged { device_id: String, state: DeviceState },
}
```

### Tauri 事件名称

| 事件名称 | 触发时机 | Payload 类型 |
|---------|---------|-------------|
| `device:state_changed` | 设备状态变化 | `DeviceStateChangedEvent` |
| `device:power_changed` | 功率变化 | `DevicePowerChangedEvent` |
| `device:info_updated` | 设备信息更新 | `DeviceInfoUpdatedEvent` |
| `device:battery_updated` | 电池电量更新 | `DeviceBatteryUpdatedEvent` |
| `device:error` | 设备错误 | `DeviceErrorEvent` |

### 前端事件监听

```typescript
import { listen } from '@tauri-apps/api/event';

// 监听设备状态变化
await listen<DeviceStateChangedEvent>('device:state_changed', (event) => {
  console.log('Device state changed:', event.payload);
  // 更新 store
  deviceStore.updateDeviceState(event.payload.device_id, event.payload.state);
});

// 监听功率变化
await listen<DevicePowerChangedEvent>('device:power_changed', (event) => {
  console.log('Power changed:', event.payload);
  // 更新 UI
});
```

---

## API 参考

### dglab-core API

#### SessionManager

```rust
impl SessionManager {
    /// 创建新的会话管理器
    pub fn new() -> Self;
    
    /// 添加设备到会话
    /// 
    /// # 参数
    /// - `device`: 实现了 Device trait 的设备对象
    /// 
    /// # 返回
    /// - `Ok(String)`: 设备 ID
    /// - `Err(CoreError)`: 添加失败
    pub async fn add_device(&self, device: Box<dyn Device>) -> Result<String>;
    
    /// 移除设备
    /// 
    /// # 参数
    /// - `device_id`: 设备 ID
    pub async fn remove_device(&self, device_id: &str) -> Result<()>;
    
    /// 获取设备列表
    /// 
    /// # 返回
    /// 所有设备的信息列表
    pub async fn list_devices(&self) -> Vec<DeviceInfo>;
    
    /// 控制设备功率
    /// 
    /// # 参数
    /// - `device_id`: 设备 ID
    /// - `power_a`: 通道 A 功率 (0-200)
    /// - `power_b`: 通道 B 功率 (0-200)
    pub async fn control_device(
        &self,
        device_id: &str,
        power_a: u8,
        power_b: u8
    ) -> Result<()>;
    
    /// 订阅会话事件
    /// 
    /// # 返回
    /// 事件接收器
    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent>;
}
```

#### WaveformGenerator

```rust
impl WaveformGenerator {
    /// 生成正弦波
    /// 
    /// # 参数
    /// - `frequency`: 频率 (Hz)
    /// - `amplitude`: 振幅 (0-200)
    /// - `duration_ms`: 持续时间 (毫秒)
    /// 
    /// # 返回
    /// 波形数据点数组
    pub fn sine_wave(
        frequency: f32,
        amplitude: u8,
        duration_ms: u32
    ) -> Vec<u8>;
    
    /// 生成方波
    /// 
    /// # 参数
    /// - `frequency`: 频率 (Hz)
    /// - `amplitude`: 振幅 (0-200)
    /// - `duty_cycle`: 占空比 (0.0-1.0)
    pub fn square_wave(
        frequency: f32,
        amplitude: u8,
        duty_cycle: f32
    ) -> Vec<u8>;
    
    /// 生成锯齿波
    pub fn sawtooth_wave(frequency: f32, amplitude: u8) -> Vec<u8>;
    
    /// 生成随机波形
    /// 
    /// # 参数
    /// - `min`: 最小值
    /// - `max`: 最大值
    /// - `length`: 数据点数量
    pub fn random_wave(min: u8, max: u8, length: usize) -> Vec<u8>;
}
```

### dglab-protocol API

#### BleScanner

```rust
impl BleScanner {
    /// 创建新的 BLE 扫描器
    pub async fn new() -> Result<Self>;
    
    /// 开始扫描设备
    /// 
    /// # 参数
    /// - `timeout`: 扫描超时时间 (秒)
    /// 
    /// # 返回
    /// 扫描到的设备列表
    pub async fn scan(&mut self, timeout: Duration) -> Result<Vec<DeviceInfo>>;
    
    /// 停止扫描
    pub async fn stop_scan(&mut self) -> Result<()>;
}
```

#### V3BleDevice

```rust
impl V3BleDevice {
    /// 从 BLE 外设创建设备
    pub async fn from_peripheral(peripheral: Peripheral) -> Result<Self>;
    
    /// 连接到设备
    pub async fn connect(&mut self) -> Result<()>;
    
    /// 断开连接
    pub async fn disconnect(&mut self) -> Result<()>;
    
    /// 设置通道功率
    /// 
    /// # 参数
    /// - `channel_a`: 通道 A 功率 (0-200)
    /// - `channel_b`: 通道 B 功率 (0-200)
    pub async fn set_power(&mut self, channel_a: u8, channel_b: u8) -> Result<()>;
    
    /// 发送脉冲数据
    /// 
    /// # 参数
    /// - `channel_a`: 通道 A 波形数据
    /// - `channel_b`: 通道 B 波形数据
    pub async fn send_pulse(
        &mut self,
        channel_a: Vec<u8>,
        channel_b: Vec<u8>
    ) -> Result<()>;
}
```

---

## 性能考虑

### 异步并发

- 使用 `tokio` 异步运行时处理所有 I/O 操作
- BLE 通信和 WebSocket 通信均为非阻塞异步操作
- 多设备并发控制通过 `Arc<RwLock<>>` 实现线程安全

### 内存管理

- 使用 `Arc` 智能指针共享设备实例,避免不必要的克隆
- 事件系统使用 `broadcast` 通道,支持多个订阅者
- 波形数据生成采用惰性计算,按需生成

### 错误处理

- 所有公共 API 返回 `Result<T, Error>` 类型
- 使用 `thiserror` 定义清晰的错误类型
- 协议层错误会被转换为核心层错误向上传播

---

## 安全性

### 权限管理

**Tauri 权限配置** (`capabilities/default.json`):

```json
{
  "permissions": [
    "core:default",
    "shell:allow-open"
  ]
}
```

### 数据验证

- 所有用户输入在 Tauri 命令层进行验证
- 功率值限制在 0-200 范围内
- BLE 数据包进行 CRC-16 校验

### 连接安全

- BLE 连接默认加密
- WiFi WebSocket 支持 TLS/SSL (计划中)

---

## 测试策略

### 单元测试

- 协议层: 测试数据包编解码、CRC 校验
- 核心层: 测试设备状态机、会话管理逻辑
- 波形生成: 测试各类波形生成算法

**运行测试**:

```bash
# 运行所有测试
cargo test

# 运行特定 crate 的测试
cargo test -p dglab-protocol
cargo test -p dglab-core

# 运行单个测试
cargo test test_v3_packet_encode
```

### 集成测试

- 测试 CLI 命令执行流程
- 测试 Tauri 命令调用和事件传播
- 测试前后端数据同步

### 硬件测试

- 使用真实 DG-LAB Coyote 3.0 设备进行测试
- 验证功率控制精度
- 测试连接稳定性和断线重连

---

## 扩展性

### 添加新设备支持

1. 在 `dglab-protocol` 中实现新的协议模块
2. 在 `dglab-core` 中实现 `Device` trait
3. 在 `SessionManager` 中注册新设备类型

### 添加新波形类型

1. 在 `WaveformGenerator` 中添加新的生成函数
2. 更新前端 `waveformStore` 中的类型定义
3. 在 UI 中添加对应的控件

### 添加新的 Tauri 命令

1. 在 `src-tauri/src/commands/` 中添加新命令
2. 在 `lib.rs` 的 `invoke_handler` 中注册
3. 在前端封装对应的 TypeScript 函数

---

## 未来路线图

### Phase 4: Android 移动端 (计划中)

- 使用 Tauri Mobile 构建 Android 应用
- 实现移动端 BLE 权限管理
- 适配移动端 UI 布局
- 添加后台服务支持

### 高级功能 (计划中)

- **脚本引擎**: 支持自定义控制脚本
- **云同步**: 预设和配置云端同步
- **多用户支持**: 账号系统和权限管理
- **数据分析**: 使用历史记录和统计图表
- **远程控制**: 通过互联网远程控制设备

---

## 参考资源

- [Tauri 官方文档](https://tauri.app/)
- [btleplug 文档](https://docs.rs/btleplug/)
- [tokio 异步编程指南](https://tokio.rs/)
- [React 官方文档](https://react.dev/)
- [Zustand 状态管理](https://zustand-demo.pmnd.rs/)

---

**文档版本**: v0.1.0  
**最后更新**: 2026-02-16  
**维护者**: DG-LAB Contributors
