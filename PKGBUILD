# Maintainer: Your Name <your.email@example.com>
pkgname=dglab-gui-tauri
pkgver=0.1.3
pkgrel=1
pkgdesc="DG-LAB 设备控制器 - Tauri GUI 应用"
arch=('x86_64')
url="https://github.com/userzbb/DG-LAB"
license=('MIT')
depends=('webkit2gtk' 'gtk3' 'libayatana-appindicator')
makedepends=('rust' 'cargo' 'nodejs' 'npm')
source=("$pkgname-$pkgver.tar.gz::https://github.com/userzbb/DG-LAB/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "DG-LAB-$pkgver"
  
  # 安装前端依赖
  cd apps/dglab-gui-tauri
  npm ci
  
  # 构建 Tauri 应用
  npm run tauri build
}

package() {
  cd "DG-LAB-$pkgver"
  
  # 安装二进制文件
  install -Dm755 "target/release/dglab-gui-tauri" "$pkgdir/usr/bin/dglab-gui"
  
  # 安装桌面文件
  install -Dm644 "apps/dglab-gui-tauri/src-tauri/icons/128x128.png" \
    "$pkgdir/usr/share/pixmaps/dglab-gui.png"
  
  # 创建桌面快捷方式
  install -Dm644 /dev/stdin "$pkgdir/usr/share/applications/dglab-gui.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=DG-LAB Controller
Comment=DG-LAB 设备控制器
Exec=dglab-gui
Icon=dglab-gui
Categories=Utility;
Terminal=false
EOF
}
