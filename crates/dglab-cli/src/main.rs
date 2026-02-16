//! DG-LAB 命令行工具

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod commands;
mod error;
mod tui;

use commands::DglabCli;

/// DG-LAB 控制器
#[derive(Parser, Debug)]
#[command(name = "dglab")]
#[command(about = "DG-LAB device controller", long_about = None)]
#[command(version)]
struct Cli {
    /// 启用调试日志
    #[arg(short, long, global = true)]
    debug: bool,

    /// 子命令
    #[command(subcommand)]
    command: Commands,
}

/// 子命令
#[derive(Parser, Debug)]
enum Commands {
    /// 扫描设备
    Scan(commands::ScanArgs),
    /// 连接设备
    Connect(commands::ConnectArgs),
    /// 控制设备
    Control(commands::ControlArgs),
    /// 预设管理
    Preset(commands::PresetArgs),
    /// 运行脚本
    Script(commands::ScriptArgs),
    /// WiFi 连接
    Wifi(commands::WifiArgs),
    /// 桥接模式（BLE + WebSocket）
    Bridge(commands::BridgeArgs),
    /// 启动 TUI 界面
    Tui,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 初始化日志
    let log_level = if cli.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("dglab={}", log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 执行命令
    let mut app = DglabCli::new().await?;

    match cli.command {
        Commands::Scan(args) => app.scan(args).await?,
        Commands::Connect(args) => app.connect(args).await?,
        Commands::Control(args) => app.control(args).await?,
        Commands::Preset(args) => app.preset(args).await?,
        Commands::Script(args) => app.script(args).await?,
        Commands::Wifi(args) => app.wifi(args).await?,
        Commands::Bridge(args) => app.bridge(args).await?,
        Commands::Tui => app.run_tui().await?,
    }

    Ok(())
}
