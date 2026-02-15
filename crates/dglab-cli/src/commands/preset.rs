//! 预设管理命令

use clap::Parser;
use tracing::info;

use super::DglabCli;

/// 预设管理子命令
#[derive(Parser, Debug)]
pub struct PresetArgs {
    #[command(subcommand)]
    command: PresetCommand,
}

/// 预设子命令
#[derive(Parser, Debug)]
enum PresetCommand {
    /// 列出所有预设
    List,
    /// 显示预设详情
    Show { name: String },
    /// 应用预设
    Apply {
        /// 预设名称
        name: String,
        /// 设备 ID
        #[arg(short, long)]
        device: Option<String>,
    },
    /// 创建新预设
    Create {
        /// 预设名称
        name: String,
        /// 预设描述
        #[arg(short, long)]
        description: Option<String>,
        /// 通道 A 最大强度
        #[arg(long = "a")]
        power_a: Option<u8>,
        /// 通道 B 最大强度
        #[arg(long = "b")]
        power_b: Option<u8>,
    },
    /// 删除预设
    Delete { name: String },
}

/// 执行预设命令
pub async fn execute(app: &mut DglabCli, args: PresetArgs) -> crate::error::Result<()> {
    match args.command {
        PresetCommand::List => {
            let presets = app.preset_manager().list_presets();

            println!("\nPresets ({}):", presets.len());
            println!("{}", "-".repeat(50));

            if presets.is_empty() {
                println!("No presets found");
            } else {
                for preset in presets {
                    println!("  - {}", preset.name);
                    if !preset.description.is_empty() {
                        println!("    {}", preset.description);
                    }
                }
            }
        }

        PresetCommand::Show { name } => {
            if let Some(preset) = app.preset_manager().find_preset_by_name(&name) {
                println!("\nPreset: {}", preset.name);
                println!("{}", "-".repeat(50));
                println!("Description: {}", preset.description);
                println!("Created:     {}", preset.created_at);
                println!("Updated:     {}", preset.updated_at);
                println!("\nChannel A:");
                println!("  Enabled:   {}", preset.channel_a.enabled);
                println!("  Max Power: {}", preset.channel_a.max_power);
                println!("\nChannel B:");
                println!("  Enabled:   {}", preset.channel_b.enabled);
                println!("  Max Power: {}", preset.channel_b.max_power);
            } else {
                println!("Preset not found: {}", name);
            }
        }

        PresetCommand::Apply { name, device } => {
            info!("Applying preset: {}", name);

            let Some(preset) = app.preset_manager().find_preset_by_name(&name) else {
                println!("Preset not found: {}", name);
                return Ok(());
            };

            // 获取设备
            let device_ids = app.session_manager().list_devices().await;
            let device_id = device.or_else(|| device_ids.first().cloned());

            let Some(device_id) = device_id else {
                println!("No connected devices");
                return Ok(());
            };

            let Some(dev) = app.session_manager().get_device(&device_id).await else {
                println!("Device not found: {}", device_id);
                return Ok(());
            };

            let mut dev = dev.write().await;

            // 应用预设（这里只是设置最大强度的示例）
            if preset.channel_a.enabled {
                dev.set_power(0, preset.channel_a.max_power).await?;
            }
            if preset.channel_b.enabled {
                dev.set_power(1, preset.channel_b.max_power).await?;
            }

            println!("Applied preset '{}' to device '{}'", name, device_id);
        }

        PresetCommand::Create {
            name,
            description,
            power_a,
            power_b,
        } => {
            info!("Creating preset: {}", name);

            if app.preset_manager().find_preset_by_name(&name).is_some() {
                println!("Preset already exists: {}", name);
                return Ok(());
            }

            let mut preset =
                dglab_core::preset::Preset::new(name.clone(), description.unwrap_or_default());

            if let Some(p) = power_a {
                preset.channel_a.max_power = p;
            }
            if let Some(p) = power_b {
                preset.channel_b.max_power = p;
            }

            // 添加到管理器
            let preset_id = preset.id.clone();
            app.preset_manager_mut().add_preset(preset)?;

            // 保存到磁盘
            app.preset_manager().save_preset(&preset_id).await?;

            println!("Preset created: {}", name);
        }

        PresetCommand::Delete { name } => {
            info!("Deleting preset: {}", name);

            let Some(preset) = app.preset_manager().find_preset_by_name(&name) else {
                println!("Preset not found: {}", name);
                return Ok(());
            };

            let preset_id = preset.id.clone();

            // 从管理器移除
            app.preset_manager_mut().remove_preset(&preset_id)?;

            // 删除文件
            app.preset_manager().delete_preset_file(&preset_id).await?;

            println!("Preset deleted: {}", name);
        }
    }

    Ok(())
}
