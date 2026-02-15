//! DG-LAB GUI 应用

use eframe::egui;
use tracing::info;

mod app;
mod ui;

use app::DglabApp;

fn main() -> eframe::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    info!("Starting DG-LAB GUI");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 700.0)),
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };

    eframe::run_native(
        "DG-LAB Controller",
        options,
        Box::new(|_cc| Box::<DglabApp>::default()),
    )
}
