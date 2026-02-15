//! TUI 终端界面

use crate::commands::DglabCli;
use crate::error::Result;

pub mod app;
pub mod widgets;

/// 运行 TUI
pub async fn run(_app: &mut DglabCli) -> Result<()> {
    println!("TUI interface is not implemented yet");
    println!("Please use the CLI commands instead");
    Ok(())
}
