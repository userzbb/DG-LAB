//! 设备面板

use eframe::egui;

/// 设备信息
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// 设备 ID
    pub id: String,
    /// 设备名称
    pub name: String,
    /// 信号强度
    pub rssi: Option<i16>,
    /// 是否已连接
    pub connected: bool,
}

/// 设备面板
pub struct DevicePanel {
    /// 扫描中
    scanning: bool,
    /// 发现的设备
    devices: Vec<DeviceInfo>,
    /// 选中的设备
    selected_device: Option<usize>,
}

impl Default for DevicePanel {
    fn default() -> Self {
        Self {
            scanning: false,
            devices: Vec::new(),
            selected_device: None,
        }
    }
}

impl DevicePanel {
    /// 渲染 UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Device Manager");
        ui.add_space(10.0);

        // 扫描按钮
        ui.horizontal(|ui| {
            if ui.button(if self.scanning { "⏹ Stop Scan" } else { "🔍 Scan for Devices" }).clicked() {
                self.scanning = !self.scanning;
                if self.scanning {
                    self.devices.clear();
                    self.simulate_scan();
                }
            }

            if self.scanning {
                ui.spinner();
                ui.label("Scanning...");
            }
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        // 设备列表
        ui.heading("Available Devices");
        ui.add_space(5.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            if self.devices.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label("No devices found\nClick 'Scan for Devices' to search");
                });
            } else {
                for (i, device) in self.devices.iter().enumerate() {
                    let is_selected = self.selected_device == Some(i);
                    let response = ui.selectable_label(is_selected, format!("📡 {}", device.name));

                    if response.clicked() {
                        self.selected_device = Some(i);
                    }

                    ui.indent(format!("device_{}", i), |ui| {
                        ui.label(format!("ID: {}", device.id));
                        if let Some(rssi) = device.rssi {
                            ui.label(format!("Signal: {} dBm", rssi));
                        }
                        ui.label(if device.connected { "Status: Connected" } else { "Status: Disconnected" });
                    });
                    ui.add_space(5.0);
                }
            }
        });

        ui.add_space(10.0);
        ui.separator();

        // 连接按钮
        ui.horizontal(|ui| {
            let has_selection = self.selected_device.is_some();

            if ui.add_enabled(has_selection, egui::Button::new("🔌 Connect")).clicked() {
                if let Some(i) = self.selected_device {
                    if let Some(device) = self.devices.get_mut(i) {
                        device.connected = true;
                    }
                }
            }

            if ui.add_enabled(has_selection, egui::Button::new("⏏️ Disconnect")).clicked() {
                if let Some(i) = self.selected_device {
                    if let Some(device) = self.devices.get_mut(i) {
                        device.connected = false;
                    }
                }
            }
        });
    }

    /// 模拟扫描（演示用）
    fn simulate_scan(&mut self) {
        self.devices = vec![
            DeviceInfo {
                id: "device_001".to_string(),
                name: "DG-LAB Coyote".to_string(),
                rssi: Some(-65),
                connected: false,
            },
            DeviceInfo {
                id: "device_002".to_string(),
                name: "DG-LAB 2.0".to_string(),
                rssi: Some(-78),
                connected: false,
            },
        ];
    }
}
