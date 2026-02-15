# WiFi WebSocket 协议

DG-LAB 设备通过 WebSocket 协议进行远程控制。

## 目录

- [连接方式](#连接方式)
- [消息格式](#消息格式)
- [指令类型](#指令类型)
- [错误码](#错误码)

## 连接方式

### 官方服务器

- URL: `wss://ws.dungeon-lab.cn/{clientId}`
- 也支持不加密的 `ws://` 连接

### 二维码格式

```
https://www.dungeon-lab.com/app-download.php#DGLAB-SOCKET#wss://ws.dungeon-lab.cn/xxxx-xxxx-xxxx
```

APP 扫描二维码后会解析出 WebSocket 地址并连接。

## 消息格式

所有消息均为 JSON 格式：

```json
{
  "type": "xxx",
  "clientId": "xxx",
  "targetId": "xxx",
  "message": "xxx"
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `type` | string | 指令类型 |
| `clientId` | string | 发送方客户端 ID |
| `targetId` | string | 接收方客户端 ID |
| `message` | string | 消息内容 |

## 指令类型

### heartbeat - 心跳包

保持连接活跃。建议每分钟发送一次。

```json
{
  "type": "heartbeat",
  "clientId": "xxx",
  "targetId": "",
  "message": ""
}
```

### bind - 关系绑定

建立两个客户端之间的绑定关系。

#### 1. Socket 服务返回 clientId

新连接建立时，服务端会发送：

```json
{
  "type": "bind",
  "clientId": "xxx",
  "targetId": "",
  "message": "targetId"
}
```

其中 `clientId` 是分配给当前连接的 ID，`message` 字段值为 `"targetId"` 表示需要获取目标 ID。

#### 2. 第三方终端显示二维码

第三方终端（如网页）获取到 clientId 后，生成二维码供 APP 扫描。

#### 3. APP 发送绑定请求

APP 扫描二维码后，发送绑定请求：

```json
{
  "type": "bind",
  "clientId": "app-id",
  "targetId": "terminal-id",
  "message": "DGLAB"
}
```

#### 4. 绑定成功

服务端返回绑定成功：

```json
{
  "type": "bind",
  "clientId": "terminal-id",
  "targetId": "app-id",
  "message": "200"
}
```

### msg - 数据指令

用于发送波形、强度变化、队列清空等数据。

#### 接收强度数据 (APP → 第三方终端)

APP 会定期发送当前强度信息：

```json
{
  "type": "msg",
  "clientId": "app-id",
  "targetId": "terminal-id",
  "message": "strength-11+7+100+35"
}
```

消息格式：`strength-{A}+{B}+{A上限}+{B上限}`

| 参数 | 说明 |
|------|------|
| A | A 通道当前强度 |
| B | B 通道当前强度 |
| A上限 | A 通道最大强度 |
| B上限 | B 通道最大强度 |

#### 强度操作 (第三方终端 → APP)

控制强度变化：

```json
{
  "type": "msg",
  "clientId": "terminal-id",
  "targetId": "app-id",
  "message": "strength-1+1+5"
}
```

消息格式：`strength-{通道}+{模式}+{数值}`

| 参数 | 值 | 说明 |
|------|-----|------|
| 通道 | 1/2 | 1=A通道, 2=B通道 |
| 模式 | 0/1/2 | 0=减少, 1=增加, 2=指定数值 |
| 数值 | 整数 | 变化量或目标值 |

示例：
- `strength-1+1+5` - A 通道强度 +5
- `strength-2+0+3` - B 通道强度 -3
- `strength-1+2+0` - A 通道强度归零

#### 波形操作 (第三方终端 → APP)

发送波形数据：

```json
{
  "type": "msg",
  "clientId": "terminal-id",
  "targetId": "app-id",
  "message": "pulse-A:[0101640101640101,0101640101640101]"
}
```

消息格式：`pulse-{通道}:[{波形数据数组}]`

| 参数 | 说明 |
|------|------|
| 通道 | A 或 B |
| 波形数据 | 8 字节 HEX 格式，每条代表 100ms |

限制：
- 单个数组最大长度：100（10 秒数据）
- APP 队列最大：500（50 秒数据）
- message 总长度不超过 1950 字符

波形数据格式（8 字节）：
```
[X, X, A, X, X, B, X, X]
```
- A: A 通道强度（0-100）
- B: B 通道强度（0-100）

#### 清空波形队列

清空指定通道的波形队列：

```json
{
  "type": "msg",
  "clientId": "terminal-id",
  "targetId": "app-id",
  "message": "clear-1"
}
```

消息格式：`clear-{通道}`

| 通道值 | 说明 |
|--------|------|
| 1 | 清空 A 通道 |
| 2 | 清空 B 通道 |

#### APP 反馈 (APP → 第三方终端)

APP 上的按钮被按下时发送反馈：

```json
{
  "type": "msg",
  "clientId": "app-id",
  "targetId": "terminal-id",
  "message": "feedback-0"
}
```

消息格式：`feedback-{index}`

| 按钮索引 | 说明 |
|----------|------|
| 0-4 | A 通道按钮 |
| 5-9 | B 通道按钮 |

### break - 连接断开

对方客户端断开连接时收到：

```json
{
  "type": "break",
  "clientId": "",
  "targetId": "xxx",
  "message": "209"
}
```

### error - 服务错误

服务端返回错误：

```json
{
  "type": "error",
  "clientId": "",
  "targetId": "xxx",
  "message": "{errorCode}"
}
```

## 错误码

| 错误码 | 说明 |
|--------|------|
| 200 | 成功 |
| 209 | 对方客户端已断开 |
| 210 | 二维码中没有有效的 clientID |
| 211 | 服务器迟迟不下发 app 端的 id |
| 400 | 此 id 已被其他客户端绑定 |
| 401 | 要绑定的目标客户端不存在 |
| 402 | 收信方和寄信方不是绑定关系 |
| 403 | 发送的内容不是标准 json 对象 |
| 404 | 未找到收信人（离线） |
| 405 | 下发的 message 长度大于 1950 |
| 500 | 服务器内部异常 |
