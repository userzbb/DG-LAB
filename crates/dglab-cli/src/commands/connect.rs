//! 连接设备命令

use clap::Parser;
use tracing::info;

use super::DglabCli;
use dglab_core::device::{CoyoteDevice, Device};

/// 连接设备参数
#[derive(Parser, Debug)]
pub struct ConnectArgs {
    /// 设备 ID
    device_id: Option<String>,

    /// 设备名称（模糊匹配）
    #[arg(short, long)]
    name: Option<String>,

    /// 断开连接
    #[arg(short, long)]
    disconnect: bool,
}

/// 执行连接命令
pub async fn execute(app: &mut DglabCli, args: ConnectArgs) -> crate::error::Result<()> {
    if args.disconnect {
        if let Some(device_id) = args.device_id {
            info!("Disconnecting device: {}", device_id);
            app.session_manager().remove_device(&device_id).await?;
            println!("Disconnected from device: {}", device_id);
        } else {
            info!("Disconnecting all devices");
            app.session_manager().disconnect_all().await?;
            println!("Disconnected from all devices");
        }
        return Ok(());
    }

    // 先扫描获取设备列表
    info!("Scanning for devices...");
    
    let ble_manager = app.ble_manager().expect("BLE manager should be initialized");
    
    ble_manager.start_scan().await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    ble_manager.stop_scan().await?;

    let results = ble_manager.get_scan_results().await?;

    if results.is_empty() {
        println!("No devices found");
        return Ok(());
    }

    // 选择要连接的设备
    let selected_device = if let Some(device_id) = args.device_id {
        results.iter().find(|d| d.id == device_id)
    } else if let Some(name) = args.name {
        results
            .iter()
            .find(|d| d.name.to_lowercase().contains(&name.to_lowercase()))
    } else if results.len() == 1 {
        results.first()
    } else {
        // 显示设备列表让用户选择
        println!("\nAvailable devices:");
        for (i, device) in results.iter().enumerate() {
            println!("{}. {} ({})", i + 1, device.name, device.id);
        }

        print!("\nSelect device (1-{}): ", results.len());
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let index: usize = input.trim().parse()?;
        if index < 1 || index > results.len() {
            return Err(crate::error::CliError::InvalidInput(
                "Invalid selection".to_string(),
            ));
        }

        results.get(index - 1)
    };

    let Some(device_info) = selected_device else {
        println!("No matching device found");
        return Ok(());
    };

    info!(
        "Connecting to device: {} ({})",
        device_info.name, device_info.id
    );

    // 连接设备
    let device = ble_manager.connect(&device_info.id).await?;
    let mut coyote = CoyoteDevice::new(device_info.id.clone(), device_info.name.clone());
    coyote.set_protocol_device(device);
    coyote.connect().await?;

    // 添加到会话管理器
    app.session_manager().add_device(Box::new(coyote)).await?;

    println!("Connected to: {} ({})", device_info.name, device_info.id);

    Ok(())
}
