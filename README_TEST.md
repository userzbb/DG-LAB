# DG-LAB 桥接模式测试指南

## 快速开始 (推荐使用 uv)

### 1. 安装 uv

**Windows (PowerShell)**:
```powershell
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

**Linux/macOS**:
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### 2. 启动桥接程序

在 Windows 上编译并运行:
```cmd
cargo build --release
.\target\release\dglab.exe bridge --device 47L121000
```

或使用预编译的 exe:
```cmd
.\dglab.exe bridge --device 47L121000
```

**记下显示的 Client ID**（在二维码 URL 的 `#` 后面）

### 3. 运行测试脚本

**使用 uv (推荐，无需手动安装依赖)**:

Windows:
```cmd
test-bridge-uv.bat
```

或直接运行:
```cmd
uv run test-bridge.py
```

Linux/macOS:
```bash
uv run test-bridge.py
```

**传统方式 (需要手动安装依赖)**:

```bash
pip install pydglab-ws
python test-bridge.py
```

### 4. 输入 Client ID

测试脚本启动后，会提示输入目标 Client ID，将桥接程序显示的 Client ID 粘贴进去。

### 5. 开始测试

绑定成功后，你可以使用以下命令：

```
a+10      # A通道增加10
a-10      # A通道减少10
a=50      # A通道设置为50
b+10      # B通道增加10
b-10      # B通道减少10
b=50      # B通道设置为50
ca        # 清空A通道
cb        # 清空B通道
pulse     # 发送测试波形
auto      # 自动测试序列
quit      # 退出
```

## 什么是 uv？

[uv](https://github.com/astral-sh/uv) 是一个极快的 Python 包管理器和项目管理工具，由 Astral (Ruff 的开发者) 开发。

### 为什么使用 uv？

1. **无需手动安装依赖**: `uv run` 会自动创建虚拟环境并安装依赖
2. **极快的速度**: 比 pip 快 10-100 倍
3. **零配置**: 依赖信息直接写在脚本文件中 (PEP 723)
4. **跨平台**: Windows/Linux/macOS 统一体验

### uv 的工作原理

脚本文件中包含特殊注释块:
```python
# /// script
# requires-python = ">=3.9"
# dependencies = [
#     "pydglab-ws>=1.0.0",
# ]
# ///
```

运行 `uv run test-bridge.py` 时:
1. uv 读取依赖信息
2. 自动创建临时虚拟环境（缓存在 `~/.cache/uv/`）
3. 安装所需的包（如果未缓存）
4. 运行脚本

**首次运行**: 会下载安装依赖（几秒钟）  
**后续运行**: 直接从缓存运行（毫秒级启动）

## 故障排除

### Windows 上中文显示乱码

脚本已设置 UTF-8 编码，如果仍有问题：
1. 在 PowerShell 中运行: `chcp 65001`
2. 或在 cmd 中运行: `chcp 65001`
3. 使用 Windows Terminal（推荐）

### uv 命令找不到

**Windows**: 重启终端或运行 `refreshenv`（如果使用 Chocolatey）

**Linux/macOS**: 
```bash
source ~/.bashrc  # 或 ~/.zshrc
```

或手动添加到 PATH:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### 桥接程序无法连接 BLE

- 确保蓝牙已启用
- 确保设备名称正确（47L121000）
- 尝试关闭其他蓝牙应用
- 在设备管理器中检查蓝牙驱动

### WebSocket 连接失败

- 检查网络连接
- 尝试使用 `--server wss://ws.dungeon-lab.cn`
- 查看桥接程序的详细日志: `--verbose`

## 完整测试流程示例

```cmd
REM 1. 编译程序
cargo build --release

REM 2. 启动桥接（新终端窗口）
.\target\release\dglab.exe bridge --device 47L121000 --verbose

REM 输出示例:
REM ✓ 已连接到 WebSocket 服务器
REM ✓ Client ID: abc123def456
REM 
REM 扫描此二维码进行控制:
REM [二维码...]

REM 3. 运行测试（新终端窗口）
test-bridge-uv.bat

REM 4. 输入 Client ID
目标 Client ID> abc123def456

REM 5. 测试命令
命令> a=50
命令> b=50
命令> auto
命令> quit
```

## 进阶使用

### 自定义服务器

```bash
uv run test-bridge.py wss://your-server.com
```

### 修改测试脚本

测试脚本是普通 Python 文件，可以直接编辑 `test-bridge.py`：

- 修改自动测试序列: `run_auto_test()` 函数
- 添加新命令: `parse_and_send_command()` 函数
- 自定义波形: `run_pulse_test()` 函数

### 查看 PyDGLab-WS 文档

```bash
# 在浏览器中打开
start https://pydglab-ws.readthedocs.io
```

## 相关链接

- **uv 官网**: https://docs.astral.sh/uv/
- **PyDGLab-WS**: https://github.com/Ljzd-PRO/PyDGLab-WS
- **DG-LAB 官方文档**: https://github.com/DG-LAB-OPENSOURCE/DG-LAB-OPENSOURCE/tree/main/socket
