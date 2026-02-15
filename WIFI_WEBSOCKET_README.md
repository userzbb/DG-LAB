# DG-LAB WiFi / WebSocket 模式说明

## 两种连接模式

DG-LAB 设备支持两种控制方式：

### 1. BLE 模式（直连）

```
┌─────────────┐
│   你的程序   │
│  (PC/手机)  │
└──────┬──────┘
       │ 蓝牙
       │
┌──────▼──────┐
│ DG-LAB 硬件 │
│   设备      │
└─────────────┘
```

- 电脑/手机直接通过蓝牙连接设备
- 适用于近距离使用
- 延迟低，无需互联网

### 2. WiFi 模式（远程）

```
┌─────────────┐                    ┌─────────────┐
│   你的程序   │                    │ DG-LAB APP │
│  (PC/手机)  │                    │   (手机)    │
└──────┬──────┘                    └──────┬──────┘
       │                                   │
       │ WebSocket                         │ 蓝牙
       │                                   │
┌──────▼──────┐                    ┌──────▼──────┐
│   服务器     │◄──────────────────►│ DG-LAB 硬件 │
│ (官方/自建)  │  WebSocket         │   设备      │
└─────────────┘                    └─────────────┘
```

- 通过 WebSocket 连接服务器
- 手机上的 DG-LAB APP 也连接同一服务器
- APP 通过蓝牙控制实际设备
- 支持跨互联网远程控制

## 为什么叫 "WiFi" 而不是 "WebSocket"？

虽然技术上是 WebSocket 通信，但在 DG-LAB 官方文档和社区中：

- **BLE** = 直连蓝牙模式
- **WiFi** = 通过 APP + 互联网的远程模式

这是从用户视角的命名，强调"需要网络"而不是具体的技术实现。

---

## 自建本地服务器

当前代码已支持连接任意 WebSocket 服务器：

```rust
// 连接官方服务器
let client = WsClient::connect_official().await?;

// 连接本地服务器
let client = WsClient::connect("ws://localhost:8080").await?;

// 连接局域网内其他机器
let client = WsClient::connect("ws://192.168.1.100:8080").await?;
```

### 为什么要自建服务器？

| 原因 | 说明 |
|------|------|
| **隐私** | 数据不经过第三方服务器 |
| **延迟** | 局域网内延迟更低 |
| **扩展** | 可以自由添加功能 |
| **离线** | 没有互联网也能用 |

---

## 扩展玩法 ideas

### 1. 多人协作控制

```
控制器 A ──┐
            │
控制器 B ──┼──► 本地服务器 ──► APP ──► 设备
            │
控制器 C ──┘
```

- 多个控制器可以同时连接
- 可以投票、轮流控制
- 适合派对/互动场景

### 2. 语音控制

```
智能音箱/麦克风
       │
       │ 语音识别
       │
   本地服务器
       │
       └─► APP ──► 设备
```

支持的命令：
- "强度增加 10"
- "通道 A 设置为 50"
- "开始" / "停止"
- "切换到柔和模式"

### 3. 游戏联动

```
游戏 (例如 CS:GO, Minecraft)
       │
       │ 游戏事件 API / 日志分析
       │
   本地服务器
       │
       └─► APP ──► 设备
```

触发场景：
- 受到攻击时强度增加
- 完成成就时触发脉冲
- 死亡时归零
- 根据游戏角色状态自动调整

### 4. 直播互动

```
直播弹幕 / 打赏
       │
       │ 弹幕姬 / 直播平台 API
       │
   本地服务器
       │
       └─► APP ──► 设备
```

互动方式：
- 观众发送特定弹幕触发
- 礼物打赏对应强度等级
- 点赞累计触发特殊波形
- PK 胜负触发反馈

### 5. 传感器联动

```
各种传感器:
- 心率监测
- 运动传感器
- 麦克风音量
- 键盘敲击
- 鼠标移动
       │
       │
   本地服务器
       │
       └─► APP ──► 设备
```

示例场景：
- 心率超过阈值时降低强度
- 根据呼吸节奏调整波形频率
- 麦克风检测到声音时触发
- 键盘/鼠标操作越激烈强度越高

### 6. AI 智能控制

```
用户语音/文字描述
       │
       │ LLM (例如 GPT-4, Claude)
       │
   本地服务器
       │
       └─► APP ──► 设备
```

自然语言控制：
- "来个 10 分钟的渐进模式，从 20 慢慢升到 80"
- "切换到呼吸波形，频率 0.5Hz"
- "随机波动，范围 30-60"

### 7. 音乐可视化

```
音乐播放器 / 麦克风
       │
       │ FFT 频谱分析
       │
   本地服务器
       │
       ├─► 低频 → 通道 A
       │
       └─► 高频 → 通道 B
```

- 低音鼓点对应强脉冲
- 旋律对应强度变化
- 不同频段映射到不同通道

### 8. 时间脚本引擎

```
TOML/YAML 脚本文件
       │
       │ 脚本解析执行
       │
   本地服务器
       │
       └─► APP ──► 设备
```

脚本示例 (TOML):

```toml
name = "晚间放松"
description = "30 分钟渐进放松"

[[steps]]
action = "SetPower"
channel = "A"
power = 20

[[steps]]
action = "Wait"
duration_ms = 60000

[[steps]]
action = "FadePower"
channel = "A"
start_power = 20
end_power = 50
duration_ms = 300000

[[steps]]
action = "Wait"
duration_ms = 600000
```

---

## 协议说明

### 消息格式 (JSON)

```json
{
  "type": "bind",
  "clientId": "client-123",
  "targetId": "target-456",
  "message": "..."
}
```

### 消息类型

| type | 说明 |
|------|------|
| `heartbeat` | 心跳包 |
| `bind` | 绑定 / 客户端 ID |
| `msg` | 数据指令 |
| `break` | 断开连接 |
| `error` | 错误 |

### 常用指令格式

| 指令 | 格式 | 示例 |
|------|------|------|
| 强度设置 | `strength-{channel}+{mode}+{value}` | `strength-1+2+50` |
| 波形数据 | `pulse-{channel}:[{pulse1},{pulse2}...]` | `pulse-A:[01011e01011e0101]` |
| 清空队列 | `clear-{channel}` | `clear-1` |
| APP 反馈 | `feedback-{index}` | `feedback-0` |

**channel 说明:**
- `1` = 通道 A
- `2` = 通道 B

**mode 说明:**
- `0` = 减少
- `1` = 增加
- `2` = 设置

---

## 本地服务器实现示例 (Python)

这是一个最小化的本地服务器示例：

```python
import asyncio
import json
import websockets

connected_clients = {}

async def handle_client(websocket):
    client_id = None
    try:
        async for message in websocket:
            try:
                data = json.loads(message)
                msg_type = data.get("type")
                sender_id = data.get("clientId")
                target_id = data.get("targetId")
                payload = data.get("message", "")

                if msg_type == "bind":
                    if payload == "targetId":
                        # 分配 clientId
                        client_id = f"client-{id(websocket)}"
                        connected_clients[client_id] = websocket
                        response = {
                            "type": "bind",
                            "clientId": client_id,
                            "targetId": "targetId",
                            "message": client_id
                        }
                        await websocket.send(json.dumps(response))
                    else:
                        # 绑定目标
                        if target_id in connected_clients:
                            client_id = sender_id
                            connected_clients[client_id] = websocket
                            # 通知双方绑定成功
                            response = {
                                "type": "bind",
                                "clientId": sender_id,
                                "targetId": target_id,
                                "message": "200"
                            }
                            await websocket.send(json.dumps(response))
                            if target_id in connected_clients:
                                await connected_clients[target_id].send(json.dumps(response))

                elif msg_type == "msg" or msg_type == "heartbeat":
                    # 转发消息到目标
                    if target_id in connected_clients:
                        await connected_clients[target_id].send(message)

                elif msg_type == "break":
                    # 断开通知
                    if target_id in connected_clients:
                        await connected_clients[target_id].send(message)

            except json.JSONDecodeError:
                pass

    finally:
        if client_id and client_id in connected_clients:
            del connected_clients[client_id]

async def main():
    server = await websockets.serve(handle_client, "localhost", 8080)
    print("Local server running on ws://localhost:8080")
    await server.wait_closed()

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 相关文件位置

| 文件 | 说明 |
|------|------|
| `crates/dglab-protocol/src/wifi/mod.rs` | WiFi/WebSocket 协议定义 |
| `crates/dglab-protocol/src/wifi/client.rs` | WebSocket 客户端实现 |
