//! WiFi 连接命令

use clap::Parser;
use qrcode::{render::unicode, QrCode};
use tracing::{debug, info};

use super::DglabCli;
use dglab_core::device::{Device, DeviceState, WsCoyoteDevice};

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

            println!("\n╔══════════════════════════════════════════════════════╗");
            println!("║           DG-LAB WiFi 连接向导                      ║");
            println!("╚══════════════════════════════════════════════════════╝\n");

            // 先创建 WsCoyoteDevice，连接并显示二维码
            let mut wifi_device = if let Some(srv) = &server {
                println!("📡 正在连接到自定义服务器: {}", srv);
                WsCoyoteDevice::with_server(device_id.clone(), device_name.clone(), srv.clone())
            } else {
                println!("📡 正在连接到官方服务器: wss://ws.dungeon-lab.cn");
                WsCoyoteDevice::new(device_id.clone(), device_name.clone())
            };

            // 连接到 WebSocket 服务器
            print!("⏳ 建立 WebSocket 连接... ");
            wifi_device.connect().await?;
            println!("✓");

            // 等待获取 clientId
            print!("⏳ 等待服务器分配 ID... ");
            let mut retries = 0;
            let qr_url = loop {
                if let Some(url) = wifi_device.qr_url().await {
                    println!("✓");
                    break url;
                }
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                retries += 1;
                if retries > 25 {
                    // 5 秒超时
                    println!("✗");
                    println!("\n❌ 错误: 超时未收到服务器 clientId");
                    return Ok(());
                }
            };

            // 显示二维码
            println!("\n╔══════════════════════════════════════════════════════╗");
            println!("║              📱 请使用 DG-LAB APP 扫码               ║");
            println!("╚══════════════════════════════════════════════════════╝\n");

            // 生成并显示 ASCII 二维码
            if let Ok(code) = QrCode::new(&qr_url) {
                let qr_string = code
                    .render::<unicode::Dense1x2>()
                    .dark_color(unicode::Dense1x2::Light)
                    .light_color(unicode::Dense1x2::Dark)
                    .build();
                println!("{}", qr_string);
            } else {
                println!("⚠️  无法生成二维码，请手动输入以下 URL：");
            }

            println!("\n🔗 连接 URL:");
            println!("   {}\n", qr_url);

            // 等待绑定
            print!("⏳ 等待 APP 扫码绑定");
            let mut dots = 0;
            loop {
                if wifi_device.is_bound().await {
                    println!(" ✓\n");
                    break;
                }

                // 检查设备状态
                match wifi_device.state() {
                    DeviceState::Connected => {
                        // 继续等待
                    }
                    DeviceState::Disconnected => {
                        println!(" ✗\n");
                        println!("❌ 连接已断开");
                        return Ok(());
                    }
                    _ => {}
                }

                // 显示动画
                print!(".");
                if let Err(e) = std::io::Write::flush(&mut std::io::stdout()) {
                    debug!("Failed to flush stdout: {}", e);
                }
                dots += 1;
                if dots > 60 {
                    // 每行最多 60 个点
                    print!("\n⏳ 仍在等待 APP 扫码绑定");
                    dots = 0;
                }

                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }

            // 添加到会话管理器
            app.session_manager()
                .add_device(Box::new(wifi_device))
                .await?;

            println!("╔══════════════════════════════════════════════════════╗");
            println!("║                  ✅ 绑定成功！                      ║");
            println!("╚══════════════════════════════════════════════════════╝\n");
            println!("📱 设备已就绪，可以开始控制");
            println!("💡 提示: 使用 'dglab wifi control' 命令控制设备");
            println!("💡 提示: 使用 'dglab wifi status' 查看设备状态\n");

            // 保持连接，等待用户中断
            println!("⚡ WiFi 连接已建立，按 Ctrl+C 退出...\n");
            tokio::signal::ctrl_c().await?;
            println!("\n👋 正在断开连接...");
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
