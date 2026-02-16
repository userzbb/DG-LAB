# 构建管理脚本

本目录包含用于管理项目构建产物的实用脚本。

## 清理脚本

### `clean.sh` - 完全清理

删除所有构建产物和依赖：
- Rust `target/` 目录
- Node.js `node_modules/` 目录
- 前端 `dist/` 目录
- Tauri 构建产物

**使用场景**：
- 需要完全重新开始
- 磁盘空间不足
- 依赖有问题需要重新安装

**使用方法**：
```bash
./scripts/clean.sh
```

**预期释放空间**：10-15 GB

---

### `clean-build.sh` - 清理构建产物

只删除构建产物，保留依赖：
- Rust `target/` 目录
- 前端 `dist/` 目录
- Tauri 构建产物
- **保留** `node_modules/`（约 200MB）

**使用场景**：
- 日常开发清理
- 释放磁盘空间但不想重新下载依赖
- 切换构建配置

**使用方法**：
```bash
./scripts/clean-build.sh
```

**预期释放空间**：10-15 GB

---

## 为什么需要清理？

### Rust `target/` 目录会变得很大

- **Debug 构建**：约 8-12 GB
  - 包含完整的调试符号
  - 每次 `cargo build` 或 `cargo run` 都会生成
  
- **Release 构建**：约 1-2 GB
  - 优化后的二进制
  - `cargo build --release` 生成
  
- **增量编译缓存**：2-3 GB
  - 加速重新编译
  - 可以安全删除

**总计**：10-15 GB 是常见大小

### 何时清理？

✅ **推荐清理的情况**：
- 磁盘空间不足
- 切换分支后出现编译错误
- 依赖更新后出现奇怪问题
- 准备提交代码前

❌ **不需要清理的情况**：
- 正常开发中（增量编译会更快）
- 频繁测试同一个项目

---

## 手动清理命令

如果不想用脚本，也可以手动执行：

```bash
# 清理所有 Rust 构建产物
cargo clean

# 只清理 release 构建
cargo clean --release

# 只清理 debug 构建
cargo clean --debug

# 清理前端
cd apps/dglab-gui-tauri
rm -rf dist node_modules
npm ci  # 重新安装依赖
```

---

## CI/CD 说明

本项目已配置 GitHub Actions CI/CD：
- 所有平台的构建都在云端完成
- 本地不需要保留构建产物
- 推荐定期运行 `./scripts/clean-build.sh`

---

## 磁盘空间监控

查看当前磁盘使用：
```bash
# 查看 target 目录大小
du -sh target

# 查看各子目录大小
du -h target --max-depth=1 | sort -hr

# 查看整个项目大小
du -sh .

# 查看磁盘剩余空间
df -h .
```

---

## 最佳实践

1. **日常开发**：不需要清理，保持增量编译缓存
2. **切换分支前**：运行 `cargo clean` 避免编译缓存问题
3. **每周或磁盘不足时**：运行 `./scripts/clean-build.sh`
4. **依赖有问题时**：运行 `./scripts/clean.sh` 完全重置
5. **提交代码前**：确保 `.gitignore` 正确，不要提交 `target/`

---

## 自动化

可以添加到 git hooks 或定时任务：

```bash
# 添加到 .git/hooks/post-checkout (切换分支后自动清理)
#!/bin/bash
./scripts/clean-build.sh

# 或添加到 crontab (每周日凌晨清理)
0 3 * * 0 cd /path/to/DG_LAB && ./scripts/clean-build.sh
```
