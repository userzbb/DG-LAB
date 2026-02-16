#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.9"
# dependencies = [
#     "pydglab-ws>=1.0.0",
# ]
# ///
"""
DG-LAB 桥接模式测试脚本

使用 PyDGLab-WS 库测试 BLE-WebSocket 桥接功能
文档: https://pydglab-ws.readthedocs.io

使用方法:
  uv run test-bridge.py
  uv run test-bridge.py wss://custom-server.com
"""

import asyncio
from datetime import datetime
from pydglab_ws import (
    DGLabWSClient,
    StrengthOperationType,
    Channel,
    RetCode,
    FeedbackButton,
)


class BridgeTestController:
    """桥接模式测试控制器"""

    def __init__(self, server_url: str = "wss://ws.dungeon-lab.cn"):
        self.client = DGLabWSClient(server_url)
        self.target_id = None
        self.running = False

    async def connect(self):
        """连接到服务器"""
        print(f"🔌 连接到服务器...")
        ret = await self.client.bind()

        if ret == RetCode.SUCCESS:
            print(f"✓ 连接成功")
            print(f"✓ Client ID: {self.client.client_id}")
            return True
        else:
            print(f"❌ 连接失败: {ret}")
            return False

    async def bind_to_target(self, target_id: str):
        """绑定到目标设备（APP/桥接程序）"""
        print(f"\n🔗 绑定到目标: {target_id}")
        self.target_id = target_id

        ret = await self.client.bind(target_id)

        if ret == RetCode.SUCCESS:
            print("✓ 绑定成功！")
            return True
        else:
            print(f"❌ 绑定失败: {ret}")
            return False

    async def send_strength(
        self, channel: Channel, op_type: StrengthOperationType, value: int
    ):
        """发送强度操作"""
        ret = await self.client.add_strength(channel, op_type, value)

        timestamp = datetime.now().strftime("%H:%M:%S")
        if ret == RetCode.SUCCESS:
            op_name = {
                StrengthOperationType.INCREASE: "增加",
                StrengthOperationType.DECREASE: "减少",
                StrengthOperationType.SET_TO: "设置为",
            }.get(op_type, str(op_type))

            ch_name = "A" if channel == Channel.A else "B"
            print(f"[{timestamp}] 📤 {ch_name}通道 {op_name} {value}")
        else:
            print(f"[{timestamp}] ❌ 发送失败: {ret}")

    async def send_clear(self, channel: Channel):
        """发送清空操作"""
        ret = await self.client.clear_pulses(channel)

        timestamp = datetime.now().strftime("%H:%M:%S")
        ch_name = "A" if channel == Channel.A else "B"

        if ret == RetCode.SUCCESS:
            print(f"[{timestamp}] 📤 清空 {ch_name}通道")
        else:
            print(f"[{timestamp}] ❌ 清空失败: {ret}")

    async def send_pulse(self, channel: Channel, pulses: list):
        """发送波形数据"""
        ret = await self.client.add_pulses(channel, pulses)

        timestamp = datetime.now().strftime("%H:%M:%S")
        ch_name = "A" if channel == Channel.A else "B"

        if ret == RetCode.SUCCESS:
            print(f"[{timestamp}] 📤 发送波形到 {ch_name}通道 ({len(pulses)} 个脉冲)")
        else:
            print(f"[{timestamp}] ❌ 发送波形失败: {ret}")

    async def listen_for_updates(self):
        """监听来自设备的状态更新"""
        print("\n📊 开始监听设备状态更新...\n")
        self.running = True

        # 注册回调
        @self.client.on_strength_data
        async def on_strength(strength_data):
            timestamp = datetime.now().strftime("%H:%M:%S")
            print(
                f"[{timestamp}] 📥 设备状态: "
                f"A={strength_data.a}, B={strength_data.b}, "
                f"MaxA={strength_data.a_limit}, MaxB={strength_data.b_limit}"
            )

        @self.client.on_client_disconnected
        async def on_disconnected():
            print("\n⚠️  目标设备断开连接")
            self.running = False

        @self.client.on_error_message
        async def on_error(error_data):
            print(f"\n❌ 服务器错误: {error_data}")

        # 保持运行
        try:
            while self.running:
                await asyncio.sleep(0.1)
        except KeyboardInterrupt:
            pass

    async def close(self):
        """关闭连接"""
        self.running = False
        await self.client.close()


async def run_interactive_test(controller: BridgeTestController):
    """运行交互式测试"""
    print("\n" + "=" * 60)
    print("  DG-LAB 桥接模式控制器")
    print("=" * 60)
    print("\n可用命令:")
    print("  a+<值>   - A通道增加强度 (例: a+10)")
    print("  a-<值>   - A通道减少强度 (例: a-10)")
    print("  a=<值>   - A通道设置强度 (例: a=50)")
    print("  b+<值>   - B通道增加强度")
    print("  b-<值>   - B通道减少强度")
    print("  b=<值>   - B通道设置强度")
    print("  ca       - 清空A通道")
    print("  cb       - 清空B通道")
    print("  pulse    - 发送测试波形")
    print("  auto     - 自动测试模式")
    print("  quit     - 退出")
    print()

    # 启动监听任务
    listen_task = asyncio.create_task(controller.listen_for_updates())

    try:
        while controller.running:
            try:
                # 非阻塞输入
                cmd = await asyncio.wait_for(
                    asyncio.get_event_loop().run_in_executor(None, input, "命令> "),
                    timeout=1.0,
                )
                cmd = cmd.strip().lower()

                if not cmd:
                    continue

                if cmd == "quit":
                    break

                elif cmd == "auto":
                    print("\n🤖 开始自动测试...")
                    await run_auto_test(controller)
                    print("✓ 自动测试完成\n")

                elif cmd == "pulse":
                    await run_pulse_test(controller)

                elif cmd.startswith("a") or cmd.startswith("b"):
                    await parse_and_send_command(controller, cmd)

                else:
                    print("❌ 未知命令")

            except asyncio.TimeoutError:
                continue
            except EOFError:
                break

    except KeyboardInterrupt:
        print("\n\n🛑 收到中断信号")

    finally:
        controller.running = False
        listen_task.cancel()


async def parse_and_send_command(controller: BridgeTestController, cmd: str):
    """解析并发送命令"""
    try:
        if cmd == "ca":
            await controller.send_clear(Channel.A)
        elif cmd == "cb":
            await controller.send_clear(Channel.B)
        else:
            channel = Channel.A if cmd[0] == "a" else Channel.B
            op = cmd[1]
            value = int(cmd[2:])

            if op == "+":
                op_type = StrengthOperationType.INCREASE
            elif op == "-":
                op_type = StrengthOperationType.DECREASE
            elif op == "=":
                op_type = StrengthOperationType.SET_TO
            else:
                print("❌ 无效操作符")
                return

            await controller.send_strength(channel, op_type, value)

    except ValueError:
        print("❌ 无效数值")
    except Exception as e:
        print(f"❌ 错误: {e}")


async def run_auto_test(controller: BridgeTestController):
    """运行自动测试序列"""
    tests = [
        ("设置 A=50", Channel.A, StrengthOperationType.SET_TO, 50),
        ("设置 B=50", Channel.B, StrengthOperationType.SET_TO, 50),
        ("A通道 +10", Channel.A, StrengthOperationType.INCREASE, 10),
        ("B通道 +10", Channel.B, StrengthOperationType.INCREASE, 10),
        ("A通道 -20", Channel.A, StrengthOperationType.DECREASE, 20),
        ("B通道 -20", Channel.B, StrengthOperationType.DECREASE, 20),
    ]

    for desc, channel, op_type, value in tests:
        print(f"  • {desc}")
        await controller.send_strength(channel, op_type, value)
        await asyncio.sleep(2)

    # 清空
    print(f"  • 清空 A通道")
    await controller.send_clear(Channel.A)
    await asyncio.sleep(1)

    print(f"  • 清空 B通道")
    await controller.send_clear(Channel.B)


async def run_pulse_test(controller: BridgeTestController):
    """运行波形测试"""
    print("\n🌊 开始波形测试...")

    # 创建一个简单的测试波形（10个100ms的脉冲）
    # 每个脉冲格式为 8 字节 hex 字符串
    # 示例：0A0A320A0A640A0A (简单方波)
    test_pulse = "0A0A320A0A640A0A"
    pulses = [test_pulse] * 10

    print(f"  • 发送 {len(pulses)} 个测试脉冲到 A 通道")
    await controller.send_pulse(Channel.A, pulses)
    await asyncio.sleep(2)

    print(f"  • 发送 {len(pulses)} 个测试脉冲到 B 通道")
    await controller.send_pulse(Channel.B, pulses)

    print("✓ 波形测试完成\n")


async def main():
    """主函数"""
    import sys

    print("""
╔════════════════════════════════════════════════════════╗
║       DG-LAB 桥接模式测试脚本 (PyDGLab-WS)             ║
║                                                        ║
║  测试 BLE-WebSocket 桥接功能                           ║
╚════════════════════════════════════════════════════════╝
    """)

    # 解析命令行参数
    server_url = "wss://ws.dungeon-lab.cn"
    if len(sys.argv) > 1:
        server_url = sys.argv[1]

    controller = BridgeTestController(server_url)

    try:
        # 1. 连接到服务器
        if not await controller.connect():
            return

        # 2. 提示用户输入目标 ID
        print("\n" + "=" * 60)
        print("现在请启动桥接程序:")
        print("  cargo run --bin dglab -- bridge --device 47L121000")
        print()
        print("或在 Windows 上:")
        print("  .\\dglab.exe bridge --device 47L121000")
        print()
        print("等待桥接程序连接后，从输出中复制 Client ID")
        print("(Client ID 在二维码 URL 的 # 后面)")
        print("=" * 60)

        target_id = input("\n目标 Client ID> ").strip()

        if not target_id:
            print("❌ 未输入目标 ID")
            return

        # 3. 绑定到目标
        if not await controller.bind_to_target(target_id):
            return

        # 4. 运行交互式测试
        await run_interactive_test(controller)

    except KeyboardInterrupt:
        print("\n\n🛑 测试中断")
    except Exception as e:
        print(f"\n❌ 错误: {e}")
        import traceback

        traceback.print_exc()
    finally:
        await controller.close()
        print("\n✓ 已断开连接")


if __name__ == "__main__":
    asyncio.run(main())
