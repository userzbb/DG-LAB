#!/bin/bash
# 轻量清理 - 只清理 release 构建（如果有）

set -e

echo "🧹 轻量清理模式（保留 debug 构建加速开发）..."

# 只清理 release 构建
if [ -d "target/release" ]; then
    echo "📦 清理 release 构建..."
    cargo clean --release
    echo "✅ Release 构建已清理"
else
    echo "ℹ️  没有 release 构建需要清理"
fi

# 清理前端 dist
if [ -d "apps/dglab-gui-tauri/dist" ]; then
    echo "📦 清理前端 dist..."
    rm -rf apps/dglab-gui-tauri/dist
    echo "✅ 前端 dist 已清理"
fi

echo ""
echo "ℹ️  保留了 target/debug/ 以加速增量编译"
echo "ℹ️  如需完全清理，请运行 ./scripts/clean.sh"

echo ""
echo "📊 当前磁盘使用："
du -sh target 2>/dev/null || echo "target 目录不存在"
df -h . | tail -1

echo ""
echo "✨ 轻量清理完成！"
