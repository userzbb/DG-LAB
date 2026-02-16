# DG-Lab Coyote Game Hub 深度分析报告

## 项目概览

DG-Lab Coyote Game Hub 是一个基于 Node.js + TypeScript 的郊狼游戏控制器，用于通过 WebSocket 协议控制 DG-Lab 设备。项目采用前后端分离架构，服务器端提供 WebSocket 和 HTTP API，前端提供可视化控制面板。

**技术栈**:
- 后端: Node.js + TypeScript + Koa + WebSocket (ws)
- 前端: Vue.js
- 数据库: TypeORM (SQLite)
- 配置管理: @hyperzlib/node-reactive-config

---

## 1. WebSocket 协议实现

### 1.1 消息类型和格式

#### 消息类型枚举 (MessageType)

```typescript
export enum MessageType {
    HEARTBEAT = "heartbeat",  // 心跳消息
    BIND = "bind",            // 绑定消息
    MSG = "msg",              // 普通消息（包含控制指令和反馈）
    BREAK = "break",          // 断开连接
    ERROR = "error",          // 错误消息
}
```

#### 消息数据头 (MessageDataHead)

```typescript
export enum MessageDataHead {
    TARGET_ID = "targetId",   // 目标ID
    DG_LAB = "DGLAB",         // DG-Lab 标识
    STRENGTH = "strength",    // 强度控制
    PULSE = "pulse",          // 波形数据
    CLEAR = "clear",          // 清除波形
    FEEDBACK = "feedback",    // 按钮反馈
}
```

#### 消息格式

所有 WebSocket 消息都遵循统一的 JSON 格式：

```typescript
export type DGLabMessage = {
    type: MessageType | string,  // 消息类型
    clientId: string,            // 客户端ID
    targetId: string,            // 目标ID (设备ID)
    message: string,             // 消息内容
}
```

**示例消息**:

```json
{
    "type": "bind",
    "clientId": "3ab0773d-69d0-41af-b74b-9c6ce6507f65",
    "targetId": "DGLAB-12345",
    "message": "targetId"
}
```

### 1.2 客户端连接管理

#### DGLabWSManager - WebSocket 连接管理器

`DGLabWSManager` 是单例模式，负责管理所有 DG-Lab 客户端连接：

**核心职责**:
1. WebSocket 连接接受和验证
2. 客户端 ID 唯一性检查
3. 客户端实例创建和生命周期管理
4. 连接数限制（单机模式最多 10 个客户端）

**关键代码**:

```typescript
async handleWebSocket(rawWs: WebSocket, req: IncomingMessage, routeParams: Record<string, string>): Promise<void> {
    const clientId = routeParams.id;
    const ws = wrapAsyncWebSocket(rawWs);

    // 1. 验证 clientId
    if (!clientId) {
        await ws.sendAsync(JSON.stringify({
            type: 'error',
            clientId: '',
            targetId: '',
            message: RetCode.INVALID_CLIENT_ID,
        }));
        ws.close();
        return;
    }

    // 2. 检查 ID 是否已被占用
    if (this.clientIdToClient.has(clientId)) {
        await ws.sendAsync(JSON.stringify({
            type: 'error',
            clientId: clientId,
            targetId: '',
            message: RetCode.ID_ALREADY_BOUND,
        }));
        ws.close();
        return;
    }

    // 3. 创建客户端实例
    const client = new DGLabWSClient(this.ctx, ws, clientId);
    await client.initialize();
    
    // 4. 触发客户端连接事件
    this.events.emit('clientConnected', client);

    // 5. 注册客户端
    this.clientIdToClient.set(clientId, client);

    // 6. 监听断开事件
    client.once('close', async () => {
        this.clientIdToClient.delete(client.clientId);
    });
}
```

#### WebSocketAsync 包装器

为了简化异步操作，项目实现了 `WebSocketAsync` 包装器，将原生的回调式 API 转换为 Promise 风格：

```typescript
export const wrapAsyncWebSocket = (ws: WebSocket): AsyncWebSocket => {
    const asyncWs = ws as WebSocket & WebSocketAsyncExtension;

    asyncWs.sendAsync = (...args: any[]) => {
        return new Promise((resolve, reject) => {
            const cb = (error?: Error) => {
                if (error) reject(error);
                else resolve();
            };
            
            if (args.length === 1) {
                ws.send(args[0], cb);
            } else if (args.length === 2) {
                ws.send(args[0], args[1], cb);
            } else {
                reject(new Error('Invalid arguments'));
            }
        });
    };

    return asyncWs;
};
```

### 1.3 绑定流程实现

#### DGLabWSClient - 客户端实例

每个连接的 DG-Lab APP 对应一个 `DGLabWSClient` 实例。

**绑定流程**:

```typescript
public async initialize(): Promise<void> {
    this.bindEvents();  // 绑定 WebSocket 事件

    console.log(`Client connected: ${this.clientId}`);
    
    // 1. 发送绑定请求
    await this.send(MessageType.BIND, MessageDataHead.TARGET_ID);

    // 2. 等待绑定成功（带超时）
    const start_time = Date.now();
    while (!this.targetId) {
        await asleep(500);
        if (Date.now() - start_time > HEARTBEAT_TIMEOUT * 1000) {
            await this.send(MessageType.BREAK, RetCode.SERVER_DELAY);
            this.socket.close();
            throw new Error("Bind timeout");
        }
    }

    // 3. 重置设备状态（清除波形）
    await this.reset();
    await new Promise((resolve) => setTimeout(resolve, 500));

    // 4. 启动心跳任务
    this.heartbeatTask = setInterval(
        () => this.runHeartbeatTask(),
        HEARTBEAT_INTERVAL * 1000
    );

    // 5. 加载波形列表
    const pulseService = DGLabPulseService.instance;
    this.pulseList = pulseService.pulseList;
}
```

**绑定消息处理**:

```typescript
private async handleMessage(message: DGLabMessage): Promise<void> {
    if (message.type === MessageType.BIND) {
        if (message.message === MessageDataHead.DG_LAB) {
            // 绑定成功
            this.targetId = message.targetId;
            console.log(`Bind success: ${this.clientId} -> ${this.targetId}`);
            await this.send(MessageType.BIND, RetCode.SUCCESS);
        } else {
            console.log(`Bind failed: ${message.message}`);
        }
    }
    // ... 其他消息处理
}
```

### 1.4 心跳机制实现

**心跳参数**:
```typescript
const HEARTBEAT_INTERVAL = 20.0;  // 心跳间隔 20 秒
const HEARTBEAT_TIMEOUT = 20.0;   // 心跳超时 20 秒
```

**心跳发送**:

```typescript
public async runHeartbeatTask(): Promise<void> {
    await this.send(MessageType.HEARTBEAT, RetCode.SUCCESS);
}
```

心跳任务在 `initialize()` 中启动：

```typescript
this.heartbeatTask = setInterval(
    () => this.runHeartbeatTask(),
    HEARTBEAT_INTERVAL * 1000
);
```

**心跳响应处理**:

```typescript
if (message.type === MessageType.HEARTBEAT) {
    if (message.message === MessageDataHead.DG_LAB) {
        // console.log(`Heartbeat success`);
    } else {
        console.log(`Heartbeat failed: ${message.message}`);
    }
}
```

---

## 2. 错误码体系

### 2.1 RetCode 定义

```typescript
export enum RetCode {
    SUCCESS = "200",                      // 成功
    CLIENT_DISCONNECTED = "209",          // 客户端断开连接
    INVALID_CLIENT_ID = "210",            // 无效的客户端ID
    SERVER_DELAY = "211",                 // 服务器延迟（超时）
    ID_ALREADY_BOUND = "400",             // ID 已被绑定
    TARGET_CLIENT_NOT_FOUND = "401",      // 目标客户端未找到
    INCOMPATIBLE_RELATIONSHIP = "402",    // 不兼容的关系
    NON_JSON_CONTENT = "403",             // 非 JSON 内容
    RECIPIENT_NOT_FOUND = "404",          // 接收者未找到
    MESSAGE_TOO_LONG = "405",             // 消息过长
    SERVER_INTERNAL_ERROR = "500",        // 服务器内部错误
}
```

### 2.2 错误处理流程

**连接阶段错误**:

```typescript
// 无效的 clientId
if (!clientId) {
    await ws.sendAsync(JSON.stringify({
        type: 'error',
        clientId: '',
        targetId: '',
        message: RetCode.INVALID_CLIENT_ID,
    }));
    ws.close();
    return;
}

// ID 已被占用
if (this.clientIdToClient.has(clientId)) {
    await ws.sendAsync(JSON.stringify({
        type: 'error',
        clientId: clientId,
        targetId: '',
        message: RetCode.ID_ALREADY_BOUND,
    }));
    ws.close();
    return;
}
```

**绑定超时错误**:

```typescript
if (Date.now() - start_time > HEARTBEAT_TIMEOUT * 1000) {
    await this.send(MessageType.BREAK, RetCode.SERVER_DELAY);
    this.socket.close();
    throw new Error("Bind timeout");
}
```

**消息发送错误**:

```typescript
try {
    await this.socket.sendAsync(jsonStr);
    return true;
} catch (error: unknown) {
    if (error instanceof Error) {
        if (error.message === "WebSocket is not open: readyState 3 (CLOSED)") {
            this.close();
            return false;
        }
    }
    console.error("Failed to send message:", error);
    this.close();
    return false;
}
```

---

## 3. 状态管理

### 3.1 客户端状态机

#### 强度状态 (StrengthInfo)

```typescript
export interface StrengthInfo {
    strength: number;  // 当前强度
    limit: number;     // 强度上限
};
```

每个客户端维护两个通道的强度状态：

```typescript
public strength: StrengthInfo = { strength: 0, limit: 0 };        // A 通道
public strengthChannelB: StrengthInfo = { strength: 0, limit: 0 }; // B 通道
```

#### 客户端激活状态

```typescript
public active: boolean = true;  // 客户端是否活跃
private closed = false;         // 客户端是否已关闭
```

### 3.2 绑定状态管理

#### 绑定状态字段

```typescript
public clientId: string = '';   // 服务器分配的客户端ID
public targetId: string = '';   // DG-Lab 设备返回的设备ID
```

**状态转换**:

1. **未绑定**: `targetId` 为空字符串
2. **绑定中**: 发送 BIND 消息，等待响应
3. **已绑定**: `targetId` 被设置为设备 ID
4. **绑定失败**: 连接关闭或超时

### 3.3 超时处理

#### 绑定超时

```typescript
const start_time = Date.now();
while (!this.targetId) {
    await asleep(500);
    if (Date.now() - start_time > HEARTBEAT_TIMEOUT * 1000) {
        await this.send(MessageType.BREAK, RetCode.SERVER_DELAY);
        this.socket.close();
        throw new Error("Bind timeout");
    }
}
```

#### 通用异步超时工具

```typescript
export async function asleep(time: number, ab?: AbortController): Promise<boolean> {
    return new Promise((resolve) => {
        const timeout = setTimeout(() => {
            resolve(true);
        }, time);

        if (ab) {
            ab.signal.addEventListener('abort', () => {
                clearTimeout(timeout);
                resolve(false);
            });
        }
    });
}
```

---

## 4. 强度控制

### 4.1 强度数据格式

#### 强度设置消息格式

```typescript
// 格式: "strength-<channel>+<operation>+<value>"
// channel: 1 = A通道, 2 = B通道
// operation: 0 = 减少, 1 = 增加, 2 = 设置为
// value: 强度值 (0-200)

await this.send(MessageType.MSG, `strength-${channel}+2+${strength}`);
```

**示例**:
- `strength-1+2+50`: 设置 A 通道强度为 50
- `strength-2+2+30`: 设置 B 通道强度为 30

#### 强度变化上报格式

```typescript
// 格式: "strength-<A强度>+<B强度>+<A上限>+<B上限>"
// 示例: "strength-20+15+100+100"

private async handleMsgStrengthChanged(message: string): Promise<void> {
    const strengthData = message.split("-")[1].split("+");

    this.strength = {
        strength: parseInt(strengthData[0]),  // A通道当前强度
        limit: parseInt(strengthData[2]),     // A通道上限
    };
    this.strengthChannelB = {
        strength: parseInt(strengthData[1]),  // B通道当前强度
        limit: parseInt(strengthData[3]),     // B通道上限
    };

    this.events.emit("strengthChanged", this.strength, this.strengthChannelB);
}
```

### 4.2 强度上限管理

#### 强度验证

```typescript
public async setStrength(channel: Channel, strength: number): Promise<boolean> {
    if (channel === Channel.A) {
        if (strength > this.strength.limit) {
            throw new Error("Strength out of limit");
        }
    } else if (channel === Channel.B) {
        if (strength > this.strengthChannelB.limit) {
            throw new Error("Strength out of limit");
        }
    }

    let ret = await this.send(MessageType.MSG, `${MessageDataHead.STRENGTH}-${channel}+2+${strength}`);
    this.events.emit("setStrength", channel, strength);

    return ret;
}
```

#### 游戏控制器中的强度上限

```typescript
// 游戏配置中的强度配置
export type GameStrengthConfig = {
    strength: number;         // 基础强度
    randomStrength: number;   // 随机强度范围
};

// 计算目标强度时考虑上限
let targetStrength = this.strengthConfig.strength + randomInt(0, this.strengthConfig.randomStrength);
targetStrength = Math.min(targetStrength, this.clientStrength.limit);
```

### 4.3 强度同步逻辑

#### 渐进式强度调整

```typescript
// 当目标强度大于当前强度时，逐步增加
if (targetStrength > currentStrength) {
    let setStrengthInterval = setInterval(() => {
        if (ab.signal.aborted) {
            clearInterval(setStrengthInterval);
            return;
        }

        this.setClientStrength(currentStrength).catch((error) => {
            console.error('Failed to set strength:', error);
        });

        if (currentStrength >= targetStrength) {
            clearInterval(setStrengthInterval);
        }

        currentStrength = Math.min(currentStrength + 2, targetStrength);
    }, 200);  // 每 200ms 增加 2 点强度
}
```

#### B 通道强度同步

```typescript
await this.client.setStrength(Channel.A, strength);
if (this.gameConfig.enableBChannel) {
    // B 通道强度 = A 通道强度 × 倍率，但不超过上限
    let bStrength = Math.min(
        strength * this.gameConfig.bChannelStrengthMultiplier, 
        this.clientStrength.limit
    );
    await this.client.setStrength(Channel.B, bStrength);
}
```

---

## 5. 架构设计

### 5.1 DGLabWSClient 实现

#### 职责分层

1. **连接管理层**: 处理 WebSocket 连接、断开、重连
2. **协议层**: 处理消息的序列化、反序列化、类型判断
3. **业务层**: 处理绑定、心跳、强度控制、波形输出

#### 事件驱动架构

```typescript
export interface DGLabWSEvents {
    strengthChanged: [strength: StrengthInfo, strength_b: StrengthInfo];  // 强度变化
    setStrength: [channel: Channel, strength: number];                    // 设置强度
    sendPulse: [channel: Channel, pulse: string[]];                       // 发送波形
    clearPulse: [channel: Channel];                                       // 清除波形
    feedback: [button: FeedbackButton];                                   // 按钮反馈
    close: [];                                                            // 连接关闭
};
```

#### 核心方法

**波形输出**:

```typescript
public async outputPulse(pulseId: string, time: number, options: OutputPulseOptions = {}) {
    let totalDuration = 0;
    const pulseService = DGLabPulseService.instance;
    const currentPulseInfo = pulseService.getPulse(pulseId, options.customPulseList) 
                             ?? pulseService.getDefaultPulse();

    let startTime = Date.now();
    let harvest = () => {};
    if (options.abortController) {
        harvest = createHarvest(options.abortController);
    }

    // 预发送多个波形数据包，确保不间断
    for (let i = 0; i < 50; i++) {
        let [pulseData, pulseDuration] = pulseService.getPulseHexData(currentPulseInfo);

        await this.sendPulse(Channel.A, pulseData);
        if (options.bChannel) {
            await this.sendPulse(Channel.B, pulseData);
        }

        harvest();  // 检查是否被中断

        totalDuration += pulseDuration;
        if (totalDuration > time) {
            break;
        }
    }

    await asleep(time, options.abortController);
    harvest();
    
    options.onTimeEnd?.();  // 时间结束回调

    // 等待剩余时间
    if (totalDuration > time) {
        const waitTime = totalDuration - time - onTimeEndDuration - netDuration;
        await asleep(waitTime, options.abortController);
    }
}
```

**波形消息格式**:

```typescript
private async sendPulse(channel: Channel, pulse: string[]): Promise<boolean> {
    const pulse_str = JSON.stringify(pulse);
    const channel_id = channel === Channel.A ? "A" : "B";

    // 格式: "pulse-A:[\"hex1\",\"hex2\",...]"
    let ret = await this.send(MessageType.MSG, `${MessageDataHead.PULSE}-${channel_id}:${pulse_str}`);

    this.events.emit("sendPulse", channel, pulse);
    return ret;
}
```

### 5.2 消息处理流程

#### WebSocket 路由

```typescript
export class WebSocketRouter {
    public routes: WebSocketRouteInfo[];

    public get(url: string, callback: WebSocketRouterCallback) {
        const regexp = pathToRegexp(url);
        this.routes.push({ url, regexp, callback });
    }

    public match(url: string): WebSocketRouteMatchResult | null {
        for (const route of this.routes) {
            if (route.regexp.test(url)) {
                // 提取路由参数
                let routeParams: Record<string, string> = {};
                const keys = route.regexp.keys;
                const matches = route.regexp.exec(url);
                if (matches) {
                    for (let i = 1; i < matches.length; i++) {
                        routeParams[keys[i - 1].name] = matches[i];
                    }
                }
                return { route, params: routeParams };
            }
        }
        return null;
    }
}
```

#### 消息分发

```typescript
socketEvents.on("message", async (data, isBinary) => {
    if (isBinary) {
        return; // 忽略二进制数据
    }

    const message = JSON.parse(data.toString());

    if (!message.type) {
        console.log("Invalid message: " + data.toString());
        return;
    }

    await this.handleMessage(message);
});
```

### 5.3 事件分发机制

#### EventStore - 事件监听器管理

```typescript
export class EventStore {
    private listeners: Map<EventEmitter, Map<string, Function[]>> = new Map();

    wrap<T extends EventEmitter>(emitter: T): T {
        // 拦截 on 方法，记录监听器
        const originalOn = emitter.on.bind(emitter);
        emitter.on = (event: string, listener: Function) => {
            this.recordListener(emitter, event, listener);
            return originalOn(event, listener);
        };
        return emitter;
    }

    removeAllListeners() {
        for (const [emitter, eventMap] of this.listeners) {
            for (const [event, listeners] of eventMap) {
                for (const listener of listeners) {
                    emitter.off(event, listener);
                }
            }
        }
        this.listeners.clear();
    }
}
```

**用途**: 自动管理事件监听器的生命周期，防止内存泄漏。

#### ExEventEmitter - 增强型事件发射器

支持带命名空间的事件，可以针对特定 ID 触发事件：

```typescript
gameConfigEvents.on('configUpdated', this.clientId, throttle((newConfig) => {
    this.handleConfigUpdated(newConfig);
}, 100));
```

---

## 6. 游戏控制逻辑

### 6.1 CoyoteGameController

#### 游戏循环 (Game Loop)

```typescript
private async runGameTask(ab: AbortController, harvest: () => void, round: number): Promise<void> {
    if (!this.client) {
        await this.stopGame();
        return;
    }

    // 优先执行特殊动作（如一键开火）
    if (this.actionList.length > 0) {
        const currentAction = this.actionList[0];
        await currentAction.execute(ab, harvest, () => {
            this.actionList.shift();  // 执行完毕后移除
        });
        return;
    }

    // 默认输出任务：随机强度 + 波形输出
    let pulseId = this.pulsePlayList.getCurrentPulseId();
    
    const strengthChangeInterval = this.gameConfig.strengthChangeInterval;
    let outputTime = randomInt(strengthChangeInterval[0], strengthChangeInterval[1]) * 1000;
    let targetStrength = this.strengthConfig.strength + randomInt(0, this.strengthConfig.randomStrength);
    targetStrength = Math.min(targetStrength, this.clientStrength.limit);

    // 渐进式调整强度
    let currentStrength = this.client.strength.strength;
    if (targetStrength > currentStrength) {
        let setStrengthInterval = setInterval(() => {
            if (ab.signal.aborted) {
                clearInterval(setStrengthInterval);
                return;
            }

            this.setClientStrength(currentStrength).catch((error) => {
                console.error('Failed to set strength:', error);
            });

            if (currentStrength >= targetStrength) {
                clearInterval(setStrengthInterval);
            }

            currentStrength = Math.min(currentStrength + 2, targetStrength);
        }, 200);
    } else {
        await this.setClientStrength(targetStrength);
    }

    harvest();

    // 输出波形
    await this.client.outputPulse(pulseId, outputTime, {
        abortController: ab,
        bChannel: this.gameConfig.enableBChannel,
        customPulseList: this.customPulseList,
    });
}
```

### 6.2 Task 任务系统

#### Task 类

```typescript
export class Task {
    private abortController: AbortController = new AbortController();
    public running: boolean = false;

    constructor(handler: TaskHandler, options?: TaskOptions) {
        this.handler = handler;
        this.run().catch((error) => this.handleError(error));
    }

    public async run(): Promise<void> {
        if (this.running) return;

        let harvest = createHarvest(this.abortController);
        this.running = true;
        let round = 0;

        while (this.running) {
            try {
                await this.handler(this.abortController, harvest, round);
                harvest();  // 确保触发 TaskAborted
            } catch (error) {
                if (error instanceof TaskAbortedError) {
                    if (!this.isRestarting) {
                        break;  // 停止任务
                    } else {
                        // 重启任务
                        this.isRestarting = false;
                        this.abortController = new AbortController();
                        harvest = createHarvest(this.abortController);
                    }
                } else {
                    throw error;
                }
            }

            await asleep(this.minInterval);
            round++;
        }
    }

    public restart(): void {
        if (!this.running) return;
        this.isRestarting = true;
        this.abortController.abort();
    }

    public async abort(): Promise<void> {
        const stopPromise = this.stop();
        this.abortController.abort();
        await stopPromise;
    }
}
```

**关键设计**:
- 使用 `AbortController` 实现任务中断
- `harvest()` 函数检查中断信号，抛出 `TaskAbortedError`
- 支持任务重启而无需停止整个循环

### 6.3 Action 系统

#### 抽象基类

```typescript
export abstract class AbstractGameAction<ActionConfig = any> {
    static readonly defaultPriority = 0;
    public game!: CoyoteGameController;
    public priority: number;

    constructor(
        public config: ActionConfig,
        priority: number = AbstractGameAction.defaultPriority,
    ) {
        this.priority = priority;
    }

    abstract execute(ab: AbortController, harvest: () => void, done: () => void): Promise<void>;
    abstract updateConfig(config: ActionConfig): void;
}
```

#### 一键开火 Action

```typescript
export class GameFireAction extends AbstractGameAction<GameFireActionConfig> {
    public static readonly actionId = "fire";
    public fireStrength: number = 0;
    public fireEndTimestamp: number = 0;

    async execute(ab: AbortController, harvest: () => void, done: () => void): Promise<void> {
        // 1. 设置初始安全强度
        this.currentFireStrength = Math.min(this.fireStrength, SAFE_FIRE_STRENGTH);
        this.game.tempStrength = this.currentFireStrength;

        let absoluteStrength = Math.min(
            this.game.strengthConfig.strength + this.currentFireStrength, 
            this.game.gameStrength.limit
        );
        await this.game.setClientStrength(absoluteStrength);

        // 2. 渐进式增强强度
        let boostAb = new AbortController();
        let setStrengthInterval = setInterval(() => {
            if (boostAb.signal.aborted) {
                clearInterval(setStrengthInterval);
                return;
            }

            if (this.currentFireStrength >= this.fireStrength || 
                absoluteStrength >= this.game.clientStrength.limit) {
                return;
            }

            this.currentFireStrength = Math.min(
                this.currentFireStrength + FIRE_BOOST_STRENGTH, 
                this.fireStrength
            );
            this.game.tempStrength = this.currentFireStrength;
            absoluteStrength = Math.min(
                this.game.strengthConfig.strength + this.currentFireStrength, 
                this.game.clientStrength.limit
            );

            this.game.setClientStrength(absoluteStrength).catch(...);
        }, 200);

        // 3. 输出波形
        await this.game.client?.outputPulse(this.firePulseId, outputTime, {
            abortController: ab,
            bChannel: this.game.gameConfig.enableBChannel,
            onTimeEnd: () => {
                boostAb.abort();  // 停止增强
                if (Date.now() > this.fireEndTimestamp) {
                    // 提前降低强度
                    this.game.setClientStrength(this.game.strengthConfig.strength);
                }
            }
        });

        // 4. 检查是否结束
        if (Date.now() > this.fireEndTimestamp) {
            this.game.tempStrength = 0;
            done();
        }
    }

    updateConfig(config: GameFireActionConfig): void {
        this.config = config;

        if (config.updateMode === 'replace') {
            this.fireEndTimestamp = Date.now() + config.time;
        } else if (config.updateMode === 'append') {
            this.fireEndTimestamp += config.time;
        }
    }
}
```

---

## 7. 波形管理

### 7.1 波形数据结构

```typescript
export interface DGLabPulseBaseInfo {
    id: string;      // 波形 ID (8位十六进制)
    name: string;    // 波形名称
}

export interface DGLabPulseInfo extends DGLabPulseBaseInfo {
    pulseData: string[];  // 波形数据（十六进制字符串数组）
}
```

**波形数据示例**:

```json
{
    "id": "d6f83af0",
    "name": "呼吸",
    "pulseData": [
        "0A0A0A0A00000000",
        "0F0F0F0F00000000",
        "14141414",
        "0F0F0F0F00000000",
        "0A0A0A0A00000000"
    ]
}
```

### 7.2 波形播放列表

```typescript
export class PulsePlayList {
    public mode: 'single' | 'sequence' | 'random' = 'single';
    public pulseIds: string[] = [];
    public currentIndex = 0;
    public changeInterval = 0;
    public nextChangeTime = 0;

    public getCurrentPulseId(): string {
        if (this.mode === 'single') {
            return this.pulseIds[0];
        }
        
        if (Date.now() > this.nextChangeTime) {
            this.nextChangeTime = Date.now() + this.changeInterval * 1000;
            this.currentIndex++;

            if (this.currentIndex >= this.pulseIds.length) {
                this.currentIndex = 0;
                if (this.mode === 'random') {
                    this.suffle();  // 重新洗牌
                }
            }
        }

        return this.pulseIds[this.currentIndex];
    }

    private suffle() {
        this.pulseIds = this.pulseIds.sort(() => Math.random() - 0.5);
    }
}
```

### 7.3 波形服务

```typescript
export class DGLabPulseService {
    public pulseList: DGLabPulseInfo[] = [];
    private pulseConfig: ReactiveConfig<DGLabPulseInfo[]>;

    public async initialize(): Promise<void> {
        this.pulseConfig.on('data', (value) => {
            this.pulseList = value;
            this.events.emit('pulseListUpdated', this.getPulseInfoList());
        });

        await this.pulseConfig.initialize();
    }

    public getPulse(pulseId: string, customPulseList?: DGLabPulseInfo[]): DGLabPulseInfo | null {
        // 优先查找自定义波形
        if (customPulseList) {
            const customPulse = customPulseList.find(pulse => pulse.id === pulseId);
            if (customPulse) return customPulse;
        }

        // 查找系统波形
        return this.pulseList.find(pulse => pulse.id === pulseId) ?? null;
    }

    public getPulseHexData(pulse: DGLabPulseInfo): [string[], number] {
        let totalDuration = pulse.pulseData.length * PULSE_WINDOW;
        return [pulse.pulseData, totalDuration];
    }
}
```

**PULSE_WINDOW**: 每个波形数据包的持续时间为 100ms。

---

## 8. 配置管理

### 8.1 游戏配置

```typescript
export type MainGameConfig = {
    fireStrengthLimit: number;                // 一键开火强度限制 (默认30)
    strengthChangeInterval: [number, number]; // 强度变化间隔 [最小, 最大] 秒
    enableBChannel: boolean;                  // 是否启用 B 通道
    bChannelStrengthMultiplier: number;       // B 通道强度倍率
    pulseId: string | string[];               // 波形ID或ID列表
    firePulseId?: string | null;              // 一键开火波形ID
    pulseMode: 'single' | 'sequence' | 'random'; // 波形播放模式
    pulseChangeInterval: number;              // 波形切换间隔（秒）
};

export type GameStrengthConfig = {
    strength: number;         // 基础强度 (0-100)
    randomStrength: number;   // 随机强度 (0-100)
};
```

### 8.2 配置缓存

```typescript
// 使用 LRU Cache 缓存游戏配置，用于断线重连恢复
public configCache: LRUCache<string, any> = new LRUCache({
    max: 1000,
    ttl: 1000 * 60 * 30, // 30 分钟
});

// 保存配置
const configCachePrefix = `coyoteLiveGameConfig:${this.clientId}:`;
configCache.set(`${configCachePrefix}:strength`, this.strengthConfig);

// 恢复配置
let cachedGameStrengthConfig = configCache.get(`${configCachePrefix}:strength`);
if (cachedGameStrengthConfig) {
    this.strengthConfig = cachedGameStrengthConfig;
    this.strengthConfigModified = Date.now();
}
```

---

## 9. HTTP API

### 9.1 API 路由

项目使用 Koa + koa-swagger-decorator 实现 RESTful API。

**主要端点**:

- `GET /api/v2/game/{clientId}` - 获取游戏信息
- `GET /api/v2/game/{clientId}/strength` - 获取游戏强度
- `POST /api/v2/game/{clientId}/strength` - 设置游戏强度
- `GET /api/v2/game/{clientId}/pulse` - 获取当前波形
- `POST /api/v2/game/{clientId}/pulse` - 设置波形
- `POST /api/v2/game/{clientId}/action/fire` - 一键开火
- `GET /api/v2/pulse_list` - 获取波形列表
- `GET /api/docs` - Swagger 文档

### 9.2 设置强度 API

**请求格式**:

```typescript
type SetStrengthConfigRequest = {
    strength?: {
        add?: number;  // 增加基础强度
        sub?: number;  // 减少强度
        set?: number;  // 设置强度
    },
    randomStrength?: {
        add?: number;
        sub?: number;
        set?: number;
    }
}
```

**示例**:

```json
{
    "strength": {
        "add": 5
    }
}
```

**广播模式**: 如果 `allowBroadcastToClients: true`，可以将 `{clientId}` 设置为 `all`，将设置应用到所有客户端。

### 9.3 一键开火 API

**请求格式**:

```typescript
{
    "strength": 20,           // 一键开火强度（最高40）
    "time": 5000,            // 持续时间（毫秒，默认5000，最高30000）
    "override": false,       // 是否重置时间（true重置，false叠加）
    "pulseId": "d6f83af0"   // 可选：指定波形ID
}
```

---

## 10. 关键实现细节

### 10.1 防抖和节流

```typescript
// 节流：限制函数执行频率
const throttle = (fn: Function, delay: number) => {
    let lastCall = 0;
    return (...args: any[]) => {
        const now = Date.now();
        if (now - lastCall >= delay) {
            lastCall = now;
            fn(...args);
        }
    };
};

// 防抖：延迟执行，重复调用时重置计时器
const debounce = (fn: Function, delay: number) => {
    let timeout: NodeJS.Timeout;
    return (...args: any[]) => {
        clearTimeout(timeout);
        timeout = setTimeout(() => fn(...args), delay);
    };
};
```

### 10.2 延迟调试

```typescript
export class LatencyLogger {
    private startTime: number = 0;
    private lastLogTime: number = 0;
    private logs: string[] = [];

    start(label: string) {
        this.startTime = Date.now();
        this.lastLogTime = this.startTime;
        this.logs = [`Start: ${label}`];
    }

    log(label: string) {
        const now = Date.now();
        const delta = now - this.lastLogTime;
        this.logs.push(`+${delta}ms: ${label}`);
        this.lastLogTime = now;
    }

    finish() {
        const total = Date.now() - this.startTime;
        if (total > 100) {  // 只记录超过100ms的操作
            console.log(`[Latency] Total: ${total}ms\n${this.logs.join('\n')}`);
        }
    }
}
```

### 10.3 资源清理

项目使用 `EventStore` 自动管理事件监听器，防止内存泄漏：

```typescript
const clientEvents = this.eventStore.wrap(this.client);
clientEvents.on('close', () => {
    clientEvents.removeAllListeners();  // 自动清除所有监听器
    this.onlineSockets.delete('dgclient');
});
```

当 `Game` 或 `Client` 销毁时，调用 `eventStore.removeAllListeners()` 清除所有关联的事件监听器。

---

## 11. Rust 实现建议

基于以上分析，以下是对 Rust 实现的关键建议：

### 11.1 类型定义

```rust
// 消息类型
pub enum MessageType {
    Heartbeat,
    Bind,
    Msg,
    Break,
    Error,
}

// 错误码
pub enum RetCode {
    Success = 200,
    ClientDisconnected = 209,
    InvalidClientId = 210,
    ServerDelay = 211,
    IdAlreadyBound = 400,
    // ...
}

// 消息结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DGLabMessage {
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub client_id: String,
    pub target_id: String,
    pub message: String,
}

// 强度信息
#[derive(Debug, Clone, Copy)]
pub struct StrengthInfo {
    pub strength: u8,
    pub limit: u8,
}
```

### 11.2 WebSocket 客户端

```rust
pub struct DGLabWSClient {
    client_id: String,
    target_id: Arc<RwLock<Option<String>>>,
    ws_tx: Arc<Mutex<SplitSink<WebSocketStream<...>, Message>>>,
    strength: Arc<RwLock<StrengthInfo>>,
    strength_b: Arc<RwLock<StrengthInfo>>,
    event_tx: broadcast::Sender<ClientEvent>,
    heartbeat_task: Option<JoinHandle<()>>,
}

impl DGLabWSClient {
    pub async fn initialize(&mut self) -> Result<()> {
        self.send(MessageType::Bind, "targetId").await?;

        // 等待绑定成功（带超时）
        let start = Instant::now();
        loop {
            let target_id = self.target_id.read().await;
            if target_id.is_some() {
                break;
            }
            if start.elapsed() > Duration::from_secs(20) {
                return Err(CoreError::BindTimeout);
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        // 启动心跳任务
        self.start_heartbeat();

        Ok(())
    }

    pub async fn set_strength(&self, channel: Channel, strength: u8) -> Result<()> {
        // 验证强度上限
        let limit = match channel {
            Channel::A => self.strength.read().await.limit,
            Channel::B => self.strength_b.read().await.limit,
        };

        if strength > limit {
            return Err(CoreError::StrengthOutOfLimit);
        }

        let msg = format!("strength-{}+2+{}", channel as u8, strength);
        self.send(MessageType::Msg, &msg).await?;

        Ok(())
    }

    async fn handle_message(&self, msg: DGLabMessage) -> Result<()> {
        match msg.message_type {
            MessageType::Bind => {
                if msg.message == "DGLAB" {
                    *self.target_id.write().await = Some(msg.target_id.clone());
                    self.send(MessageType::Bind, "200").await?;
                }
            },
            MessageType::Msg => {
                if msg.message.starts_with("strength-") {
                    self.parse_strength_update(&msg.message).await?;
                } else if msg.message.starts_with("feedback-") {
                    self.parse_feedback(&msg.message).await?;
                }
            },
            // ...
        }
        Ok(())
    }
}
```

### 11.3 任务系统

```rust
pub struct GameTask {
    abort_tx: watch::Sender<bool>,
    handle: Option<JoinHandle<()>>,
}

impl GameTask {
    pub fn new<F, Fut>(handler: F) -> Self
    where
        F: Fn(watch::Receiver<bool>, usize) -> Fut + Send + 'static,
        Fut: Future<Output = Result<()>> + Send,
    {
        let (abort_tx, abort_rx) = watch::channel(false);
        let handle = tokio::spawn(async move {
            let mut round = 0;
            while !*abort_rx.borrow() {
                match handler(abort_rx.clone(), round).await {
                    Ok(_) => {},
                    Err(e) if e.is_aborted() => {
                        if !is_restarting {
                            break;
                        }
                    },
                    Err(e) => {
                        error!("Task error: {:?}", e);
                    }
                }
                round += 1;
            }
        });

        Self {
            abort_tx,
            handle: Some(handle),
        }
    }

    pub fn restart(&self) {
        // 发送中断信号
        let _ = self.abort_tx.send(true);
        // 立即重置
        let _ = self.abort_tx.send(false);
    }

    pub async fn stop(mut self) {
        let _ = self.abort_tx.send(true);
        if let Some(handle) = self.handle.take() {
            let _ = handle.await;
        }
    }
}
```

### 11.4 强度控制

```rust
impl CoyoteGameController {
    async fn set_client_strength(&self, strength: u8) -> Result<()> {
        if let Some(client) = &self.client {
            client.set_strength(Channel::A, strength).await?;

            if self.game_config.enable_b_channel {
                let b_strength = (strength as u16 * self.game_config.b_channel_strength_multiplier as u16 / 100)
                    .min(client.strength_b.read().await.limit as u16) as u8;
                client.set_strength(Channel::B, b_strength).await?;
            }
        }
        Ok(())
    }

    async fn gradual_strength_adjustment(&self, target: u8, abort_rx: watch::Receiver<bool>) {
        let mut current = self.client.as_ref().unwrap().strength.read().await.strength;

        while current < target && !*abort_rx.borrow() {
            current = (current + 2).min(target);
            let _ = self.set_client_strength(current).await;
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }
}
```

### 11.5 波形输出

```rust
impl DGLabWSClient {
    pub async fn output_pulse(
        &self,
        pulse_id: &str,
        duration_ms: u64,
        options: OutputPulseOptions,
    ) -> Result<()> {
        let pulse_service = DGLabPulseService::instance();
        let pulse_info = pulse_service.get_pulse(pulse_id, options.custom_pulse_list)
            .ok_or(CoreError::PulseNotFound)?;

        let (pulse_data, pulse_duration) = pulse_service.get_pulse_hex_data(&pulse_info);

        let mut total_duration = 0;

        // 预发送多个波形数据包
        for _ in 0..50 {
            if let Some(ref abort_rx) = options.abort_rx {
                if *abort_rx.borrow() {
                    return Err(CoreError::TaskAborted);
                }
            }

            self.send_pulse(Channel::A, &pulse_data).await?;
            if options.b_channel {
                self.send_pulse(Channel::B, &pulse_data).await?;
            }

            total_duration += pulse_duration;
            if total_duration > duration_ms {
                break;
            }
        }

        // 等待剩余时间
        tokio::time::sleep(Duration::from_millis(duration_ms)).await;

        if let Some(on_time_end) = options.on_time_end {
            on_time_end().await;
        }

        Ok(())
    }

    async fn send_pulse(&self, channel: Channel, pulse_data: &[String]) -> Result<()> {
        let channel_id = match channel {
            Channel::A => "A",
            Channel::B => "B",
        };

        let pulse_json = serde_json::to_string(pulse_data)?;
        let msg = format!("pulse-{}:{}", channel_id, pulse_json);

        self.send(MessageType::Msg, &msg).await?;
        Ok(())
    }
}
```

---

## 12. 总结

DG-Lab Coyote Game Hub 的核心设计要点：

1. **WebSocket 协议**: 基于 JSON 的文本协议，消息类型清晰，易于扩展
2. **绑定流程**: 三次握手式绑定，带超时保护
3. **心跳机制**: 20 秒间隔，确保连接活跃
4. **强度控制**: 渐进式调整，双通道支持，上限保护
5. **波形管理**: 支持多种播放模式（单一、顺序、随机）
6. **任务系统**: 基于 AbortController 的可中断任务，支持重启
7. **Action 系统**: 优先级队列，支持动态配置更新
8. **事件驱动**: EventEmitter + EventStore 模式，自动资源清理
9. **配置缓存**: LRU Cache 支持断线重连恢复

在 Rust 实现中，应重点关注：

- 使用 `tokio` 的异步运行时
- 使用 `Arc<RwLock<T>>` 管理共享状态
- 使用 `broadcast::channel` 实现事件分发
- 使用 `watch::channel` 实现任务中断
- 使用 `thiserror` 定义错误类型
- 使用 `async-trait` 实现异步 trait

希望这份分析对你的 Rust 重构工作有所帮助！
