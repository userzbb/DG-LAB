#!/bin/bash
# 清理脚本（保留依赖） - 只删除构建产物，保留 node_modules

set -e

echo "🧹 开始清理构建产物（保留依赖）..."

# 清理 Rust 构建产物
if [ -d "target" ]; then
    echo "📦 清理 Rust target 目录..."
    cargo clean
    echo "✅ Rust 构建产物已清理"
else
    echo "ℹ️  target 目录不存在"
fi

# 清理前端构建产物（保留 node_modules）
if [ -d "apps/dglab-gui-tauri" ]; then
    echo "📦 清理前端构建产物..."
    cd apps/dglab-gui-tauri
    
    if [ -d "dist" ]; then
        echo "  - 删除 dist..."
        rm -rf dist
    fi
    
    if [ -d "src-tauri/target" ]; then
        echo "  - 删除 src-tauri/target..."
        rm -rf src-tauri/target
    fi
    
    cd ../..
    echo "✅ 前端构建产物已清理"
    echo "ℹ️  node_modules 已保留"
fi

# 显示清理后的磁盘空间
echo ""
echo "📊 磁盘空间使用情况："
df -h . | tail -1

echo ""
echo "✨ 清理完成！"
