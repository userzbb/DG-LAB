# DG-LAB 基础功能测试计划

## 目标
验证通过 BLE 控制设备的基础功能是否正常工作

## 测试环境
- 设备：DG-LAB Coyote 3.0 (47L121000)
- 系统：Linux
- 工具：dglab CLI

## 测试步骤

### 1. 扫描设备

```bash
./target/debug/dglab scan
```

**预期结果**：
- 能找到 47L121000 设备
- 显示设备信息（ID、名称、信号强度）

### 2. 连接设备

```bash
./target/debug/dglab connect 47L121000
```

**预期结果**：
- 成功连接设备
- 显示连接成功消息
- 设备进入已连接状态

### 3. 控制设备强度

```bash
# 设置 A 通道强度为 10
./target/debug/dglab control --a 10

# 设置 B 通道强度为 15
./target/debug/dglab control --b 15

# 同时设置两个通道为 20
./target/debug/dglab control --power 20
```

**预期结果**：
- 设备实际强度变化
- 命令执行成功

### 4. 启动/停止输出

```bash
# 启动输出
./target/debug/dglab control --start

# 停止输出
./target/debug/dglab control --stop
```

**预期结果**：
- 设备开始/停止输出
- 能感觉到电流变化

### 5. 查看设备状态

```bash
./target/debug/dglab control --status
```

**预期结果**：
- 显示当前设备状态
- 显示通道强度、设备信息等

## 已知问题

### 问题 1：control 命令参数冲突
- **状态**：✅ 已修复
- **描述**：`--start` 和 `--stop` 都使用了 `-s` 短选项
- **修复**：移除短选项，只保留长选项

### 问题 2：[待测试]
- 需要实际设备测试

## 下一步改进

1. **如果基础功能正常**：
   - 实现 TUI 界面，提供更好的交互体验
   - 添加预设功能
   - 实现波形控制

2. **如果基础功能有问题**：
   - 调试 BLE 连接逻辑
   - 修复协议实现
   - 增加错误处理和日志
