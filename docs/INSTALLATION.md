# DG-LAB 安装指南

本文档提供 DG-LAB 控制器在各平台的详细安装说明。

## 🚀 快速开始

### 当前可用版本 (v0.1.4)

| 平台 | CLI 工具 | GUI 应用 | 状态 |
|------|---------|---------|------|
| **Linux** | ✅ 可用 | ✅ 可用 | 完整支持 |
| **Windows** | ✅ 可用 | ⏳ 准备中 | CLI 可用，推荐使用桥接模式 |
| **macOS** | ✅ 可用 | ⏳ 准备中 | CLI 可用，推荐使用桥接模式 |

**推荐使用**：
- **Linux 用户**：下载 [GUI 应用](#linux) 获得完整图形界面体验
- **Windows/macOS 用户**：下载 [CLI 工具](#cli-工具推荐用于桥接功能) 使用桥接模式连接设备
- **所有用户**：v0.1.5 即将发布，将包含所有平台的完整 GUI 支持

### 桥接模式是什么？

桥接模式允许你的电脑替代官方 DG-LAB APP，通过蓝牙连接设备并同时连接 WebSocket 服务器，让第三方控制器（如 Coyote Remote）能够远程控制你的 DG-LAB 设备。

**架构**：
```
第三方控制端 → Socket 服务器 ← 本程序 ← BLE ← DG-LAB 设备
```

---

## 目录

- [快速开始](#快速开始)
- [系统要求](#系统要求)
- [预编译安装包 (推荐)](#预编译安装包-推荐)
  - [Windows](#windows)
  - [macOS](#macos)
  - [Linux](#linux)
- [从源码构建](#从源码构建)
  - [Windows 从源码构建](#windows-从源码构建)
  - [macOS 从源码构建](#macos-从源码构建)
  - [Linux 从源码构建](#linux-从源码构建)
- [安装 CLI 工具](#安装-cli-工具)
- [验证安装](#验证安装)
- [故障排除](#故障排除)

---

## 系统要求

### 最低配置

| 组件 | 要求 |
|------|------|
| 操作系统 | Windows 10+ / macOS 10.15+ / Linux (kernel 5.4+) |
| CPU | x64 双核 1.5GHz+ |
| 内存 | 2 GB RAM |
| 磁盘空间 | 500 MB 可用空间 |
| 蓝牙 | 支持 BLE 4.0+ 的蓝牙适配器 |
| 显示器 | 1280x720 分辨率+ |

### 推荐配置

| 组件 | 要求 |
|------|------|
| 操作系统 | Windows 11 / macOS 13+ / Ubuntu 22.04+ |
| CPU | x64 四核 2.0GHz+ |
| 内存 | 4 GB RAM+ |
| 蓝牙 | 支持 BLE 5.0+ |

### 软件依赖

#### 所有平台
- **Rust**: 1.70+ (仅从源码构建时需要)
- **Node.js**: 18+ (仅从源码构建 GUI 时需要)

#### Linux 特定
- GTK 3.24+
- WebKit2GTK 4.1+
- D-Bus
- BlueZ (蓝牙支持)

---

## 预编译安装包 (推荐)

### Windows

#### GUI 应用

> **注意**：Windows GUI 预编译版本正在准备中，当前版本 (v0.1.4) 仅提供 Linux 版本。请使用以下方式：
> 1. **推荐**：使用 CLI 工具的桥接功能（见下方 CLI 工具安装）
> 2. 等待 v0.1.5 发布（将包含所有平台的 GUI 版本）
> 3. [从源码构建](#windows-从源码构建) GUI 应用

#### CLI 工具（推荐用于桥接功能）

**CLI 工具提供完整的桥接模式支持**，可以替代官方 APP 连接设备。

1. **下载预编译版本**
   ```powershell
   # 下载地址
   https://github.com/userzbb/DG-LAB/releases/download/v0.1.4/dglab-cli-windows-x64.zip
   ```

2. **解压并使用**
   ```powershell
   # 解压到任意目录
   Expand-Archive dglab-cli-windows-x64.zip -DestinationPath .\dglab-cli
   
   # 进入目录
   cd dglab-cli
   
   # 查看版本
   .\dglab.exe --version
   
   # 扫描附近设备
   .\dglab.exe scan
   
   # 启动桥接模式（替代官方 APP）
   .\dglab.exe bridge --device 47L121000
   ```

3. **桥接模式使用**

   启动桥接后，程序会：
   - 通过蓝牙连接到 DG-LAB 设备
   - 连接到 WebSocket 服务器 (`wss://dg-lab-socket.nanami.tech/ws`)
   - 注册设备名称为 `coyote-3-bridge`（或你指定的名称）
   
   然后你可以使用任何支持 DG-LAB Socket 协议的第三方控制器（如 Coyote Remote）连接到 `coyote-3-bridge` 进行控制。

   ```powershell
   # 使用自定义设备名称
   .\dglab.exe bridge --device 47L121000 --name "my-device"
   
   # 使用自定义 WebSocket 服务器
   .\dglab.exe bridge --device 47L121000 --ws-url "wss://your-server.com/ws"
   
   # 查看更多选项
   .\dglab.exe bridge --help
   ```

4. **添加到 PATH（可选）**
   ```powershell
   # 复制到用户 bin 目录
   $binDir = "$env:USERPROFILE\bin"
   New-Item -ItemType Directory -Force -Path $binDir
   Copy-Item dglab.exe $binDir\
   
   # 添加到 PATH
   [Environment]::SetEnvironmentVariable(
       "Path",
       [Environment]::GetEnvironmentVariable("Path", "User") + ";$binDir",
       "User"
   )
   
   # 重启终端后验证
   dglab --version
   ```



### macOS

#### GUI 应用

> **注意**：macOS GUI 预编译版本正在准备中，当前版本 (v0.1.4) 仅提供 Linux 版本。请使用以下方式：
> 1. **推荐**：使用 CLI 工具的桥接功能（见下方 CLI 工具安装）
> 2. 等待 v0.1.5 发布（将包含所有平台的 GUI 版本）
> 3. [从源码构建](#macos-从源码构建) GUI 应用

#### CLI 工具

1. **下载预编译版本**
   ```bash
   # 下载最新版本
   curl -LO https://github.com/userzbb/DG-LAB/releases/latest/download/dglab-cli-macos-universal.tar.gz
   ```

2. **解压并安装**
   ```bash
   # 解压
   tar xzf dglab-cli-macos-universal.tar.gz
   
   # 安装到系统（需要管理员权限）
   sudo install -m 755 dglab /usr/local/bin/
   
   # 或安装到用户目录（无需管理员权限）
   mkdir -p ~/bin
   install -m 755 dglab ~/bin/
   # 确保 ~/bin 在 PATH 中
   echo 'export PATH="$HOME/bin:$PATH"' >> ~/.zshrc
   source ~/.zshrc
   ```

3. **验证安装**
   ```bash
   dglab --version
   dglab scan
   dglab bridge --device 47L121000
   ```

4. **首次运行权限**
   ```bash
   # 如果提示无法验证开发者
   sudo xattr -rd com.apple.quarantine /usr/local/bin/dglab
   # 或
   sudo xattr -rd com.apple.quarantine ~/bin/dglab
   ```

### Linux

#### 预编译二进制 (推荐，快速安装)

**适用于所有发行版**：

```bash
# 下载最新版本的 CLI 工具
wget https://github.com/userzbb/DG-LAB/releases/latest/download/dglab-cli-linux-x64.tar.gz

# 解压
tar xzf dglab-cli-linux-x64.tar.gz

# 安装到系统
sudo install -m 755 dglab /usr/local/bin/

# 验证安装
dglab --version

# 使用示例
dglab scan                          # 扫描设备
dglab bridge --device 47L121000     # 桥接模式
```

**GUI 应用**：

1. **下载预编译版本**
   ```bash
   # 下载最新版本
   wget https://github.com/userzbb/DG-LAB/releases/download/v0.1.4/dglab-gui-tauri-linux-x64-bin.tar.gz
   ```

2. **解压文件**
   ```bash
   # 解压到当前目录
   tar xzf dglab-gui-tauri-linux-x64-bin.tar.gz
   
   # 进入解压后的目录
   cd dglab-gui-tauri-linux-x64-bin
   ```

3. **添加执行权限并运行**
   ```bash
   # 添加执行权限
   chmod +x dglab-gui-tauri
   
   # 运行应用
   ./dglab-gui-tauri
   ```

4. **安装到系统（可选）**
   ```bash
   # 复制到系统 bin 目录
   sudo install -m 755 dglab-gui-tauri /usr/local/bin/
   
   # 以后可以直接运行
   dglab-gui-tauri
   ```

5. **创建桌面快捷方式（可选）**
   ```bash
   # 创建 .desktop 文件
   cat > ~/.local/share/applications/dglab-gui.desktop << 'EOF'
   [Desktop Entry]
   Name=DG-LAB Controller
   Comment=DG-LAB 设备控制器
   Exec=/usr/local/bin/dglab-gui-tauri
   Icon=application-default-icon
   Terminal=false
   Type=Application
   Categories=Utility;
   EOF
   
   # 更新桌面数据库
   update-desktop-database ~/.local/share/applications/
   ```

**系统依赖**：

GUI 应用需要以下运行时依赖，请根据你的发行版安装：

**Debian/Ubuntu**:
```bash
sudo apt install -y \
    libwebkit2gtk-4.1-0 \
    libayatana-appindicator3-1 \
    libdbus-1-3 \
    bluez
```

**Arch Linux**:
```bash
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator bluez bluez-utils
```

**Fedora**:
```bash
sudo dnf install webkit2gtk4.1 libappindicator-gtk3 bluez
```

**蓝牙权限**：

如果无法扫描 BLE 设备，需要配置蓝牙权限：

```bash
# 方法 1: 将用户添加到 bluetooth 组（推荐）
sudo usermod -aG bluetooth $USER
# 注销并重新登录生效

# 方法 2: 使用 sudo 运行
sudo dglab-gui-tauri

# 方法 3: 设置 capabilities
sudo setcap 'cap_net_raw,cap_net_admin+eip' /usr/local/bin/dglab-gui-tauri
```

**已知问题**：

当前版本 (v0.1.3) 的 BLE 连接可能显示"未知错误"，这个问题在最新代码中已修复，将在 v0.1.5 发布。临时解决方案：使用 CLI 工具的桥接模式。

#### 其他 Linux 发行版的包管理器安装

> **注意**：以下安装方式（.deb、.rpm、AppImage、Flatpak）正在准备中。
> 当前请使用上方的 **预编译二进制** 或 **从源码构建** 方式安装。

---

## 从源码构建

### 准备工作

#### 安装 Rust

```bash
# Windows / macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 重新加载环境变量
source $HOME/.cargo/env

# 验证安装
rustc --version
cargo --version
```

#### 安装 Node.js

**Windows / macOS**:
- 下载安装器: https://nodejs.org/
- 选择 LTS 版本

**Linux**:
```bash
# Ubuntu / Debian
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs

# Fedora
sudo dnf install nodejs

# Arch
sudo pacman -S nodejs npm
```

### Windows 从源码构建

1. **安装系统依赖**
   - Visual Studio 2019+ 或 Build Tools for Visual Studio
   - 确保安装了 "使用 C++ 的桌面开发" 工作负载

2. **克隆仓库**
   ```powershell
   git clone https://github.com/your-username/DG_LAB.git
   cd DG_LAB
   ```

3. **构建 Rust 后端**
   ```powershell
   cargo build --release
   ```

4. **构建 Tauri GUI**
   ```powershell
   cd apps/dglab-gui-tauri
   npm install
   npm run tauri build
   ```

5. **安装包位置**
   - 安装程序: `apps/dglab-gui-tauri/src-tauri/target/release/bundle/msi/DG-LAB_*_x64_en-US.msi`
   - 可执行文件: `apps/dglab-gui-tauri/src-tauri/target/release/dglab-gui-tauri.exe`

### macOS 从源码构建

1. **安装 Xcode Command Line Tools**
   ```bash
   xcode-select --install
   ```

2. **克隆仓库**
   ```bash
   git clone https://github.com/your-username/DG_LAB.git
   cd DG_LAB
   ```

3. **构建 Rust 后端**
   ```bash
   cargo build --release
   ```

4. **构建 Tauri GUI**
   ```bash
   cd apps/dglab-gui-tauri
   npm install
   npm run tauri build
   ```

5. **安装包位置**
   - DMG: `apps/dglab-gui-tauri/src-tauri/target/release/bundle/dmg/DG-LAB_*.dmg`
   - App bundle: `apps/dglab-gui-tauri/src-tauri/target/release/bundle/macos/DG-LAB.app`

6. **安装 App**
   ```bash
   # 复制到 Applications
   cp -r apps/dglab-gui-tauri/src-tauri/target/release/bundle/macos/DG-LAB.app /Applications/
   ```

### Linux 从源码构建

1. **安装系统依赖**

   **Ubuntu / Debian**:
   ```bash
   sudo apt update
   sudo apt install -y \
     build-essential \
     curl \
     wget \
     file \
     libssl-dev \
     libgtk-3-dev \
     libwebkit2gtk-4.1-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev \
     libdbus-1-dev \
     libbluetooth-dev \
     pkg-config
   ```

   **Fedora**:
   ```bash
   sudo dnf groupinstall "C Development Tools and Libraries"
   sudo dnf install \
     webkit2gtk4.1-devel \
     openssl-devel \
     gtk3-devel \
     libappindicator-gtk3-devel \
     librsvg2-devel \
     dbus-devel \
     bluez-libs-devel
   ```

   **Arch**:
   ```bash
   sudo pacman -S --needed \
     base-devel \
     curl \
     wget \
     file \
     openssl \
     gtk3 \
     webkit2gtk-4.1 \
     libappindicator-gtk3 \
     librsvg \
     dbus \
     bluez-libs
   ```

2. **克隆仓库**
   ```bash
   git clone https://github.com/your-username/DG_LAB.git
   cd DG_LAB
   ```

3. **构建 Rust 后端**
   ```bash
   cargo build --release
   ```

4. **构建 Tauri GUI**
   ```bash
   cd apps/dglab-gui-tauri
   npm install
   npm run tauri build
   ```

5. **安装包位置**
   - AppImage: `apps/dglab-gui-tauri/src-tauri/target/release/bundle/appimage/dg-lab_*_amd64.AppImage`
   - .deb: `apps/dglab-gui-tauri/src-tauri/target/release/bundle/deb/dglab_*_amd64.deb`
   - .rpm: `apps/dglab-gui-tauri/src-tauri/target/release/bundle/rpm/dglab-*.x86_64.rpm`

6. **系统安装**

   **Debian-based**:
   ```bash
   sudo dpkg -i apps/dglab-gui-tauri/src-tauri/target/release/bundle/deb/dglab_*_amd64.deb
   ```

   **RPM-based**:
   ```bash
   sudo rpm -i apps/dglab-gui-tauri/src-tauri/target/release/bundle/rpm/dglab-*.x86_64.rpm
   ```

   **AppImage**:
   ```bash
   chmod +x apps/dglab-gui-tauri/src-tauri/target/release/bundle/appimage/dg-lab_*_amd64.AppImage
   ./apps/dglab-gui-tauri/src-tauri/target/release/bundle/appimage/dg-lab_*_amd64.AppImage
   ```

---

## 安装 CLI 工具

### 使用 Cargo 安装 (推荐)

```bash
# 从源码安装
cd DG_LAB
cargo install --path crates/dglab-cli

# 验证安装
dglab --version
```

### 手动安装

#### Windows

```powershell
# 构建
cargo build --release -p dglab-cli

# 复制到 PATH
Copy-Item target\release\dglab.exe C:\Windows\System32\

# 或添加到用户目录
Copy-Item target\release\dglab.exe $env:USERPROFILE\bin\
# 将 %USERPROFILE%\bin 添加到 PATH 环境变量
```

#### macOS / Linux

```bash
# 构建
cargo build --release -p dglab-cli

# 复制到 /usr/local/bin
sudo cp target/release/dglab /usr/local/bin/

# 添加执行权限
sudo chmod +x /usr/local/bin/dglab

# 验证
dglab --version
```

---

## 验证安装

### 验证 GUI

1. 启动应用程序
2. 应该看到仪表盘界面
3. 检查以下功能：
   - [ ] 导航栏正常显示
   - [ ] 主题切换工作
   - [ ] 点击各菜单项可以跳转

### 验证 CLI

```bash
# 查看版本
dglab --version

# 查看帮助
dglab --help

# 测试扫描功能
dglab scan --timeout 5
```

### 验证蓝牙功能

#### Windows
```powershell
# 检查蓝牙服务
Get-Service bthserv

# 应该显示 Status : Running
```

#### macOS
```bash
# 检查蓝牙状态
system_profiler SPBluetoothDataType

# 或使用 GUI: 系统偏好设置 → 蓝牙
```

#### Linux
```bash
# 检查 BlueZ 服务
sudo systemctl status bluetooth

# 测试蓝牙适配器
hcitool dev

# 扫描设备
bluetoothctl scan on
```

---

## 故障排除

### GUI 无法启动

#### Windows

**问题**: 双击没有反应或闪退

**解决方案**:
```powershell
# 1. 安装 Visual C++ Redistributable
# 下载: https://aka.ms/vs/17/release/vc_redist.x64.exe

# 2. 以管理员身份运行
# 右键 → 以管理员身份运行

# 3. 检查 Windows Defender
# 设置 → 更新和安全 → Windows 安全中心 → 病毒和威胁防护 → 允许的威胁
```

#### macOS

**问题**: 提示"DG-LAB.app 已损坏"

**解决方案**:
```bash
# 移除隔离属性
sudo xattr -rd com.apple.quarantine /Applications/DG-LAB.app

# 允许未签名应用
sudo spctl --master-disable
```

#### Linux

**问题**: 缺少共享库

**解决方案**:
```bash
# 检查依赖
ldd /usr/bin/dglab-gui

# Ubuntu/Debian: 安装缺失的库
sudo apt install --fix-broken

# 运行时查看错误
dglab-gui 2>&1 | tee error.log
```

### 蓝牙无法使用

#### Windows

```powershell
# 重启蓝牙服务
Restart-Service bthserv

# 检查设备管理器
# Win + X → 设备管理器 → 蓝牙
```

#### macOS

```bash
# 重置蓝牙模块
sudo killall bluetoothd
sudo launchctl start com.apple.bluetoothd

# 或使用 GUI: Option + 点击蓝牙图标 → 重置蓝牙模块
```

#### Linux

```bash
# 重启蓝牙服务
sudo systemctl restart bluetooth

# 检查用户组
groups | grep bluetooth

# 如果没有，添加用户到 bluetooth 组
sudo usermod -a -G bluetooth $USER
# 注销后重新登录

# 检查蓝牙适配器
sudo rfkill list
# 如果被阻止，解除阻止
sudo rfkill unblock bluetooth
```

### 权限问题

#### Linux: 蓝牙权限不足

```bash
# 方法 1: 添加用户到 bluetooth 组
sudo usermod -a -G bluetooth $USER

# 方法 2: 添加 udev 规则
sudo tee /etc/udev/rules.d/99-bluetooth.rules <<EOF
SUBSYSTEM=="bluetooth", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="8087", MODE="0666"
EOF

sudo udevadm control --reload-rules
sudo udevadm trigger
```

#### macOS: 权限提示

```bash
# 重新授权
tccutil reset Bluetooth com.dglab.app

# 手动授权: 系统偏好设置 → 安全性与隐私 → 隐私 → 蓝牙
```

### 构建失败

#### Rust 版本过低

```bash
# 更新 Rust
rustup update stable
rustc --version
# 应该 >= 1.70
```

#### Node.js 版本过低

```bash
# 检查版本
node --version
npm --version

# 更新 Node.js
# 使用 nvm (推荐)
nvm install --lts
nvm use --lts
```

#### 依赖安装失败

```bash
# 清除缓存重试
# Cargo
cargo clean
rm -rf ~/.cargo/registry

# npm
cd apps/dglab-gui-tauri
rm -rf node_modules package-lock.json
npm cache clean --force
npm install
```

### 连接问题

参见 [用户指南 - 常见问题](USER_GUIDE.md#常见问题)

---

## 卸载

### Windows

1. **使用安装程序卸载**
   - 控制面板 → 程序和功能
   - 找到 "DG-LAB"，点击卸载

2. **删除用户数据 (可选)**
   ```powershell
   Remove-Item -Recurse $env:APPDATA\DG-LAB
   ```

### macOS

1. **删除应用**
   ```bash
   rm -rf /Applications/DG-LAB.app
   ```

2. **删除用户数据 (可选)**
   ```bash
   rm -rf ~/Library/Application\ Support/DG-LAB
   rm -rf ~/Library/Caches/com.dglab.app
   rm -rf ~/Library/Preferences/com.dglab.app.plist
   ```

### Linux

#### Debian/Ubuntu

```bash
# 卸载应用
sudo apt remove dglab

# 删除用户数据 (可选)
rm -rf ~/.config/DG-LAB
rm -rf ~/.local/share/DG-LAB
```

#### Fedora/RHEL

```bash
# 卸载应用
sudo dnf remove dglab

# 删除用户数据 (可选)
rm -rf ~/.config/DG-LAB
rm -rf ~/.local/share/DG-LAB
```

#### Arch

```bash
# 卸载应用
sudo pacman -R dglab
# 或 yay -R dglab-bin

# 删除用户数据 (可选)
rm -rf ~/.config/DG-LAB
rm -rf ~/.local/share/DG-LAB
```

#### AppImage

```bash
# 删除 AppImage 文件
rm DG-LAB-x86_64.AppImage

# 删除用户数据 (可选)
rm -rf ~/.config/DG-LAB
rm -rf ~/.local/share/DG-LAB
```

---

## 获取帮助

如果遇到未在本文档中解决的问题：

1. **查看日志文件**
   - Windows: `%APPDATA%\DG-LAB\logs\`
   - macOS: `~/Library/Logs/DG-LAB/`
   - Linux: `~/.local/share/DG-LAB/logs/`

2. **搜索已知问题**
   - [GitHub Issues](https://github.com/your-username/DG_LAB/issues)

3. **提交新问题**
   - 提供系统信息 (OS、版本、架构)
   - 附上错误日志
   - 描述复现步骤

4. **社区支持**
   - [GitHub Discussions](https://github.com/your-username/DG_LAB/discussions)

---

**安装完成后，请参阅 [用户指南](USER_GUIDE.md) 了解如何使用。**
