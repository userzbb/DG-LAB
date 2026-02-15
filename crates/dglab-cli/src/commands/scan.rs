//! 扫描设备命令

use clap::Parser;
use std::time::Duration;
use tracing::info;

use super::DglabCli;

/// 扫描设备参数
#[derive(Parser, Debug)]
pub struct ScanArgs {
    /// 扫描持续时间（秒）
    #[arg(short, long, default_value = "5")]
    duration: u64,
}

/// 执行扫描命令
pub async fn execute(app: &mut DglabCli, args: ScanArgs) -> crate::error::Result<()> {
    info!("Starting BLE scan for {} seconds...", args.duration);

    app.ble_manager().start_scan().await?;

    // 等待扫描
    tokio::time::sleep(Duration::from_secs(args.duration)).await;

    app.ble_manager().stop_scan().await?;

    // 获取扫描结果
    let results = app.ble_manager().get_scan_results().await?;

    println!("\nFound {} devices:", results.len());
    println!("{}", "-".repeat(60));

    if results.is_empty() {
        println!("No DG-LAB devices found.");
    } else {
        for (i, device) in results.iter().enumerate() {
            let rssi = device
                .rssi
                .map(|r| format!("{} dBm", r))
                .unwrap_or_else(|| "N/A".to_string());
            println!("{}. ID:      {}", i + 1, device.id);
            println!("   Name:    {}", device.name);
            println!("   Address: {}", device.address);
            println!("   Signal:  {}", rssi);
            println!();
        }
    }

    Ok(())
}
