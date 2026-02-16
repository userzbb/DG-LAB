# DG-LAB 桥接模式测试指南

## 测试环境准备

### 1. Windows 环境（用于运行桥接程序）

需要：
- DG-LAB 设备（如郊狼 3.0，设备名：47L121000）
- 蓝牙适配器
- 从 GitHub Release 下载的 `dglab.exe`

### 2. 测试脚本环境（可在任意设备）

安装 Python 依赖（使用 PyDGLab-WS 库）：

```bash
pip install pydglab-ws
```

或者：

```bash
pip install -r requirements-test.txt
```

PyDGLab-WS 文档：https://pydglab-ws.readthedocs.io

## 测试步骤

### 第一步：启动桥接程序（Windows）

```powershell
# 扫描设备
.\dglab.exe scan

# 启动桥接模式（替换为你的设备名）
.\dglab.exe bridge --device 47L121000

# 使用详细输出查看所有状态变化
.\dglab.exe bridge --device 47L121000 --verbose
```

程序会输出类似内容：

```
🌉 启动 BLE-WebSocket 桥接模式

📡 步骤 1: 扫描 BLE 设备...
✓ 找到设备: 47L121000 (xx:xx:xx:xx:xx:xx)

🔧 步骤 2: 创建桥接设备...
📲 步骤 3: 连接 BLE 设备...
✓ BLE 设备已连接

🌐 步骤 4: 连接 WebSocket 服务器...
✓ 已连接到服务器

📱 步骤 5: 获取二维码...
███████████████████████████████████
█ ▄▄▄▄▄ █▀▄█  ▄▀▄▀▀▄▀▀█ ▄▄▄▄▄ █
█ █   █ █▄  ▄▄██▀█▄ ▀ █ █   █ █
█ █▄▄▄█ █ █▀▀▄█▄▀▀█▀▄ █ █▄▄▄█ █
...

🔗 URL: https://www.dungeon-lab.cn/app-download.php#XXXXXX

⏳ 等待控制器连接...

✅ 桥接模式已启动！

📊 实时状态：
  • BLE 设备: 47L121000
  • WebSocket: wss://ws.dungeon-lab.cn

💡 提示：
  • 第三方控制器可以通过 WebSocket 发送控制指令
  • 程序会自动将指令转发给 BLE 设备
  • BLE 设备状态会同步到 WebSocket 服务器
  • 按 Ctrl+C 停止
```

**重要**：记下输出的 Client ID（在 URL 的 # 后面）

### 第二步：运行测试脚本

在另一台设备或另一个终端：

```bash
# 运行测试脚本
python test-bridge.py

# 或使用自定义服务器
python test-bridge.py wss://your-server.com
```

### 第三步：输入目标 ID

脚本会提示你输入目标 Client ID，复制桥接程序输出的 ID 并粘贴：

```
目标 Client ID> XXXXXX
```

### 第四步：测试控制指令

脚本连接成功后，可以使用以下命令：

#### 基本命令

```bash
# A通道操作
a+10    # A通道增加 10
a-10    # A通道减少 10
a=50    # A通道设置为 50

# B通道操作
b+10    # B通道增加 10
b-10    # B通道减少 10
b=50    # B通道设置为 50

# 清空操作
ca      # 清空 A通道
cb      # 清空 B通道

# 波形测试
pulse   # 发送测试波形数据

# 自动测试
auto    # 运行一系列自动测试

# 退出
quit    # 退出程序
```

#### 示例对话

```
命令> a=50
[14:32:10] 📤 发送: strength-1+2+50
[14:32:10] 📥 设备状态: A=50, B=0, MaxA=200, MaxB=200

命令> b=50
[14:32:15] 📤 发送: strength-2+2+50
[14:32:15] 📥 设备状态: A=50, B=50, MaxA=200, MaxB=200

命令> a+20
[14:32:20] 📤 发送: strength-1+1+20
[14:32:20] 📥 设备状态: A=70, B=50, MaxA=200, MaxB=200

命令> ca
[14:32:25] 📤 发送: clear-1
[14:32:25] 📥 设备状态: A=0, B=50, MaxA=200, MaxB=200
```

## 预期结果

### ✅ 成功标志

1. **桥接程序**：
   - 成功扫描到 BLE 设备
   - BLE 连接成功
   - WebSocket 连接成功
   - 显示二维码和 Client ID

2. **测试脚本**：
   - 连接到 WebSocket 服务器
   - 绑定成功
   - 能发送控制指令
   - 能接收设备状态更新

3. **设备响应**：
   - DG-LAB 设备实际输出强度变化
   - 桥接程序显示状态更新日志
   - 测试脚本显示设备反馈

### ❌ 常见问题

1. **BLE 设备未找到**
   - 确认设备已开机并处于可连接状态
   - 确认蓝牙适配器正常工作
   - 尝试重新扫描：`.\dglab.exe scan`

2. **WebSocket 连接失败**
   - 检查网络连接
   - 确认防火墙未阻止
   - 尝试使用备用服务器

3. **绑定失败**
   - 确认 Client ID 输入正确
   - 确认桥接程序仍在运行
   - 重启两端程序重试

4. **指令无响应**
   - 检查桥接程序是否显示 "已启动"
   - 查看桥接程序日志输出
   - 确认设备 BLE 连接未断开

## 调试技巧

### 查看详细日志

```powershell
# Windows 桥接程序
.\dglab.exe --debug bridge --device 47L121000 --verbose
```

### 手动测试 WebSocket 消息

可以使用浏览器控制台或 `wscat` 工具：

```bash
npm install -g wscat
wscat -c wss://ws.dungeon-lab.cn

# 连接后发送
{"type":"bind","clientId":"YOUR_ID","targetId":"TARGET_ID","message":"targetId"}
{"type":"msg","clientId":"YOUR_ID","targetId":"TARGET_ID","message":"strength-1+2+50"}
```

## 测试清单

- [ ] BLE 设备扫描成功
- [ ] BLE 设备连接成功
- [ ] WebSocket 服务器连接成功
- [ ] Client ID 获取成功
- [ ] 测试脚本连接成功
- [ ] 测试脚本绑定成功
- [ ] A通道增加指令生效
- [ ] A通道减少指令生效
- [ ] A通道设置指令生效
- [ ] B通道增加指令生效
- [ ] B通道减少指令生效
- [ ] B通道设置指令生效
- [ ] A通道清空指令生效
- [ ] B通道清空指令生效
- [ ] 设备状态实时同步
- [ ] 自动测试序列完成
- [ ] 断开连接正常

## 性能基准

- BLE 连接时间：< 5 秒
- WebSocket 连接时间：< 2 秒
- 指令响应延迟：< 100ms
- 状态同步延迟：< 200ms

## 下一步

测试通过后，可以：

1. 开发自己的第三方控制器（网页、游戏、脚本等）
2. 集成到现有应用
3. 添加 GUI 支持
4. 实现波形数据功能
5. 添加更多控制特性

## 技术支持

如遇到问题，请提供：

1. 完整的错误日志
2. 设备型号和名称
3. 操作系统版本
4. 复现步骤

在 GitHub Issues 中报告：https://github.com/YOUR_REPO/issues
