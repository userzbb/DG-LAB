//! 波形生成器使用示例
//!
//! 本示例展示 WaveformGenerator 的各种波形类型及其输出，
//! 以及 PresetManager 的预设管理功能。
//!
//! 运行：`cargo run -p dglab-core --example waveform_demo`

use dglab_core::preset::Preset;
use dglab_core::waveform::{Waveform, WaveformGenerator, WaveformParams, WaveformType};

fn main() {
    println!("=== DG-LAB 波形生成器示例 ===\n");

    // ── 1. 基本波形类型演示 ─────────────────────────────────
    println!("--- 1. 各种波形类型 ---\n");

    let wave_types = [
        ("连续波 (Continuous)", WaveformType::Continuous),
        ("脉冲波 (Pulse)", WaveformType::Pulse),
        ("锯齿波 (Sawtooth)", WaveformType::Sawtooth),
        ("正弦波 (Sine)", WaveformType::Sine),
        ("方波 (Square)", WaveformType::Square),
        ("三角波 (Triangle)", WaveformType::Triangle),
        ("呼吸波 (Breathing)", WaveformType::Breathing),
        ("渐强渐弱 (Fade)", WaveformType::Fade),
    ];

    for (name, wt) in &wave_types {
        let waveform = Waveform {
            name: name.to_string(),
            description: format!("{name} 示例"),
            params: WaveformParams {
                waveform_type: *wt,
                frequency: 100,
                pulse_width: 200,
                min_power: 10,
                max_power: 90,
                period_ms: 1000,
                duty_cycle: 50,
            },
            custom_points: None,
        };

        let mut gen = WaveformGenerator::with_waveform(waveform);
        gen.start();

        // 采样 10 个时间点（每 100ms）
        let mut values = Vec::new();
        for _ in 0..10 {
            let v = gen.update(100);
            values.push(v);
        }

        // 打印 ASCII 条形图
        print!("  {name:<24} |");
        for v in &values {
            // 将 0-100 映射到 0-8 的条形
            let bar_len = (*v as usize) / 12;
            print!("{:>4}", "#".repeat(bar_len.min(8)));
        }
        println!("|  [{:?}]", values);
    }
    println!();

    // ── 2. 自定义波形 ───────────────────────────────────────
    println!("--- 2. 自定义波形 ---");

    let custom = Waveform {
        name: "阶梯波".to_string(),
        description: "自定义阶梯状波形".to_string(),
        params: WaveformParams {
            waveform_type: WaveformType::Custom,
            frequency: 100,
            pulse_width: 200,
            min_power: 0,
            max_power: 100,
            period_ms: 1000,
            duty_cycle: 50,
        },
        custom_points: Some(vec![
            (0, 20),
            (250, 20),
            (250, 50),
            (500, 50),
            (500, 80),
            (750, 80),
            (750, 100),
            (1000, 100),
        ]),
    };

    let mut gen = WaveformGenerator::with_waveform(custom);
    gen.start();

    print!("  阶梯波                   |");
    let mut values = Vec::new();
    for _ in 0..10 {
        let v = gen.update(100);
        values.push(v);
    }
    for v in &values {
        let bar_len = (*v as usize) / 12;
        print!("{:>4}", "#".repeat(bar_len.min(8)));
    }
    println!("|  [{:?}]", values);
    println!();

    // ── 3. 波形生成器控制 ───────────────────────────────────
    println!("--- 3. 波形生成器控制 ---");

    let mut gen = WaveformGenerator::new();
    println!("  默认波形: {:?}", gen.waveform().params.waveform_type);

    // 切换波形
    let sine = Waveform {
        name: "Sine".to_string(),
        description: "正弦波".to_string(),
        params: WaveformParams {
            waveform_type: WaveformType::Sine,
            min_power: 0,
            max_power: 100,
            period_ms: 2000,
            ..WaveformParams::default()
        },
        custom_points: None,
    };

    gen.set_waveform(sine);
    println!("  切换后: {:?}", gen.waveform().params.waveform_type);

    gen.start();
    let v1 = gen.update(500); // 1/4 周期
    let v2 = gen.update(500); // 1/2 周期
    let v3 = gen.update(500); // 3/4 周期
    let v4 = gen.update(500); // 满周期
    println!("  正弦波采样 (2s 周期): [{v1}, {v2}, {v3}, {v4}]");

    gen.stop();
    gen.reset();
    println!("  已停止并重置");
    println!();

    // ── 4. 预设波形列表 ─────────────────────────────────────
    println!("--- 4. 预设波形列表 ---");

    let presets = WaveformGenerator::preset_waveforms();
    println!("  共 {} 个预设波形:", presets.len());
    for (i, w) in presets.iter().enumerate() {
        println!(
            "    {}. {} - {} (类型: {:?}, 频率: {}Hz, 周期: {}ms)",
            i + 1,
            w.name,
            w.description,
            w.params.waveform_type,
            w.params.frequency,
            w.params.period_ms,
        );
    }
    println!();

    // ── 5. 预设管理 ─────────────────────────────────────────
    println!("--- 5. 预设管理（内存模式）---");

    // 创建预设
    let mut preset = Preset::new("我的预设".to_string(), "测试用预设".to_string());
    println!("  创建预设: {} (ID: {})", preset.name, &preset.id[..8]);

    // 配置通道
    preset.set_max_power(0, 80); // A 通道最大 80
    preset.set_max_power(1, 60); // B 通道最大 60
    println!("  通道 A 最大强度: {}", preset.channel_a.max_power);
    println!("  通道 B 最大强度: {}", preset.channel_b.max_power);

    // 序列化验证
    let json = serde_json::to_string_pretty(&preset).unwrap();
    println!("  序列化 JSON 长度: {} 字节", json.len());

    let deserialized: Preset = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, preset.name);
    println!("  反序列化验证通过");

    println!("\n=== 示例完成 ===");
}
