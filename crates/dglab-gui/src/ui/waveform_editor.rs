//! 波形编辑器

use eframe::egui;

/// 波形类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveformType {
    Continuous,
    Pulse,
    Sawtooth,
    Sine,
    Square,
    Triangle,
    Breathing,
    Fade,
}

/// 波形编辑器
pub struct WaveformEditor {
    /// 当前波形类型
    waveform_type: WaveformType,
    /// 频率
    frequency: u16,
    /// 脉宽
    pulse_width: u16,
    /// 最小强度
    min_power: u8,
    /// 最大强度: u8,
    max_power: u8,
    /// 周期 (ms)
    period: u32,
    /// 占空比
    duty_cycle: u8,
}

impl Default for WaveformEditor {
    fn default() -> Self {
        Self {
            waveform_type: WaveformType::Continuous,
            frequency: 100,
            pulse_width: 200,
            min_power: 0,
            max_power: 100,
            period: 5000,
            duty_cycle: 50,
        }
    }
}

impl WaveformEditor {
    /// 渲染 UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Waveform Editor");
        ui.add_space(10.0);

        ui.columns(2, |columns| {
            // 左侧：波形选择和参数
            columns[0].group(|ui| {
                ui.heading("Waveform Type");
                ui.add_space(5.0);

                ui.vertical(|ui| {
                    ui.radio_value(&mut self.waveform_type, WaveformType::Continuous, "🔹 Continuous");
                    ui.radio_value(&mut self.waveform_type, WaveformType::Pulse, "🔸 Pulse");
                    ui.radio_value(&mut self.waveform_type, WaveformType::Sine, "〰️ Sine");
                    ui.radio_value(&mut self.waveform_type, WaveformType::Square, "▪️ Square");
                    ui.radio_value(&mut self.waveform_type, WaveformType::Triangle, "🔺 Triangle");
                    ui.radio_value(&mut self.waveform_type, WaveformType::Sawtooth, "📐 Sawtooth");
                    ui.radio_value(&mut self.waveform_type, WaveformType::Breathing, "🫁 Breathing");
                    ui.radio_value(&mut self.waveform_type, WaveformType::Fade, "🌅 Fade");
                });

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);

                ui.heading("Parameters");
                ui.add_space(5.0);

                ui.add(egui::Slider::new(&mut self.frequency, 10..=500).text("Frequency (Hz)"));
                ui.add(egui::Slider::new(&mut self.pulse_width, 50..=500).text("Pulse Width (μs)"));
                ui.add(egui::Slider::new(&mut self.period, 1000..=10000).text("Period (ms)"));
                ui.add(egui::Slider::new(&mut self.duty_cycle, 10..=90).text("Duty Cycle (%)"));

                ui.add_space(10.0);
                ui.heading("Power Range");
                ui.add(egui::Slider::new(&mut self.min_power, 0..=50).text("Min Power"));
                ui.add(egui::Slider::new(&mut self.max_power, 50..=100).text("Max Power"));
            });

            // 右侧：波形预览
            columns[1].group(|ui| {
                ui.heading("Waveform Preview");
                ui.add_space(10.0);

                // 绘制波形预览区域
                let (rect, response) = ui.allocate_at_least(
                    egui::vec2(ui.available_width(), 250.0),
                    egui::Sense::hover(),
                );

                let visuals = ui.style().visuals.clone();
                ui.painter().rect_filled(rect, 5.0, visuals.extreme_bg_color);
                ui.painter().rect_stroke(rect, 5.0, (1.0, visuals.faint_bg_color));

                // 绘制网格
                let painter = ui.painter();
                let color = visuals.faint_bg_color;

                for i in 0..=5 {
                    let y = rect.top() + rect.height() * i as f32 / 5.0;
                    painter.line_segment(
                        [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                        (1.0, color),
                    );
                }

                // 绘制波形
                self.draw_waveform(painter, rect);
            });
        });
    }

    /// 绘制波形
    fn draw_waveform(&self, painter: &egui::Painter, rect: egui::Rect) {
        let color = egui::Color32::from_rgb(100, 150, 255);
        let points: Vec<egui::Pos2> = (0..=100)
            .map(|i| {
                let t = i as f32 / 100.0;
                let x = rect.left() + t * rect.width();
                let y = self.waveform_y(t, rect);
                egui::pos2(x, y)
            })
            .collect();

        painter.line(points, (2.0, color));
    }

    /// 计算波形 Y 坐标
    fn waveform_y(&self, t: f32, rect: egui::Rect) -> f32 {
        let min_p = self.min_power as f32 / 100.0;
        let max_p = self.max_power as f32 / 100.0;

        let value = match self.waveform_type {
            WaveformType::Continuous => max_p,
            WaveformType::Pulse => {
                let duty = self.duty_cycle as f32 / 100.0;
                if t < duty {
                    max_p
                } else {
                    min_p
                }
            }
            WaveformType::Sine => {
                let mid = (min_p + max_p) / 2.0;
                let amp = (max_p - min_p) / 2.0;
                mid + amp * (t * 2.0 * std::f32::consts::PI).sin()
            }
            WaveformType::Square => {
                let duty = self.duty_cycle as f32 / 100.0;
                if t < duty {
                    max_p
                } else {
                    min_p
                }
            }
            WaveformType::Triangle => {
                if t < 0.5 {
                    min_p + t * 2.0 * (max_p - min_p)
                } else {
                    max_p - (t - 0.5) * 2.0 * (max_p - min_p)
                }
            }
            WaveformType::Sawtooth => min_p + t * (max_p - min_p),
            WaveformType::Breathing => {
                let t2 = if t < 0.5 {
                    (t * 2.0).powi(2)
                } else {
                    1.0 - ((t - 0.5) * 2.0).powi(2)
                };
                min_p + t2 * (max_p - min_p)
            }
            WaveformType::Fade => {
                let t2 = if t < 0.5 {
                    t * 2.0
                } else {
                    2.0 - t * 2.0
                };
                min_p + t2 * (max_p - min_p)
            }
        };

        rect.bottom() - value * rect.height()
    }
}
