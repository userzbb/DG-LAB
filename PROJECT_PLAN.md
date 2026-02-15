# DG-LAB Rust 跨平台控制器 - 实现计划

## Context

本项目旨在使用 Rust 开发一个完整的 DG-LAB 设备跨平台控制器。DG-LAB 是一个 WiFi/网络设备，控制器需要支持：
- 桌面端全平台（Windows/macOS/Linux）
- 移动端（iOS/Android）
- 服务器端
- 同时提供 CLI 和 GUI 两种用户界面

由于需要从零研究协议，项目将从协议分析开始，逐步构建完整的功能。

## 推荐方案

### 技术栈选择

| 层级 | 技术选型 |
|------|---------|
| **异步运行时** | tokio |
| **BLE 通信** | btleplug |
| **网络通信** | tokio-rustls, WebSocket |
| **序列化** | serde + bincode |
| **CLI 框架** | clap + ratatui |
| **GUI 框架** | Tauri + egui |
| **日志** | tracing |
| **错误处理** | anyhow + thiserror |

### 项目架构

```
dglab-rs/
├── Cargo.toml                          # 工作空间配置
├── PROJECT_PLAN.md                     # 本计划文件
├── docs/                               # 协议文档
│   └── protocol/
│       ├── ble.md                      # BLE 协议文档
│       ├── wifi.md                     # WiFi 协议文档
│       └── packets.md                  # 数据包格式文档
├── crates/
│   ├── dglab-protocol/                # 协议库（BLE/WiFi 通信）
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── ble/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── scanner.rs
│   │   │   │   └── device.rs
│   │   │   ├── wifi/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── client.rs
│   │   │   │   └── server.rs
│   │   │   └── packet/
│   │   │       ├── mod.rs
│   │   │       ├── encoder.rs
│   │   │       └── decoder.rs
│   │   └── Cargo.toml
│   ├── dglab-core/                    # 核心业务逻辑
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── device/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── traits.rs
│   │   │   │   └── coyote.rs
│   │   │   ├── session/
│   │   │   │   ├── mod.rs
│   │   │   │   └── manager.rs
│   │   │   ├── waveform/
│   │   │   │   ├── mod.rs
│   │   │   │   └── generator.rs
│   │   │   ├── preset/
│   │   │   │   ├── mod.rs
│   │   │   │   └── storage.rs
│   │   │   └── script/
│   │   │       ├── mod.rs
│   │   │       └── engine.rs
│   │   └── Cargo.toml
│   ├── dglab-cli/                     # 命令行界面
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── commands/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── scan.rs
│   │   │   │   ├── connect.rs
│   │   │   │   ├── control.rs
│   │   │   │   └── script.rs
│   │   │   └── tui/
│   │   │       ├── mod.rs
│   │   │       ├── app.rs
│   │   │       └── widgets.rs
│   │   └── Cargo.toml
│   └── dglab-gui/                     # 图形界面（Tauri + egui）
│       ├── src/
│       │   ├── main.rs
│       │   ├── app.rs
│       │   └── ui/
│       │       ├── mod.rs
│       │       ├── device_panel.rs
│       │       ├── control_panel.rs
│       │       ├── waveform_editor.rs
│       │       └── settings_panel.rs
│       ├── Cargo.toml
│       └── tauri.conf.json
├── examples/                           # 示例代码
│   ├── basic_ble.rs
│   ├── basic_wifi.rs
│   └── waveform_demo.rs
└── scripts/                            # 辅助脚本
    └── setup.sh
```

### 核心模块

1. **dglab-protocol** - 协议层
   - BLE 通信实现
   - WiFi 通信实现
   - 数据包编码/解码
   - 加密/认证

2. **dglab-core** - 核心层
   - 设备抽象 (`Device` trait)
   - 会话管理
   - 波形生成器
   - 预设管理
   - 脚本引擎

3. **dglab-cli** - CLI 界面
   - 设备扫描/连接命令
   - 实时控制命令
   - TUI 终端界面
   - 脚本执行

4. **dglab-gui** - GUI 界面 请参考[GUI_TAURI_REACT_PLAN.md](GUI_TAURI_REACT_PLAN.md)
   - 设备管理面板
   - 控制面板（滑块/按钮）
   - 波形编辑器
   - 预设管理 UI

## 实现步骤

### 阶段 1: 协议研究与基础库（最高优先级）

| 步骤 | 任务 | 关键文件 |
|------|------|---------|
| 1.1 | 搭建协议分析环境（Wireshark + BLE sniffer） | - |
| 1.2 | 抓包分析官方 App 通信 | `docs/protocol/*.md` |
| 1.3 | 解析数据包结构 | `docs/protocol/packets.md` |
| 1.4 | 实现 dglab-protocol BLE 模块 | `crates/dglab-protocol/src/ble/mod.rs` |
| 1.5 | 实现数据包编码/解码 | `crates/dglab-protocol/src/packet/{encoder,decoder}.rs` |

### 阶段 2: 核心库

| 步骤 | 任务 | 关键文件 |
|------|------|---------|
| 2.1 | 创建设备 trait 和 Coyote 实现 | `crates/dglab-core/src/device/traits.rs` |
| 2.2 | 实现会话管理 | `crates/dglab-core/src/session/manager.rs` |
| 2.3 | 实现强度/模式控制 | `crates/dglab-core/src/device/coyote.rs` |
| 2.4 | 实现波形生成器 | `crates/dglab-core/src/waveform/generator.rs` |
| 2.5 | 预设管理（保存/加载） | `crates/dglab-core/src/preset/storage.rs` |

### 阶段 3: CLI 工具

| 步骤 | 任务 | 关键文件 |
|------|------|---------|
| 3.1 | 实现 CLI 框架（clap） | `crates/dglab-cli/src/main.rs` |
| 3.2 | 实现 scan/connect 命令 | `crates/dglab-cli/src/commands/scan.rs` |
| 3.3 | 实现 control 命令 | `crates/dglab-cli/src/commands/control.rs` |
| 3.4 | 实现 TUI 模式（ratatui） | `crates/dglab-cli/src/tui/app.rs` |
| 3.5 | 脚本执行功能 | `crates/dglab-cli/src/commands/script.rs` |

### 阶段 4: GUI 桌面端
(GUI_TAURI_REACT_PLAN.md)[GUI_TAURI_REACT_PLAN.md]

### 阶段 5: WiFi 协议和高级功能

| 步骤 | 任务 |
|------|------|
| 5.1 | WiFi 协议逆向分析 |
| 5.2 | 实现 WiFi 模块 |
| 5.3 | 多设备同步 |
| 5.4 | 远程服务器模式 |

### 阶段 6: 移动端（可选）

| 步骤 | 任务 |
|------|------|
| 6.1 | Flutter 项目搭建 |
| 6.2 | flutter_rust_bridge 集成 |
| 6.3 | 移动端 UI 实现 |

## 关键文件清单

| 文件路径 | 说明 |
|----------|------|
| `Cargo.toml` | 工作空间配置 |
| `crates/dglab-protocol/src/ble/mod.rs` | BLE 通信主模块 |
| `crates/dglab-protocol/src/packet/{encoder,decoder}.rs` | 数据包编解码 |
| `crates/dglab-core/src/device/traits.rs` | 设备 trait 定义 |
| `crates/dglab-core/src/device/coyote.rs` | Coyote 设备实现 |
| `crates/dglab-core/src/session/manager.rs` | 会话管理器 |
| `crates/dglab-core/src/waveform/generator.rs` | 波形生成器 |
| `crates/dglab-cli/src/main.rs` | CLI 入口 |
| `crates/dglab-cli/src/tui/app.rs` | TUI 应用 |
| `crates/dglab-gui/src/main.rs` | Tauri 入口 |
| `crates/dglab-gui/src/app.rs` | GUI 应用状态 |
| `docs/protocol/ble.md` | BLE 协议文档 |
| `docs/protocol/packets.md` | 数据包格式文档 |

## 验证方法

1. **协议验证**: 使用 Wireshark 抓包对比官方 App 和我们的实现
2. **单元测试**: 为协议编解码、设备控制编写单元测试
3. **集成测试**: 真实设备连接测试
4. **端到端测试**:
   - 运行 CLI 扫描并连接设备
   - 运行 GUI 进行实时控制
   - 测试预设保存/加载功能

## 风险与注意事项

1. **协议逆向**: 需要先花费时间分析 BLE/WiFi 协议，这是项目的最大不确定因素
2. **BLE 跨平台**: btleplug 在不同平台上可能有细微差异，需要充分测试

