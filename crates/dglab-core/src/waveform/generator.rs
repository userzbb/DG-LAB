//! 波形生成器

use serde::{Deserialize, Serialize};

/// 波形类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaveformType {
    /// 连续波
    Continuous,
    /// 脉冲波
    Pulse,
    /// 锯齿波
    Sawtooth,
    /// 正弦波
    Sine,
    /// 方波
    Square,
    /// 三角波
    Triangle,
    /// 呼吸波
    Breathing,
    /// 渐强渐弱
    Fade,
    /// 自定义
    Custom,
}

/// 波形参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveformParams {
    /// 波形类型
    pub waveform_type: WaveformType,
    /// 频率 (Hz)
    pub frequency: u16,
    /// 脉宽 (微秒)
    pub pulse_width: u16,
    /// 最小强度
    pub min_power: u8,
    /// 最大强度
    pub max_power: u8,
    /// 周期 (毫秒)
    pub period_ms: u32,
    /// 占空比 (0-100)
    pub duty_cycle: u8,
}

impl Default for WaveformParams {
    fn default() -> Self {
        Self {
            waveform_type: WaveformType::Continuous,
            frequency: 100,
            pulse_width: 200,
            min_power: 0,
            max_power: 100,
            period_ms: 5000,
            duty_cycle: 50,
        }
    }
}

/// 波形
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waveform {
    /// 波形名称
    pub name: String,
    /// 波形描述
    pub description: String,
    /// 波形参数
    pub params: WaveformParams,
    /// 自定义数据点
    pub custom_points: Option<Vec<(u32, u8)>>,
}

impl Default for Waveform {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            description: "Default waveform".to_string(),
            params: WaveformParams::default(),
            custom_points: None,
        }
    }
}

/// 波形生成器
pub struct WaveformGenerator {
    /// 当前波形
    current_waveform: Waveform,
    /// 开始时间
    start_time: Option<std::time::Instant>,
    /// 当前相位
    phase: f64,
}

impl WaveformGenerator {
    /// 创建新的波形生成器
    pub fn new() -> Self {
        Self {
            current_waveform: Waveform::default(),
            start_time: None,
            phase: 0.0,
        }
    }

    /// 使用指定波形创建生成器
    pub fn with_waveform(waveform: Waveform) -> Self {
        Self {
            current_waveform: waveform,
            start_time: None,
            phase: 0.0,
        }
    }

    /// 设置波形
    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.current_waveform = waveform;
        self.start_time = None;
        self.phase = 0.0;
    }

    /// 获取当前波形
    pub fn waveform(&self) -> &Waveform {
        &self.current_waveform
    }

    /// 开始生成
    pub fn start(&mut self) {
        self.start_time = Some(std::time::Instant::now());
        self.phase = 0.0;
    }

    /// 停止生成
    pub fn stop(&mut self) {
        self.start_time = None;
    }

    /// 重置生成器
    pub fn reset(&mut self) {
        self.start_time = None;
        self.phase = 0.0;
    }

    /// 获取当前强度值
    pub fn current_power(&mut self) -> u8 {
        let params = &self.current_waveform.params;

        match params.waveform_type {
            WaveformType::Continuous => params.max_power,
            WaveformType::Pulse => self.pulse_wave(params),
            WaveformType::Sawtooth => self.sawtooth_wave(params),
            WaveformType::Sine => self.sine_wave(params),
            WaveformType::Square => self.square_wave(params),
            WaveformType::Triangle => self.triangle_wave(params),
            WaveformType::Breathing => self.breathing_wave(params),
            WaveformType::Fade => self.fade_wave(params),
            WaveformType::Custom => self.custom_wave(params),
        }
    }

    /// 更新并获取当前强度值
    pub fn update(&mut self, delta_ms: u64) -> u8 {
        let params = &self.current_waveform.params;
        let period = params.period_ms as f64;

        self.phase += delta_ms as f64 / period;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        self.current_power()
    }

    /// 脉冲波
    fn pulse_wave(&self, params: &WaveformParams) -> u8 {
        let duty = params.duty_cycle as f64 / 100.0;
        if self.phase < duty {
            params.max_power
        } else {
            params.min_power
        }
    }

    /// 锯齿波
    fn sawtooth_wave(&self, params: &WaveformParams) -> u8 {
        let range = (params.max_power - params.min_power) as f64;
        let value = params.min_power as f64 + self.phase * range;
        value.round() as u8
    }

    /// 正弦波
    fn sine_wave(&self, params: &WaveformParams) -> u8 {
        let range = (params.max_power - params.min_power) as f64 / 2.0;
        let mid = (params.max_power + params.min_power) as f64 / 2.0;
        let value = mid + range * (self.phase * 2.0 * std::f64::consts::PI).sin();
        value.round() as u8
    }

    /// 方波
    fn square_wave(&self, params: &WaveformParams) -> u8 {
        let duty = params.duty_cycle as f64 / 100.0;
        if self.phase < duty {
            params.max_power
        } else {
            params.min_power
        }
    }

    /// 三角波
    fn triangle_wave(&self, params: &WaveformParams) -> u8 {
        let range = (params.max_power - params.min_power) as f64;
        let value = if self.phase < 0.5 {
            self.phase * 2.0 * range
        } else {
            (1.0 - (self.phase - 0.5) * 2.0) * range
        };
        (params.min_power as f64 + value).round() as u8
    }

    /// 呼吸波
    fn breathing_wave(&self, params: &WaveformParams) -> u8 {
        // 类似正弦波但有更平缓的上升和更陡的下降
        let range = (params.max_power - params.min_power) as f64;
        let t = self.phase;
        let value = if t < 0.5 {
            // 平缓上升 (0.0 -> 0.5)
            (t * 2.0).powi(2)
        } else {
            // 快速下降 (0.5 -> 1.0)
            1.0 - ((t - 0.5) * 2.0).powi(2)
        };
        (params.min_power as f64 + value * range).round() as u8
    }

    /// 渐强渐弱波
    fn fade_wave(&self, params: &WaveformParams) -> u8 {
        let range = (params.max_power - params.min_power) as f64;
        let t = self.phase;
        // 0-0.5: 渐强, 0.5-1: 渐弱
        let value = if t < 0.5 { t * 2.0 } else { 2.0 - t * 2.0 };
        (params.min_power as f64 + value * range).round() as u8
    }

    /// 自定义波形
    fn custom_wave(&self, params: &WaveformParams) -> u8 {
        if let Some(points) = &self.current_waveform.custom_points {
            if points.is_empty() {
                return params.max_power;
            }

            let t = self.phase;
            // 在点之间进行线性插值
            let total_time = points.last().unwrap().0.max(1) as f64;
            let current_time = t * total_time;

            // 找到当前时间点所在的区间
            let mut idx = 0;
            while idx < points.len() - 1 && points[idx + 1].0 as f64 <= current_time {
                idx += 1;
            }

            if idx == points.len() - 1 {
                return points[idx].1;
            }

            // 线性插值
            let (t1, v1) = (points[idx].0 as f64, points[idx].1 as f64);
            let (t2, v2) = (points[idx + 1].0 as f64, points[idx + 1].1 as f64);

            if t2 == t1 {
                return v1.round() as u8;
            }

            let ratio = (current_time - t1) / (t2 - t1);
            let value = v1 + ratio * (v2 - v1);
            value.round() as u8
        } else {
            params.max_power
        }
    }

    /// 获取预设波形
    pub fn preset_waveforms() -> Vec<Waveform> {
        vec![
            Waveform {
                name: "Continuous".to_string(),
                description: "Continuous wave with constant intensity".to_string(),
                params: WaveformParams {
                    waveform_type: WaveformType::Continuous,
                    frequency: 100,
                    pulse_width: 200,
                    min_power: 50,
                    max_power: 50,
                    period_ms: 1000,
                    duty_cycle: 100,
                },
                custom_points: None,
            },
            Waveform {
                name: "Pulse".to_string(),
                description: "Pulsing wave".to_string(),
                params: WaveformParams {
                    waveform_type: WaveformType::Pulse,
                    frequency: 50,
                    pulse_width: 300,
                    min_power: 0,
                    max_power: 80,
                    period_ms: 2000,
                    duty_cycle: 30,
                },
                custom_points: None,
            },
            Waveform {
                name: "Breathing".to_string(),
                description: "Breathing-like wave pattern".to_string(),
                params: WaveformParams {
                    waveform_type: WaveformType::Breathing,
                    frequency: 100,
                    pulse_width: 200,
                    min_power: 20,
                    max_power: 80,
                    period_ms: 4000,
                    duty_cycle: 50,
                },
                custom_points: None,
            },
            Waveform {
                name: "Sawtooth".to_string(),
                description: "Rising sawtooth wave".to_string(),
                params: WaveformParams {
                    waveform_type: WaveformType::Sawtooth,
                    frequency: 100,
                    pulse_width: 200,
                    min_power: 0,
                    max_power: 100,
                    period_ms: 3000,
                    duty_cycle: 50,
                },
                custom_points: None,
            },
            Waveform {
                name: "Fade".to_string(),
                description: "Fade in and out".to_string(),
                params: WaveformParams {
                    waveform_type: WaveformType::Fade,
                    frequency: 100,
                    pulse_width: 200,
                    min_power: 0,
                    max_power: 100,
                    period_ms: 5000,
                    duty_cycle: 50,
                },
                custom_points: None,
            },
        ]
    }
}

impl Default for WaveformGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === WaveformParams 测试 ===

    #[test]
    fn test_waveform_params_default() {
        let params = WaveformParams::default();
        assert_eq!(params.waveform_type, WaveformType::Continuous);
        assert_eq!(params.frequency, 100);
        assert_eq!(params.pulse_width, 200);
        assert_eq!(params.min_power, 0);
        assert_eq!(params.max_power, 100);
        assert_eq!(params.period_ms, 5000);
        assert_eq!(params.duty_cycle, 50);
    }

    #[test]
    fn test_waveform_params_serde_roundtrip() {
        let params = WaveformParams::default();
        let json = serde_json::to_string(&params).unwrap();
        let deserialized: WaveformParams = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.waveform_type, params.waveform_type);
        assert_eq!(deserialized.frequency, params.frequency);
    }

    // === Waveform 测试 ===

    #[test]
    fn test_waveform_default() {
        let wf = Waveform::default();
        assert_eq!(wf.name, "Default");
        assert_eq!(wf.description, "Default waveform");
        assert!(wf.custom_points.is_none());
    }

    #[test]
    fn test_waveform_serde_roundtrip() {
        let wf = Waveform {
            name: "Test".to_string(),
            description: "Test wave".to_string(),
            params: WaveformParams::default(),
            custom_points: Some(vec![(0, 0), (500, 100), (1000, 0)]),
        };
        let json = serde_json::to_string(&wf).unwrap();
        let deserialized: Waveform = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "Test");
        assert_eq!(deserialized.custom_points.unwrap().len(), 3);
    }

    // === WaveformGenerator 基础测试 ===

    #[test]
    fn test_generator_new() {
        let gen = WaveformGenerator::new();
        assert_eq!(gen.waveform().name, "Default");
        assert_eq!(gen.phase, 0.0);
        assert!(gen.start_time.is_none());
    }

    #[test]
    fn test_generator_default() {
        let gen = WaveformGenerator::default();
        assert_eq!(gen.waveform().name, "Default");
    }

    #[test]
    fn test_generator_with_waveform() {
        let wf = Waveform {
            name: "Custom".to_string(),
            description: "Custom wave".to_string(),
            params: WaveformParams::default(),
            custom_points: None,
        };
        let gen = WaveformGenerator::with_waveform(wf);
        assert_eq!(gen.waveform().name, "Custom");
    }

    #[test]
    fn test_generator_set_waveform_resets_state() {
        let mut gen = WaveformGenerator::new();
        gen.phase = 0.5;
        gen.start();

        let wf = Waveform::default();
        gen.set_waveform(wf);

        assert_eq!(gen.phase, 0.0);
        assert!(gen.start_time.is_none());
    }

    #[test]
    fn test_generator_start_stop_reset() {
        let mut gen = WaveformGenerator::new();
        assert!(gen.start_time.is_none());

        gen.start();
        assert!(gen.start_time.is_some());

        gen.stop();
        assert!(gen.start_time.is_none());

        gen.start();
        gen.phase = 0.75;
        gen.reset();
        assert!(gen.start_time.is_none());
        assert_eq!(gen.phase, 0.0);
    }

    // === 波形输出测试 ===

    #[test]
    fn test_continuous_wave() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Continuous;
        gen.current_waveform.params.max_power = 75;

        let power = gen.current_power();
        assert_eq!(power, 75);
    }

    #[test]
    fn test_pulse_wave_on_phase() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Pulse;
        gen.current_waveform.params.min_power = 0;
        gen.current_waveform.params.max_power = 100;
        gen.current_waveform.params.duty_cycle = 50;

        // phase < duty -> max_power
        gen.phase = 0.25;
        assert_eq!(gen.current_power(), 100);
    }

    #[test]
    fn test_pulse_wave_off_phase() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Pulse;
        gen.current_waveform.params.min_power = 0;
        gen.current_waveform.params.max_power = 100;
        gen.current_waveform.params.duty_cycle = 50;

        // phase >= duty -> min_power
        gen.phase = 0.75;
        assert_eq!(gen.current_power(), 0);
    }

    #[test]
    fn test_square_wave_same_as_pulse() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Square;
        gen.current_waveform.params.min_power = 10;
        gen.current_waveform.params.max_power = 90;
        gen.current_waveform.params.duty_cycle = 30;

        gen.phase = 0.1; // < 0.3 duty
        assert_eq!(gen.current_power(), 90);

        gen.phase = 0.5; // >= 0.3 duty
        assert_eq!(gen.current_power(), 10);
    }

    #[test]
    fn test_sawtooth_wave() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Sawtooth;
        gen.current_waveform.params.min_power = 0;
        gen.current_waveform.params.max_power = 100;

        gen.phase = 0.0;
        assert_eq!(gen.current_power(), 0);

        gen.phase = 0.5;
        assert_eq!(gen.current_power(), 50);

        gen.phase = 1.0;
        assert_eq!(gen.current_power(), 100);
    }

    #[test]
    fn test_sawtooth_wave_with_offset() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Sawtooth;
        gen.current_waveform.params.min_power = 20;
        gen.current_waveform.params.max_power = 80;

        gen.phase = 0.0;
        assert_eq!(gen.current_power(), 20);

        gen.phase = 0.5;
        assert_eq!(gen.current_power(), 50);

        gen.phase = 1.0;
        assert_eq!(gen.current_power(), 80);
    }

    #[test]
    fn test_sine_wave_at_key_phases() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Sine;
        gen.current_waveform.params.min_power = 0;
        gen.current_waveform.params.max_power = 100;

        // phase=0: sin(0) = 0 → mid(50) + 0 = 50
        gen.phase = 0.0;
        assert_eq!(gen.current_power(), 50);

        // phase=0.25: sin(π/2) = 1 → mid(50) + 50 = 100
        gen.phase = 0.25;
        assert_eq!(gen.current_power(), 100);

        // phase=0.5: sin(π) ≈ 0 → ≈ 50
        gen.phase = 0.5;
        assert_eq!(gen.current_power(), 50);

        // phase=0.75: sin(3π/2) = -1 → mid(50) - 50 = 0
        gen.phase = 0.75;
        assert_eq!(gen.current_power(), 0);
    }

    #[test]
    fn test_triangle_wave() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Triangle;
        gen.current_waveform.params.min_power = 0;
        gen.current_waveform.params.max_power = 100;

        gen.phase = 0.0;
        assert_eq!(gen.current_power(), 0);

        gen.phase = 0.25;
        assert_eq!(gen.current_power(), 50);

        gen.phase = 0.5;
        assert_eq!(gen.current_power(), 100);

        gen.phase = 0.75;
        assert_eq!(gen.current_power(), 50);
    }

    #[test]
    fn test_breathing_wave_boundary_values() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Breathing;
        gen.current_waveform.params.min_power = 0;
        gen.current_waveform.params.max_power = 100;

        gen.phase = 0.0; // (0*2)^2 = 0 → 0
        assert_eq!(gen.current_power(), 0);

        gen.phase = 0.5; // (0.5*2)^2 = 1 → 100
        assert_eq!(gen.current_power(), 100);
    }

    #[test]
    fn test_fade_wave() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Fade;
        gen.current_waveform.params.min_power = 0;
        gen.current_waveform.params.max_power = 100;

        gen.phase = 0.0;
        assert_eq!(gen.current_power(), 0);

        gen.phase = 0.25; // 0.25*2 = 0.5 → 50
        assert_eq!(gen.current_power(), 50);

        gen.phase = 0.5; // 2 - 0.5*2 = 1.0 → 100
        assert_eq!(gen.current_power(), 100);

        gen.phase = 0.75; // 2 - 0.75*2 = 0.5 → 50
        assert_eq!(gen.current_power(), 50);
    }

    // === update 方法测试 ===

    #[test]
    fn test_update_advances_phase() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Continuous;
        gen.current_waveform.params.max_power = 50;
        gen.current_waveform.params.period_ms = 1000;

        let power = gen.update(500); // 500ms out of 1000ms period
        assert_eq!(power, 50);
        assert!((gen.phase - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_update_wraps_phase() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.period_ms = 1000;

        gen.phase = 0.9;
        let _ = gen.update(200); // 0.9 + 0.2 = 1.1 → wraps to 0.1
        assert!((gen.phase - 0.1).abs() < 0.001);
    }

    // === 自定义波形测试 ===

    #[test]
    fn test_custom_wave_no_points_returns_max() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Custom;
        gen.current_waveform.params.max_power = 60;
        gen.current_waveform.custom_points = None;

        assert_eq!(gen.current_power(), 60);
    }

    #[test]
    fn test_custom_wave_empty_points_returns_max() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Custom;
        gen.current_waveform.params.max_power = 60;
        gen.current_waveform.custom_points = Some(vec![]);

        assert_eq!(gen.current_power(), 60);
    }

    #[test]
    fn test_custom_wave_interpolation() {
        let mut gen = WaveformGenerator::new();
        gen.current_waveform.params.waveform_type = WaveformType::Custom;
        gen.current_waveform.custom_points = Some(vec![(0, 0), (500, 100), (1000, 0)]);

        // phase=0.0 → time=0 → value=0
        gen.phase = 0.0;
        assert_eq!(gen.current_power(), 0);

        // phase=0.25 → time=250 → between (0,0)-(500,100) → 50
        gen.phase = 0.25;
        assert_eq!(gen.current_power(), 50);

        // phase=0.5 → time=500 → value=100
        gen.phase = 0.5;
        assert_eq!(gen.current_power(), 100);

        // phase=0.75 → time=750 → between (500,100)-(1000,0) → 50
        gen.phase = 0.75;
        assert_eq!(gen.current_power(), 50);
    }

    // === 预设波形测试 ===

    #[test]
    fn test_preset_waveforms_not_empty() {
        let presets = WaveformGenerator::preset_waveforms();
        assert!(!presets.is_empty());
        assert!(presets.len() >= 5);
    }

    #[test]
    fn test_preset_waveforms_have_names() {
        let presets = WaveformGenerator::preset_waveforms();
        for preset in &presets {
            assert!(!preset.name.is_empty());
            assert!(!preset.description.is_empty());
        }
    }

    #[test]
    fn test_preset_waveforms_unique_names() {
        let presets = WaveformGenerator::preset_waveforms();
        let mut names: Vec<_> = presets.iter().map(|p| &p.name).collect();
        let before = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), before, "预设波形名称应该唯一");
    }
}
