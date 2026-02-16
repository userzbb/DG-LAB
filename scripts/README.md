# 开发脚本

此目录包含用于开发的辅助脚本。

## 可用脚本

### `install-hooks.sh`

安装 Git pre-commit hooks，确保提交的代码符合项目规范。

**使用方法**:
```bash
./scripts/install-hooks.sh
```

**功能**:
- 在提交前自动检查 Rust 代码格式 (rustfmt)
- 在提交前自动运行 Clippy 检查
- 在提交前自动检查编译
- 在提交前自动检查 TypeScript 类型

**跳过检查**（不推荐）:
```bash
git commit --no-verify
```

### `pre-commit`

Pre-commit hook 模板文件，由 `install-hooks.sh` 安装到 `.git/hooks/pre-commit`。

**手动安装**:
```bash
cp scripts/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

## 注意事项

- Git hooks 存储在 `.git/hooks/` 目录中，不会被 git 跟踪
- 每个克隆的仓库都需要单独安装 hooks
- 建议在首次克隆仓库后立即运行 `install-hooks.sh`
- Hook 检查失败时提交会被阻止，请修复问题后重新提交

## 开发建议

在开始开发前运行以下命令确保环境正确：

```bash
# 1. 安装 Git hooks
./scripts/install-hooks.sh

# 2. 安装前端依赖
cd apps/dglab-gui-tauri
npm install
cd ../..

# 3. 运行测试验证
cargo test --workspace

# 4. 检查格式和 Clippy
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```
