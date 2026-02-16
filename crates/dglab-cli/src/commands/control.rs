//! 控制设备命令

use clap::Parser;
use tracing::{debug, info};

use super::DglabCli;

/// 控制设备参数
#[derive(Parser, Debug)]
pub struct ControlArgs {
    /// 设备 ID（如果不指定，使用第一个设备）
    device_id: Option<String>,

    /// 通道 A 强度
    #[arg(long = "a")]
    power_a: Option<u8>,

    /// 通道 B 强度
    #[arg(long = "b")]
    power_b: Option<u8>,

    /// 同时设置两个通道的强度
    #[arg(short, long)]
    power: Option<u8>,

    /// 开始输出
    #[arg(long)]
    start: bool,

    /// 停止输出
    #[arg(long)]
    stop: bool,

    /// 显示设备状态
    #[arg(short, long)]
    status: bool,
}

/// 执行控制命令
pub async fn execute(app: &mut DglabCli, args: ControlArgs) -> crate::error::Result<()> {
    // 获取设备
    let device_ids = app.session_manager().list_devices().await;

    if device_ids.is_empty() {
        println!("No connected devices. Use 'connect' command first.");
        return Ok(());
    }

    let device_id = args.device_id.unwrap_or_else(|| device_ids[0].clone());

    let Some(device) = app.session_manager().get_device(&device_id).await else {
        println!("Device not found: {}", device_id);
        return Ok(());
    };

    let mut dev = device.write().await;

    if args.status {
        let info = dev.info();
        println!("\nDevice Status:");
        println!("{}", "-".repeat(40));
        println!("ID:      {}", info.id);
        println!("Name:    {}", info.name);
        println!("State:   {:?}", dev.state());
        println!("Power A: {} / {}", info.power_a, info.max_power_a);
        println!("Power B: {} / {}", info.power_b, info.max_power_b);
        println!("Battery: {}%", info.battery_level);
        return Ok(());
    }

    if args.start {
        info!("Starting device output");
        dev.start().await?;
        println!("Device output started");
    }

    if args.stop {
        info!("Stopping device output");
        dev.stop().await?;
        println!("Device output stopped");
    }

    // 设置强度
    if let Some(power) = args.power {
        debug!("Setting both channels to {}", power);
        dev.set_power(0, power).await?;
        dev.set_power(1, power).await?;
        println!("Set both channels to {}", power);
    } else {
        if let Some(power) = args.power_a {
            debug!("Setting channel A to {}", power);
            dev.set_power(0, power).await?;
            println!("Set channel A to {}", power);
        }

        if let Some(power) = args.power_b {
            debug!("Setting channel B to {}", power);
            dev.set_power(1, power).await?;
            println!("Set channel B to {}", power);
        }
    }

    Ok(())
}
