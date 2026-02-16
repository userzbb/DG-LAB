#!/usr/bin/env python3
"""
本地 DG-LAB WebSocket 测试服务器
用于测试 CLI 和 GUI 的 WiFi 连接功能
"""

import asyncio
import websockets
import json
from datetime import datetime
import uuid

# 服务器配置
HOST = "0.0.0.0"
PORT = 8765

# 已连接的客户端
connected_clients = {}


async def handle_client(websocket, path):
    """处理客户端连接"""
    client_id = str(uuid.uuid4())
    print(f"\n📱 新客户端连接: {client_id}")
    connected_clients[client_id] = websocket

    try:
        # 发送 clientId
        await websocket.send(
            json.dumps(
                {"type": "clientId", "clientId": client_id, "message": "connected"}
            )
        )
        print(f"📨 已发送 clientId: {client_id}")

        # 显示二维码 URL
        qr_url = f"http://localhost:{PORT}/bind?clientId={client_id}"
        print(f"\n🔗 模拟二维码 URL: {qr_url}")
        print("💡 (在真实环境中，这会显示为二维码)")
        print("⏳ 等待绑定... (按 Ctrl+C 停止)")

        # 模拟绑定（等待 5 秒后自动绑定）
        await asyncio.sleep(5)

        # 发送绑定成功消息
        await websocket.send(
            json.dumps(
                {
                    "type": "bind",
                    "clientId": client_id,
                    "targetId": "test-target-id",
                    "message": "bound",
                }
            )
        )
        print(f"\n✅ 模拟绑定成功!")

        # 保持连接，处理消息
        async for message in websocket:
            try:
                data = json.loads(message)
                print(f"\n📨 收到消息: {data}")

                # 发送心跳响应
                if data.get("type") == "heartbeat":
                    await websocket.send(
                        json.dumps(
                            {
                                "type": "heartbeat",
                                "timestamp": datetime.now().isoformat(),
                            }
                        )
                    )
            except json.JSONDecodeError:
                print(f"\n⚠️  无法解析消息: {message}")

    except websockets.exceptions.ConnectionClosed:
        print(f"\n❌ 客户端断开: {client_id}")
    finally:
        if client_id in connected_clients:
            del connected_clients[client_id]


async def main():
    """主函数"""
    print("╔══════════════════════════════════════════════════════╗")
    print("║       DG-LAB 本地 WebSocket 测试服务器              ║")
    print("╚══════════════════════════════════════════════════════╝")
    print(f"\n🚀 服务器启动中...")
    print(f"📍 监听地址: ws://{HOST}:{PORT}")
    print(f"💡 提示: 在另一个终端运行:")
    print(f"   dglab wifi connect --server ws://localhost:{PORT}")
    print(f"   或")
    print(f"   dglab bridge --device 47L121000 --ws-url ws://localhost:{PORT}")
    print(f"\n⏳ 等待客户端连接... (按 Ctrl+C 停止)\n")

    async with websockets.serve(handle_client, HOST, PORT):
        await asyncio.Future()  # 永久运行


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\n\n👋 服务器已停止")
