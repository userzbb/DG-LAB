# 发布指南

## 自动发布流程

本项目使用 GitHub Actions 自动构建和发布。每次推送 tag 时会自动触发构建。

### 创建新版本

1. **更新版本号**（如果需要）
   ```bash
   # 编辑 Cargo.toml 中的版本号
   vim Cargo.toml
   ```

2. **提交代码**
   ```bash
   git add .
   git commit -m "feat: 新功能描述"
   git push origin main
   ```

3. **创建并推送 tag**
   ```bash
   # 创建 tag
   git tag -a v0.1.5 -m "Release v0.1.5: 功能描述"
   
   # 推送 tag（这会触发自动构建）
   git push origin v0.1.5
   ```

4. **等待构建完成**
   
   查看构建进度：
   ```bash
   gh run list --limit 5
   gh run view <run-id>
   gh run watch <run-id>
   ```
   
   或访问 GitHub Actions 页面：
   https://github.com/userzbb/DG-LAB/actions

5. **检查发布**
   
   构建完成后，访问 Release 页面：
   https://github.com/userzbb/DG-LAB/releases
   
   应该能看到以下文件：
   - `dglab-cli-linux-x64.tar.gz` - Linux CLI 二进制
   - `dglab-cli-macos-universal.tar.gz` - macOS CLI 二进制（Universal）
   - `dglab-cli-windows-x64.zip` - Windows CLI 二进制
   - `dglab-gui-tauri-*.dmg` - macOS GUI 安装包
   - `dglab-gui-tauri-*.msi` - Windows GUI MSI 安装包
   - `dglab-gui-tauri-*.exe` - Windows GUI NSIS 安装包
   - `dglab-gui-tauri-linux-x64-bin.tar.gz` - Linux GUI 二进制

## 手动发布（不推荐）

如果需要手动创建 release：

```bash
# 创建 tag
git tag -a v0.1.5 -m "Release v0.1.5"
git push origin v0.1.5

# 使用 gh CLI 创建 release
gh release create v0.1.5 \
  --title "v0.1.5: 功能描述" \
  --notes "详细的 release notes"
```

## Release Notes 模板

创建 tag 时建议使用以下格式：

```
Release v0.1.5: 简短描述

新功能:
- 功能 1
- 功能 2

修复:
- 问题 1
- 问题 2

改进:
- 改进 1
- 改进 2

使用方法:
1. 步骤 1
2. 步骤 2

详见 README.md
```

## 自动构建说明

### 触发条件

- 推送符合 `v*.*.*` 格式的 tag（例如 v0.1.0, v1.2.3）
- 手动触发 workflow（在 GitHub Actions 页面）

### 构建产物

**CLI 二进制**:
- Linux x64: `dglab-cli-linux-x64.tar.gz`
- macOS Universal: `dglab-cli-macos-universal.tar.gz` (支持 Intel 和 Apple Silicon)
- Windows x64: `dglab-cli-windows-x64.zip`

**GUI 安装包** (Tauri):
- Linux: `dglab-gui-tauri-linux-x64-bin.tar.gz` (原始二进制，供 Arch 等滚动发行版)
- macOS: `.dmg` 文件 (Universal Binary)
- Windows: `.msi` 和 `.exe` 安装包

### 构建环境

- **Linux**: Ubuntu 22.04
- **macOS**: macOS Latest (支持 Universal Binary)
- **Windows**: Windows Latest

### 依赖

构建会自动安装所需依赖：
- Rust toolchain (stable)
- Node.js 18
- 平台特定的系统库（BLE、DBus、WebKit 等）

## 测试 Release

### 下载并测试 CLI

**Linux/macOS**:
```bash
# 下载
wget https://github.com/userzbb/DG-LAB/releases/download/v0.1.5/dglab-cli-linux-x64.tar.gz

# 解压
tar xzf dglab-cli-linux-x64.tar.gz

# 运行
./dglab --version
./dglab scan
./dglab bridge --device 47L121000
```

**Windows**:
```powershell
# 下载并解压
Invoke-WebRequest -Uri "https://github.com/userzbb/DG-LAB/releases/download/v0.1.5/dglab-cli-windows-x64.zip" -OutFile "dglab-cli.zip"
Expand-Archive dglab-cli.zip

# 运行
.\dglab-cli\dglab.exe --version
.\dglab-cli\dglab.exe bridge --device 47L121000
```

### 测试桥接功能

1. 下载对应平台的 CLI 二进制
2. 运行桥接程序：
   ```bash
   ./dglab bridge --device 47L121000
   ```
3. 记录 Client ID
4. 运行测试脚本：
   ```bash
   uv run test-bridge.py
   ```
5. 输入 Client ID 并测试控制命令

## 版本号约定

遵循语义化版本 (Semantic Versioning):

- **主版本号** (MAJOR): 不兼容的 API 变更
- **次版本号** (MINOR): 向后兼容的功能新增
- **修订号** (PATCH): 向后兼容的问题修正

示例：
- `v0.1.0` - 初始版本
- `v0.1.1` - Bug 修复
- `v0.2.0` - 新增功能
- `v1.0.0` - 正式版本

### 预发布版本

使用后缀标识预发布版本：
- `v0.2.0-alpha.1` - Alpha 测试版
- `v0.2.0-beta.1` - Beta 测试版
- `v0.2.0-rc.1` - Release Candidate

GitHub Actions 会自动将带有这些后缀的版本标记为 "Pre-release"。

## 故障排除

### 构建失败

1. 查看失败的 job 日志：
   ```bash
   gh run view <run-id> --log-failed
   ```

2. 常见问题：
   - **依赖安装失败**: 检查系统库是否可用
   - **编译错误**: 本地先测试 `cargo build --release`
   - **权限错误**: 检查 GITHUB_TOKEN 权限

### Release 已存在

如果 tag 已存在：
```bash
# 删除本地 tag
git tag -d v0.1.5

# 删除远程 tag
git push origin :refs/tags/v0.1.5

# 重新创建
git tag -a v0.1.5 -m "新的描述"
git push origin v0.1.5
```

### 手动重新运行构建

在 GitHub Actions 页面点击 "Re-run all jobs"，或使用命令：
```bash
gh run rerun <run-id>
```

## 相关链接

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [Tauri 构建文档](https://tauri.app/v1/guides/building/)
- [语义化版本规范](https://semver.org/lang/zh-CN/)
