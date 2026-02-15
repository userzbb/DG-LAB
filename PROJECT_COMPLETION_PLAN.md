# DG-LAB 项目完成计划

## 📋 概述

### 项目当前状态

DG-LAB Rust 跨平台控制器已完成**协议层、核心业务逻辑、CLI 框架**的主体开发。以下为各模块完成度评估：

| 模块 | 完成度 | 代码行数 | 测试数 | 说明 |
|------|--------|---------|--------|------|
| `dglab-protocol` | **90%** | ~1,600 | 9 | BLE/WiFi/Packet 协议完整，缺少 BLE 和 Packet 模块测试 |
| `dglab-core` | **75%** | ~1,850 | 0 | 设备/会话/波形/预设完整，脚本引擎为空壳，**零测试** |
| `dglab-cli` | **70%** | ~780 | 0 | 主要命令完整，TUI 和脚本为占位符，**零测试** |
| `dglab-gui` (egui) | **已弃用** | ~400 | 0 | 将被 Tauri + React 替代 |
| `dglab-gui-tauri` | **0%** | 0 | 0 | 尚未创建 |

**关键缺口**：
- 整个项目仅 **9 个测试**（全在 `dglab-protocol::wifi`）
- `dglab-core` 和 `dglab-cli` **零测试**
- 脚本引擎 (`script`) 和 TUI 为空壳 `unimplemented!()`
- GUI 尚未开始（已有 999 行详细计划文档 `GUI_TAURI_REACT_PLAN.md`）
- 无 CI/CD 配置
- 无真实设备测试验证
- `examples/` 目录为空

### 完成目标

1. **代码质量**：测试覆盖率达到合理水平，消除所有 clippy 警告
2. **桌面 GUI**：完成 Tauri + React 桌面端，支持 Windows/macOS/Linux
3. **文档交付**：完善用户文档和开发文档
4. **Android 移动端**：基于 Tauri 2.0 实现 Android 版本

### 时间预期

**总计：3-4 周**（全职开发），可根据实际进展调整。

---

## 🎯 开发路线图

```
Week 1                    Week 2                    Week 3                    Week 4
├── 阶段1: 代码质量 ──────┤                           │                           │
│   [3-4 天]               ├── 阶段2: Tauri GUI ──────┤                           │
│                          │   [7-10 天]               ├── 阶段3: 文档交付 ────────┤
│                          │                           │   [2-3 天]                ├── 阶段4: Android ──┤
│                          │                           │                           │   [5-7 天]          │
```

---

## 阶段 1：代码质量提升（3-4 天）

### 目标
- 为核心模块编写单元测试
- 消除 clippy 警告
- 补充缺失的文档注释
- 清理空壳代码

### 1.1 dglab-protocol 测试补全（1 天）

当前状态：WiFi 模块有 9 个测试，BLE 和 Packet 模块零测试。

| 任务 | 文件 | 优先级 | 说明 |
|------|------|--------|------|
| Packet 编码测试 | `crates/dglab-protocol/src/packet/encoder.rs` | 高 | 测试所有 `encode_*` 方法的输出字节 |
| Packet 解码测试 | `crates/dglab-protocol/src/packet/decoder.rs` | 高 | 测试 `try_decode`、`decode_all`、边界情况 |
| Packet 往返测试 | `crates/dglab-protocol/src/packet/mod.rs` | 高 | encode → decode 往返一致性 |
| CommandType 测试 | `crates/dglab-protocol/src/packet/types.rs` | 中 | 测试枚举转换 |
| BLE Scanner 测试 | `crates/dglab-protocol/src/ble/scanner.rs` | 低 | 需要 mock btleplug，可用集成测试 |
| WiFi 补充测试 | `crates/dglab-protocol/src/wifi/client.rs` | 中 | 连接状态管理、消息序列化 |

**验证方法**：
```bash
cargo test -p dglab-protocol -- --nocapture
cargo test -p dglab-protocol --lib  # 确保全部通过
```

### 1.2 dglab-core 测试编写（1-1.5 天）

当前状态：零测试，这是最关键的缺口。

| 任务 | 文件 | 优先级 | 说明 |
|------|------|--------|------|
| WaveformGenerator 测试 | `crates/dglab-core/src/waveform/generator.rs` | 高 | 测试 8 种波形输出、边界值、update() |
| SessionManager 测试 | `crates/dglab-core/src/session/manager.rs` | 高 | 测试设备增删查、connect_all/disconnect_all |
| PresetManager 测试 | `crates/dglab-core/src/preset/storage.rs` | 高 | 测试保存/加载/删除/默认预设 |
| DeviceState 测试 | `crates/dglab-core/src/device/traits.rs` | 中 | 测试状态枚举、DeviceConfig 默认值 |
| BaseDevice 测试 | `crates/dglab-core/src/device/mod.rs` | 中 | 测试事件广播机制 |
| Error 类型测试 | `crates/dglab-core/src/error.rs` | 低 | 测试错误转换 |

**测试策略**：
- 波形生成器可直接单元测试（纯函数）
- SessionManager 需要 mock `Device` trait
- PresetManager 使用临时目录（`tempfile` crate）
- 设备实现需要 mock BLE/WiFi 底层

**验证方法**：
```bash
cargo test -p dglab-core -- --nocapture
```

### 1.3 Clippy 和代码清理（0.5 天）

| 任务 | 说明 |
|------|------|
| `cargo clippy --workspace` | 修复所有警告 |
| `cargo fmt --check` | 确保格式一致 |
| 清理脚本空壳 | 在 `script/mod.rs` 中将 `unimplemented!()` 改为返回 `Err(ScriptError::NotImplemented)` |
| 清理 TUI 空壳 | 在 `tui/mod.rs` 中改为返回有意义的错误信息 |
| 检查 `unused_crate_dependencies` | 移除未使用的依赖 |
| 审查 packet decoder | 修复 decoder.rs:108 的 "示例实现" 注释标注的代码 |

**验证方法**：
```bash
cargo clippy --workspace -- -D warnings  # 零警告
cargo fmt -- --check                      # 格式正确
cargo build --workspace                   # 编译通过
cargo test --workspace                    # 全部测试通过
```

### 1.4 文档补充（0.5 天）

| 任务 | 说明 |
|------|------|
| 补充 `examples/` | 至少创建 2 个示例：BLE 扫描连接、WiFi 连接控制 |
| 更新 README.md | 更新开发状态清单，反映 WiFi 已完成、GUI 迁移计划 |
| 补充模块文档 | 检查并补全缺少 `//!` 模块文档的文件 |

### 阶段 1 验收标准

- [ ] `cargo test --workspace` 通过，测试数量 ≥ 40
- [ ] `cargo clippy --workspace -- -D warnings` 零警告
- [ ] `cargo fmt -- --check` 通过
- [ ] `cargo doc --workspace --no-deps` 无警告
- [ ] `examples/` 包含至少 2 个可运行示例
- [ ] 无 `unimplemented!()` 调用（改为返回错误）

---

## 阶段 2：Tauri + React GUI - 桌面端（7-10 天）

### 目标
- 创建 Tauri 2.0 项目，集成现有 Rust 后端
- 实现 React 前端界面（BLE/WiFi/控制/波形/预设）
- 桌面端打包（Windows/macOS/Linux）

> 详细技术方案参见 [`GUI_TAURI_REACT_PLAN.md`](./GUI_TAURI_REACT_PLAN.md)

### 2.1 项目初始化（1 天）

| 任务 | 说明 |
|------|------|
| 创建 Tauri 项目 | `npm create tauri-app@latest dglab-gui-tauri` |
| 配置前端工具链 | React 18 + TypeScript + Vite 5 |
| 安装 UI 库 | Tailwind CSS + shadcn-ui + Lucide React |
| 安装状态管理 | Zustand |
| 配置 Tauri 后端 | 添加 `dglab-core`、`dglab-protocol` 依赖 |
| 更新 workspace | 将 `dglab-gui-tauri/src-tauri` 加入 workspace |

**目录结构**：
```
dglab-gui-tauri/
├── src/                    # React 前端
│   ├── components/         # UI 组件
│   ├── hooks/              # React hooks
│   ├── stores/             # Zustand stores
│   ├── types/              # TypeScript 类型
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/              # Tauri 后端 (Rust)
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/       # Tauri commands
│   │   └── state.rs        # 应用状态
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── vite.config.ts
└── tailwind.config.js
```

**验证方法**：
```bash
cd dglab-gui-tauri && npm run tauri dev  # 应用启动，显示空白页面
```

### 2.2 Tauri 后端命令层（2 天）

| 任务 | 文件 | 说明 |
|------|------|------|
| 应用状态管理 | `src-tauri/src/state.rs` | `AppState` 包含 `SessionManager`、`BleManager` 等 |
| BLE 扫描命令 | `src-tauri/src/commands/ble.rs` | `scan_start`, `scan_stop`, `get_scan_results` |
| BLE 连接命令 | `src-tauri/src/commands/ble.rs` | `connect_device`, `disconnect_device` |
| WiFi 连接命令 | `src-tauri/src/commands/wifi.rs` | `wifi_connect`, `wifi_disconnect`, `wifi_status`, `get_qr_url` |
| 设备控制命令 | `src-tauri/src/commands/control.rs` | `set_power`, `start_output`, `stop_output`, `get_status` |
| 波形控制命令 | `src-tauri/src/commands/waveform.rs` | `set_waveform`, `get_waveform_types`, `get_waveform_preview` |
| 预设管理命令 | `src-tauri/src/commands/preset.rs` | `list_presets`, `apply_preset`, `save_preset`, `delete_preset` |
| 事件推送 | `src-tauri/src/commands/events.rs` | 使用 `tauri::Emitter` 推送设备事件到前端 |

**关键模式**：
```rust
#[tauri::command]
async fn scan_start(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.ble_manager.lock().await.start_scan().await.map_err(|e| e.to_string())
}
```

**验证方法**：
```bash
cargo build -p dglab-gui-tauri  # 编译通过
cargo clippy -p dglab-gui-tauri # 零警告
```

### 2.3 React 前端实现（4-5 天）

#### 2.3.1 基础布局和导航（0.5 天）

| 任务 | 说明 |
|------|------|
| 主布局 | 侧边栏导航 + 主内容区 |
| 路由/Tab | 设备、控制、波形、预设、设置 5 个页面 |
| 主题 | 暗色主题为主，支持亮/暗切换 |
| 全局状态 | Zustand store 初始化 |

#### 2.3.2 设备页面 - BLE（1 天）

| 任务 | 组件 | 说明 |
|------|------|------|
| 扫描面板 | `BleScanner.tsx` | 开始/停止扫描，设备列表（名称、信号强度、地址） |
| 设备卡片 | `DeviceCard.tsx` | 显示设备信息，连接/断开按钮 |
| 连接状态 | `ConnectionStatus.tsx` | 实时显示连接状态（颜色指示器） |

#### 2.3.3 设备页面 - WiFi（0.5 天）

| 任务 | 组件 | 说明 |
|------|------|------|
| WiFi 连接面板 | `WifiConnect.tsx` | 服务器地址输入、连接按钮 |
| 二维码显示 | `QrCodeDisplay.tsx` | 展示连接二维码（使用 `qrcode.react`） |
| 连接状态 | 复用 `ConnectionStatus.tsx` | WiFi 连接状态显示 |

#### 2.3.4 控制页面（1 天）

| 任务 | 组件 | 说明 |
|------|------|------|
| 功率滑块 | `PowerSlider.tsx` | 双通道 (A/B) 功率调节，0-100 范围 |
| 启停控制 | `OutputControl.tsx` | 开始/停止输出按钮 |
| 实时状态 | `StatusPanel.tsx` | 当前功率、波形类型、运行时间 |
| 安全控制 | `SafetyPanel.tsx` | 紧急停止按钮、功率限制设置 |

#### 2.3.5 波形页面（1 天）

| 任务 | 组件 | 说明 |
|------|------|------|
| 波形选择器 | `WaveformSelector.tsx` | 8 种波形类型网格展示 |
| 波形预览 | `WaveformPreview.tsx` | 实时波形图（Canvas 或 SVG） |
| 参数调节 | `WaveformParams.tsx` | 频率、占空比等参数滑块 |

#### 2.3.6 预设页面（0.5 天）

| 任务 | 组件 | 说明 |
|------|------|------|
| 预设列表 | `PresetList.tsx` | 卡片式预设展示 |
| 预设编辑 | `PresetEditor.tsx` | 创建/编辑预设对话框 |
| 快速应用 | `PresetQuickApply.tsx` | 一键应用预设 |

#### 2.3.7 设置页面（0.5 天）

| 任务 | 组件 | 说明 |
|------|------|------|
| 主题设置 | `ThemeSettings.tsx` | 亮/暗模式切换 |
| 安全设置 | `SafetySettings.tsx` | 最大功率限制、自动断连超时 |
| 关于 | `AboutSection.tsx` | 版本信息、项目链接 |

**验证方法**：
```bash
cd dglab-gui-tauri && npm run tauri dev  # 完整功能可操作
npm run lint                              # 零 lint 错误
npm run build                             # 构建成功
```

### 2.4 桌面端打包（1 天）

| 任务 | 说明 |
|------|------|
| Windows | `npm run tauri build -- --target x86_64-pc-windows-msvc` → `.msi` / `.exe` |
| macOS | `npm run tauri build -- --target aarch64-apple-darwin` → `.dmg` |
| Linux | `npm run tauri build -- --target x86_64-unknown-linux-gnu` → `.deb` / `.AppImage` |
| 配置 `tauri.conf.json` | 应用名称、图标、窗口大小、权限 |
| 应用图标 | 准备各平台图标（`.ico`, `.icns`, `.png`） |

**验证方法**：
```bash
npm run tauri build  # 生成安装包
# 安装并运行，验证所有功能正常
```

### 阶段 2 验收标准

- [ ] `npm run tauri dev` 正常启动
- [ ] BLE 扫描和连接功能可用
- [ ] WiFi 连接和二维码展示可用
- [ ] 双通道功率控制可用
- [ ] 波形选择和预览可用
- [ ] 预设管理（增删改查）可用
- [ ] 桌面端至少一个平台打包成功
- [ ] UI 响应流畅，无明显卡顿
- [ ] 前端 TypeScript 零类型错误

---

## 阶段 3：文档和桌面版交付（2-3 天）

### 目标
- 完善用户文档和开发文档
- 创建 CI/CD 配置
- 完成桌面版正式发布

### 3.1 用户文档（1 天）

| 任务 | 文件 | 说明 |
|------|------|------|
| 更新 README.md | `README.md` | 更新功能列表、截图、安装说明 |
| 使用指南 | `docs/USER_GUIDE.md` | GUI 使用教程（含截图） |
| CLI 参考 | `docs/CLI_REFERENCE.md` | 所有命令详细说明 |
| 安装指南 | `docs/INSTALLATION.md` | 各平台安装步骤 |

### 3.2 开发文档（0.5 天）

| 任务 | 文件 | 说明 |
|------|------|------|
| 架构文档 | `docs/ARCHITECTURE.md` | 系统架构图、模块关系 |
| 贡献指南 | `CONTRIBUTING.md` | 开发环境配置、代码规范、PR 流程 |
| API 文档 | `cargo doc` | 确保 `cargo doc --open` 可用 |

### 3.3 CI/CD 配置（0.5 天）

| 任务 | 文件 | 说明 |
|------|------|------|
| GitHub Actions | `.github/workflows/ci.yml` | 编译、测试、clippy、fmt |
| Release 工作流 | `.github/workflows/release.yml` | Tag 触发多平台构建 |

**CI 流水线**：
```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt -- --check
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo test --workspace
      - run: cargo doc --workspace --no-deps
```

### 3.4 桌面版发布（1 天）

| 任务 | 说明 |
|------|------|
| 版本号确定 | 设置为 `v0.1.0` |
| CHANGELOG 编写 | 首次发布的变更记录 |
| GitHub Release | 创建 Release，附带各平台安装包 |
| 测试安装流程 | 在干净环境测试安装和运行 |

### 阶段 3 验收标准

- [ ] README.md 包含最新截图和准确的功能描述
- [ ] 用户可以根据文档独立完成安装和使用
- [ ] CI 流水线在 GitHub 上正常运行
- [ ] 至少一个平台的安装包可从 GitHub Release 下载
- [ ] `cargo doc` 无警告且文档完整

---

## 阶段 4：Android 移动端开发（5-7 天）

### 目标
- 基于 Tauri 2.0 的 Android 支持，将桌面应用移植到 Android
- 适配移动端 UI
- 处理 Android 特有的权限和 BLE 交互

### 4.1 Android 开发环境搭建（0.5 天）

**前提条件**：

| 工具 | 版本要求 | 安装说明 |
|------|---------|---------|
| Android Studio | 最新稳定版 | 需要 SDK Manager |
| Android SDK | API 24+ (Android 7.0+) | 通过 SDK Manager 安装 |
| Android NDK | r25+ | 通过 SDK Manager 安装 |
| JDK | 17+ | Android Studio 自带 |
| Rust Android targets | - | `rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android` |

**环境变量配置**：
```bash
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/<version>"
export JAVA_HOME="/path/to/jdk17"
export PATH="$PATH:$ANDROID_HOME/platform-tools:$ANDROID_HOME/tools"
```

**验证方法**：
```bash
rustup target list --installed | grep android  # 确认 Android targets
adb devices                                     # ADB 可用
cd dglab-gui-tauri && npm run tauri android init  # 初始化 Android 项目
```

### 4.2 Tauri Android 项目配置（0.5 天）

| 任务 | 说明 |
|------|------|
| 初始化 Android | `npm run tauri android init` |
| 配置 `tauri.conf.json` | 添加 Android bundle identifier |
| 配置权限 | AndroidManifest.xml 中添加 BLE、WiFi、网络权限 |
| 配置 Proguard | 保留 Tauri JNI 相关类 |

**AndroidManifest.xml 关键权限**：
```xml
<!-- BLE 权限 -->
<uses-permission android:name="android.permission.BLUETOOTH" />
<uses-permission android:name="android.permission.BLUETOOTH_ADMIN" />
<uses-permission android:name="android.permission.BLUETOOTH_SCAN" />
<uses-permission android:name="android.permission.BLUETOOTH_CONNECT" />
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />

<!-- WiFi/网络权限 -->
<uses-permission android:name="android.permission.INTERNET" />
<uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
<uses-permission android:name="android.permission.ACCESS_WIFI_STATE" />

<!-- BLE 功能声明 -->
<uses-feature android:name="android.hardware.bluetooth_le" android:required="true" />
```

### 4.3 移动端 UI 适配（2-3 天）

#### 4.3.1 响应式布局改造

| 任务 | 说明 |
|------|------|
| 移动端检测 | 使用 Tauri API 检测平台或媒体查询 |
| 导航改造 | 桌面侧边栏 → 移动端底部 Tab 栏 |
| 触摸优化 | 增大按钮/滑块的触摸区域 (最小 44x44px) |
| 布局断点 | `<768px` 移动端布局，`≥768px` 桌面布局 |

#### 4.3.2 移动端特有组件

| 组件 | 说明 |
|------|------|
| `BottomTabBar.tsx` | 底部 Tab 导航栏（设备/控制/波形/预设/设置） |
| `MobilePowerSlider.tsx` | 大号触摸友好的功率滑块 |
| `PullToRefresh.tsx` | 下拉刷新设备列表 |
| `SwipeableCard.tsx` | 可滑动的设备/预设卡片 |

#### 4.3.3 Tailwind 响应式示例

```tsx
// 导航栏适配
<nav className="
  hidden md:flex md:flex-col md:w-60    // 桌面：侧边栏
  fixed bottom-0 left-0 right-0          // 移动：底部栏
  md:relative md:bottom-auto
">
```

### 4.4 Android BLE 适配（1-2 天）

**关键问题**：`btleplug` 在 Android 上的支持情况需要验证。

| 任务 | 说明 |
|------|------|
| 验证 btleplug Android | 检查 `btleplug` 0.11 的 Android 支持状态 |
| 备选方案：Tauri BLE 插件 | 若 btleplug 不支持，使用 `tauri-plugin-blec` 或自写 JNI 桥接 |
| 运行时权限请求 | Android 12+ 需要动态请求 BLE 扫描/连接权限 |
| BLE 扫描适配 | Android 特有的扫描模式（低功耗/平衡/低延迟） |
| 前台服务 | 长时间 BLE 连接需要前台服务保活 |

**btleplug Android 备选方案**：

如果 `btleplug` 不原生支持 Android，有以下方案：

1. **Tauri Plugin（推荐）**：使用或开发 Tauri BLE 插件，通过 JavaScript 桥接 Android BLE API
2. **JNI 桥接**：直接通过 JNI 调用 Android Java BLE API
3. **Platform Abstraction**：在 `dglab-protocol` 中创建平台抽象层

```rust
// 平台抽象层示例
#[cfg(target_os = "android")]
mod ble_android;

#[cfg(not(target_os = "android"))]
mod ble_desktop;
```

### 4.5 Android 构建和测试（1 天）

| 任务 | 说明 |
|------|------|
| 开发构建 | `npm run tauri android dev` |
| 模拟器测试 | Android Emulator (注意：模拟器不支持真实 BLE) |
| 真机测试 | USB 调试或 WiFi 调试连接真机 |
| APK 签名 | 生成签名密钥，配置签名 |
| Release 构建 | `npm run tauri android build` → `.apk` |

**验证方法**：
```bash
npm run tauri android dev    # 开发模式连接设备
npm run tauri android build  # 生成 APK
adb install target/android/release/app.apk  # 安装测试
```

### 阶段 4 验收标准

- [ ] Android 开发环境配置完成
- [ ] `npm run tauri android dev` 正常启动
- [ ] 移动端 UI 适配完成，触摸操作流畅
- [ ] BLE 功能在 Android 真机上可用
- [ ] WiFi 功能在 Android 上可用
- [ ] 权限请求流程正常（BLE、位置）
- [ ] 生成可安装的 Release APK
- [ ] 在至少 2 款 Android 设备上测试通过

---

## 📱 详细任务清单

### 全部任务汇总

| # | 阶段 | 任务 | 估时 | 依赖 |
|---|------|------|------|------|
| 1 | 1 | Protocol 测试补全 | 1d | 无 |
| 2 | 1 | Core 测试编写 | 1.5d | 无 |
| 3 | 1 | Clippy + 代码清理 | 0.5d | 无 |
| 4 | 1 | 文档补充 + 示例 | 0.5d | 无 |
| 5 | 2 | Tauri 项目初始化 | 1d | #3 |
| 6 | 2 | Tauri 后端命令层 | 2d | #5 |
| 7 | 2 | React 前端 - 基础布局 | 0.5d | #5 |
| 8 | 2 | React 前端 - 设备页面 | 1.5d | #6, #7 |
| 9 | 2 | React 前端 - 控制页面 | 1d | #6, #7 |
| 10 | 2 | React 前端 - 波形页面 | 1d | #6, #7 |
| 11 | 2 | React 前端 - 预设+设置 | 1d | #6, #7 |
| 12 | 2 | 桌面端打包 | 1d | #8-#11 |
| 13 | 3 | 用户文档 | 1d | #12 |
| 14 | 3 | 开发文档 + CI/CD | 0.5d | #1-#4 |
| 15 | 3 | 桌面版发布 | 1d | #12, #13 |
| 16 | 4 | Android 环境搭建 | 0.5d | #5 |
| 17 | 4 | Tauri Android 配置 | 0.5d | #16 |
| 18 | 4 | 移动端 UI 适配 | 2.5d | #8-#11, #17 |
| 19 | 4 | Android BLE 适配 | 1.5d | #17 |
| 20 | 4 | Android 构建和测试 | 1d | #18, #19 |

### 并行工作建议

以下任务可以并行进行：

- **#1-#4**（阶段 1 所有任务）可以同时推进
- **#6** 和 **#7** 可以同时开发（后端和前端基础）
- **#8, #9, #10, #11** 前端各页面可以并行
- **#13** 和 **#14** 可以并行
- **#18** 和 **#19** 可以并行

---

## 📅 时间线

```
Day  1  2  3  4  5  6  7  8  9  10  11  12  13  14  15  16  17  18  19  20  21
     ├──── 阶段 1 ────┤
     │ Protocol 测试   │
     │ Core 测试       │
     │ Clippy/清理     │
     │ 文档/示例       │
                       ├────────────── 阶段 2 ──────────────────┤
                       │ Tauri 初始化                            │
                       │ 后端命令层                               │
                       │      前端布局                            │
                       │        设备页面                          │
                       │          控制页面                        │
                       │            波形页面                      │
                       │              预设+设置                   │
                       │                        桌面打包          │
                                                                 ├── 阶段 3 ──┤
                                                                 │ 用户文档    │
                                                                 │ 开发文档    │
                                                                 │ 桌面发布    │
                                                                              ├──── 阶段 4 ────┤
                                                                              │ Android 环境    │
                                                                              │ Android 配置    │
                                                                              │ UI 适配         │
                                                                              │ BLE 适配        │
                                                                              │ 构建测试        │
```

**里程碑**：

| 里程碑 | 预期日期 | 标志 |
|--------|---------|------|
| M1: 代码质量达标 | Day 4 | 测试 ≥ 40，clippy 零警告 |
| M2: GUI 桌面端可用 | Day 14 | `npm run tauri dev` 完整功能 |
| M3: 桌面版正式发布 | Day 17 | GitHub Release v0.1.0 |
| M4: Android 版本可用 | Day 21 | APK 可安装运行 |

---

## ✅ 验收标准汇总

### 代码质量

- [ ] `cargo test --workspace` ≥ 40 个测试全部通过
- [ ] `cargo clippy --workspace -- -D warnings` 零警告
- [ ] `cargo fmt -- --check` 通过
- [ ] 无 `unimplemented!()` 或 `todo!()` 调用
- [ ] 无 `unwrap()` 或 `expect()` 在库代码中

### 桌面 GUI

- [ ] 应用启动时间 < 3 秒
- [ ] BLE 设备扫描和连接正常
- [ ] WiFi 连接和二维码显示正常
- [ ] 双通道功率控制响应及时
- [ ] 波形选择和实时预览正常
- [ ] 预设增删改查正常
- [ ] 暗色/亮色主题切换正常
- [ ] 紧急停止按钮可用

### Android

- [ ] APK 大小 < 30MB
- [ ] 支持 Android 7.0+ (API 24+)
- [ ] BLE 权限请求流程顺畅
- [ ] 触摸操作流畅（滑块、按钮）
- [ ] 后台 BLE 连接稳定
- [ ] 竖屏/横屏布局正常

### 文档

- [ ] README.md 包含安装说明和截图
- [ ] 用户指南覆盖主要功能
- [ ] `cargo doc` 文档完整无警告

---

## 💡 注意事项

### 技术风险

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| `btleplug` Android 支持不完整 | 阶段 4 BLE 功能 | 准备 Tauri Plugin 备选方案 |
| WebView 兼容性 | 低版本 Android | 设置最低 API 24，测试 WebView 特性 |
| BLE 协议未经真机验证 | 全部 BLE 功能 | 尽早获取真实设备测试 |
| Tauri 2.0 仍在快速迭代 | 构建稳定性 | 锁定依赖版本，关注 release notes |

### 开发建议

1. **测试优先**：阶段 1 的测试工作对后续所有阶段都有保障作用，不要跳过
2. **增量开发**：每完成一个 Tauri command，立即在前端验证，避免大量集成
3. **真机测试**：BLE 功能必须在真机上测试，模拟器不支持蓝牙
4. **Git 分支策略**：每个阶段使用独立分支，完成后合并到 main
5. **每日构建**：每天至少运行一次 `cargo build --workspace && cargo test --workspace`

### 已知技术债

| 项目 | 位置 | 说明 |
|------|------|------|
| 脚本引擎空壳 | `dglab-core/src/script/` | 决定是否实现或移除 |
| TUI 空壳 | `dglab-cli/src/tui/` | 决定是否实现或移除 |
| Packet decoder "示例实现" | `dglab-protocol/src/packet/decoder.rs:108` | 需要根据真实协议验证 |
| egui GUI 已弃用 | `crates/dglab-gui/` | 完成 Tauri GUI 后可删除 |

---

## 🔧 工具链准备

### Rust 工具链

```bash
# 确保 stable toolchain
rustup update stable
rustup default stable

# 安装 Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android

# 安装常用工具
cargo install cargo-watch   # 文件变更自动编译
cargo install cargo-tarpaulin  # 测试覆盖率
```

### 前端工具链

```bash
# Node.js 18+ (推荐使用 nvm)
nvm install 20
nvm use 20

# 包管理器（选择一个）
npm install -g pnpm  # 推荐
```

### Android 开发环境

```bash
# 1. 安装 Android Studio
# 下载: https://developer.android.com/studio

# 2. 通过 SDK Manager 安装
#    - Android SDK Platform 34
#    - Android SDK Build-Tools 34
#    - Android NDK (Side by side) r25+
#    - Android SDK Command-line Tools

# 3. 环境变量 (~/.bashrc 或 ~/.zshrc)
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/25.2.9519653"  # 替换为实际版本
export JAVA_HOME="/usr/lib/jvm/java-17-openjdk"    # 替换为实际路径
export PATH="$PATH:$ANDROID_HOME/platform-tools"
export PATH="$PATH:$ANDROID_HOME/cmdline-tools/latest/bin"

# 4. 验证
sdkmanager --version
adb --version
rustup target list --installed | grep android
```

### Tauri CLI

```bash
# 安装 Tauri CLI
cargo install tauri-cli

# 或通过 npm
npm install -g @tauri-apps/cli

# 验证
cargo tauri --version
```

---

## 📦 最终交付物

### 阶段 1 交付

| 交付物 | 说明 |
|--------|------|
| 测试套件 | ≥ 40 个测试，覆盖核心模块 |
| 代码示例 | `examples/` 下至少 2 个示例 |

### 阶段 2 交付

| 交付物 | 说明 |
|--------|------|
| 桌面应用源码 | `dglab-gui-tauri/` 完整项目 |
| Windows 安装包 | `.msi` 或 `.exe` |
| macOS 安装包 | `.dmg` |
| Linux 安装包 | `.deb` / `.AppImage` |

### 阶段 3 交付

| 交付物 | 说明 |
|--------|------|
| 用户文档 | `docs/USER_GUIDE.md`, `docs/CLI_REFERENCE.md`, `docs/INSTALLATION.md` |
| 开发文档 | `docs/ARCHITECTURE.md`, `CONTRIBUTING.md` |
| CI/CD 配置 | `.github/workflows/ci.yml`, `release.yml` |
| GitHub Release | v0.1.0 正式发布 |

### 阶段 4 交付

| 交付物 | 说明 |
|--------|------|
| Android APK | 签名的 Release APK |
| 移动端适配代码 | 响应式组件和移动专用组件 |
| Android 开发文档 | 环境配置和构建说明 |

### 完整项目最终结构

```
DG_LAB/
├── Cargo.toml                    # workspace 配置
├── README.md                     # 项目说明（含截图）
├── CONTRIBUTING.md               # 贡献指南
├── CHANGELOG.md                  # 变更记录
├── LICENSE-MIT / LICENSE-APACHE  # 许可证
├── .github/
│   └── workflows/
│       ├── ci.yml                # CI 流水线
│       └── release.yml           # Release 工作流
├── crates/
│   ├── dglab-protocol/           # 协议层（含完整测试）
│   ├── dglab-core/               # 核心层（含完整测试）
│   └── dglab-cli/                # CLI 工具
├── dglab-gui-tauri/              # Tauri + React GUI
│   ├── src/                      # React 前端
│   ├── src-tauri/                # Tauri 后端
│   └── package.json
├── docs/
│   ├── USER_GUIDE.md
│   ├── CLI_REFERENCE.md
│   ├── INSTALLATION.md
│   ├── ARCHITECTURE.md
│   └── protocol/                 # 协议文档
├── examples/                     # 示例代码
└── scripts/                      # 辅助脚本
```
