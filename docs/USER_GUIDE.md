# DG-LAB 用户指南

欢迎使用 DG-LAB Rust 跨平台控制器！本指南将帮助您快速上手使用桌面 GUI 和命令行工具。

## 目录

- [桌面 GUI 使用指南](#桌面-gui-使用指南)
  - [首次启动](#首次启动)
  - [设备扫描和连接](#设备扫描和连接)
  - [功率控制](#功率控制)
  - [波形生成器](#波形生成器)
  - [预设管理](#预设管理)
- [命令行 CLI 使用指南](#命令行-cli-使用指南)
- [终端 TUI 使用指南](#终端-tui-使用指南)
- [常见问题](#常见问题)

---

## 桌面 GUI 使用指南

### 首次启动

1. **启动应用**
   ```bash
   cd apps/dglab-gui-tauri
   npm run tauri dev
   ```
   或者双击已安装的桌面图标。

2. **主界面**
   
   启动后会看到仪表盘（Dashboard），显示：
   - 当前连接状态
   - 设备信息
   - 快速操作卡片
   - 系统状态

### 设备扫描和连接

#### 方式一：BLE 蓝牙连接

1. **进入扫描页面**
   - 点击仪表盘的"扫描设备"卡片
   - 或使用顶部导航栏进入"设备扫描"页面

2. **开始扫描**
   - 点击"开始扫描"按钮
   - 确保您的 DG-LAB 设备已开启并处于配对模式
   - 扫描过程约需 5-10 秒

3. **查看扫描结果**
   - 扫描完成后，页面会显示所有发现的设备
   - 每个设备卡片显示：
     - 设备名称
     - 设备 ID
     - 信号强度 (RSSI)
     - MAC 地址

4. **连接设备**
   - 点击目标设备卡片上的"连接"按钮
   - 等待连接成功的 Toast 通知
   - 连接成功后会自动跳转到控制页面

#### 方式二：WiFi 连接

1. **确保设备和电脑在同一 WiFi 网络**

2. **使用二维码连接**
   - 在设备扫描页面切换到"WiFi"标签
   - 点击"生成二维码"
   - 使用 DG-LAB 设备扫描二维码
   - 设备会自动连接

3. **手动输入地址**
   - 输入设备的 WebSocket 地址（如 `ws://192.168.1.100:8080`）
   - 点击"连接"

### 功率控制

连接设备后，进入功率控制页面：

#### 基本控制

1. **双通道功率调节**
   - 使用滑块调节通道 A 和通道 B 的功率（0-200）
   - 实时显示当前功率值
   - 支持精确数值输入

2. **快速控制按钮**
   - **启动 (Start)**: 开始输出电刺激
   - **停止 (Stop)**: 停止输出
   - **紧急停止 (Emergency Stop)**: 立即将所有通道功率归零并停止

3. **设备状态显示**
   - 连接状态：Disconnected / Connecting / Connected / Running
   - 电池电量：实时显示剩余电量百分比
   - 设备信息：固件版本、硬件版本

#### 安全提示

- ⚠️ 首次使用建议从低功率开始（<50）
- ⚠️ 紧急停止按钮可随时中断输出
- ⚠️ 注意观察身体反应，适时调整

### 波形生成器

波形生成器允许您自定义电刺激的输出模式。

#### 使用步骤

1. **选择通道**
   - 切换通道 A 或通道 B 标签
   - 每个通道可独立配置

2. **选择波形类型**
   
   支持 8 种内置波形：
   - **Continuous (连续波)**: 恒定输出
   - **Pulse (脉冲波)**: 规律脉冲
   - **Sine (正弦波)**: 平滑起伏
   - **Square (方波)**: 方形脉冲
   - **Triangle (三角波)**: 线性上升下降
   - **Sawtooth (锯齿波)**: 快速上升缓慢下降
   - **Breathing (呼吸波)**: 模拟呼吸节奏
   - **Fade (渐强渐弱)**: 缓慢渐变

3. **调整参数**
   
   每种波形有不同的可调参数：
   
   - **频率 (Frequency)**: 1-500 Hz，控制波形振荡速度
   - **脉宽 (Pulse Width)**: 50-1000 μs，控制每个脉冲的持续时间
   - **占空比 (Duty Cycle)**: 0-100%，控制高电平占比
   - **周期 (Period)**: 100-10000 ms，控制完整周期长度
   - **功率范围**:
     - 最小功率 (Min Power): 0-200
     - 最大功率 (Max Power): 0-200

4. **应用波形**
   - 点击"应用波形"按钮
   - 波形会立即生效（如果设备正在运行）
   - Toast 通知确认应用成功

5. **保存自定义波形**
   - 配置完成后点击"保存为预设"
   - 输入预设名称和描述
   - 保存到预设库供日后使用

#### 波形使用建议

| 波形类型 | 适用场景 | 推荐参数 |
|---------|---------|---------|
| Continuous | 稳定持续刺激 | 功率 30-50 |
| Pulse | 节奏感训练 | 频率 10-50 Hz，脉宽 200 μs |
| Sine | 柔和渐变 | 周期 2000-5000 ms |
| Square | 明显对比 | 占空比 50%，频率 20 Hz |
| Breathing | 放松模式 | 周期 4000-6000 ms |

### 预设管理

预设功能让您保存和快速加载自定义配置。

#### 创建预设

1. **在波形生成器或控制页面配置好参数**
2. **点击"保存预设"按钮**
3. **填写预设信息**:
   - 预设名称（必填）
   - 描述（可选）
4. **点击"创建"**
5. **Toast 通知确认创建成功**

#### 管理预设

1. **进入预设管理页面**
   - 从仪表盘点击"预设管理"
   - 或使用导航栏进入

2. **查看所有预设**
   - 卡片式展示所有已保存的预设
   - 显示预设名称、描述、创建时间

3. **加载预设**
   - 点击预设卡片上的"加载"按钮
   - 预设参数会自动应用到设备
   - Toast 通知确认加载成功

4. **删除预设**
   - 点击预设卡片上的"删除"按钮
   - 确认删除对话框
   - Toast 通知确认删除成功

#### 预设文件位置

预设保存在本地文件系统：
- **Windows**: `%APPDATA%/DG-LAB/presets/`
- **macOS**: `~/Library/Application Support/DG-LAB/presets/`
- **Linux**: `~/.config/DG-LAB/presets/`

---

## 命令行 CLI 使用指南

命令行工具提供脚本化控制和自动化能力。

### 基本命令

```bash
# 查看帮助
dglab --help
dglab <COMMAND> --help

# 启用调试日志
dglab --debug <COMMAND>
```

### 设备扫描

```bash
# 扫描 BLE 设备（默认 10 秒）
dglab scan

# 自定义扫描时长（20 秒）
dglab scan --timeout 20

# 输出 JSON 格式
dglab scan --format json
```

### 设备连接

```bash
# 交互式连接（显示设备列表选择）
dglab connect

# 直接连接指定设备
dglab connect --id "DG-LAB-XXXX"

# WiFi 连接
dglab connect --wifi --address "ws://192.168.1.100:8080"
```

### 功率控制

```bash
# 设置双通道相同功率
dglab control --power 50

# 分别设置通道 A 和 B
dglab control --a 30 --b 40

# 启动输出
dglab control --start

# 停止输出
dglab control --stop

# 紧急停止
dglab control --emergency-stop
```

### 波形控制

```bash
# 列出可用波形
dglab waveform list

# 应用波形到通道 A
dglab waveform apply --channel A --type Sine --frequency 50

# 应用自定义波形
dglab waveform apply --channel B --type Custom --file wave.json
```

### 预设管理

```bash
# 列出所有预设
dglab preset list

# 应用预设
dglab preset apply <preset-name>

# 保存当前配置为预设
dglab preset save --name "我的预设" --description "描述信息"

# 删除预设
dglab preset delete <preset-name>

# 导出预设
dglab preset export <preset-name> --output preset.json

# 导入预设
dglab preset import preset.json
```

### 会话管理

```bash
# 查看当前会话信息
dglab session info

# 列出所有已连接设备
dglab session devices

# 断开所有设备
dglab session disconnect-all
```

---

## 终端 TUI 使用指南

终端 UI 提供交互式全屏界面。

### 启动 TUI

```bash
dglab tui
```

### 界面布局

```
┌─────────────────────────────────────────────────────────────┐
│  DG-LAB Controller TUI                          [Esc] 退出  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  设备列表              │  功率控制                           │
│  ┌──────────────────┐ │  ┌────────────────────────────────┐ │
│  │ DG-LAB-XXXX     │ │  │ 通道 A: [████████░░] 80        │ │
│  │ 状态: Connected │ │  │ 通道 B: [██████░░░░] 60        │ │
│  │ 电量: 85%       │ │  │                                 │ │
│  └──────────────────┘ │  │ [Start] [Stop] [Emergency]     │ │
│                       │  └────────────────────────────────┘ │
│  波形设置              │  预设                               │
│  ┌──────────────────┐ │  ┌────────────────────────────────┐ │
│  │ 类型: Sine      │ │  │ ▸ 预设1                        │ │
│  │ 频率: 50 Hz     │ │  │   预设2                        │ │
│  │ ...             │ │  │   预设3                        │ │
│  └──────────────────┘ │  └────────────────────────────────┘ │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 键盘快捷键

| 按键 | 功能 |
|------|------|
| `Esc` / `q` | 退出 TUI |
| `Tab` | 切换焦点区域 |
| `↑` / `↓` | 上下移动 |
| `←` / `→` | 左右调节 |
| `Enter` | 确认/应用 |
| `Space` | 启动/停止 |
| `e` | 紧急停止 |
| `s` | 扫描设备 |
| `c` | 连接设备 |
| `d` | 断开设备 |
| `1-9` | 快速切换预设 |
| `h` / `?` | 显示帮助 |

---

## 常见问题

### 1. 无法扫描到设备

**问题**: 扫描时未发现任何设备

**解决方案**:
- ✓ 确保设备已开启并处于配对模式
- ✓ 检查蓝牙适配器是否正常工作
- ✓ Linux 用户确保有蓝牙权限：
  ```bash
  sudo usermod -a -G bluetooth $USER
  sudo systemctl start bluetooth
  ```
- ✓ 尝试增加扫描时长：`dglab scan --timeout 30`
- ✓ 检查设备是否已被其他应用连接

### 2. 连接失败或频繁断开

**问题**: 连接时出错或连接后立即断开

**解决方案**:
- ✓ 确保设备电量充足（>20%）
- ✓ 减少设备与电脑之间的距离（<5 米）
- ✓ 关闭其他正在使用蓝牙的应用
- ✓ 重启设备和蓝牙适配器
- ✓ 检查系统日志：`dglab --debug connect`

### 3. 功率无法调节

**问题**: 设置功率后没有响应

**解决方案**:
- ✓ 确认设备状态为 "Running"（已启动输出）
- ✓ 检查通道最大功率限制
- ✓ 尝试先停止再重新启动
- ✓ 查看是否有错误 Toast 通知

### 4. WiFi 连接问题

**问题**: 无法通过 WiFi 连接设备

**解决方案**:
- ✓ 确保设备和电脑在同一局域网
- ✓ 检查 WebSocket 地址格式：`ws://IP:PORT`
- ✓ 确认设备 WiFi 功能已启用
- ✓ 检查防火墙设置
- ✓ 使用二维码扫描连接（更可靠）

### 5. GUI 无法启动

**问题**: 启动 GUI 时报错或黑屏

**解决方案**:

**Linux**:
```bash
# 安装系统依赖
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libdbus-1-dev

# 重新构建
cd apps/dglab-gui-tauri
npm install
npm run tauri build
```

**macOS**:
```bash
# 安装 Xcode Command Line Tools
xcode-select --install
```

**Windows**:
- 确保已安装 Visual C++ Redistributable
- 以管理员身份运行

### 6. 预设无法保存

**问题**: 点击保存后没有反应

**解决方案**:
- ✓ 检查磁盘空间是否充足
- ✓ 确认预设目录有写入权限：
  ```bash
  # Linux/macOS
  chmod 755 ~/.config/DG-LAB/presets/
  
  # Windows: 右键文件夹 → 属性 → 安全
  ```
- ✓ 预设名称不能包含特殊字符（`/`, `\`, `:`, `*`, `?`, `"`, `<`, `>`, `|`）

### 7. 性能问题

**问题**: GUI 卡顿或响应慢

**解决方案**:
- ✓ 关闭其他占用资源的应用
- ✓ 检查 CPU 和内存使用率
- ✓ 更新显卡驱动（Linux 用户特别注意）
- ✓ 降低刷新频率（如果有该设置）

### 8. 找不到命令

**问题**: 终端显示 `command not found: dglab`

**解决方案**:

**方式一: 使用 cargo run**
```bash
cargo run --bin dglab -- <COMMAND>
```

**方式二: 添加到 PATH**
```bash
# 构建 release 版本
cargo build --release

# 复制到 PATH
# Linux/macOS
sudo cp target/release/dglab /usr/local/bin/

# Windows (管理员 PowerShell)
Copy-Item target/release/dglab.exe C:\Windows\System32\
```

**方式三: 使用 cargo install**
```bash
cargo install --path crates/dglab-cli
```

---

## 获取帮助

如果遇到其他问题：

1. **查看日志**
   ```bash
   # GUI 日志（开发模式）
   cd apps/dglab-gui-tauri
   npm run tauri dev
   # 查看终端输出
   
   # CLI 调试日志
   dglab --debug <COMMAND>
   ```

2. **搜索已知问题**
   - [GitHub Issues](https://github.com/your-username/DG_LAB/issues)

3. **提交新问题**
   - 描述问题现象
   - 提供系统信息（OS、版本）
   - 附上日志输出
   - 说明复现步骤

4. **加入讨论**
   - [GitHub Discussions](https://github.com/your-username/DG_LAB/discussions)

---

## 安全须知

⚠️ **重要提醒**:

1. **身体安全**
   - 电刺激设备可能对某些人造成不适或危险
   - 如有心脏疾病、怀孕等情况，请勿使用
   - 不要在驾驶或操作机械时使用
   - 避免将电极放置在心脏、头部、颈部等敏感部位

2. **设备使用**
   - 首次使用从低功率开始（建议 <30）
   - 逐渐增加功率，注意身体反应
   - 感到不适立即停止
   - 定期检查电极片和导线

3. **数据隐私**
   - 本软件不收集任何用户数据
   - 所有配置保存在本地
   - 预设文件可自行备份

---

**祝您使用愉快！** 🎉

如有疑问，欢迎提交 Issue 或参与 Discussions。
