# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned
- Android mobile application
- Script engine for custom control logic
- Cloud preset synchronization
- Multi-user support with authentication
- Remote control over the internet
- Usage statistics and analytics

---

## [0.1.0] - 2026-02-16

### 🎉 初始版本发布

这是 DG-LAB Rust 控制器的首个公开版本,提供了跨平台桌面 GUI、命令行工具和 TUI 界面,支持通过 BLE 和 WiFi 控制 DG-LAB Coyote 3.0 设备。

### ✨ 主要特性

#### 核心功能
- **多协议支持**: 完整实现 DG-LAB V3 BLE 协议和 WiFi WebSocket 协议
- **设备管理**: 支持多设备并发连接和控制
- **会话管理**: 统一的会话管理器,管理设备生命周期
- **波形生成**: 内置 4 种波形类型 (正弦波、方波、锯齿波、随机波)
- **预设系统**: 保存和加载控制预设,快速切换场景
- **实时通信**: 设备状态变化实时推送到 UI

#### 桌面 GUI (Tauri + React)
- **5 个功能页面**:
  - 仪表盘 (Dashboard) - 设备概览和快速操作
  - 设备扫描器 (Scanner) - BLE 设备扫描和连接
  - 功率控制 (Control) - 实时功率调节 (通道 A/B)
  - 波形生成器 (Waveform) - 自定义波形生成和预览
  - 预设管理器 (Presets) - 预设保存、分类和管理
- **现代化 UI**: 基于 Tailwind CSS v4 和 shadcn-ui 组件库
- **状态管理**: 使用 Zustand 进行全局状态管理
- **通知系统**: Sonner toast 通知,提供即时反馈
- **响应式设计**: 适配不同屏幕尺寸

#### 命令行工具 (CLI)
- `dglab scan` - 扫描附近的 BLE 设备
- `dglab connect <DEVICE_ID>` - 连接到指定设备
- `dglab control --power-a <A> --power-b <B>` - 控制设备功率
- `dglab tui` - 启动交互式终端界面 (TUI)
- 支持 `--debug` 标志启用调试日志

#### 终端界面 (TUI)
- 实时设备状态显示
- 交互式功率控制 (方向键调节)
- 键盘快捷键支持
- 彩色终端输出

### 🏗️ 架构设计

#### Rust 库
- **dglab-protocol**: 协议实现层
  - BLE 设备扫描和连接 (`btleplug`)
  - V3 协议数据包编解码
  - WiFi WebSocket 客户端
  - CRC-16 校验
- **dglab-core**: 核心业务逻辑层
  - `Device` trait 抽象
  - `SessionManager` 会话管理器
  - `WaveformGenerator` 波形生成器
  - `PresetStorage` 预设存储
- **dglab-cli**: 命令行工具
  - Clap 命令行解析
  - Ratatui TUI 框架

#### Tauri 应用
- **前端**: React 19 + TypeScript 5 + Vite 5
  - 4 个 Zustand stores (app, device, waveform, preset)
  - 10+ shadcn-ui 组件
  - Tauri API 封装
- **后端**: Rust + Tauri 2.0
  - 11 个 Tauri commands
  - 5 种设备事件类型
  - 应用状态管理

### 🧪 测试

- **263 个测试通过**:
  - dglab-protocol: 113 tests
  - dglab-core: 144 tests
  - Doc tests: 6 tests
- **测试覆盖**: 核心逻辑 > 80%
- **0 Clippy 警告**: 通过所有 lint 检查
- **0 TypeScript 错误**: 完整类型安全

### 📦 构建产物

- **前端构建**:
  - JavaScript: 432.38 KB (136.41 KB gzipped)
  - CSS: 26.70 KB (5.50 KB gzipped)
- **后端**: 优化的 Rust 二进制文件
- **支持平台**:
  - Windows (x64)
  - macOS (Apple Silicon + Intel)
  - Linux (x64)

### 📚 文档

- **README.md**: 项目概览和快速开始
- **docs/USER_GUIDE.md**: 完整用户手册 (450+ 行)
  - GUI 使用指南
  - CLI 命令参考
  - TUI 键盘快捷键
  - 故障排除 (8 个常见问题)
- **docs/INSTALLATION.md**: 安装指南 (600+ 行)
  - 系统要求
  - 预构建安装包说明
  - 从源码构建
  - 平台特定说明
  - 卸载步骤
- **docs/ARCHITECTURE.md**: 架构文档
  - 系统架构图
  - 模块设计
  - 数据流图
  - API 参考
- **CONTRIBUTING.md**: 贡献指南
  - 开发环境配置
  - 代码规范
  - Git 工作流
  - PR 流程

### 🔧 技术栈

**后端 (Rust)**
- Rust 2021 Edition
- tokio 1.35 (异步运行时)
- btleplug 0.11 (BLE 通信)
- Tauri 2.0 (GUI 框架)
- serde 1.0 (序列化)
- tracing 0.1 (日志)
- thiserror 1.0 (错误处理)

**前端 (TypeScript/React)**
- React 19
- TypeScript 5
- Vite 5 (构建工具)
- Tailwind CSS v4 (样式)
- shadcn-ui (UI 组件)
- Zustand (状态管理)
- Sonner (通知)

### ⚠️ 已知限制

1. **BLE 连接稳定性**
   - 某些 Linux 发行版可能需要额外配置 BlueZ
   - macOS 需要在系统设置中授予蓝牙权限
   - Windows 需要 Windows 10 (Build 1903) 或更高版本

2. **功率范围**
   - 当前支持 0-200 范围,设备硬件实际支持范围取决于固件版本

3. **WiFi 连接**
   - WiFi WebSocket 协议已实现但未在 GUI 中暴露
   - 需要设备固件支持 WiFi 功能

4. **预设同步**
   - 预设仅保存在本地,暂不支持云同步

5. **平台支持**
   - 桌面平台: ✅ 完全支持
   - Android: ⏳ 计划中 (Phase 4)
   - iOS: ❌ 暂无计划

### 🐛 Bug 修复

无 (初始版本)

### 🔒 安全性

- BLE 连接默认加密
- Tauri 沙箱限制
- 无远程代码执行风险
- 本地数据存储,无云端传输

### 📈 性能

- BLE 扫描: < 10 秒 (典型)
- 连接建立: < 3 秒
- 功率控制延迟: < 100ms
- GUI 启动时间: < 2 秒
- 内存占用: ~50MB (GUI), ~5MB (CLI)

### 🙏 致谢

感谢所有参与测试和反馈的社区成员!

---

## 版本说明

### 版本号规则

项目遵循 [语义化版本](https://semver.org/) (Semantic Versioning):

```
MAJOR.MINOR.PATCH
```

- **MAJOR**: 不兼容的 API 变更
- **MINOR**: 向后兼容的功能新增
- **PATCH**: 向后兼容的 Bug 修复

### 发布周期

- **主版本** (1.0, 2.0): 重大架构变更或破坏性更新
- **次版本** (0.1, 0.2): 新功能发布,每 1-2 个月
- **修订版本** (0.1.1, 0.1.2): Bug 修复,按需发布

### Alpha/Beta/RC 版本

预发布版本命名:
- Alpha: `0.2.0-alpha.1` (早期测试)
- Beta: `0.2.0-beta.1` (功能完整,测试中)
- RC: `0.2.0-rc.1` (发布候选)

---

## 迁移指南

### 从未来版本迁移

暂无 (当前为初始版本)

---

## 链接

- [源代码](https://github.com/your-org/dglab-rs)
- [问题跟踪](https://github.com/your-org/dglab-rs/issues)
- [发布页面](https://github.com/your-org/dglab-rs/releases)
- [文档](https://github.com/your-org/dglab-rs/tree/main/docs)

---

## 贡献者

感谢以下贡献者对本项目的贡献:

<!-- 这里会自动生成贡献者列表 -->

如果您想贡献代码,请阅读 [CONTRIBUTING.md](CONTRIBUTING.md)。

---

**[0.1.0]**: https://github.com/your-org/dglab-rs/releases/tag/v0.1.0
