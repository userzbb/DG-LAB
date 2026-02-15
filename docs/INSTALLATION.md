# DG-LAB 安装指南

本文档提供 DG-LAB 控制器在各平台的详细安装说明。

## 目录

- [系统要求](#系统要求)
- [预编译安装包 (推荐)](#预编译安装包-推荐)
- [从源码构建](#从源码构建)
  - [Windows](#windows-从源码构建)
  - [macOS](#macos-从源码构建)
  - [Linux](#linux-从源码构建)
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

1. **下载安装程序**
   - 访问 [Releases 页面](https://github.com/your-username/DG_LAB/releases)
   - 下载最新版本的 `DG-LAB-Setup-x64.exe`

2. **运行安装程序**
   - 双击 `.exe` 文件
   - 根据安装向导提示操作
   - 选择安装目录 (默认: `C:\Program Files\DG-LAB\`)
   - 勾选"创建桌面快捷方式"

3. **首次运行**
   - 双击桌面图标或开始菜单中的"DG-LAB"
   - Windows Defender 可能提示安全警告，选择"仍要运行"

#### 便携版 (无需安装)

1. 下载 `DG-LAB-Portable-x64.zip`
2. 解压到任意目录
3. 运行 `DG-LAB.exe`

### macOS

1. **下载安装包**
   - 访问 [Releases 页面](https://github.com/your-username/DG_LAB/releases)
   - 下载 `DG-LAB_x64.dmg` (Intel) 或 `DG-LAB_aarch64.dmg` (Apple Silicon)

2. **安装应用**
   - 双击 `.dmg` 文件打开
   - 将 `DG-LAB.app` 拖动到 `Applications` 文件夹
   - 卸载磁盘映像

3. **首次运行**
   - 打开 `应用程序` 文件夹，找到 `DG-LAB`
   - 右键点击 → 选择"打开" (绕过 Gatekeeper 检查)
   - 如果提示"无法打开"，前往 `系统偏好设置` → `安全性与隐私` → 点击"仍要打开"

4. **赋予蓝牙权限**
   - 首次使用蓝牙功能时会提示
   - 前往 `系统偏好设置` → `安全性与隐私` → `隐私` → `蓝牙`
   - 勾选 `DG-LAB`

### Linux

#### Debian / Ubuntu / Linux Mint

```bash
# 下载 .deb 包
wget https://github.com/your-username/DG_LAB/releases/download/v0.1.0/dglab_0.1.0_amd64.deb

# 安装
sudo dpkg -i dglab_0.1.0_amd64.deb

# 如果有依赖问题，运行：
sudo apt-get install -f

# 启动应用
dglab-gui
```

#### Fedora / RHEL / CentOS

```bash
# 下载 .rpm 包
wget https://github.com/your-username/DG_LAB/releases/download/v0.1.0/dglab-0.1.0.x86_64.rpm

# 安装
sudo dnf install ./dglab-0.1.0.x86_64.rpm

# 或使用 yum
sudo yum localinstall ./dglab-0.1.0.x86_64.rpm

# 启动应用
dglab-gui
```

#### Arch Linux / Manjaro

```bash
# 使用 AUR 助手 (如 yay)
yay -S dglab-bin

# 或手动安装
git clone https://aur.archlinux.org/dglab-bin.git
cd dglab-bin
makepkg -si

# 启动应用
dglab-gui
```

#### AppImage (通用格式)

```bash
# 下载 AppImage
wget https://github.com/your-username/DG_LAB/releases/download/v0.1.0/DG-LAB-x86_64.AppImage

# 添加执行权限
chmod +x DG-LAB-x86_64.AppImage

# 运行
./DG-LAB-x86_64.AppImage

# (可选) 集成到系统
./DG-LAB-x86_64.AppImage --appimage-extract
sudo mv squashfs-root /opt/dglab
sudo ln -s /opt/dglab/AppRun /usr/local/bin/dglab-gui
```

#### Flatpak

```bash
# 添加 Flathub 仓库 (如果尚未添加)
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# 安装应用
flatpak install flathub com.dglab.Controller

# 运行
flatpak run com.dglab.Controller
```

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
