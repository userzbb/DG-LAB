//! 设置面板

use eframe::egui;

/// 设置面板
pub struct SettingsPanel {
    /// 主题
    theme: Theme,
    /// 自动重连
    auto_reconnect: bool,
    /// 安全限制
    safety_limit: u8,
    /// 语言
    language: String,
    /// 显示高级选项
    show_advanced: bool,
    /// 日志级别
    log_level: String,
}

/// 主题
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Theme {
    Dark,
    Light,
    System,
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            auto_reconnect: true,
            safety_limit: 50,
            language: "English".to_string(),
            show_advanced: false,
            log_level: "Info".to_string(),
        }
    }
}

impl SettingsPanel {
    /// 渲染 UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            // 常规设置
            ui.collapsing("General", |ui| {
                ui.group(|ui| {
                    ui.label("Theme:");
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.theme, Theme::Dark, "🌙 Dark");
                        ui.selectable_value(&mut self.theme, Theme::Light, "☀️ Light");
                        ui.selectable_value(&mut self.theme, Theme::System, "💻 System");
                    });

                    ui.add_space(10.0);

                    ui.label("Language:");
                    egui::ComboBox::from_label("")
                        .selected_text(&self.language)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.language, "English".to_string(), "English");
                            ui.selectable_value(&mut self.language, "中文".to_string(), "中文");
                            ui.selectable_value(&mut self.language, "日本語".to_string(), "日本語");
                        });

                    ui.add_space(10.0);

                    ui.checkbox(&mut self.auto_reconnect, "🔄 Auto Reconnect");
                });
            });

            ui.add_space(10.0);

            // 安全设置
            ui.collapsing("Safety", |ui| {
                ui.group(|ui| {
                    ui.label("⚠️ Safety Limit (Max Power):");
                    ui.add(egui::Slider::new(&mut self.safety_limit, 10..=100).text("%"));
                    ui.label(format!("All channels will be limited to {}%", self.safety_limit));

                    ui.add_space(10.0);

                    ui.checkbox(&mut self.show_advanced, "Show Advanced Safety Options");

                    if self.show_advanced {
                        ui.add_space(5.0);
                        ui.weak("⚠️ Advanced settings - use with caution");
                        ui.checkbox(&mut false, "Allow exceeding safety limit temporarily");
                        ui.checkbox(&mut false, "Enable emergency stop button");
                    }
                });
            });

            ui.add_space(10.0);

            // 日志设置
            ui.collapsing("Logging", |ui| {
                ui.group(|ui| {
                    ui.label("Log Level:");
                    egui::ComboBox::from_label("")
                        .selected_text(&self.log_level)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.log_level, "Error".to_string(), "Error");
                            ui.selectable_value(&mut self.log_level, "Warn".to_string(), "Warn");
                            ui.selectable_value(&mut self.log_level, "Info".to_string(), "Info");
                            ui.selectable_value(&mut self.log_level, "Debug".to_string(), "Debug");
                            ui.selectable_value(&mut self.log_level, "Trace".to_string(), "Trace");
                        });
                });
            });

            ui.add_space(10.0);

            // 关于
            ui.collapsing("About", |ui| {
                ui.group(|ui| {
                    ui.heading("DG-LAB Controller");
                    ui.label("Version: 0.1.0");
                    ui.label("License: MIT OR Apache-2.0");
                    ui.add_space(10.0);
                    ui.label("A cross-platform controller for DG-LAB devices.");
                    ui.hyperlink("https://github.com/your-org/dglab-rs");
                });
            });

            ui.add_space(20.0);

            // 操作按钮
            ui.horizontal(|ui| {
                if ui.button("🔄 Reset to Defaults").clicked() {
                    *self = Self::default();
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("💾 Save Settings").clicked() {
                        // TODO: 保存设置
                    }
                });
            });
        });
    }
}
