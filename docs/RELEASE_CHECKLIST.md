# GitHub Release v0.1.0 准备清单

本文档提供了创建 v0.1.0 版本 GitHub Release 的完整步骤。

---

## 📋 发布前检查清单

### ✅ 已完成

- [x] **所有代码已合并到 `main` 分支**
  - Phase 1: Core Libraries (100%)
  - Phase 2: Tauri + React GUI (100%)
  - Phase 3: Documentation (100%)

- [x] **所有测试通过**
  - 263 tests passing (144 core + 113 protocol + 6 doc-tests)
  - 0 compilation errors
  - 0 clippy warnings (except unused event types)
  - 0 TypeScript errors

- [x] **文档已完成**
  - [x] README.md
  - [x] docs/USER_GUIDE.md
  - [x] docs/INSTALLATION.md
  - [x] docs/ARCHITECTURE.md
  - [x] CONTRIBUTING.md
  - [x] CHANGELOG.md

- [x] **CI/CD 配置完成**
  - [x] `.github/workflows/ci.yml` - Continuous Integration
  - [x] `.github/workflows/release.yml` - Release automation

### ⏳ 待完成

- [ ] **版本号更新**
  - [ ] 更新 `Cargo.toml` 中的版本号
  - [ ] 更新 `apps/dglab-gui-tauri/package.json` 中的版本号
  - [ ] 更新 `apps/dglab-gui-tauri/src-tauri/Cargo.toml` 中的版本号
  - [ ] 更新 `apps/dglab-gui-tauri/src-tauri/tauri.conf.json` 中的版本号

- [ ] **Git 标签创建**
  - [ ] 创建并推送 `v0.1.0` 标签

- [ ] **构建和测试**
  - [ ] 本地测试构建 (至少一个平台)
  - [ ] 验证 CI 工作流通过
  - [ ] 验证 Release 工作流触发

- [ ] **发布审查**
  - [ ] 检查所有构建产物
  - [ ] 测试安装包
  - [ ] 编写发布公告

---

## 📝 详细步骤

### 步骤 1: 更新版本号

#### 1.1 更新根 `Cargo.toml`

```bash
# 编辑 Cargo.toml
# 将 workspace.package.version 设置为 "0.1.0"
```

```toml
[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["DG-LAB Contributors"]
license = "MIT OR Apache-2.0"
```

#### 1.2 更新 GUI `package.json`

```bash
cd apps/dglab-gui-tauri
# 编辑 package.json
# 将 "version" 设置为 "0.1.0"
```

```json
{
  "name": "dglab-gui-tauri",
  "version": "0.1.0",
  "description": "DG-LAB Device Controller - Desktop GUI",
  ...
}
```

#### 1.3 更新 Tauri `tauri.conf.json`

```bash
# 编辑 apps/dglab-gui-tauri/src-tauri/tauri.conf.json
# 将 package.version 设置为 "0.1.0"
```

```json
{
  "package": {
    "productName": "DG-LAB Controller",
    "version": "0.1.0"
  },
  ...
}
```

#### 1.4 验证版本更新

```bash
# 从项目根目录运行
cargo check
cd apps/dglab-gui-tauri && npm run tauri build --help
```

### 步骤 2: 提交版本更新

```bash
# 回到项目根目录
cd /home/zizimiku/DG_LAB

# 添加所有版本更新
git add Cargo.toml apps/dglab-gui-tauri/package.json apps/dglab-gui-tauri/src-tauri/Cargo.toml apps/dglab-gui-tauri/src-tauri/tauri.conf.json

# 提交版本更新
git commit -m "chore: bump version to 0.1.0"

# 推送到远程
git push origin main
```

### 步骤 3: 创建 Git 标签

```bash
# 创建带注释的标签
git tag -a v0.1.0 -m "Release v0.1.0 - Initial public release

Major features:
- Cross-platform desktop GUI (Tauri + React)
- CLI tool with TUI support
- Full DG-LAB V3 BLE protocol implementation
- WiFi WebSocket protocol support
- Waveform generator (4 types)
- Preset management system
- Real-time device state updates

See CHANGELOG.md for full details."

# 查看标签
git tag -l -n9 v0.1.0

# 推送标签到远程 (这将触发 Release workflow)
git push origin v0.1.0
```

### 步骤 4: 监控 GitHub Actions

1. **访问 GitHub Actions 页面**:
   ```
   https://github.com/your-org/dglab-rs/actions
   ```

2. **检查 CI 工作流**:
   - 确保所有测试通过
   - 确保 clippy 检查通过
   - 确保格式检查通过

3. **检查 Release 工作流**:
   - 监控构建进度
   - 检查所有平台的构建状态
   - 确认没有错误

### 步骤 5: 验证构建产物

Release 工作流完成后,检查以下构建产物:

#### GUI 安装包

**Linux**:
- [ ] `DG-LAB-Controller_0.1.0_amd64.AppImage` (~50-70 MB)
- [ ] `dglab-gui-tauri_0.1.0_amd64.deb` (~40-60 MB)

**macOS**:
- [ ] `DG-LAB-Controller_0.1.0_universal.dmg` (~30-50 MB)

**Windows**:
- [ ] `DG-LAB-Controller_0.1.0_x64_en-US.msi` (~20-30 MB)
- [ ] `DG-LAB-Controller_0.1.0_x64-setup.exe` (NSIS installer, ~20-30 MB)

#### CLI 二进制

- [ ] `dglab-cli-linux-x64.tar.gz` (~5-10 MB)
- [ ] `dglab-cli-macos-universal.tar.gz` (~5-10 MB)
- [ ] `dglab-cli-windows-x64.zip` (~5-10 MB)

### 步骤 6: 下载并测试安装包

#### Linux 测试

```bash
# 下载 AppImage
wget https://github.com/your-org/dglab-rs/releases/download/v0.1.0/DG-LAB-Controller_0.1.0_amd64.AppImage

# 添加执行权限
chmod +x DG-LAB-Controller_0.1.0_amd64.AppImage

# 运行
./DG-LAB-Controller_0.1.0_amd64.AppImage

# 测试 DEB 包
wget https://github.com/your-org/dglab-rs/releases/download/v0.1.0/dglab-gui-tauri_0.1.0_amd64.deb
sudo dpkg -i dglab-gui-tauri_0.1.0_amd64.deb
dglab-gui-tauri
```

#### macOS 测试

```bash
# 下载 DMG
curl -LO https://github.com/your-org/dglab-rs/releases/download/v0.1.0/DG-LAB-Controller_0.1.0_universal.dmg

# 挂载并安装
open DG-LAB-Controller_0.1.0_universal.dmg
# 拖拽到 Applications 文件夹
```

#### Windows 测试

1. 下载 MSI 或 EXE 安装包
2. 双击运行安装程序
3. 按照向导完成安装
4. 从开始菜单启动应用

#### CLI 测试

```bash
# Linux
tar xzf dglab-cli-linux-x64.tar.gz
./dglab --version
./dglab --help

# macOS
tar xzf dglab-cli-macos-universal.tar.gz
./dglab --version
./dglab --help

# Windows
unzip dglab-cli-windows-x64.zip
dglab.exe --version
dglab.exe --help
```

### 步骤 7: 编辑 GitHub Release

1. **访问 Release 页面**:
   ```
   https://github.com/your-org/dglab-rs/releases/tag/v0.1.0
   ```

2. **点击 "Edit release"**

3. **完善 Release Notes**:
   - 添加安装说明链接
   - 添加用户指南链接
   - 添加已知问题说明
   - 添加平台支持矩阵

4. **添加安装说明**:

```markdown
## 🎉 DG-LAB Controller v0.1.0 - 初始发布

这是 DG-LAB Rust 控制器的首个公开版本!

### 📦 下载

#### 桌面 GUI

| 平台 | 文件 | 大小 |
|------|------|------|
| 🐧 Linux (AppImage) | [DG-LAB-Controller_0.1.0_amd64.AppImage](#) | ~60 MB |
| 🐧 Linux (DEB) | [dglab-gui-tauri_0.1.0_amd64.deb](#) | ~50 MB |
| 🍎 macOS (Universal) | [DG-LAB-Controller_0.1.0_universal.dmg](#) | ~40 MB |
| 🪟 Windows (MSI) | [DG-LAB-Controller_0.1.0_x64_en-US.msi](#) | ~25 MB |
| 🪟 Windows (EXE) | [DG-LAB-Controller_0.1.0_x64-setup.exe](#) | ~25 MB |

#### CLI 工具

| 平台 | 文件 | 大小 |
|------|------|------|
| 🐧 Linux | [dglab-cli-linux-x64.tar.gz](#) | ~8 MB |
| 🍎 macOS | [dglab-cli-macos-universal.tar.gz](#) | ~8 MB |
| 🪟 Windows | [dglab-cli-windows-x64.zip](#) | ~8 MB |

### 📖 文档

- [安装指南](https://github.com/your-org/dglab-rs/blob/main/docs/INSTALLATION.md)
- [用户手册](https://github.com/your-org/dglab-rs/blob/main/docs/USER_GUIDE.md)
- [架构文档](https://github.com/your-org/dglab-rs/blob/main/docs/ARCHITECTURE.md)
- [贡献指南](https://github.com/your-org/dglab-rs/blob/main/CONTRIBUTING.md)

### ✨ 主要特性

- ✅ 跨平台桌面 GUI (Tauri + React)
- ✅ 命令行工具 (CLI + TUI)
- ✅ DG-LAB V3 BLE 协议支持
- ✅ WiFi WebSocket 协议支持
- ✅ 波形生成器 (4 种波形类型)
- ✅ 预设管理系统
- ✅ 实时设备状态更新

详见 [CHANGELOG.md](https://github.com/your-org/dglab-rs/blob/main/CHANGELOG.md)

### ⚠️ 已知限制

1. **BLE 连接**: Linux 用户可能需要配置 BlueZ 权限
2. **WiFi 功能**: WiFi 协议已在 GUI 中完全支持 (v0.1.2+)
3. **平台支持**: Android 版本正在开发中 (Phase 4)

### 🐛 问题反馈

如遇到问题,请在 [Issues](https://github.com/your-org/dglab-rs/issues) 页面报告。

### 🙏 致谢

感谢所有参与测试和反馈的社区成员!

---

**完整变更记录**: https://github.com/your-org/dglab-rs/blob/main/CHANGELOG.md
```

5. **保存 Release**

### 步骤 8: 发布公告

#### 在 GitHub Discussions 发布

1. 访问 Discussions 页面
2. 创建新的 "Announcements" 主题
3. 标题: "🎉 DG-LAB Controller v0.1.0 Released!"
4. 内容: 包含主要特性、下载链接和使用指南

#### 社交媒体 (可选)

- 在相关社区发布公告
- 分享项目链接和主要特性

---

## 🔧 故障排除

### Release Workflow 失败

#### 问题 1: Tauri 构建失败

**症状**: "Failed to build Tauri app"

**解决方案**:
1. 检查 `tauri.conf.json` 配置
2. 确保所有依赖已正确安装
3. 本地测试构建: `npm run tauri build`
4. 查看 GitHub Actions 日志获取详细错误

#### 问题 2: 上传产物失败

**症状**: "Failed to upload release asset"

**解决方案**:
1. 检查 `GITHUB_TOKEN` 权限
2. 确认 Release 已创建
3. 验证文件路径正确
4. 手动上传失败的产物

#### 问题 3: macOS 签名问题

**症状**: "Code signing failed"

**解决方案**:
1. 配置 `TAURI_PRIVATE_KEY` secret
2. 配置 `TAURI_KEY_PASSWORD` secret
3. 或暂时禁用签名 (仅用于测试)

### 本地构建问题

#### Linux

```bash
# 如果缺少依赖
sudo apt-get install -y libudev-dev libdbus-1-dev libwebkit2gtk-4.1-dev

# 如果 Rust 版本过旧
rustup update stable
```

#### macOS

```bash
# 如果缺少 Xcode Command Line Tools
xcode-select --install

# 如果需要添加 target
rustup target add aarch64-apple-darwin x86_64-apple-darwin
```

#### Windows

- 确保安装了 Visual Studio Build Tools
- 确保安装了 WebView2 Runtime

---

## ✅ 最终检查清单

发布前最后确认:

- [ ] 所有构建产物已上传
- [ ] 所有构建产物已测试
- [ ] Release Notes 已完善
- [ ] 文档链接可用
- [ ] CHANGELOG.md 正确
- [ ] 版本号一致
- [ ] 标签已推送
- [ ] GitHub Release 已发布
- [ ] 发布公告已撰写

---

## 📞 联系方式

如有问题,请联系:
- GitHub Issues: https://github.com/your-org/dglab-rs/issues
- GitHub Discussions: https://github.com/your-org/dglab-rs/discussions

---

**祝发布顺利!** 🚀
