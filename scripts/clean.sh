#!/bin/bash
# 清理脚本 - 删除所有构建产物和依赖

set -e

echo "🧹 开始清理构建产物..."

# 清理 Rust 构建产物
if [ -d "target" ]; then
    echo "📦 清理 Rust target 目录..."
    cargo clean
    echo "✅ Rust 构建产物已清理"
else
    echo "ℹ️  target 目录不存在"
fi

# 清理 Node.js 依赖和构建产物
if [ -d "apps/dglab-gui-tauri" ]; then
    echo "📦 清理前端构建产物..."
    cd apps/dglab-gui-tauri
    
    if [ -d "node_modules" ]; then
        echo "  - 删除 node_modules..."
        rm -rf node_modules
    fi
    
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
fi

# 显示清理后的磁盘空间
echo ""
echo "📊 磁盘空间使用情况："
df -h . | tail -1

echo ""
echo "✨ 清理完成！"
