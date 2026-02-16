# Arch Linux 打包指南

本项目为 Arch Linux 用户提供两种安装方式：

## 方式 1: 从源码构建 (推荐开发者)

使用 `PKGBUILD` 文件从源码编译安装：

```bash
# 下载 PKGBUILD
wget https://raw.githubusercontent.com/userzbb/DG-LAB/main/PKGBUILD

# 构建并安装
makepkg -si
```

这种方式会：
- 下载完整源代码
- 安装所有构建依赖（Rust、Node.js 等）
- 从源码编译应用
- 安装到系统

**优点**: 完全从源码构建，可自定义编译选项  
**缺点**: 需要完整的构建工具链，编译时间较长

---

## 方式 2: 预编译二进制 (推荐普通用户)

使用 `PKGBUILD.bin` 文件直接安装预编译的二进制：

```bash
# 下载二进制版 PKGBUILD
wget https://raw.githubusercontent.com/userzbb/DG-LAB/main/PKGBUILD.bin -O PKGBUILD

# 安装
makepkg -si
```

这种方式会：
- 从 GitHub Releases 下载预编译二进制
- 只需要运行时依赖（不需要 Rust、Node.js）
- 快速安装

**优点**: 无需编译，安装快速  
**缺点**: 仅支持 x86_64 架构

---

## 依赖说明

### 运行时依赖（两种方式都需要）
- `webkit2gtk` - WebView 支持
- `gtk3` - GTK 界面
- `libayatana-appindicator` - 系统托盘支持

### 构建依赖（仅方式 1 需要）
- `rust` - Rust 编译器
- `cargo` - Rust 包管理器
- `nodejs` - Node.js 运行时
- `npm` - Node.js 包管理器

---

## 发布到 AUR (维护者)

### 发布源码包到 AUR

1. 克隆 AUR 仓库：
```bash
git clone ssh://aur@aur.archlinux.org/dglab-gui-tauri.git
cd dglab-gui-tauri
```

2. 复制 PKGBUILD：
```bash
cp /path/to/DG-LAB/PKGBUILD .
```

3. 更新 .SRCINFO：
```bash
makepkg --printsrcinfo > .SRCINFO
```

4. 提交并推送：
```bash
git add PKGBUILD .SRCINFO
git commit -m "Update to version X.Y.Z"
git push
```

### 发布二进制包到 AUR

1. 克隆 AUR 仓库：
```bash
git clone ssh://aur@aur.archlinux.org/dglab-gui-tauri-bin.git
cd dglab-gui-tauri-bin
```

2. 复制 PKGBUILD.bin：
```bash
cp /path/to/DG-LAB/PKGBUILD.bin PKGBUILD
```

3. 更新 checksums：
```bash
# 下载二进制文件
wget https://github.com/userzbb/DG-LAB/releases/download/vX.Y.Z/dglab-gui-tauri-linux-x64-bin.tar.gz

# 生成 SHA256
sha256sum dglab-gui-tauri-linux-x64-bin.tar.gz

# 更新 PKGBUILD 中的 sha256sums
```

4. 更新 .SRCINFO：
```bash
makepkg --printsrcinfo > .SRCINFO
```

5. 提交并推送：
```bash
git add PKGBUILD .SRCINFO
git commit -m "Update to version X.Y.Z"
git push
```

---

## 本地测试

在推送到 AUR 之前，本地测试：

```bash
# 源码包测试
cd /path/to/PKGBUILD
makepkg -si

# 清理
makepkg --clean

# 二进制包测试
cd /path/to/PKGBUILD.bin
cp PKGBUILD.bin PKGBUILD
makepkg -si
```

---

## 卸载

```bash
sudo pacman -R dglab-gui-tauri
# 或
sudo pacman -R dglab-gui-tauri-bin
```

---

## 常见问题

### Q: 为什么有两个包？

A: 
- `dglab-gui-tauri` (源码包) - 适合想要从源码编译的用户
- `dglab-gui-tauri-bin` (二进制包) - 适合想要快速安装的用户

两个包不能同时安装（conflicts 设置），选择一个即可。

### Q: 如何更新？

A: 
```bash
# AUR helper (yay/paru)
yay -Syu dglab-gui-tauri

# 或手动
cd /path/to/pkgbuild
git pull  # 如果是 AUR 克隆
makepkg -si
```

### Q: 缺少依赖？

A: 确保安装了所有运行时依赖：
```bash
sudo pacman -S webkit2gtk gtk3 libayatana-appindicator
```
