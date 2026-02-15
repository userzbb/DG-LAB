# 贡献指南 (Contributing Guide)

感谢您对 DG-LAB Rust 控制器项目的关注!本文档提供了参与项目开发的详细指南。

欢迎任何形式的贡献,包括但不限于:
- 报告 Bug
- 提出新功能建议
- 提交代码改进
- 完善文档
- 分享使用经验

---

## 目录

- [开发环境配置](#开发环境配置)
- [项目结构](#项目结构)
- [代码风格与规范](#代码风格与规范)
- [开发工作流](#开发工作流)
- [测试要求](#测试要求)
- [提交规范](#提交规范)
- [Pull Request 流程](#pull-request-流程)
- [问题反馈](#问题反馈)

---

## 开发环境配置

### 前置要求

#### 必需工具

1. **Rust 工具链** (>= 1.70)
   ```bash
   # 安装 Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # 验证安装
   rustc --version
   cargo --version
   ```

2. **Node.js** (>= 18.0) 和 **npm** (>= 9.0)
   ```bash
   # 使用 nvm 安装 (推荐)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 18
   nvm use 18
   
   # 验证安装
   node --version
   npm --version
   ```

3. **Git**
   ```bash
   # Debian/Ubuntu
   sudo apt-get install git
   
   # macOS
   brew install git
   
   # 验证安装
   git --version
   ```

#### 平台特定依赖

**Linux (Debian/Ubuntu)**:
```bash
# BLE 支持 (必需)
sudo apt-get install -y libudev-dev libdbus-1-dev

# Tauri 依赖
sudo apt-get install -y libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

**macOS**:
```bash
# BLE 支持已内置,仅需安装 Xcode Command Line Tools
xcode-select --install
```

**Windows**:
- 安装 [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- 安装 [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### 克隆项目

```bash
git clone https://github.com/your-org/dglab-rs.git
cd dglab-rs
```

### 构建项目

#### 1. 构建 Rust 后端

```bash
# 构建所有 crate
cargo build

# 或构建特定 crate
cargo build -p dglab-protocol
cargo build -p dglab-core
cargo build -p dglab-cli
```

#### 2. 构建 Tauri GUI

```bash
cd apps/dglab-gui-tauri

# 安装前端依赖
npm install

# 开发模式运行 (热重载)
npm run tauri dev

# 生产构建
npm run tauri build
```

### 验证安装

```bash
# 运行测试
cargo test

# 运行 Clippy 检查
cargo clippy

# 检查代码格式
cargo fmt -- --check

# 运行 CLI
cargo run --bin dglab -- --help
```

---

## 项目结构

```
DG_LAB/
├── crates/                          # Rust 库
│   ├── dglab-protocol/              # 协议实现
│   │   ├── src/
│   │   │   ├── ble/                 # BLE 通信
│   │   │   ├── v3/                  # V3 协议
│   │   │   ├── wifi/                # WiFi WebSocket
│   │   │   └── error.rs             # 错误定义
│   │   └── Cargo.toml
│   ├── dglab-core/                  # 核心逻辑
│   │   ├── src/
│   │   │   ├── device/              # 设备抽象
│   │   │   ├── session/             # 会话管理
│   │   │   ├── waveform/            # 波形生成
│   │   │   ├── preset/              # 预设管理
│   │   │   └── script/              # 脚本引擎 (计划中)
│   │   └── Cargo.toml
│   └── dglab-cli/                   # CLI 工具
│       ├── src/
│       │   ├── commands/            # CLI 命令
│       │   └── tui/                 # TUI 实现
│       └── Cargo.toml
├── apps/                            # 应用程序
│   └── dglab-gui-tauri/             # Tauri + React GUI
│       ├── src/                     # React 前端
│       │   ├── pages/               # 页面组件
│       │   ├── components/          # UI 组件
│       │   ├── stores/              # Zustand 状态管理
│       │   └── lib/                 # 工具函数
│       ├── src-tauri/               # Rust 后端
│       │   └── src/
│       │       ├── commands/        # Tauri 命令
│       │       ├── events.rs        # 事件定义
│       │       └── state.rs         # 应用状态
│       ├── package.json
│       └── tauri.conf.json
├── docs/                            # 文档
│   ├── USER_GUIDE.md
│   ├── INSTALLATION.md
│   └── ARCHITECTURE.md
├── examples/                        # 示例代码
├── scripts/                         # 构建脚本
├── Cargo.toml                       # Workspace 配置
├── CONTRIBUTING.md                  # 本文档
├── CHANGELOG.md                     # 版本历史
└── README.md                        # 项目说明
```

---

## 代码风格与规范

### Rust 代码规范

#### 1. 格式化

项目使用 `rustfmt` 进行代码格式化。所有代码提交前必须通过格式检查:

```bash
# 格式化代码
cargo fmt

# 检查格式 (CI 会运行此命令)
cargo fmt -- --check
```

**配置** (`rustfmt.toml`):
```toml
edition = "2021"
max_width = 100
tab_spaces = 4
```

#### 2. Lint 规则

项目使用 `clippy` 进行代码检查。所有代码必须通过 clippy 检查:

```bash
# 运行 clippy
cargo clippy

# 自动修复部分警告
cargo clippy --fix
```

**Workspace Lints** (`Cargo.toml`):
```toml
[workspace.lints.rust]
unused_crate_dependencies = "warn"
unused_qualifications = "warn"
unused_results = "warn"
```

#### 3. 模块组织

**导入顺序**: 分组导入,组间空行分隔:

```rust
// 1. 标准库
use std::collections::HashMap;
use std::sync::Arc;

// 2. 外部 crate (按字母顺序)
use anyhow::Result;
use tokio::sync::RwLock;
use tracing::{debug, info};

// 3. 内部 crate
use crate::device::{Device, DeviceState};
use crate::error::CoreError;

// 4. 父模块/同级模块
use super::session::SessionManager;
```

#### 4. 文档注释

所有公共 API 必须有文档注释:

```rust
//! 模块级文档 (文件顶部)
//!
//! 本模块实现了设备会话管理功能。

/// 会话管理器
///
/// 管理所有设备的生命周期,提供统一的设备控制接口。
///
/// # 示例
///
/// ```
/// use dglab_core::SessionManager;
///
/// let manager = SessionManager::new();
/// // ... 使用 manager
/// ```
pub struct SessionManager {
    /// 设备映射表
    devices: HashMap<String, Device>,
}

impl SessionManager {
    /// 创建新的会话管理器
    ///
    /// # 返回
    ///
    /// 返回一个空的会话管理器实例
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }
    
    /// 添加设备到会话
    ///
    /// # 参数
    ///
    /// - `device`: 实现了 Device trait 的设备对象
    ///
    /// # 返回
    ///
    /// - `Ok(String)`: 设备 ID
    /// - `Err(CoreError)`: 添加失败时的错误
    ///
    /// # 错误
    ///
    /// 当设备 ID 已存在时返回错误。
    pub async fn add_device(&mut self, device: Box<dyn Device>) -> Result<String> {
        // 实现...
    }
}
```

#### 5. 错误处理

- 库代码禁止使用 `unwrap()` 或 `expect()`
- 使用 `thiserror` 定义错误类型
- 使用 `Result<T>` 传播错误

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    /// 协议错误
    #[error("Protocol error: {0}")]
    ProtocolError(#[from] dglab_protocol::ProtocolError),
    
    /// 设备未连接
    #[error("Device not connected: {0}")]
    DeviceNotConnected(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
```

#### 6. 命名约定

| 类型 | 规则 | 示例 |
|------|------|------|
| Crate | kebab-case | `dglab-protocol`, `dglab-core` |
| Module | snake_case | `session_manager`, `device_traits` |
| Struct/Enum/Trait | PascalCase | `SessionManager`, `DeviceState` |
| Function/Variable | snake_case | `add_device`, `device_id` |
| Constant | SCREAMING_SNAKE_CASE | `MAX_POWER`, `SERVICE_UUID` |
| Type Alias | PascalCase + 描述后缀 | `DeviceBox`, `DeviceMap` |

#### 7. Async 和并发

- 使用 `tokio` 作为异步运行时
- Async trait 使用 `async-trait` crate
- 共享状态使用 `Arc<RwLock<T>>`

```rust
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::RwLock;

#[async_trait]
pub trait Device: Send + Sync {
    async fn connect(&mut self) -> Result<()>;
}

type DeviceMap = Arc<RwLock<HashMap<String, Box<dyn Device>>>>;
```

### TypeScript/React 代码规范

#### 1. ESLint 和 Prettier

项目使用 ESLint 和 Prettier 进行代码检查和格式化:

```bash
# 在 apps/dglab-gui-tauri 目录下
npm run lint          # 运行 ESLint
npm run format        # 运行 Prettier
```

#### 2. 文件组织

```
src/
├── pages/              # 页面组件 (PascalCase.tsx)
├── components/         # 可复用组件
│   └── ui/            # shadcn-ui 组件
├── stores/            # Zustand stores (camelCase.ts)
├── lib/               # 工具函数
├── types/             # TypeScript 类型定义
└── styles/            # 全局样式
```

#### 3. 命名约定

| 类型 | 规则 | 示例 |
|------|------|------|
| 组件文件 | PascalCase.tsx | `Dashboard.tsx`, `PowerControl.tsx` |
| 非组件文件 | camelCase.ts | `deviceStore.ts`, `tauri.ts` |
| 组件名 | PascalCase | `function Dashboard() {}` |
| 函数/变量 | camelCase | `connectDevice`, `deviceId` |
| 常量 | SCREAMING_SNAKE_CASE | `MAX_POWER`, `API_TIMEOUT` |
| 类型/接口 | PascalCase | `DeviceInfo`, `AppState` |

#### 4. TypeScript 类型

强制使用 TypeScript,禁止 `any` 类型:

```typescript
// ❌ 不好
const data: any = await invoke('get_data');

// ✅ 好
interface DeviceInfo {
  id: string;
  name: string;
  state: DeviceState;
}

const data: DeviceInfo = await invoke<DeviceInfo>('get_device_info', { deviceId });
```

#### 5. React Hooks 规范

```typescript
import { useState, useEffect } from 'react';

function MyComponent() {
  // 1. 所有 hooks 在组件顶部
  const [state, setState] = useState<string>('');
  const deviceStore = useDeviceStore();
  
  // 2. useEffect 放在 hooks 后面
  useEffect(() => {
    // 副作用逻辑
    return () => {
      // 清理逻辑
    };
  }, [dependencies]);
  
  // 3. 事件处理函数
  const handleClick = () => {
    // ...
  };
  
  // 4. 返回 JSX
  return <div>...</div>;
}
```

---

## 开发工作流

### 分支策略

项目使用 **Git Flow** 分支模型:

```
main (production)
  ├── develop (development)
  │   ├── feature/xxx (新功能)
  │   ├── fix/xxx (bug 修复)
  │   └── docs/xxx (文档)
  └── release/v0.x.x (发布分支)
```

### 开发流程

#### 1. 创建新分支

```bash
# 从 develop 创建功能分支
git checkout develop
git pull origin develop
git checkout -b feature/your-feature-name

# Bug 修复
git checkout -b fix/bug-description

# 文档改进
git checkout -b docs/documentation-update
```

#### 2. 本地开发

```bash
# 编写代码...

# 运行测试
cargo test

# 运行 clippy
cargo clippy

# 格式化代码
cargo fmt

# 提交代码 (见下方提交规范)
git add .
git commit -m "feat: add new device support"
```

#### 3. 推送并创建 PR

```bash
# 推送到远程
git push origin feature/your-feature-name

# 在 GitHub 上创建 Pull Request
# 目标分支: develop
```

---

## 测试要求

### 单元测试

所有新功能必须包含单元测试:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_manager_creation() {
        let manager = SessionManager::new();
        assert_eq!(manager.device_count(), 0);
    }
    
    #[tokio::test]
    async fn test_add_device() {
        let mut manager = SessionManager::new();
        let device = MockDevice::new();
        let id = manager.add_device(Box::new(device)).await.unwrap();
        assert!(!id.is_empty());
    }
}
```

### 测试覆盖率

- 核心业务逻辑: **> 80%**
- 协议实现: **> 70%**
- CLI/GUI: **> 50%**

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定 crate 的测试
cargo test -p dglab-core

# 显示测试输出
cargo test -- --nocapture

# 运行单个测试
cargo test test_name
```

### 集成测试

集成测试放在 `tests/` 目录:

```rust
// tests/integration_test.rs
use dglab_core::SessionManager;

#[tokio::test]
async fn test_full_workflow() {
    let manager = SessionManager::new();
    // 完整流程测试...
}
```

---

## 提交规范

项目遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范:

### 提交消息格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

### 类型 (type)

| 类型 | 说明 | 示例 |
|------|------|------|
| `feat` | 新功能 | `feat(core): add waveform generator` |
| `fix` | Bug 修复 | `fix(ble): fix connection timeout issue` |
| `docs` | 文档更新 | `docs: update installation guide` |
| `style` | 代码格式 (不影响功能) | `style: run cargo fmt` |
| `refactor` | 重构 | `refactor(session): simplify device management` |
| `perf` | 性能优化 | `perf(protocol): optimize packet encoding` |
| `test` | 测试相关 | `test(core): add session manager tests` |
| `chore` | 构建/工具相关 | `chore: update dependencies` |
| `ci` | CI/CD 相关 | `ci: add GitHub Actions workflow` |

### 作用域 (scope)

可选,表示影响的模块:
- `protocol` - dglab-protocol
- `core` - dglab-core
- `cli` - dglab-cli
- `gui` - dglab-gui-tauri
- `ble` - BLE 相关
- `wifi` - WiFi 相关

### 示例

```bash
# 简单提交
git commit -m "feat(core): add preset storage"

# 详细提交
git commit -m "feat(protocol): implement V3 BLE protocol

- Add packet encoding/decoding
- Implement CRC-16 checksum
- Add device discovery
- Add connection management

Closes #42"

# 破坏性变更
git commit -m "feat(core)!: change Device trait API

BREAKING CHANGE: Device::connect() now requires &mut self"
```

---

## Pull Request 流程

### PR 标题格式

与提交消息格式相同:

```
feat(core): add waveform generator
```

### PR 描述模板

```markdown
## 变更类型
- [ ] 新功能 (feat)
- [ ] Bug 修复 (fix)
- [ ] 文档 (docs)
- [ ] 代码重构 (refactor)
- [ ] 性能优化 (perf)
- [ ] 测试 (test)
- [ ] 其他

## 变更描述
<!-- 描述你的变更内容 -->

## 相关 Issue
Closes #xxx

## 测试
- [ ] 已添加单元测试
- [ ] 已添加集成测试
- [ ] 所有测试通过
- [ ] 已运行 clippy 检查
- [ ] 已运行 cargo fmt

## 截图 (如适用)
<!-- 添加截图展示变更 -->

## Checklist
- [ ] 代码遵循项目代码规范
- [ ] 已更新相关文档
- [ ] 无破坏性变更,或已在提交消息中标注
- [ ] PR 标题遵循 Conventional Commits 规范
```

### Code Review 要求

所有 PR 必须:
1. **通过 CI 检查** (测试、clippy、格式化)
2. **至少 1 位维护者审核通过**
3. **无未解决的评论**

### 合并策略

- 功能分支 → `develop`: **Squash and Merge** (保持历史清晰)
- `develop` → `main`: **Merge Commit** (保留版本历史)
- Hotfix → `main`: **Merge Commit**

---

## 问题反馈

### 报告 Bug

使用 [Bug Report 模板](https://github.com/your-org/dglab-rs/issues/new?template=bug_report.md):

```markdown
**描述 Bug**
简要描述遇到的问题。

**复现步骤**
1. 执行 '...'
2. 点击 '...'
3. 看到错误

**期望行为**
描述你期望发生的行为。

**实际行为**
描述实际发生的行为。

**环境信息**
- OS: [e.g. Ubuntu 22.04]
- Rust 版本: [e.g. 1.75.0]
- 项目版本: [e.g. v0.1.0]

**日志**
```
粘贴相关日志
```

**截图**
如适用,添加截图帮助说明问题。
```

### 功能建议

使用 [Feature Request 模板](https://github.com/your-org/dglab-rs/issues/new?template=feature_request.md):

```markdown
**功能描述**
清晰简洁地描述你希望添加的功能。

**使用场景**
描述该功能的使用场景和解决的问题。

**期望行为**
描述你期望该功能如何工作。

**替代方案**
描述你考虑过的其他替代方案。

**其他信息**
添加任何其他相关信息或截图。
```

---

## 代码审查清单

审查 PR 时,请检查以下内容:

### 代码质量
- [ ] 代码逻辑正确,无明显 bug
- [ ] 错误处理恰当,无 `unwrap()`/`expect()` 滥用
- [ ] 性能合理,无明显性能问题
- [ ] 代码可读性好,命名清晰

### 测试
- [ ] 包含足够的单元测试
- [ ] 测试覆盖关键路径
- [ ] 所有测试通过

### 文档
- [ ] 公共 API 有文档注释
- [ ] 复杂逻辑有代码注释
- [ ] 相关文档已更新

### 规范
- [ ] 通过 `cargo fmt` 检查
- [ ] 通过 `cargo clippy` 检查
- [ ] 提交消息符合规范
- [ ] PR 描述完整

---

## 发布流程

> 仅维护者可执行

### 1. 创建 Release 分支

```bash
git checkout develop
git pull origin develop
git checkout -b release/v0.2.0
```

### 2. 更新版本号

```bash
# 更新 Cargo.toml 版本
# 更新 CHANGELOG.md
# 提交变更
git commit -am "chore: bump version to 0.2.0"
```

### 3. 合并到 main

```bash
git checkout main
git merge --no-ff release/v0.2.0
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin main --tags
```

### 4. 回合并到 develop

```bash
git checkout develop
git merge --no-ff release/v0.2.0
git push origin develop
```

### 5. 触发 Release Workflow

GitHub Actions 将自动构建所有平台的安装包并创建 GitHub Release。

---

## 社区准则

### 行为准则

- **尊重**: 尊重所有贡献者,保持友好和专业
- **包容**: 欢迎不同背景和经验水平的贡献者
- **建设性**: 提供建设性的反馈,帮助项目和社区成长
- **合作**: 开放合作,共同解决问题

### 沟通渠道

- **GitHub Issues**: Bug 报告和功能建议
- **GitHub Discussions**: 一般讨论和问答
- **Pull Requests**: 代码审查和技术讨论

---

## 许可证

通过贡献代码,您同意您的贡献将在与项目相同的许可证 (MIT OR Apache-2.0) 下发布。

---

## 联系方式

- **项目主页**: https://github.com/your-org/dglab-rs
- **问题跟踪**: https://github.com/your-org/dglab-rs/issues
- **讨论区**: https://github.com/your-org/dglab-rs/discussions

---

**感谢您的贡献!** 🎉

如果您在参与贡献过程中遇到任何问题,请随时在 GitHub Discussions 中提问。
