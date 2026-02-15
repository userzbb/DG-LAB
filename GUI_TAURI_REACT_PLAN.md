# GUI 技术栈更新计划 - Tauri + React + shadcn-ui + Tailwind CSS

## 变更说明

将 GUI 技术栈从 **egui** 更新为 **Tauri 2.0 + React + shadcn-ui + Tailwind CSS**，实现跨平台桌面 + 移动应用。

**支持平台：**
- ✅ Windows
- ✅ macOS
- ✅ Linux
- ✅ Android
- ❌ iOS（暂不考虑，缺少工具链）

---

## Tauri 2.0 架构

### 核心概念

Tauri 2.0 是一个跨平台应用开发框架，采用 **混合架构**：

```
┌─────────────────────────────────────────────────────────┐
│                     前端 (Frontend)                      │
│  ┌───────────────────────────────────────────────────┐  │
│  │  React + TypeScript + Vite                       │  │
│  │  - UI 渲染                                        │  │
│  │  - 用户交互                                        │  │
│  │  - 状态管理 (Zustand)                              │  │
│  └───────────────────────────────────────────────────┘  │
│                          │                              │
│                  IPC (Inter-Process Communication)      │
│                  - invoke() 调用命令                     │
│                  - listen() 监听事件                     │
├─────────────────────────────────────────────────────────┤
│                     后端 (Backend)                       │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Tauri Core (Rust)                                │  │
│  │  - Window 管理                                    │  │
│  │  - 系统 API 访问                                  │  │
│  │  - 安全沙箱                                       │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │  业务逻辑 (Rust)                                  │  │
│  │  - Tauri Commands (命令层)                        │  │
│  │  - dglab-core (核心库)                            │  │
│  │  - BLE/WiFi 通信                                  │  │
│  └───────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────┤
│                   系统 WebView                           │
│  - Windows: WebView2                                    │
│  - macOS: WebKit                                        │
│  - Linux: WebKitGTK                                     │
│  - Android: Android WebView                             │
└─────────────────────────────────────────────────────────┘
```

### 关键组件

| 组件 | 说明 |
|------|------|
| **WebView** | 使用系统原生 WebView 渲染前端，无需打包 Chromium |
| **Tauri Core** | Rust 编写的核心，管理窗口、系统 API、安全策略 |
| **IPC** | 前后端通信桥梁，支持命令调用和事件监听 |
| **Commands** | 前端可调用的 Rust 函数，用 `#[tauri::command]` 标记 |
| **State** | 后端全局状态，用 `tauri::State` 管理 |

### 相比 Electron 的优势

| 特性 | Tauri 2.0 | Electron |
|------|-----------|----------|
| 包体积 | ~10-20MB | ~100-200MB |
| 内存占用 | 低 (使用系统 WebView) | 高 (打包 Chromium) |
| 安全性 | 沙箱 + 权限控制 | 相对较弱 |
| 后端语言 | Rust | Node.js |
| 移动端 | 支持 (Android/iOS) | 不支持 |

---

## 技术栈详解

| 技术 | 版本 | 说明 |
|------|------|------|
| Tauri | 2.0+ | 使用系统 WebView，轻量级、安全 |
| React | 18+ | 前端框架 |
| TypeScript | 5.0+ | 类型安全 |
| Vite | 5.0+ | 构建工具 |
| Tailwind CSS | 3.4+ | 原子化 CSS 框架 |
| shadcn-ui | latest | 基于 Radix UI 的组件库 |
| Lucide React | latest | 图标库 |
| Zustand | latest | 轻量级状态管理 |

---

## 项目结构

```
DG_LAB/
├── crates/
│   ├── dglab-protocol/
│   ├── dglab-core/
│   ├── dglab-cli/
│   └── dglab-gui/          (原 egui 版本，保留作为参考)
│
└── apps/
    └── dglab-gui-tauri/     (新 Tauri + React 版本)
        ├── src-tauri/        (Rust 后端)
        │   ├── Cargo.toml
        │   ├── tauri.conf.json
        │   ├── src/
        │   │   ├── main.rs       (Tauri 入口 - 桌面端)
        │   │   ├── lib.rs        (Tauri 库 - 移动端共享)
        │   │   ├── commands.rs   (Tauri commands)
        │   │   ├── state.rs      (应用状态管理)
        │   │   └── error.rs      (错误处理)
        │   ├── build.rs
        │   ├── gen/            (自动生成的移动端代码)
        │   └── mobile/         (移动端特定代码)
        │       └── android/
        │
        ├── src/              (React 前端)
        │   ├── main.tsx
        │   ├── App.tsx
        │   ├── components/
        │   │   ├── ui/              (shadcn-ui 组件)
        │   │   ├── BleDevicePanel.tsx
        │   │   ├── WifiPanel.tsx
        │   │   ├── ControlPanel.tsx
        │   │   ├── WaveformEditor.tsx
        │   │   ├── PresetsPanel.tsx
        │   │   └── SettingsPanel.tsx
        │   ├── hooks/
        │   │   ├── use-device.ts
        │   │   ├── use-power.ts
        │   │   └── use-platform.ts   (平台检测)
        │   ├── stores/
        │   │   └── app-store.ts
        │   ├── types/
        │   │   └── index.ts
        │   └── lib/
        │       └── tauri.ts
        │
        ├── package.json
        ├── tsconfig.json
        ├── vite.config.ts
        ├── tailwind.config.js
        ├── postcss.config.js
        ├── capgo.config.json    (Capgo 配置 - 移动端热更新)
        └── .gitignore
```

---

## 初始化步骤

### 1. 创建 Tauri 项目

```bash
# 使用 Tauri 2.0 模板
npm create tauri-app@latest apps/dglab-gui-tauri

# 选择以下选项：
# - Package manager: npm
# - Frontend template: React + TypeScript
# - UI library: None (我们手动添加 shadcn-ui)
```

### 2. 安装依赖

```bash
cd apps/dglab-gui-tauri

# 安装 Tailwind CSS
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

# 安装 shadcn-ui
npx shadcn@latest init

# 安装需要的组件
npx shadcn@latest add button
npx shadcn@latest add card
npx shadcn@latest add input
npx shadcn@latest add slider
npx shadcn@latest add switch
npx shadcn@latest add tabs
npx shadcn@latest add badge
npx shadcn@latest add separator
npx shadcn@latest add label
npx shadcn@latest add dialog
npx shadcn@latest add select
npx shadcn@latest add list

# 安装其他依赖
npm install lucide-react qrcode.react zustand
```

### 3. 配置 Tailwind CSS

```js
// tailwind.config.js
/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ["class"],
  content: [
    "./index.html",
    "./src/**/*.{ts,tsx}",
  ],
  theme: {
    extend: {
      borderRadius: {
        lg: 'var(--radius)',
        md: 'calc(var(--radius) - 2px)',
        sm: 'calc(var(--radius) - 4px)'
      },
      colors: {
        background: 'hsl(var(--background))',
        foreground: 'hsl(var(--foreground))',
        card: {
          DEFAULT: 'hsl(var(--card))',
          foreground: 'hsl(var(--card-foreground))'
        },
        popover: {
          DEFAULT: 'hsl(var(--popover))',
          foreground: 'hsl(var(--popover-foreground))'
        },
        primary: {
          DEFAULT: 'hsl(var(--primary))',
          foreground: 'hsl(var(--primary-foreground))'
        },
        secondary: {
          DEFAULT: 'hsl(var(--secondary))',
          foreground: 'hsl(var(--secondary-foreground))'
        },
        muted: {
          DEFAULT: 'hsl(var(--muted))',
          foreground: 'hsl(var(--muted-foreground))'
        },
        accent: {
          DEFAULT: 'hsl(var(--accent))',
          foreground: 'hsl(var(--accent-foreground))'
        },
        destructive: {
          DEFAULT: 'hsl(var(--destructive))',
          foreground: 'hsl(var(--destructive-foreground))'
        },
        border: 'hsl(var(--border))',
        input: 'hsl(var(--input))',
        ring: 'hsl(var(--ring))',
        chart: {
          '1': 'hsl(var(--chart-1))',
          '2': 'hsl(var(--chart-2))',
          '3': 'hsl(var(--chart-3))',
          '4': 'hsl(var(--chart-4))',
          '5': 'hsl(var(--chart-5))'
        },
        sidebar: {
          DEFAULT: 'hsl(var(--sidebar-background))',
          foreground: 'hsl(var(--sidebar-foreground))',
          primary: 'hsl(var(--sidebar-primary))',
          'primary-foreground': 'hsl(var(--sidebar-primary-foreground))',
          accent: 'hsl(var(--sidebar-accent))',
          'accent-foreground': 'hsl(var(--sidebar-accent-foreground))',
          border: 'hsl(var(--sidebar-border))',
          ring: 'hsl(var(--sidebar-ring))'
        }
      }
    }
  },
  plugins: [require("tailwindcss-animate")],
}
```

```css
/* src/index.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 240 10% 3.9%;
    --card: 0 0% 100%;
    --card-foreground: 240 10% 3.9%;
    --popover: 0 0% 100%;
    --popover-foreground: 240 10% 3.9%;
    --primary: 240 5.9% 10%;
    --primary-foreground: 0 0% 98%;
    --secondary: 240 4.8% 95.9%;
    --secondary-foreground: 240 5.9% 10%;
    --muted: 240 4.8% 95.9%;
    --muted-foreground: 240 3.8% 46.1%;
    --accent: 240 4.8% 95.9%;
    --accent-foreground: 240 5.9% 10%;
    --destructive: 0 84.2% 60.2%;
    --destructive-foreground: 0 0% 98%;
    --border: 240 5.9% 90%;
    --input: 240 5.9% 90%;
    --ring: 240 5.9% 10%;
    --radius: 0.5rem;
    --chart-1: 12 76% 61%;
    --chart-2: 173 58% 39%;
    --chart-3: 197 37% 24%;
    --chart-4: 43 74% 66%;
    --chart-5: 27 87% 67%;
    --sidebar-background: 0 0% 98%;
    --sidebar-foreground: 240 5.3% 26.1%;
    --sidebar-primary: 240 5.9% 10%;
    --sidebar-primary-foreground: 0 0% 98%;
    --sidebar-accent: 240 4.8% 95.9%;
    --sidebar-accent-foreground: 240 5.9% 10%;
    --sidebar-border: 220 13% 91%;
    --sidebar-ring: 217.2 91.2% 59.8%;
  }

  .dark {
    --background: 240 10% 3.9%;
    --foreground: 0 0% 98%;
    --card: 240 10% 3.9%;
    --card-foreground: 0 0% 98%;
    --popover: 240 10% 3.9%;
    --popover-foreground: 0 0% 98%;
    --primary: 0 0% 98%;
    --primary-foreground: 240 5.9% 10%;
    --secondary: 240 3.7% 15.9%;
    --secondary-foreground: 0 0% 98%;
    --muted: 240 3.7% 15.9%;
    --muted-foreground: 240 5% 64.9%;
    --accent: 240 3.7% 15.9%;
    --accent-foreground: 0 0% 98%;
    --destructive: 0 62.8% 30.6%;
    --destructive-foreground: 0 0% 98%;
    --border: 240 3.7% 15.9%;
    --input: 240 3.7% 15.9%;
    --ring: 240 4.9% 83.9%;
    --chart-1: 220 70% 50%;
    --chart-2: 160 60% 45%;
    --chart-3: 30 80% 55%;
    --chart-4: 280 65% 60%;
    --chart-5: 340 75% 55%;
    --sidebar-background: 240 5.9% 10%;
    --sidebar-foreground: 240 4.8% 95.9%;
    --sidebar-primary: 224.3 76.3% 48%;
    --sidebar-primary-foreground: 0 0% 100%;
    --sidebar-accent: 240 3.7% 15.9%;
    --sidebar-accent-foreground: 240 4.8% 95.9%;
    --sidebar-border: 240 3.7% 15.9%;
    --sidebar-ring: 217.2 91.2% 59.8%;
  }
}

@layer base {
  * {
    @apply border-border;
  }
  body {
    @apply bg-background text-foreground;
  }
}
```

### 4. 配置 Tauri (src-tauri/Cargo.toml)

```toml
[package]
name = "dglab-gui-tauri"
version = "0.1.0"
description = "DG-LAB GUI - Tauri + React"
authors = ["DG-LAB Contributors"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/your-org/dglab-rs"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2.0", features = [] }

[dependencies]
tauri = { version = "2.0", features = [] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 本地依赖
dglab-core = { path = "../../crates/dglab-core" }
dglab-protocol = { path = "../../crates/dglab-protocol" }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

### 5. 配置 tauri.conf.json

```json
{
  "$schema": "https://schema.tauri.app/config/2.0.0",
  "productName": "DG-LAB Controller",
  "version": "0.1.0",
  "identifier": "com.dglab.controller",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "DG-LAB Controller",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "resizable": true
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

---

## Android 移动端支持

### 前置要求

- Android Studio (最新稳定版)
- Android SDK API 26+
- Android NDK r25+
- Java 17+

### 初始化 Android 项目

```bash
cd apps/dglab-gui-tauri

# 1. 添加 Android 平台
npm run tauri android init

# 2. 开发模式 (连接设备或启动模拟器)
npm run tauri android dev

# 3. 构建 APK
npm run tauri android build
```

### Android 配置 (src-tauri/gen/android/app/build.gradle.kts)

```kotlin
android {
    namespace = "com.dglab.controller"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.dglab.controller"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "0.1.0"
    }

    // 权限配置
    androidComponents {
        beforeVariants { variant ->
            variant.manifestPlaceholders["usesCleartextTraffic"] = "true"
        }
    }
}

dependencies {
    // BLE 相关依赖
    implementation("androidx.permissions:permissions:1.3.1")
}
```

### Android 权限配置 (src-tauri/gen/android/app/src/main/AndroidManifest.xml)

```xml
<manifest xmlns:android="http://schemas.android.com/apk/res/android">

    <!-- 网络权限 -->
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />

    <!-- BLE 权限 (Android 12+) -->
    <uses-permission android:name="android.permission.BLUETOOTH_SCAN"
        android:usesPermissionFlags="neverForLocation" />
    <uses-permission android:name="android.permission.BLUETOOTH_CONNECT" />

    <!-- BLE 权限 (Android 6.0 - 11) -->
    <uses-permission android:name="android.permission.BLUETOOTH" android:maxSdkVersion="30" />
    <uses-permission android:name="android.permission.BLUETOOTH_ADMIN" android:maxSdkVersion="30" />
    <uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" android:maxSdkVersion="30" />

    <!-- BLE 功能声明 -->
    <uses-feature android:name="android.hardware.bluetooth_le" android:required="false" />

    <application
        android:allowBackup="true"
        android:icon="@mipmap/ic_launcher"
        android:label="@string/app_name"
        android:supportsRtl="true"
        android:theme="@style/Theme.AppCompat.Light.NoActionBar"
        android:usesCleartextTraffic="true">

        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:theme="@style/Theme.Tauri.Activity"
            android:configChanges="orientation|keyboardHidden|keyboard|screenSize|smallestScreenSize|locale|layoutDirection|fontScale|screenLayout|density|uiMode"
            android:hardwareAccelerated="true"
            android:launchMode="singleTop"
            android:windowSoftInputMode="adjustResize">

            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>

</manifest>
```

### 移动端响应式 UI

在 React 组件中使用平台检测：

```typescript
// src/hooks/use-platform.ts
import { useEffect, useState } from 'react';
import { platform } from '@tauri-apps/plugin-os';

type Platform = 'windows' | 'macos' | 'linux' | 'android' | 'ios' | 'unknown';

export function usePlatform() {
  const [currentPlatform, setCurrentPlatform] = useState<Platform>('unknown');
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    platform().then((p) => {
      setCurrentPlatform(p as Platform);
      setIsMobile(p === 'android' || p === 'ios');
    });
  }, []);

  return {
    platform: currentPlatform,
    isMobile,
    isDesktop: !isMobile,
  };
}
```

使用示例：

```tsx
// src/App.tsx
export function App() {
  const { isMobile } = usePlatform();

  return (
    <div className={isMobile ? 'p-2' : 'p-6'}>
      {isMobile ? (
        <MobileLayout />
      ) : (
        <DesktopLayout />
      )}
    </div>
  );
}
```

### 移动端命令

```bash
# 列出已连接的 Android 设备
npm run tauri android list

# 在特定设备上运行
npm run tauri android dev -- --device <device-id>

# 构建 APK (debug)
npm run tauri android build

# 构建 APK (release)
npm run tauri android build -- --release

# 打开 Android Studio 项目
npm run tauri android open
```

---

## Tauri Commands 设计 (src-tauri/src/commands.rs)

```rust
use tauri::State;
use dglab_core::session::SessionManager;
use dglab_core::device::{Device, DeviceState};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type AppState = Arc<RwLock<SessionManager>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BleDeviceInfo {
    pub id: String,
    pub name: String,
    pub address: String,
    pub rssi: i16,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceStatus {
    pub id: String,
    pub name: String,
    pub state: DeviceState,
    pub power_a: u8,
    pub power_b: u8,
}

#[tauri::command]
pub async fn ble_scan(
    state: State<'_, AppState>,
    duration: u64,
) -> Result<Vec<BleDeviceInfo>, String> {
    // 调用 dglab-core BLE 扫描
    // 这里需要访问 BLE manager
    Ok(vec![])
}

#[tauri::command]
pub async fn ble_connect(
    state: State<'_, AppState>,
    device_id: String,
) -> Result<(), String> {
    // 连接 BLE 设备并添加到 session
    Ok(())
}

#[tauri::command]
pub async fn ble_disconnect(
    state: State<'_, AppState>,
    device_id: String,
) -> Result<(), String> {
    state.write().await.remove_device(&device_id).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn wifi_connect(
    state: State<'_, AppState>,
    server_url: Option<String>,
) -> Result<String, String> {
    // 创建 WiFi 设备并连接
    // 返回 QR URL
    Ok("".to_string())
}

#[tauri::command]
pub async fn wifi_disconnect(
    state: State<'_, AppState>,
) -> Result<(), String> {
    // 断开 WiFi
    Ok(())
}

#[tauri::command]
pub async fn set_power(
    state: State<'_, AppState>,
    device_id: Option<String>,
    channel: u8,
    power: u8,
) -> Result<(), String> {
    let devices = state.read().await.list_devices().await;
    let target_id = device_id.or_else(|| devices.first().cloned())
        .ok_or("No device connected")?;

    if let Some(device) = state.read().await.get_device(&target_id).await {
        let mut dev = device.write().await;
        dev.set_power(channel, power).await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_devices(
    state: State<'_, AppState>,
) -> Result<Vec<DeviceStatus>, String> {
    let devices = state.read().await.list_devices().await;
    let mut result = Vec::new();

    for id in devices {
        if let Some(device) = state.read().await.get_device(&id).await {
            let dev = device.read().await;
            result.push(DeviceStatus {
                id: dev.id().to_string(),
                name: dev.name().to_string(),
                state: dev.state(),
                power_a: dev.get_power(0),
                power_b: dev.get_power(1),
            });
        }
    }

    Ok(result)
}
```

---

## Tauri 主入口 (src-tauri/src/main.rs)

```rust
mod commands;
mod error;

use commands::*;
use dglab_core::session::SessionManager;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    session_manager: Arc<RwLock<SessionManager>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            session_manager: Arc::new(RwLock::new(SessionManager::new())),
        })
        .invoke_handler(tauri::generate_handler![
            ble_scan,
            ble_connect,
            ble_disconnect,
            wifi_connect,
            wifi_disconnect,
            set_power,
            get_devices,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## React 前端实现要点

### Zustand 状态管理 (src/stores/app-store.ts)

```typescript
import { create } from 'zustand';

interface Device {
  id: string;
  name: string;
  state: string;
  powerA: number;
  powerB: number;
}

interface AppStore {
  devices: Device[];
  activeTab: 'ble' | 'wifi' | 'control' | 'waveform' | 'presets' | 'settings';
  setActiveTab: (tab: AppStore['activeTab']) => void;
  refreshDevices: () => Promise<void>;
  setPower: (deviceId: string | null, channel: 0 | 1, power: number) => Promise<void>;
}

export const useAppStore = create<AppStore>((set, get) => ({
  devices: [],
  activeTab: 'ble',

  setActiveTab: (tab) => set({ activeTab: tab }),

  refreshDevices: async () => {
    const devices = await invoke('get_devices');
    set({ devices: devices as Device[] });
  },

  setPower: async (deviceId, channel, power) => {
    await invoke('set_power', { deviceId, channel, power });
    get().refreshDevices();
  },
}));
```

### Tauri 调用封装 (src/lib/tauri.ts)

```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface BleDevice {
  id: string;
  name: string;
  address: string;
  rssi: number;
}

export interface DeviceStatus {
  id: string;
  name: string;
  state: string;
  powerA: number;
  powerB: number;
}

export const tauri = {
  ble: {
    scan: async (duration: number = 3000): Promise<BleDevice[]> => {
      return await invoke('ble_scan', { duration });
    },
    connect: async (deviceId: string): Promise<void> => {
      return await invoke('ble_connect', { deviceId });
    },
    disconnect: async (deviceId: string): Promise<void> => {
      return await invoke('ble_disconnect', { deviceId });
    },
  },
  wifi: {
    connect: async (serverUrl?: string): Promise<string> => {
      return await invoke('wifi_connect', { serverUrl });
    },
    disconnect: async (): Promise<void> => {
      return await invoke('wifi_disconnect');
    },
  },
  devices: {
    list: async (): Promise<DeviceStatus[]> => {
      return await invoke('get_devices');
    },
    setPower: async (deviceId: string | null, channel: 0 | 1, power: number): Promise<void> => {
      return await invoke('set_power', { deviceId, channel, power });
    },
  },
  events: {
    listen: async <T>(event: string, handler: (payload: T) => void) => {
      return await listen<T>(event, (e) => handler(e.payload));
    },
  },
};
```

---

## 开发与构建命令

### 桌面端

```bash
cd apps/dglab-gui-tauri

# 开发模式
npm run tauri dev

# 构建生产版本
npm run tauri build

# 仅构建前端
npm run build

# 类型检查
npm run type-check
```

### Android 移动端

```bash
# 列出设备
npm run tauri android list

# 开发模式
npm run tauri android dev

# 构建 APK
npm run tauri android build

# 构建 Release APK
npm run tauri android build -- --release
```

---

## 平台特定配置

### Windows

```json
{
  "bundle": {
    "windows": {
      "certificateThumbprint": null,
      "timestampUrl": null,
      "wix": {
        "language": "zh-CN"
      }
    }
  }
}
```

### macOS

```json
{
  "bundle": {
    "macOS": {
      "entitlements": null,
      "exceptionDomain": "",
      "frameworks": [],
      "providerShortName": null,
      "signingIdentity": null
    }
  }
}
```

### Linux

```json
{
  "bundle": {
    "linux": {
      "deb": {
        "depends": []
      },
      "appimage": {
        "bundleMediaFramework": true
      }
    }
  }
}
```

---

## 实现阶段

| 阶段 | 任务 |
|------|------|
| 1️⃣ 项目初始化 | 创建 Tauri 项目，配置 Tailwind + shadcn-ui |
| 2️⃣ 后端集成 | 实现 Tauri Commands，集成 dglab-core |
| 3️⃣ 基础 UI | 实现主布局、BLE/WiFi 面板 |
| 4️⃣ 控制面板 | 实现强度控制、波形编辑器 |
| 5️⃣ 预设系统 | 实现预设保存/加载/应用 |
| 6️⃣ 响应式适配 | 移动端 UI 适配 |
| 7️⃣ Android 集成 | 配置 Android 平台、权限 |
| 8️⃣ 测试优化 | 跨平台测试、性能优化 |

---

## 注意事项

1. **BLE 权限**: Windows/macOS/Linux/Android 各平台 BLE 权限配置不同
2. **网络权限**: WiFi 功能需要网络访问权限，在 `tauri.conf.json` 中配置
3. **深色模式**: 使用 Tailwind 的 `dark:` 前缀支持
4. **类型安全**: TypeScript + Tauri 命令类型定义
5. **错误处理**: 统一的错误提示组件
6. **权限配置**: 在 `tauri.conf.json` 的 `app.security` 中配置允许访问的 API
7. **事件系统**: 使用 Tauri Events 实现后端到前端的状态推送
8. **响应式设计**: 使用 Tailwind 的响应式前缀 (sm:, md:, lg:) 和 `usePlatform` hook

---

## 参考资料

- [Tauri 2.0 官方文档](https://v2.tauri.app/zh-cn/)
- [Tauri 架构概念](https://v2.tauri.app/zh-cn/concept/architecture/)
- [Tauri Android 开发指南](https://tasukehub.com/articles/tauri-v2-mobile-guide-2025)
- [shadcn-ui 文档](https://ui.shadcn.com/)
- [Tailwind CSS 文档](https://tailwindcss.com/docs)
