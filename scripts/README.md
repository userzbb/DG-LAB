# 构建管理脚本

本目录包含用于管理项目构建产物的实用脚本。

## 清理脚本

### 推荐：`clean-light.sh` - 轻量清理（日常使用）⭐

只清理不需要的构建产物，**保留 debug 构建缓存**以加速开发：
- 删除 `target/release/`（如果存在）
- 删除前端 `dist/`
- **保留** `target/debug/`（加速增量编译）
- **保留** `node_modules/`

**使用场景**：
- ✅ **日常开发推荐**：既节省空间又保持编译速度
- 不小心运行了 release 构建
- 清理临时文件

**使用方法**：
```bash
./scripts/clean-light.sh
```

**预期释放空间**：1-2 GB（如果有 release 构建）  
**保留空间**：5-8 GB（debug 构建缓存）

---

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

#### 开发模式（推荐）

**只运行 dev 模式测试**（cargo run / npm run tauri dev）：

```
target/debug/     → 5-8 GB  （保留，加速编译）
target/release/   → 不生成   （不做 release 构建）
```

✅ **推荐清理策略**：
- **日常**：运行 `./scripts/clean-light.sh`（清理临时文件）
- **每月或磁盘不足时**：运行 `./scripts/clean-build.sh`（完全清理后重新构建）
- **依赖有问题时**：运行 `./scripts/clean.sh`（包括 node_modules）

✅ **推荐保留的情况**：
- ✅ 正常开发中（保留 target/debug/ 加速增量编译）
- ✅ 频繁测试同一个项目
- ✅ 磁盘空间充足（超过 100GB 可用）

❌ **必须清理的情况**：
- 切换分支后出现编译错误
- 依赖更新后出现奇怪问题
- 磁盘空间不足（少于 20GB 可用）

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
- 所有平台的 **release 构建**都在云端完成
- 本地只需要运行 **dev 模式**测试（cargo run / npm run tauri dev）
- 本地不会生成 release 构建，节省 5-7 GB 空间

**推荐工作流程**：
```bash
# 1. 修改代码
vim crates/dglab-core/src/device/mod.rs

# 2. 本地测试（dev 模式）
cargo run --bin dglab -- scan
# 或
cd apps/dglab-gui-tauri && npm run tauri dev

# 3. 提交并推送
git add .
git commit -m "feat: add new feature"
git push

# 4. GitHub Actions 自动构建 release 版本（云端）
# 无需本地构建 release
```

**清理策略**：
- **日常**：保留 target/debug/（5-8 GB），不需要清理
- **偶尔**：运行 `./scripts/clean-light.sh` 清理临时文件
- **磁盘不足**：运行 `./scripts/clean-build.sh` 完全清理

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

## 最佳实践（Dev 模式开发）

### 日常开发流程

1. **正常开发**：保持 target/debug/（5-8 GB）不清理
   - 增量编译快速（10-30 秒）
   - 磁盘占用稳定，不会持续增长

2. **遇到编译问题**：运行 `./scripts/clean-build.sh` 重新构建

3. **磁盘空间不足**：运行 `./scripts/clean.sh` 完全清理

4. **切换分支**：如果遇到奇怪的编译错误，运行 `cargo clean`

### 空间占用预期

| 场景 | target/ 大小 | 说明 |
|------|-------------|------|
| 首次克隆项目 | 0 GB | 干净状态 |
| 第一次 cargo run | 5-8 GB | 完整 debug 构建 |
| 修改代码后 | 5-8 GB | 增量编译，大小不变 |
| 运行 release 构建 | 10-15 GB | **不推荐本地做** |

### 什么时候 target/ 会增长？

❌ **不会增长的情况**（大小稳定在 5-8 GB）：
- 正常开发和测试
- 修改 Rust 代码后增量编译
- 修改前端代码（不涉及 Rust）

✅ **会增长的情况**：
- 运行 `cargo build --release`（+1-2 GB）
- 运行 `npm run tauri build`（+1-2 GB）
- 切换多个不同的构建配置

**解决方案**：避免本地运行 release 构建，使用 GitHub Actions。

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
