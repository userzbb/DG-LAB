@echo off
chcp 65001 >nul
REM DG-LAB 桥接模式测试脚本 (使用 uv)

echo ╔════════════════════════════════════════════════════════╗
echo ║       DG-LAB 桥接模式测试 (uv 版本)                    ║
echo ╚════════════════════════════════════════════════════════╝
echo.

REM 检查 uv
where uv >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ 错误: 未找到 uv
    echo.
    echo 请先安装 uv:
    echo   powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
    echo.
    echo 或访问: https://docs.astral.sh/uv/getting-started/installation/
    echo.
    pause
    exit /b 1
)

echo ✓ 找到 uv
echo.
echo 🚀 启动测试脚本...
echo.

REM 运行测试脚本
uv run test-bridge.py %*

if %errorlevel% neq 0 (
    echo.
    echo ❌ 脚本运行失败
    pause
    exit /b %errorlevel%
)
