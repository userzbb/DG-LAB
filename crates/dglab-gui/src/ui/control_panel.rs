//! 控制面板

use eframe::egui;

/// 控制面板
pub struct ControlPanel {
    /// 通道 A 强度
    power_a: u8,
    /// 通道 B 强度
    power_b: u8,
    /// 通道 A 最大值
    max_a: u8,
    /// 通道 B 最大值
    max_b: u8,
    /// 通道 A 启用
    enabled_a: bool,
    /// 通道 B 启用: bool,
    enabled_b: bool,
    /// 是否运行中
    running: bool,
    /// 同步两个通道
    sync: bool,
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self {
            power_a: 0,
            power_b: 0,
            max_a: 100,
            max_b: 100,
            enabled_a: true,
            enabled_b: true,
            running: false,
            sync: false,
        }
    }
}

impl ControlPanel {
    /// 渲染 UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Device Control");
        ui.add_space(10.0);

        // 运行控制
        ui.horizontal(|ui| {
            let (text, color) = if self.running {
                ("⏹ Stop", egui::Color32::RED)
            } else {
                ("▶️ Start", egui::Color32::GREEN)
            };

            if ui.add(egui::Button::new(text).fill(color)).clicked() {
                self.running = !self.running;
            }

            ui.checkbox(&mut self.sync, "🔗 Sync Channels");
        });

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(15.0);

        // 通道控制
        ui.columns(2, |columns| {
            // 通道 A
            columns[0].group(|ui| {
                ui.heading("Channel A");
                ui.add_space(10.0);

                ui.checkbox(&mut self.enabled_a, "Enabled");

                ui.add_space(10.0);

                ui.label("Power:");
                ui.add(
                    egui::Slider::new(&mut self.power_a, 0..=self.max_a)
                        .orientation(egui::SliderOrientation::Vertical)
                        .text("")
                        .step_by(1.0),
                );

                ui.horizontal(|ui| {
                    ui.label(format!("{}%", self.power_a));
                    if ui.button("0").clicked() {
                        self.power_a = 0;
                    }
                    if ui.button("25").clicked() {
                        self.power_a = 25;
                    }
                    if ui.button("50").clicked() {
                        self.power_a = 50;
                    }
                });

                ui.add_space(10.0);
                ui.label("Max Limit:");
                ui.add(egui::Slider::new(&mut self.max_a, 10..=100).text(""));
            });

            // 通道 B
            columns[1].group(|ui| {
                ui.heading("Channel B");
                ui.add_space(10.0);

                ui.checkbox(&mut self.enabled_b, "Enabled");

                ui.add_space(10.0);

                ui.label("Power:");
                ui.add(
                    egui::Slider::new(&mut self.power_b, 0..=self.max_b)
                        .orientation(egui::SliderOrientation::Vertical)
                        .text("")
                        .step_by(1.0),
                );

                ui.horizontal(|ui| {
                    ui.label(format!("{}%", self.power_b));
                    if ui.button("0").clicked() {
                        self.power_b = 0;
                    }
                    if ui.button("25").clicked() {
                        self.power_b = 25;
                    }
                    if ui.button("50").clicked() {
                        self.power_b = 50;
                    }
                });

                ui.add_space(10.0);
                ui.label("Max Limit:");
                ui.add(egui::Slider::new(&mut self.max_b, 10..=100).text(""));
            });
        });

        // 同步逻辑
        if self.sync {
            self.power_b = self.power_a;
        }
    }
}
