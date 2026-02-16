//! 桥接模式命令
//!
//! 通过 BLE 连接设备并同时连接到 WebSocket 服务器，充当 APP 角色

use clap::Args;
use tracing::{error, info};

use crate::commands::DglabCli;
use crate::error::{CliError, Result};

use dglab_core::device::{BleWsBridgeDevice, Device};
use dglab_protocol::wifi::OFFICIAL_SERVER;

/// 桥接模式参数
#[derive(Debug, Args)]
pub struct BridgeArgs {
    /// 设备名称（如：47L121000）
    #[arg(short, long)]
    pub device: String,

    /// WebSocket 服务器地址
    #[arg(short, long, default_value = OFFICIAL_SERVER)]
    pub server: String,

    /// 详细输出
    #[arg(short, long)]
    pub verbose: bool,
}

/// 执行桥接模式
pub async fn execute(cli: &mut DglabCli, args: BridgeArgs) -> Result<()> {
    println!("🌉 启动 BLE-WebSocket 桥接模式");
    println!();

    // 1. 先扫描 BLE 设备（找到目标设备）
    println!("📡 步骤 1: 扫描 BLE 设备...");
    let ble_manager = cli
        .ble_manager()
        .ok_or_else(|| CliError::Other("BLE manager not initialized".to_string()))?;

    ble_manager.start_scan().await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let scan_results = ble_manager.get_scan_results().await?;
    let target_device = scan_results
        .iter()
        .find(|d| d.name.contains(&args.device))
        .ok_or_else(|| CliError::DeviceNotFound(args.device.clone()))?;

    println!("✓ 找到设备: {} ({})", target_device.name, target_device.id);
    println!();

    // 2. 创建桥接设备
    println!("🔧 步骤 2: 创建桥接设备...");
    let mut bridge_device = if args.server == OFFICIAL_SERVER {
        BleWsBridgeDevice::new(
            format!("bridge-{}", target_device.id),
            format!("Bridge-{}", target_device.name),
            target_device.id.clone(),
            target_device.name.clone(),
        )
    } else {
        BleWsBridgeDevice::with_server(
            format!("bridge-{}", target_device.id),
            format!("Bridge-{}", target_device.name),
            target_device.id.clone(),
            target_device.name.clone(),
            args.server.clone(),
        )
    };

    // 3. 连接 WebSocket 服务器（先连接，立即显示二维码）
    println!("🌐 步骤 3: 连接 WebSocket 服务器...");
    bridge_device.connect().await?;
    println!("✓ 已连接到服务器");
    println!();

    // 4. 立即显示二维码（不需要等 BLE）
    println!("📱 步骤 4: 获取二维码...");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    if let Some(qr_url) = bridge_device.qr_url().await {
        println!("📲 请用第三方控制器扫描以下二维码或访问链接：");
        println!();

        // 显示 ASCII QR 码
        display_qr_code(&qr_url);

        println!();
    } else {
        error!("无法获取二维码 URL");
        return Err(CliError::Other("Failed to get QR URL".to_string()));
    }

    // 5. 连接 BLE 设备（二维码显示后再连）
    println!("📲 步骤 5: 连接 BLE 设备...");

    let protocol_device = ble_manager.connect(&target_device.id).await?;
    bridge_device.connect_ble(protocol_device).await?;
    println!("✓ BLE 设备已连接");
    println!();

    // 6. 等待控制器连接
    println!("⏳ 等待控制器连接...");
    println!();

    // 7. 启动桥接
    println!("🚀 步骤 6: 启动桥接模式...");
    bridge_device.start().await?;
    info!("设备已启动，开始桥接模式");

    println!("✅ 桥接模式已启动！");
    println!();
    println!("📊 实时状态：");
    println!("  • BLE 设备: {}", target_device.name);
    println!("  • WebSocket: {}", args.server);
    println!();
    println!("💡 提示：");
    println!("  • 第三方控制器可以通过 WebSocket 发送控制指令");
    println!("  • 程序会自动将指令转发给 BLE 设备");
    println!("  • BLE 设备状态会同步到 WebSocket 服务器");
    println!("  • 按 Ctrl+C 停止");
    println!();

    // 订阅设备事件
    let mut events = bridge_device.subscribe_events();

    // 监听事件
    loop {
        tokio::select! {
            event = events.recv() => {
                if let Ok(event) = event {
                    match event {
                        dglab_core::device::DeviceEvent::StateChanged(state) => {
                            println!("🔄 状态变化: {:?}", state);
                        }
                        dglab_core::device::DeviceEvent::StatusReport { power_a, power_b } => {
                            if args.verbose {
                                println!("⚡ 强度状态: A={}, B={}", power_a, power_b);
                            }
                        }
                        dglab_core::device::DeviceEvent::BatteryUpdated(level) => {
                            println!("🔋 电池: {}%", level);
                        }
                        dglab_core::device::DeviceEvent::Error(err) => {
                            error!("❌ 错误: {}", err);
                        }
                        _ => {}
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                println!();
                println!("🛑 收到停止信号");
                break;
            }
        }
    }

    // 断开连接
    bridge_device.stop().await?;
    bridge_device.disconnect().await?;

    println!("✓ 已断开连接");
    Ok(())
}

/// 显示 ASCII 二维码
fn display_qr_code(url: &str) {
    use qrcode::QrCode;

    match QrCode::new(url) {
        Ok(code) => {
            let string = code
                .render::<char>()
                .quiet_zone(false)
                .module_dimensions(2, 1)
                .build();
            println!("{}", string);
        }
        Err(e) => {
            error!("无法生成二维码: {}", e);
        }
    }
}
