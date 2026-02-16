//! DG-LAB Tauri GUI 应用

mod commands;
mod events;
mod state;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::state::AppState;

/// 初始化日志系统
fn init_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,dglab=debug")),
        )
        .init();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    init_logging();

    // 创建应用状态
    let app_state = AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Device commands
            commands::device::scan_ble_devices,
            commands::device::connect_ble_device,
            commands::device::connect_device,
            commands::device::disconnect_device,
            commands::device::get_device_info,
            commands::device::get_device_state,
            // Power commands
            commands::power::set_power,
            commands::power::start_device,
            commands::power::stop_device,
            commands::power::emergency_stop,
            // Session commands
            commands::session::get_session_info,
            commands::session::list_devices,
            // WiFi commands
            commands::wifi::wifi_connect,
            commands::wifi::wifi_check_binding,
            commands::wifi::wifi_cancel,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
