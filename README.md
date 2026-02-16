# DG-LAB 设备控制器

[![Rust CI](https://github.com/userzbb/DG-LAB/workflows/CI/badge.svg)](https://github.com/userzbb/DG-LAB/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

一个功能完整的 DG-LAB 设备控制器，使用 Rust + Tauri + React 构建。支持通过 BLE 直接控制设备，或通过 WebSocket 协议提供远程控制能力。

## ✨ 特性

- 🔌 **多种连接方式**
  - BLE 直接连接（Coyote 3.0）
  - WebSocket 客户端模式（连接到 DG-LAB APP）
  - BLE-WebSocket 桥接模式
  
- 🎮 **强大的控制功能**
  - 实时强度调整（0-200，支持安全上限）
  - 多种波形支持（连续波、脉冲波、正弦波、锯齿波等）
  - 预设管理和快速切换
  - 渐进式强度调整（防止突然变化）

- 🌐 **WebSocket 服务器**
  - 完整的消息路由和转发
  - 客户端绑定和心跳机制
  - 符合 DG-LAB Coyote Game Hub 协议

- 🖥️ **多种界面**
  - CLI 命令行工具
  - TUI 终端界面（开发中）
  - GUI 图形界面（Tauri + React）

- 🧪 **完善的测试**
  - MockDevice 支持无硬件开发
  - 265+ 单元测试
  - 集成测试覆盖

## 🚀 快速开始

### 环境要求

- Rust 1.75+ (2021 Edition)
- Node.js 18+ (用于 GUI)
- 蓝牙适配器（用于 BLE 连接）

### 安装

```bash
# 克隆仓库
git clone https://github.com/userzbb/DG-LAB.git
cd DG-LAB

# 构建项目
cargo build --release

# 或者只构建 CLI
cargo build --release -p dglab-cli
```

### 基础使用

#### 1. 扫描设备

```bash
dglab scan
```

#### 2. 连接设备

```bash
dglab connect 47L121000
```

#### 3. 控制强度

```bash
# 查看当前状态
dglab control --status

# 设置强度
dglab control --power-a 50 --power-b 30

# 启动输出
dglab control --start

# 停止输出
dglab control --stop
```

#### 4. WebSocket 桥接模式

```bash
# 启动桥接服务器（在 8080 端口）
dglab bridge 47L121000 --port 8080

# 使用 DG-LAB APP 扫描二维码连接
```

## 📚 项目结构

```
DG-LAB/
├── crates/
│   ├── dglab-protocol/    # 协议实现（BLE + WebSocket）
│   ├── dglab-core/         # 核心业务逻辑
│   └── dglab-cli/          # 命令行工具
├── apps/
│   └── dglab-gui-tauri/    # Tauri GUI 应用
├── docs/                   # 文档
└── tests/                  # 集成测试
```

### 核心 Crate

#### `dglab-protocol`
协议层实现，包括：
- BLE 通信协议（Coyote V3）
- WebSocket 消息协议
- 数据包编解码
- 设备扫描

#### `dglab-core`
核心业务逻辑，包括：
- 设备抽象和管理
- 会话管理
- 预设系统
- 波形生成器
- MockDevice（用于测试）

#### `dglab-cli`
命令行工具，提供：
- 设备扫描和连接
- 强度和波形控制
- 桥接服务器
- TUI 交互界面

#### `dglab-gui-tauri`
图形界面应用，基于：
- Tauri 2.0
- React 18
- TypeScript
- Tailwind CSS

## 🔧 开发指南

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定 crate 的测试
cargo test -p dglab-core

# 运行测试并显示输出
cargo test -- --nocapture
```

### 代码检查

```bash
# 格式化代码
cargo fmt

# 运行 Clippy
cargo clippy

# 检查编译
cargo check
```

### 使用 MockDevice 开发

在没有真实硬件的情况下（如 WSL 环境），可以使用 `MockDevice` 进行开发和测试：

```rust
use dglab_core::device::MockDevice;

#[tokio::main]
async fn main() {
    let mut device = MockDevice::new("mock-001".to_string(), "Test Device".to_string());
    
    device.connect().await.unwrap();
    device.set_power(0, 50).await.unwrap();
    device.start().await.unwrap();
}
```

### WebSocket 协议

本项目实现了与 [DG-Lab-Coyote-Game-Hub](https://github.com/hyperzlib/DG-Lab-Coyote-Game-Hub) 兼容的 WebSocket 协议：

**消息格式**:
```json
{
  "type": "msg",
  "clientId": "app-client",
  "targetId": "web-client",
  "message": "strength-1+2+50"
}
```

**消息类型**:
- `heartbeat` - 心跳（20 秒间隔）
- `bind` - 绑定请求
- `msg` - 数据消息
- `break` - 断开连接
- `error` - 错误响应

**数据消息头**:
- `targetId` - 目标客户端 ID
- `strength` - 强度控制
- `pulse` - 脉冲数据
- `clear` - 清除数据
- `feedback` - 反馈按钮

详细协议文档请参考：[DG-Lab-Coyote-Game-Hub-Analysis.md](DG-Lab-Coyote-Game-Hub-Analysis.md)

## 🛠️ 架构设计

### 连接模式

#### 1. BLE 直连模式
```
用户程序 -> dglab-core -> dglab-protocol(BLE) -> DG-LAB 设备
```

#### 2. WebSocket 客户端模式
```
用户程序 -> dglab-core -> dglab-protocol(WebSocket) -> DG-LAB APP -> 设备
```

#### 3. 桥接模式
```
Web 控制端 -> WebSocket 服务器 -> dglab-core -> BLE -> DG-LAB 设备
```

### 设备抽象

所有设备实现统一的 `Device` trait：

```rust
#[async_trait]
pub trait Device: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn state(&self) -> DeviceState;
    fn info(&self) -> DeviceInfo;
    
    async fn connect(&mut self) -> Result<()>;
    async fn disconnect(&mut self) -> Result<()>;
    async fn start(&mut self) -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
    async fn set_power(&mut self, channel: u8, power: u8) -> Result<()>;
    async fn set_waveform(&mut self, channel: u8, waveform: WaveformConfig) -> Result<()>;
    async fn heartbeat(&mut self) -> Result<()>;
    
    fn subscribe_events(&self) -> broadcast::Receiver<DeviceEvent>;
}
```

## 📖 文档

- [AGENTS.md](AGENTS.md) - 开发者指南和代码风格
- [DG-Lab-Coyote-Game-Hub-Analysis.md](DG-Lab-Coyote-Game-Hub-Analysis.md) - WebSocket 协议深度分析
- [TEST_BASIC_FEATURES.md](TEST_BASIC_FEATURES.md) - 基础功能测试计划

## 🤝 贡献

欢迎贡献！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'feat: add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 提交规范

使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

- `feat:` - 新功能
- `fix:` - 错误修复
- `docs:` - 文档更新
- `style:` - 代码格式调整
- `refactor:` - 代码重构
- `test:` - 测试相关
- `chore:` - 构建/工具相关

## 🧪 测试状态

| Crate | 测试数量 | 状态 |
|-------|---------|------|
| dglab-protocol | 113 | ✅ 通过 |
| dglab-core | 152 | ✅ 通过 |
| dglab-cli | 0 | ⚠️ 待添加 |

## 🗺️ 路线图

### 已完成 ✅
- [x] BLE 协议实现（Coyote V3）
- [x] WebSocket 协议核心
- [x] MockDevice 测试支持
- [x] CLI 基础功能
- [x] 设备事件系统
- [x] 预设管理
- [x] 波形生成器

### 进行中 🚧
- [ ] WebSocket 服务器完整实现
- [ ] TUI 终端界面
- [ ] GUI 界面完善

### 计划中 📋
- [ ] WebSocket 客户端（APP 角色）
- [ ] 强度渐进式调整
- [ ] 波形播放系统
- [ ] 游戏控制逻辑
- [ ] 脚本支持
- [ ] 配置持久化
- [ ] 断线重连

## ⚠️ 免责声明

本项目仅供学习和研究使用。使用本软件控制 DG-LAB 设备时，请务必：

1. 阅读并理解设备使用手册
2. 遵循安全操作规范
3. 设置合理的强度上限
4. 在出现任何不适时立即停止使用

开发者不对使用本软件造成的任何后果负责。

## 📄 许可证

本项目采用双许可证：

- MIT License
- Apache License 2.0

您可以选择其中任意一个许可证使用本项目。

## 🙏 致谢

- [hyperzlib/DG-Lab-Coyote-Game-Hub](https://github.com/hyperzlib/DG-Lab-Coyote-Game-Hub) - WebSocket 协议参考实现
- [btleplug](https://github.com/deviceplug/btleplug) - Rust BLE 库
- [Tauri](https://tauri.app/) - 跨平台桌面应用框架

## 📞 联系方式

- GitHub Issues: [提交问题](https://github.com/userzbb/DG-LAB/issues)
- GitHub Discussions: [讨论区](https://github.com/userzbb/DG-LAB/discussions)

---

**⚡ 由 Rust 和 ❤️ 驱动**
