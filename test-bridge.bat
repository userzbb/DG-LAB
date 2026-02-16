@echo off
REM DG-LAB 桥接模式快速测试脚本 (Windows)

echo ╔════════════════════════════════════════════════════════╗
echo ║       DG-LAB 桥接模式快速测试                          ║
echo ╚════════════════════════════════════════════════════════╝
echo.

REM 检查 Python
where python >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ 错误: 未找到 python
    echo 请先安装 Python 3.7+
    echo 下载地址: https://www.python.org/downloads/
    pause
    exit /b 1
)

REM 检查 pydglab-ws 库
python -c "import pydglab_ws" >nul 2>nul
if %errorlevel% neq 0 (
    echo 📦 安装依赖...
    pip install pydglab-ws
    echo.
)

REM 运行测试脚本
python test-bridge.py %*
