#!/bin/bash
# Git Hooks 安装脚本
# 用于安装项目的 pre-commit hook

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
HOOKS_DIR="$PROJECT_ROOT/.git/hooks"

echo "🔧 安装 Git Hooks..."

# 检查是否在 git 仓库中
if [ ! -d "$PROJECT_ROOT/.git" ]; then
    echo "❌ 错误：当前不在 git 仓库中"
    exit 1
fi

# 安装 pre-commit hook
if [ -f "$HOOKS_DIR/pre-commit" ]; then
    echo "⚠️  pre-commit hook 已存在"
    read -p "是否覆盖? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ 取消安装"
        exit 1
    fi
fi

# 复制 hook
cp "$SCRIPT_DIR/pre-commit" "$HOOKS_DIR/pre-commit"
chmod +x "$HOOKS_DIR/pre-commit"

echo "✅ pre-commit hook 安装成功！"
echo ""
echo "📋 Hook 功能："
echo "  - 自动检查 Rust 代码格式 (rustfmt)"
echo "  - 自动运行 Clippy 检查"
echo "  - 自动检查编译"
echo "  - 自动检查 TypeScript 类型"
echo ""
echo "💡 提示："
echo "  - 如需跳过检查，使用: git commit --no-verify"
echo "  - 手动运行格式化: cargo fmt --all"
echo "  - 手动运行 Clippy: cargo clippy --all-targets --all-features -- -D warnings"
