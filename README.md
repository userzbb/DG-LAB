# DG-LAB Rust 跨平台控制器

一个用 Rust 开发的完整 DG-LAB 设备跨平台控制器，支持桌面端（Windows/macOS/Linux）和移动端（Android）。

## ✨ 功能特性

### 核心功能
- ✅ **BLE (蓝牙低功耗)** - 完整支持 DG-LAB Coyote 3.0 设备
- ✅ **WiFi 连接** - WebSocket 协议支持，二维码快速配对
- ✅ **双通道功率控制** - 独立控制 A/B 两个通道，实时调节 (0-200)
- ✅ **波形生成器** - 8 种内置波形类型（连续波、脉冲波、正弦波、方波、三角波、锯齿波、呼吸波、渐强渐弱）
- ✅ **预设管理** - 保存和加载自定义预设配置
- ✅ **实时事件通知** - 设备状态、功率、电池电量实时更新
- ✅ **Toast 通知系统** - 友好的用户反馈提示

### 用户界面
- ✅ **桌面 GUI** - 基于 Tauri 2.0 + React 19 的现代化界面
- ✅ **命令行 CLI** - 完整的命令行工具，支持脚本化控制
- ✅ **终端 TUI** - 交互式终端用户界面
- 🚧 **Android 应用** - 移动端支持（即将推出）

### 平台支持
- ✅ Windows 10/11
- ✅ macOS 10.15+
- ✅ Linux (支持 GTK 3.24+)
- 🚧 Android 7.0+ (API 24+)

## 📂 项目结构

```
DG_LAB/
├── crates/
│   ├── dglab-protocol/        # 📡 协议库 (BLE/WiFi)
│   │   ├── v3.rs             # V3 BLE 协议 (Coyote 3.0)
│   │   ├── ble/              # BLE 管理器和扫描
│   │   └── wifi/             # WiFi WebSocket 协议
│   ├── dglab-core/           # 🧠 核心业务逻辑
│   │   ├── device/           # 设备抽象和实现
│   │   ├── session/          # 会话管理
│   │   ├── waveform/         # 波形生成器
│   │   └── preset/           # 预设存储
│   ├── dglab-cli/            # 💻 命令行工具
│   │   ├── commands/         # CLI 命令
│   │   └── tui/              # 终端 UI
│   └── dglab-gui/            # 🎨 旧版 GUI (已弃用)
├── apps/
│   └── dglab-gui-tauri/      # 🖥️ Tauri + React GUI
│       ├── src/              # React 前端
│       │   ├── components/   # shadcn-ui 组件
│       │   ├── pages/        # 应用页面
│       │   ├── stores/       # Zustand 状态管理
│       │   ├── hooks/        # React Hooks
│       │   └── types/        # TypeScript 类型
│       └── src-tauri/        # Rust 后端
│           ├── commands/     # Tauri 命令
│           └── events.rs     # 事件系统
├── docs/                      # 📚 文档
│   ├── protocols/            # 协议逆向分析文档
│   ├── USER_GUIDE.md         # 用户指南
│   ├── INSTALLATION.md       # 安装说明
│   └── ARCHITECTURE.md       # 架构文档
└── examples/                  # 💡 示例代码
```

## 🚀 快速开始

### 先决条件

- **Rust**: 1.70+ (安装: https://rustup.rs/)
- **Node.js**: 18+ (仅 GUI 需要)
- **系统依赖**:
  - Linux: `libdbus-1-dev`, `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`
  - macOS: Xcode Command Line Tools
  - Windows: 无额外依赖

### 安装

```bash
# 克隆仓库
git clone https://github.com/your-username/DG_LAB.git
cd DG_LAB

# 构建所有组件
cargo build --release

# 构建 GUI (需要先构建前端)
cd apps/dglab-gui-tauri
npm install
npm run tauri build
```

### 使用 CLI

```bash
# 扫描 BLE 设备
cargo run --bin dglab -- scan

# 连接设备（交互式）
cargo run --bin dglab -- connect

# 控制设备功率
cargo run --bin dglab -- control --power 50

# 启动 TUI 界面
cargo run --bin dglab -- tui

# 查看所有命令
cargo run --bin dglab -- --help
```

### 使用 GUI

```bash
# 开发模式
cd apps/dglab-gui-tauri
npm run tauri dev

# 生产构建
npm run tauri build
```

## 📊 开发状态

### Phase 1: 核心库 ✅ (100%)
- ✅ DG-LAB V3 BLE 协议完整实现
- ✅ WiFi WebSocket 协议支持
- ✅ 设备抽象层和会话管理
- ✅ 波形生成器（8 种波形类型）
- ✅ 预设存储系统
- ✅ **263 个测试** 全部通过 (144 core + 113 protocol + 6 doc-tests)

### Phase 2: Tauri + React GUI ✅ (100%)
- ✅ Tauri 2.0 项目初始化
- ✅ React 19 + TypeScript + Vite 5
- ✅ Tailwind CSS v4 + shadcn-ui 组件库
- ✅ Zustand 状态管理
- ✅ 5 个完整页面:
  - Dashboard (仪表盘)
  - Device Scanner (设备扫描)
  - Power Control (功率控制)
  - Waveform Generator (波形生成器)
  - Preset Manager (预设管理)
- ✅ 10 个 shadcn-ui 组件
- ✅ Toast 通知系统 (Sonner)
- ✅ 实时事件处理（Rust → React）
- ✅ 11 个 Tauri 命令
- ✅ 0 TypeScript 错误，0 Rust 编译错误

### Phase 3: 文档和发布 🚧 (进行中)
- 🚧 用户指南和安装文档
- 🚧 CI/CD 配置
- ⏳ GitHub Release 准备

### Phase 4: Android 移动端 ⏳ (计划中)
- ⏳ Tauri 2.0 Android 配置
- ⏳ 移动端 UI 适配
- ⏳ Android BLE 权限处理

## 🛠️ 技术栈

### 后端 (Rust)
- **异步运行时**: [tokio](https://tokio.rs/) 1.x
- **BLE 通信**: [btleplug](https://github.com/deviceplug/btleplug) 0.11
- **WebSocket**: [tungstenite](https://github.com/snapview/tungstenite-rs) + tokio-tungstenite
- **序列化**: [serde](https://serde.rs/) 1.0 + bincode
- **错误处理**: [thiserror](https://github.com/dtolnay/thiserror) 1.0
- **日志**: [tracing](https://github.com/tokio-rs/tracing) 0.1
- **CLI**: [clap](https://github.com/clap-rs/clap) 4.x
- **TUI**: [ratatui](https://github.com/ratatui-org/ratatui) 0.25

### 前端 (React + TypeScript)
- **框架**: [React](https://react.dev/) 19
- **构建工具**: [Vite](https://vitejs.dev/) 5
- **类型系统**: [TypeScript](https://www.typescriptlang.org/) 5
- **样式**: [Tailwind CSS](https://tailwindcss.com/) v4
- **UI 组件**: [shadcn-ui](https://ui.shadcn.com/)
- **状态管理**: [Zustand](https://github.com/pmndrs/zustand) 5
- **路由**: [React Router](https://reactrouter.com/) 6
- **通知**: [Sonner](https://sonner.emilkowal.ski/)
- **图标**: [Lucide React](https://lucide.dev/)

### 桌面应用
- **框架**: [Tauri](https://tauri.app/) 2.0
- **IPC**: Tauri Commands + Events
- **窗口管理**: Tauri Window API

## 🧪 测试

```bash
# 运行所有测试
cargo test --workspace

# 运行特定 crate 的测试
cargo test -p dglab-protocol
cargo test -p dglab-core
cargo test -p dglab-cli

# 运行 clippy 检查
cargo clippy --workspace -- -D warnings

# 检查代码格式
cargo fmt --check

# 生成文档
cargo doc --workspace --no-deps --open
```

**测试覆盖**:
- `dglab-protocol`: 113 个测试 ✅
- `dglab-core`: 144 个测试 ✅
- 文档测试: 6 个 ✅
- **总计**: 263 个测试全部通过 🎉

## 📖 文档

- [用户指南](docs/USER_GUIDE.md) - GUI 和 CLI 使用教程
- [安装说明](docs/INSTALLATION.md) - 各平台详细安装步骤
- [架构文档](docs/ARCHITECTURE.md) - 系统架构和模块设计
- [贡献指南](CONTRIBUTING.md) - 开发环境配置和代码规范
- [协议文档](docs/protocols/) - DG-LAB 协议逆向分析

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细信息。

### 开发工作流

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## ⚠️ 注意事项

- **实验性项目**: 此项目基于 DG-LAB 协议逆向工程，非官方实现
- **安全使用**: 请谨慎使用电刺激设备，注意安全限制
- **硬件要求**: 需要支持 BLE 的蓝牙适配器或 WiFi 连接
- **设备兼容性**: 目前仅测试 DG-LAB Coyote 3.0 设备

## 📄 许可证

本项目采用双许可证:
- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

任选其一使用。

## 🙏 致谢

- DG-LAB 团队提供的优秀硬件设备
- Rust 社区和相关开源项目
- 所有贡献者和测试用户

## 📞 联系方式

- Issues: [GitHub Issues](https://github.com/your-username/DG_LAB/issues)
- Discussions: [GitHub Discussions](https://github.com/your-username/DG_LAB/discussions)

---

**⚡ 由 Rust 驱动 | 🎨 使用 React 构建 | 💙 为社区开发**
