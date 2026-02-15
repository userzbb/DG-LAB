//! WiFi 连接命令

use clap::Parser;
use tracing::{debug, info};

use super::DglabCli;
use dglab_core::device::{Device, WsCoyoteDevice};

/// WiFi 子命令
#[derive(Parser, Debug)]
pub struct WifiArgs {
    #[command(subcommand)]
    command: WifiCommand,
}

/// WiFi 子命令
#[derive(Parser, Debug)]
enum WifiCommand {
    /// 连接 WiFi 设备（显示二维码）
    Connect {
        /// 自定义服务器地址（可选）
        #[arg(short, long)]
        server: Option<String>,
    },
    /// 断开 WiFi 设备
    Disconnect,
    /// 显示连接状态
    Status,
    /// 控制 WiFi 设备强度
    Control {
        /// 通道 (A/B)
        #[arg(short, long)]
        channel: Option<String>,
        /// 强度值
        #[arg(short, long)]
        power: Option<u8>,
        /// 增加强度
        #[arg(long)]
        up: Option<u8>,
        /// 减少强度
        #[arg(long)]
        down: Option<u8>,
    },
}

/// 执行 WiFi 命令
pub async fn execute(app: &mut DglabCli, args: WifiArgs) -> crate::error::Result<()> {
    match args.command {
        WifiCommand::Connect { server } => {
            info!("Connecting to WiFi...");

            let device_id = uuid::Uuid::new_v4().to_string();
            let device_name = "WiFi-Coyote".to_string();

            // 先创建 WsCoyoteDevice，连接并显示二维码
            let mut wifi_device = if let Some(srv) = server {
                WsCoyoteDevice::with_server(device_id.clone(), device_name.clone(), srv)
            } else {
                WsCoyoteDevice::new(device_id.clone(), device_name.clone())
            };

            // 连接
            wifi_device.connect().await?;

            // 等待获取 clientId 和二维码
            println!("\nConnecting to server...");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            if let Some(qr_url) = wifi_device.qr_url().await {
                println!("\nQR Code URL:");
                println!("{}", qr_url);
                println!("\nPlease scan this QR code with DG-LAB APP to bind.");
                println!("Waiting for binding... (press Ctrl+C to cancel)\n");
            }

            // 添加到会话管理器
            app.session_manager()
                .add_device(Box::new(wifi_device))
                .await?;

            println!("WiFi device connected and added to session.");
        }

        WifiCommand::Disconnect => {
            info!("Disconnecting WiFi...");

            let devices = app.session_manager().list_devices().await;

            // 断开所有设备
            for device_id in devices {
                app.session_manager().remove_device(&device_id).await?;
            }

            println!("WiFi device disconnected.");
        }

        WifiCommand::Status => {
            let devices = app.session_manager().list_devices().await;

            println!("\nWiFi Status:");
            println!("{}", "-".repeat(50));

            if devices.is_empty() {
                println!("No WiFi devices connected.");
            } else {
                for device_id in devices {
                    if let Some(device) = app.session_manager().get_device(&device_id).await {
                        let device = device.read().await;
                        println!("Device: {}", device.name());
                        println!("ID:     {}", device.id());
                        println!("State:  {:?}", device.state());
                        println!("Power A: {}", device.get_power(0));
                        println!("Power B: {}", device.get_power(1));
                    }
                }
            }
            println!();
        }

        WifiCommand::Control {
            channel,
            power,
            up,
            down,
        } => {
            let devices = app.session_manager().list_devices().await;

            if devices.is_empty() {
                println!("No WiFi devices connected.");
                return Ok(());
            }

            let device_id = devices.first().unwrap();
            let Some(device) = app.session_manager().get_device(device_id).await else {
                println!("Device not found.");
                return Ok(());
            };

            let mut device = device.write().await;

            // 确定要操作的通道
            let channels = match channel {
                Some(c) => match c.to_lowercase().as_str() {
                    "a" => vec![0],
                    "b" => vec![1],
                    _ => {
                        println!("Invalid channel: use A or B");
                        return Ok(());
                    }
                },
                None => vec![0, 1],
            };

            // 执行操作
            for ch in channels {
                if let Some(p) = power {
                    debug!("Setting channel {} power to {}", ch, p);
                    device.set_power(ch, p).await?;
                    println!(
                        "Channel {} power set to {}",
                        if ch == 0 { "A" } else { "B" },
                        p
                    );
                } else if let Some(delta) = up {
                    let current = device.get_power(ch);
                    let new_power = current.saturating_add(delta).min(100);
                    device.set_power(ch, new_power).await?;
                    println!(
                        "Channel {} power increased to {}",
                        if ch == 0 { "A" } else { "B" },
                        new_power
                    );
                } else if let Some(delta) = down {
                    let current = device.get_power(ch);
                    let new_power = current.saturating_sub(delta);
                    device.set_power(ch, new_power).await?;
                    println!(
                        "Channel {} power decreased to {}",
                        if ch == 0 { "A" } else { "B" },
                        new_power
                    );
                } else {
                    // 显示当前强度
                    println!(
                        "Channel {} power: {}",
                        if ch == 0 { "A" } else { "B" },
                        device.get_power(ch)
                    );
                }
            }
        }
    }

    Ok(())
}
