# DG-LAB 项目实现计划

## 项目状态概述

当前项目已完成协议层（BLE + WiFi）和大部分核心层、CLI 框架。GUI 将使用 Tauri + React 重新实现。以下是各模块的完成状态：

| 模块 | 状态 | 说明 |
|------|------|------|
| dglab-protocol::ble | ✓ 完成 | BLE 协议完整实现 |
| dglab-protocol::wifi | ✓ 完成 | WiFi WebSocket 协议完整实现 |
| dglab-protocol::packet | ✓ 完成 | 数据包编解码完整实现 |
| dglab-core::device | ✓ 完成 | BLE + WiFi 设备实现 |
| dglab-core::waveform | ✓ 完成 | 波形生成器完整实现 |
| dglab-core::session | ✓ 完成 | 会话管理器完整实现 |
| dglab-core::preset | ✓ 完成 | 已修复 unsafe，添加 save/delete |
| dglab-core::script | ✗ 未实现 | 仅占位符 |
| dglab-cli::scan | ✓ 完成 | BLE 扫描完整实现 |
| dglab-cli::connect | ✓ 完成 | 设备连接完整实现 |
| dglab-cli::control | ✓ 完成 | 设备控制完整实现 |
| dglab-cli::preset | ✓ 完成 | create/delete 已保存到磁盘 |
| dglab-cli::wifi | ✓ 完成 | WiFi 命令完整实现 |
| dglab-cli::script | ✗ 未实现 | 仅占位符 |
| dglab-cli::tui | ✗ 未实现 | 仅占位符 |
| dglab-gui (egui) | ◐ 部分完成 | 保留作为参考(弃用) |
| dglab-gui-tauri | ✗ 未实现 | Tauri + React 新 GUI |

---

## 实现阶段

### 阶段 1: WiFi 集成到 dglab-core ✓ 已完成

#### 1.1 添加 WsCoyoteDevice 实现 ✓ 已完成

**文件**: `crates/dglab-core/src/device/coyote.rs`

在现有 `CoyoteDevice` 之后添加 `WsCoyoteDevice` 结构体，使用 `Arc<Mutex>` 包装 `WsClient` 来避免克隆问题。

#### 1.2 更新 device/mod.rs ✓ 已完成

添加导出：

```rust
pub use coyote::{CoyoteDevice, WsCoyoteDevice};
```

---

### 阶段 2: CLI 添加 WiFi 支持 ✓ 已完成

#### 2.1 创建 commands/wifi.rs ✓ 已完成

**文件**: `crates/dglab-cli/src/commands/wifi.rs`

包含子命令：
- `connect` - 连接 WiFi 并显示二维码
- `disconnect` - 断开 WiFi 连接
- `status` - 显示连接状态
- `control` - 控制强度（设置、增加、减少）

#### 2.2 更新 commands/mod.rs ✓ 已完成

添加 WiFi 模块和 `wifi()` 方法到 `DglabCli`。

#### 2.3 更新 main.rs ✓ 已完成

添加 WiFi 子命令。

---

### 阶段 3: GUI (Tauri + React)

**注意**: GUI 技术栈已从 egui 更新为 **Tauri + React**。

详细计划请参考: [`GUI_TAURI_REACT_PLAN.md`](./GUI_TAURI_REACT_PLAN.md)

#### 3.1 初始化 Tauri 项目

创建新目录 `dglab-gui-tauri/`：

```bash
npm create tauri-app@latest dglab-gui-tauri
```

选择：
- Framework: React
- Language: TypeScript
- Package manager: npm 或 pnpm

#### 3.2 配置 Tauri 后端

**文件**: `dglab-gui-tauri/src-tauri/Cargo.toml`

添加依赖：
```toml
[dependencies]
dglab-core = { path = "../crates/dglab-core" }
dglab-protocol = { path = "../crates/dglab-protocol" }
tauri = { version = "2.0", features = ["api-all"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### 3.3 实现 Tauri Commands

**文件**: `dglab-gui-tauri/src-tauri/src/commands.rs`

实现命令：
- `ble_scan` - 扫描 BLE 设备
- `ble_connect` / `ble_disconnect` - BLE 连接管理
- `wifi_connect` / `wifi_disconnect` - WiFi 连接管理
- `wifi_get_qr_url` - 获取二维码 URL
- `set_power` - 设置强度
- `get_devices` - 获取设备列表
- `preset_list` / `preset_create` / `preset_delete` / `preset_apply` - 预设管理

#### 3.4 实现 React 组件

主要组件：
- `BleDevicePanel.tsx` - BLE 设备面板
- `WifiPanel.tsx` - WiFi 连接面板
- `ControlPanel.tsx` - 强度控制面板
- `WaveformEditor.tsx` - 波形编辑器
- `PresetsPanel.tsx` - 预设管理面板
- `SettingsPanel.tsx` - 设置面板

#### 3.5 保留原 egui 版本

原 `crates/dglab-gui/` crate 保留作为参考实现。

---

### 阶段 4: 完善 preset 功能 ✓ 已完成

#### 4.1 修复 preset/storage.rs ✓ 已完成

**文件**: `crates/dglab-core/src/preset/storage.rs`

**已完成:**
- 添加 `dirs` 依赖到 `dglab-core/Cargo.toml`
- 修复 `get_or_create_preset` 中的 unsafe 代码（改为返回 owned 值）
- 添加 `save_preset()` 方法 - 保存单个预设
- 添加 `delete_preset_file()` 方法 - 删除预设文件

#### 4.2 完善 CLI preset 命令 ✓ 已完成

**文件**: `crates/dglab-cli/src/commands/preset.rs`

**已完成:**
- `Create` - 创建预设并保存到磁盘
- `Delete` - 删除预设并删除文件
- 添加 `preset_manager_mut()` 到 `DglabCli`

---

### 阶段 5: 实现 script 模块（可选）

#### 5.1 选择脚本方案

**选项 A: Rhai 脚本语言**
- 优点：功能强大，灵活
- 缺点：增加依赖，学习曲线

**选项 B: 简单 DSL (TOML/YAML 配置)**
- 优点：简单易懂，无需额外依赖
- 缺点：功能有限

**推荐**: 先实现简单的时间线 DSL，后续可扩展。

#### 5.2 实现 timeline DSL 示例

```rust
//! 脚本引擎

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 脚本动作
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum ScriptAction {
    /// 设置强度
    SetPower {
        channel: u8,
        power: u8,
    },
    /// 渐变强度
    FadePower {
        channel: u8,
        start_power: u8,
        end_power: u8,
        duration_ms: u64,
    },
    /// 等待
    Wait {
        duration_ms: u64,
    },
    /// 开始输出
    Start,
    /// 停止输出
    Stop,
}

/// 脚本步骤
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScriptStep {
    /// 动作
    #[serde(flatten)]
    pub action: ScriptAction,
    /// 步骤名称（可选）
    #[serde(default)]
    pub name: Option<String>,
}

/// 脚本
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Script {
    /// 脚本名称
    pub name: String,
    /// 脚本描述
    #[serde(default)]
    pub description: String,
    /// 步骤列表
    pub steps: Vec<ScriptStep>,
}

/// 脚本引擎
pub struct ScriptEngine {
    // ...
}

impl ScriptEngine {
    /// 执行脚本
    pub async fn execute(&self, script: &Script, device: &mut dyn Device) -> Result<()> {
        // 实现执行逻辑
        // 遍历 steps，依次执行每个 action
        // 对于 Wait 和 FadePower，使用 tokio::time::sleep
    }
}
```

---

### 阶段 6: 实现 TUI（可选）

#### 6.1 选择 TUI 库

推荐使用 `ratatui` (原 tui-rs) + `crossterm`。

#### 6.2 TUI 布局设计

```
┌─────────────────────────────────────────────────┐
│ DG-LAB Controller                      [Quit] X │
├──────────┬──────────────────────────────────────┤
│ Devices  │  [x] BLE: Coyote-1234 (Connected)  │
│ Control  │  [ ] WiFi: Waiting for APP...        │
│ Presets  │                                      │
│ Scripts  │  ┌────────────────────────────────┐ │
│          │  │ Channel A:  [===      ]  30   │ │
│          │  │ Channel B:  [=====    ]  50   │ │
│          │  └────────────────────────────────┘ │
│          │                                      │
│          │  [Start] [Stop]  [Quick Presets]   │
├──────────┴──────────────────────────────────────┤
│ Status: Connected to Coyote-1234                │
└─────────────────────────────────────────────────┘
```

---

## 文件清单

### 已修改的文件

| 文件 | 修改内容 |
|------|----------|
| `crates/dglab-core/Cargo.toml` | 添加 `dirs` 依赖 ✓ |
| `crates/dglab-core/src/device/coyote.rs` | 添加 `WsCoyoteDevice` ✓ |
| `crates/dglab-core/src/device/mod.rs` | 导出 `WsCoyoteDevice` ✓ |
| `crates/dglab-core/src/preset/storage.rs` | 修复 unsafe, 添加 save/delete 方法 ✓ |
| `crates/dglab-cli/src/main.rs` | 添加 WiFi 子命令 ✓ |
| `crates/dglab-cli/src/commands/mod.rs` | 添加 WiFi 模块 ✓ |
| `crates/dglab-cli/src/commands/preset.rs` | 完善 create/delete ✓ |

### 已新建的文件

| 文件 | 说明 |
|------|------|
| `crates/dglab-cli/src/commands/wifi.rs` | WiFi 命令实现 ✓ |
| `crates/dglab-gui/src/ui/wifi_panel.rs` | WiFi UI 面板 (egui) ✓ |

### 新 GUI (Tauri + React) 文件

| 文件 | 说明 |
|------|------|
| `GUI_TAURI_REACT_PLAN.md` | Tauri + React 详细计划 ✓ |
| `dglab-gui-tauri/` | 新 GUI 项目目录 (待创建) |

---

## 验证方法

### 1. 编译测试

```bash
cargo check
cargo build
```

### 2. 单元测试

```bash
cargo test
```

### 3. CLI WiFi 命令测试

```bash
# 连接 WiFi（显示二维码）
dglab wifi connect

# 控制强度
dglab wifi control --channel A --power 50
dglab wifi control --channel A --up 10

# 查看状态
dglab wifi status

# 断开连接
dglab wifi disconnect
```

### 4. CLI Preset 命令测试

```bash
# 创建预设
dglab preset create --name "My Preset" --description "Test" --a 40 --b 30

# 列出预设
dglab preset list

# 删除预设
dglab preset delete "My Preset"
```

---

## 实现建议顺序

1. ✓ **阶段 4** (Preset 修复) - 已完成
2. ✓ **阶段 1** (WiFi 核心集成) - 已完成
3. ✓ **阶段 2** (CLI WiFi) - 已完成
4. **阶段 3** (GUI Tauri + React) - 详细计划请参考 `GUI_TAURI_REACT_PLAN.md`
5. **阶段 5** (Script) - 可选功能
6. **阶段 6** (TUI) - 可选功能
