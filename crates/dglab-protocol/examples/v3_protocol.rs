//! V3 BLE 协议使用示例
//!
//! 本示例展示如何构造和解析 DG-LAB V3 BLE 协议的各种指令。
//! 这是纯数据层的演示，不需要实际的蓝牙连接。
//!
//! 运行：`cargo run -p dglab-protocol --example v3_protocol`

use dglab_protocol::v3::{
    compress_frequency, decompress_frequency, pulse_hz_to_value, B0Command, B1Response, BFCommand,
    ChannelStrengthMode, NotifyMessage, StrengthMode, WaveformData,
};

fn main() {
    println!("=== DG-LAB V3 BLE 协议示例 ===\n");

    // ── 1. 频率转换 ──────────────────────────────────────────
    println!("--- 1. 频率转换 ---");
    let frequencies = [10, 50, 100, 200, 500, 1000];
    for freq in frequencies {
        let compressed = compress_frequency(freq);
        let decompressed = decompress_frequency(compressed);
        println!("  输入: {freq:>4} -> 压缩: {compressed:>3} -> 还原: {decompressed:>4}",);
    }
    println!();

    // Hz 到脉冲值转换
    println!("--- 脉冲 Hz 转换 ---");
    let hz_values = [10, 50, 100, 200, 500, 1000];
    for hz in hz_values {
        let value = pulse_hz_to_value(hz);
        println!("  {hz:>4} Hz -> 脉冲值: {value:>3}");
    }
    println!();

    // ── 2. B0 指令（强度 + 波形）────────────────────────────
    println!("--- 2. B0 指令构造 ---");

    // 2a. 不修改强度，只发送波形
    let waveform_a = WaveformData::uniform(50, 80); // 频率=50, 强度=80
    let waveform_b = WaveformData::silent(); // B 通道静默

    let cmd = B0Command {
        sequence: 1,
        strength_mode: StrengthMode::both_no_change(),
        strength_a: 0,
        strength_b: 0,
        waveform_a,
        waveform_b,
    };

    let bytes = cmd.encode();
    println!("  仅波形 B0: {} 字节", bytes.len());
    println!("  头部: 0x{:02X}", bytes[0]);
    println!("  原始: {:02x?}", bytes);

    // 解码验证
    let decoded = B0Command::decode(&bytes).expect("解码应成功");
    assert_eq!(decoded.sequence, cmd.sequence);
    println!("  解码序列号: {}", decoded.sequence);
    println!();

    // 2b. 设置通道 A 强度为 50（绝对模式）
    let cmd_strength = B0Command {
        sequence: 2,
        strength_mode: StrengthMode {
            channel_a: ChannelStrengthMode::Absolute,
            channel_b: ChannelStrengthMode::NoChange,
        },
        strength_a: 50,
        strength_b: 0,
        waveform_a: WaveformData::uniform(30, 60),
        waveform_b: WaveformData::silent(),
    };

    let bytes = cmd_strength.encode();
    println!("  设置强度 B0: {:02x?}", bytes);

    // 编解码往返验证
    let rt = B0Command::decode(&bytes).unwrap();
    assert_eq!(rt.strength_a, 50);
    assert_eq!(rt.strength_mode.channel_a, ChannelStrengthMode::Absolute);
    println!("  往返验证通过: 强度 A = {}", rt.strength_a);
    println!();

    // 2c. 增量/减量模式
    let cmd_inc = B0Command {
        sequence: 3,
        strength_mode: StrengthMode {
            channel_a: ChannelStrengthMode::Increase,
            channel_b: ChannelStrengthMode::Decrease,
        },
        strength_a: 5,  // A 通道增加 5
        strength_b: 10, // B 通道减少 10
        waveform_a: WaveformData::uniform(50, 50),
        waveform_b: WaveformData::uniform(50, 50),
    };

    let bytes = cmd_inc.encode();
    let rt = B0Command::decode(&bytes).unwrap();
    println!("  增减模式 B0:");
    println!("    A: {:?} +{}", rt.strength_mode.channel_a, rt.strength_a);
    println!("    B: {:?} -{}", rt.strength_mode.channel_b, rt.strength_b);
    println!();

    // ── 3. BF 指令（配置）───────────────────────────────────
    println!("--- 3. BF 指令（软上限配置）---");

    let config = BFCommand::default_config();
    let bytes = config.encode();
    println!("  默认配置: {:02x?}", bytes);
    println!("  A 通道上限: {}", config.soft_limit_a);
    println!("  B 通道上限: {}", config.soft_limit_b);
    println!(
        "  频率平衡 A/B: {}/{}",
        config.freq_balance_a, config.freq_balance_b
    );
    println!(
        "  强度平衡 A/B: {}/{}",
        config.intensity_balance_a, config.intensity_balance_b
    );

    // 自定义配置
    let custom_config = BFCommand {
        soft_limit_a: 100,
        soft_limit_b: 80,
        freq_balance_a: 128,
        freq_balance_b: 128,
        intensity_balance_a: 64,
        intensity_balance_b: 64,
    };
    let bytes = custom_config.encode();
    let rt = BFCommand::decode(&bytes).unwrap();
    assert_eq!(rt.soft_limit_a, 100);
    println!(
        "  自定义配置: A上限={}, B上限={}, 频率平衡={}/{}",
        rt.soft_limit_a, rt.soft_limit_b, rt.freq_balance_a, rt.freq_balance_b
    );
    println!();

    // ── 4. B1 回应（强度反馈）──────────────────────────────
    println!("--- 4. B1 回应解析 ---");

    // 模拟设备回应
    let response = B1Response {
        sequence: 5,
        strength_a: 42,
        strength_b: 30,
    };
    let bytes = response.encode();
    println!("  回应: {:02x?}", bytes);
    println!(
        "  序列号: {}, A: {}, B: {}",
        response.sequence, response.strength_a, response.strength_b
    );

    // 使用 NotifyMessage 解析
    let msg = NotifyMessage::parse(&bytes);
    match msg {
        NotifyMessage::Strength(b1) => {
            println!(
                "  解析为 B1 强度反馈: A={}, B={}",
                b1.strength_a, b1.strength_b
            );
        }
        NotifyMessage::Unknown(_) => {
            println!("  未知消息类型");
        }
    }
    println!();

    // ── 5. 波形数据 Hex 格式（用于 WebSocket）──────────────
    println!("--- 5. 波形数据 Hex 格式 ---");

    let wave = WaveformData {
        frequency: [10, 20, 30, 40],
        intensity: [50, 60, 70, 80],
    };

    let hex = wave.to_hex_string();
    println!("  波形 Hex: {}", hex);

    let parsed = WaveformData::from_hex_string(&hex).expect("Hex 解析应成功");
    assert_eq!(parsed, wave);
    println!("  Hex 解析验证通过");
    println!();

    // ── 6. 强度模式组合 ─────────────────────────────────────
    println!("--- 6. 强度模式组合 ---");
    let modes = [
        ("双通道不变", StrengthMode::both_no_change()),
        (
            "A 绝对, B 不变",
            StrengthMode {
                channel_a: ChannelStrengthMode::Absolute,
                channel_b: ChannelStrengthMode::NoChange,
            },
        ),
        (
            "A 增加, B 减少",
            StrengthMode {
                channel_a: ChannelStrengthMode::Increase,
                channel_b: ChannelStrengthMode::Decrease,
            },
        ),
        (
            "双通道绝对",
            StrengthMode {
                channel_a: ChannelStrengthMode::Absolute,
                channel_b: ChannelStrengthMode::Absolute,
            },
        ),
    ];

    for (name, mode) in &modes {
        let byte = mode.encode();
        let decoded = StrengthMode::decode(byte);
        println!(
            "  {name}: 编码=0x{byte:02X}, A={:?}, B={:?}",
            decoded.channel_a, decoded.channel_b
        );
    }

    println!("\n=== 示例完成 ===");
}
