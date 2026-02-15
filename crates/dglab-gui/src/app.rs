//! GUI 应用状态

use eframe::egui;

/// 当前标签页
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    /// 设备 (BLE)
    Devices,
    /// WiFi
    Wifi,
    /// 控制
    Control,
    /// 波形
    Waveform,
    /// 预设
    Presets,
    /// 设置
    Settings,
}

/// GUI 应用
pub struct DglabApp {
    /// 当前标签页
    current_tab: Tab,
    /// 设备面板
    device_panel: ui::device_panel::DevicePanel,
    /// WiFi 面板
    wifi_panel: ui::wifi_panel::WifiPanel,
    /// 控制面板
    control_panel: ui::control_panel::ControlPanel,
    /// 波形编辑器
    waveform_editor: ui::waveform_editor::WaveformEditor,
    /// 设置面板
    settings_panel: ui::settings_panel::SettingsPanel,
}

impl Default for DglabApp {
    fn default() -> Self {
        Self {
            current_tab: Tab::Devices,
            device_panel: ui::device_panel::DevicePanel::default(),
            wifi_panel: ui::wifi_panel::WifiPanel::default(),
            control_panel: ui::control_panel::ControlPanel::default(),
            waveform_editor: ui::waveform_editor::WaveformEditor::default(),
            settings_panel: ui::settings_panel::SettingsPanel::default(),
        }
    }
}

impl eframe::App for DglabApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部标签栏
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Devices, "📡 BLE");
                ui.selectable_value(&mut self.current_tab, Tab::Wifi, "📶 WiFi");
                ui.selectable_value(&mut self.current_tab, Tab::Control, "🎛️ Control");
                ui.selectable_value(&mut self.current_tab, Tab::Waveform, "📈 Waveform");
                ui.selectable_value(&mut self.current_tab, Tab::Presets, "💾 Presets");
                ui.selectable_value(&mut self.current_tab, Tab::Settings, "⚙️ Settings");
            });
        });

        // 主内容区
        egui::CentralPanel::default().show(ctx, |ui| match self.current_tab {
            Tab::Devices => {
                self.device_panel.ui(ui);
            }
            Tab::Wifi => {
                self.wifi_panel.ui(ui);
            }
            Tab::Control => {
                self.control_panel.ui(ui);
            }
            Tab::Waveform => {
                self.waveform_editor.ui(ui);
            }
            Tab::Presets => {
                ui.heading("Presets");
                ui.label("Preset management coming soon...");
            }
            Tab::Settings => {
                self.settings_panel.ui(ui);
            }
        });

        // 底部状态栏
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.colored_label(egui::Color32::YELLOW, "Disconnected");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("DG-LAB Controller v0.1.0");
                });
            });
        });
    }
}
