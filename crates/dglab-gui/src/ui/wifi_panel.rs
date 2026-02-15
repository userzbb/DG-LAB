//! WiFi 连接面板

use eframe::egui;

/// WiFi 面板
pub struct WifiPanel {
    /// 是否已连接
    connected: bool,
    /// 是否已绑定
    bound: bool,
    /// 二维码 URL
    qr_url: Option<String>,
    /// 当前强度 A
    power_a: u8,
    /// 当前强度 B
    power_b: u8,
    /// 最大强度限制
    max_power_a: u8,
    max_power_b: u8,
    /// 同步通道
    sync_channels: bool,
    /// 错误消息
    error: Option<String>,
    /// 连接中状态
    connecting: bool,
    /// 自定义服务器地址
    custom_server: String,
    /// 使用自定义服务器
    use_custom_server: bool,
}

impl Default for WifiPanel {
    fn default() -> Self {
        Self {
            connected: false,
            bound: false,
            qr_url: None,
            power_a: 0,
            power_b: 0,
            max_power_a: 100,
            max_power_b: 100,
            sync_channels: true,
            error: None,
            connecting: false,
            custom_server: String::from("ws://localhost:8080"),
            use_custom_server: false,
        }
    }
}

impl WifiPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("WiFi Connection");
        ui.add_space(10.0);

        // 服务器设置
        ui.group(|ui| {
            ui.heading("Server Settings");
            ui.add_space(8.0);

            ui.checkbox(&mut self.use_custom_server, "Use custom server");

            if self.use_custom_server {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Server URL:");
                    ui.text_edit_singleline(&mut self.custom_server);
                });
            } else {
                ui.label("Official server: wss://ws.dungeon-lab.cn");
            }
        });

        ui.add_space(10.0);

        // 连接/断开按钮
        ui.horizontal(|ui| {
            if !self.connected {
                if ui.add_enabled(!self.connecting, egui::Button::new("🔌 Connect")).clicked() {
                    self.connecting = true;
                    self.error = None;
                    // TODO: 发起连接
                }
                if self.connecting {
                    ui.spinner();
                    ui.label("Connecting...");
                }
            } else {
                if ui.button("🔌 Disconnect").clicked() {
                    // TODO: 断开连接
                    self.connected = false;
                    self.bound = false;
                    self.qr_url = None;
                    self.power_a = 0;
                    self.power_b = 0;
                }
            }
        });

        ui.add_space(10.0);
        ui.separator();

        // 显示二维码
        if self.connected && !self.bound {
            ui.group(|ui| {
                ui.heading("📱 Scan QR Code");
                ui.add_space(10.0);
                ui.label("Scan this QR code with DG-LAB APP to bind:");
                ui.add_space(8.0);

                if let Some(url) = &self.qr_url {
                    ui.label(url);
                    // TODO: 显示二维码图像
                    ui.label("[QR Code will appear here]");
                } else {
                    ui.label("Waiting for QR code...");
                    ui.spinner();
                }

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Waiting for APP to bind...");
                });
            });
        }

        // 绑定状态
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            let status_text = if self.bound {
                "✅ Bound to APP"
            } else if self.connected {
                "⏳ Waiting for APP..."
            } else {
                "❌ Disconnected"
            };
            let status_color = if self.bound {
                egui::Color32::GREEN
            } else if self.connected {
                egui::Color32::YELLOW
            } else {
                egui::Color32::GRAY
            };
            ui.label("Status:");
            ui.colored_label(status_color, status_text);
        });

        // 强度控制（绑定后显示）
        if self.bound {
            ui.add_space(20.0);
            ui.separator();
            ui.heading("🎛️ Power Control");

            // 同步开关
            ui.add_space(10.0);
            ui.checkbox(&mut self.sync_channels, "🔗 Sync Channels");

            ui.add_space(10.0);

            // 通道 A 控制
            ui.group(|ui| {
                ui.heading("Channel A");
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Power:");
                    ui.add(egui::DragValue::new(&mut self.power_a).clamp_range(0..=self.max_power_a).speed(1));
                    ui.label(format!("/ {}", self.max_power_a));
                });

                ui.add_space(8.0);
                ui.add(egui::Slider::new(&mut self.power_a, 0..=self.max_power_a).text(""));

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("0").clicked() {
                        self.power_a = 0;
                    }
                    if ui.button("25").clicked() {
                        self.power_a = 25;
                    }
                    if ui.button("50").clicked() {
                        self.power_a = 50;
                    }
                    if ui.button("75").clicked() {
                        self.power_a = 75;
                    }
                    if ui.button("100").clicked() {
                        self.power_a = self.max_power_a;
                    }
                });

                ui.add_space(8.0);
                ui.label("Max Limit:");
                ui.add(egui::Slider::new(&mut self.max_power_a, 10..=100).text(""));
            });

            // 通道 B 控制
            ui.add_space(10.0);
            ui.group(|ui| {
                ui.heading("Channel B");
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Power:");
                    ui.add(egui::DragValue::new(&mut self.power_b).clamp_range(0..=self.max_power_b).speed(1));
                    ui.label(format!("/ {}", self.max_power_b));
                });

                ui.add_space(8.0);
                ui.add(egui::Slider::new(&mut self.power_b, 0..=self.max_power_b).text(""));

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("0").clicked() {
                        self.power_b = 0;
                    }
                    if ui.button("25").clicked() {
                        self.power_b = 25;
                    }
                    if ui.button("50").clicked() {
                        self.power_b = 50;
                    }
                    if ui.button("75").clicked() {
                        self.power_b = 75;
                    }
                    if ui.button("100").clicked() {
                        self.power_b = self.max_power_b;
                    }
                });

                ui.add_space(8.0);
                ui.label("Max Limit:");
                ui.add(egui::Slider::new(&mut self.max_power_b, 10..=100).text(""));
            });

            // 同步处理
            if self.sync_channels {
                if self.power_a != self.power_b {
                    self.power_b = self.power_a;
                }
                if self.max_power_a != self.max_power_b {
                    self.max_power_b = self.max_power_a;
                }
            }

            // 快速按钮
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("▶️ Start").clicked() {
                    // TODO: 开始
                }
                if ui.button("⏹️ Stop").clicked() {
                    self.power_a = 0;
                    self.power_b = 0;
                    // TODO: 停止
                }
            });
        }

        // 错误显示
        if let Some(error) = &self.error {
            ui.add_space(10.0);
            ui.group(|ui| {
                ui.colored_label(egui::Color32::RED, "⚠️ Error");
                ui.label(error);
            });
        }
    }
}
