//! 预设存储管理

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::error::{CoreError, Result};
use crate::waveform::Waveform;

/// 通道配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetChannelConfig {
    /// 是否启用
    pub enabled: bool,
    /// 最小强度
    pub min_power: u8,
    /// 最大强度
    pub max_power: u8,
    /// 波形
    pub waveform: Option<Waveform>,
}

impl Default for PresetChannelConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_power: 0,
            max_power: 50,
            waveform: None,
        }
    }
}

/// 设备预设
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    /// 预设 ID
    pub id: String,
    /// 预设名称
    pub name: String,
    /// 预设描述
    pub description: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 最后修改时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// 通道 A 配置
    pub channel_a: PresetChannelConfig,
    /// 通道 B 配置
    pub channel_b: PresetChannelConfig,
    /// 全局设置
    pub settings: HashMap<String, String>,
}

impl Preset {
    /// 创建新预设
    pub fn new(name: String, description: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            created_at: now,
            updated_at: now,
            channel_a: PresetChannelConfig::default(),
            channel_b: PresetChannelConfig::default(),
            settings: HashMap::new(),
        }
    }

    /// 更新修改时间
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now();
    }

    /// 设置通道配置
    pub fn set_channel(&mut self, channel: u8, config: PresetChannelConfig) {
        match channel {
            0 => self.channel_a = config,
            1 => self.channel_b = config,
            _ => {}
        }
        self.touch();
    }

    /// 设置波形
    pub fn set_waveform(&mut self, channel: u8, waveform: Waveform) {
        match channel {
            0 => self.channel_a.waveform = Some(waveform),
            1 => self.channel_b.waveform = Some(waveform),
            _ => {}
        }
        self.touch();
    }

    /// 设置最大强度
    pub fn set_max_power(&mut self, channel: u8, power: u8) {
        match channel {
            0 => self.channel_a.max_power = power,
            1 => self.channel_b.max_power = power,
            _ => {}
        }
        self.touch();
    }
}

/// 预设管理器
pub struct PresetManager {
    /// 预设存储目录
    storage_dir: PathBuf,
    /// 预设集合
    presets: HashMap<String, Preset>,
}

impl PresetManager {
    /// 创建新的预设管理器
    pub fn new(storage_dir: PathBuf) -> Self {
        Self {
            storage_dir,
            presets: HashMap::new(),
        }
    }

    /// 使用默认目录创建预设管理器
    pub fn default_dir() -> Result<Self> {
        let dir = Self::default_storage_dir()?;
        Ok(Self::new(dir))
    }

    /// 获取默认存储目录
    pub fn default_storage_dir() -> Result<PathBuf> {
        let dir = dirs::config_dir()
            .ok_or_else(|| CoreError::Other("Could not find config directory".to_string()))?
            .join("dglab")
            .join("presets");

        Ok(dir)
    }

    /// 初始化存储目录
    pub async fn initialize(&mut self) -> Result<()> {
        if !self.storage_dir.exists() {
            tokio::fs::create_dir_all(&self.storage_dir).await?;
            info!("Created preset directory: {:?}", self.storage_dir);
        }

        self.load_all().await?;

        // 如果没有预设，创建默认预设
        if self.presets.is_empty() {
            self.create_default_presets()?;
            self.save_all().await?;
        }

        Ok(())
    }

    /// 创建默认预设
    fn create_default_presets(&mut self) -> Result<()> {
        let mut preset1 = Preset::new("Gentle".to_string(), "Gentle stimulation".to_string());
        preset1.channel_a.max_power = 30;
        preset1.channel_b.max_power = 30;
        self.add_preset(preset1)?;

        let mut preset2 = Preset::new("Medium".to_string(), "Medium stimulation".to_string());
        preset2.channel_a.max_power = 50;
        preset2.channel_b.max_power = 50;
        self.add_preset(preset2)?;

        let mut preset3 = Preset::new("Strong".to_string(), "Strong stimulation".to_string());
        preset3.channel_a.max_power = 80;
        preset3.channel_b.max_power = 80;
        self.add_preset(preset3)?;

        Ok(())
    }

    /// 加载所有预设
    pub async fn load_all(&mut self) -> Result<()> {
        self.presets.clear();

        if !self.storage_dir.exists() {
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(&self.storage_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                match self.load_preset_from_file(&path).await {
                    Ok(preset) => {
                        debug!("Loaded preset: {}", preset.name);
                        self.presets.insert(preset.id.clone(), preset);
                    }
                    Err(e) => {
                        debug!("Failed to load preset from {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(())
    }

    /// 从文件加载预设
    async fn load_preset_from_file(&self, path: &PathBuf) -> Result<Preset> {
        let content = tokio::fs::read_to_string(path).await?;
        let preset: Preset = serde_json::from_str(&content)?;
        Ok(preset)
    }

    /// 保存所有预设
    pub async fn save_all(&self) -> Result<()> {
        for preset in self.presets.values() {
            self.save_preset_to_file(preset).await?;
        }
        Ok(())
    }

    /// 保存预设到文件
    async fn save_preset_to_file(&self, preset: &Preset) -> Result<()> {
        let path = self.storage_dir.join(format!("{}.json", preset.id));
        let content = serde_json::to_string_pretty(preset)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }

    /// 获取所有预设
    pub fn list_presets(&self) -> Vec<&Preset> {
        let mut presets: Vec<_> = self.presets.values().collect();
        presets.sort_by(|a, b| a.name.cmp(&b.name));
        presets
    }

    /// 获取预设
    pub fn get_preset(&self, id: &str) -> Option<&Preset> {
        self.presets.get(id)
    }

    /// 按名称查找预设
    pub fn find_preset_by_name(&self, name: &str) -> Option<&Preset> {
        self.presets
            .values()
            .find(|p| p.name.to_lowercase() == name.to_lowercase())
    }

    /// 添加预设
    pub fn add_preset(&mut self, preset: Preset) -> Result<()> {
        if self.presets.contains_key(&preset.id) {
            return Err(CoreError::PresetAlreadyExists(preset.id));
        }
        self.presets.insert(preset.id.clone(), preset);
        Ok(())
    }

    /// 更新预设
    pub fn update_preset(&mut self, preset: Preset) -> Result<()> {
        if !self.presets.contains_key(&preset.id) {
            return Err(CoreError::PresetNotFound(preset.id));
        }
        self.presets.insert(preset.id.clone(), preset);
        Ok(())
    }

    /// 删除预设（仅从内存中）
    pub fn remove_preset(&mut self, id: &str) -> Result<()> {
        if self.presets.remove(id).is_none() {
            return Err(CoreError::PresetNotFound(id.to_string()));
        }
        Ok(())
    }

    /// 保存单个预设到文件
    pub async fn save_preset(&self, id: &str) -> Result<()> {
        if let Some(preset) = self.presets.get(id) {
            self.save_preset_to_file(preset).await?;
        }
        Ok(())
    }

    /// 删除预设文件
    pub async fn delete_preset_file(&self, id: &str) -> Result<()> {
        let path = self.storage_dir.join(format!("{}.json", id));
        if path.exists() {
            tokio::fs::remove_file(path).await?;
        }
        Ok(())
    }

    /// 获取或创建预设（返回 owned）
    pub fn get_or_create_preset(&mut self, name: &str) -> Preset {
        if let Some(preset) = self.find_preset_by_name(name) {
            preset.clone()
        } else {
            let preset = Preset::new(name.to_string(), String::new());
            let id = preset.id.clone();
            self.presets.insert(id.clone(), preset);
            self.presets.get(&id).unwrap().clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === PresetChannelConfig 测试 ===

    #[test]
    fn test_channel_config_default() {
        let config = PresetChannelConfig::default();
        assert!(config.enabled);
        assert_eq!(config.min_power, 0);
        assert_eq!(config.max_power, 50);
        assert!(config.waveform.is_none());
    }

    #[test]
    fn test_channel_config_serde_roundtrip() {
        let config = PresetChannelConfig {
            enabled: false,
            min_power: 10,
            max_power: 80,
            waveform: None,
        };
        let json = serde_json::to_string(&config).unwrap();
        let restored: PresetChannelConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.enabled, false);
        assert_eq!(restored.min_power, 10);
        assert_eq!(restored.max_power, 80);
    }

    // === Preset 测试 ===

    #[test]
    fn test_preset_new() {
        let preset = Preset::new("Test".to_string(), "A test preset".to_string());
        assert_eq!(preset.name, "Test");
        assert_eq!(preset.description, "A test preset");
        assert!(!preset.id.is_empty());
        assert!(preset.settings.is_empty());
        assert!(preset.channel_a.enabled);
        assert!(preset.channel_b.enabled);
    }

    #[test]
    fn test_preset_unique_ids() {
        let p1 = Preset::new("P1".to_string(), String::new());
        let p2 = Preset::new("P2".to_string(), String::new());
        assert_ne!(p1.id, p2.id);
    }

    #[test]
    fn test_preset_touch_updates_time() {
        let mut preset = Preset::new("Test".to_string(), String::new());
        let original = preset.updated_at;
        // 稍等一下确保时间变化
        std::thread::sleep(std::time::Duration::from_millis(10));
        preset.touch();
        assert!(preset.updated_at >= original);
    }

    #[test]
    fn test_preset_set_channel_a() {
        let mut preset = Preset::new("Test".to_string(), String::new());
        let config = PresetChannelConfig {
            enabled: false,
            min_power: 5,
            max_power: 95,
            waveform: None,
        };
        preset.set_channel(0, config);
        assert_eq!(preset.channel_a.enabled, false);
        assert_eq!(preset.channel_a.min_power, 5);
        assert_eq!(preset.channel_a.max_power, 95);
    }

    #[test]
    fn test_preset_set_channel_b() {
        let mut preset = Preset::new("Test".to_string(), String::new());
        let config = PresetChannelConfig {
            enabled: true,
            min_power: 20,
            max_power: 60,
            waveform: None,
        };
        preset.set_channel(1, config);
        assert_eq!(preset.channel_b.min_power, 20);
        assert_eq!(preset.channel_b.max_power, 60);
    }

    #[test]
    fn test_preset_set_channel_invalid_ignored() {
        let mut preset = Preset::new("Test".to_string(), String::new());
        let original_a = preset.channel_a.max_power;
        let original_b = preset.channel_b.max_power;
        let config = PresetChannelConfig {
            enabled: false,
            min_power: 99,
            max_power: 99,
            waveform: None,
        };
        preset.set_channel(2, config);
        assert_eq!(preset.channel_a.max_power, original_a);
        assert_eq!(preset.channel_b.max_power, original_b);
    }

    #[test]
    fn test_preset_set_max_power() {
        let mut preset = Preset::new("Test".to_string(), String::new());
        preset.set_max_power(0, 75);
        assert_eq!(preset.channel_a.max_power, 75);
        preset.set_max_power(1, 90);
        assert_eq!(preset.channel_b.max_power, 90);
    }

    #[test]
    fn test_preset_set_max_power_invalid_channel() {
        let mut preset = Preset::new("Test".to_string(), String::new());
        preset.set_max_power(2, 99);
        // 无效通道不应改变任何值
        assert_eq!(preset.channel_a.max_power, 50);
        assert_eq!(preset.channel_b.max_power, 50);
    }

    #[test]
    fn test_preset_serde_roundtrip() {
        let mut preset = Preset::new("Test Preset".to_string(), "desc".to_string());
        preset.channel_a.max_power = 70;
        preset
            .settings
            .insert("key".to_string(), "value".to_string());

        let json = serde_json::to_string_pretty(&preset).unwrap();
        let restored: Preset = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.id, preset.id);
        assert_eq!(restored.name, "Test Preset");
        assert_eq!(restored.description, "desc");
        assert_eq!(restored.channel_a.max_power, 70);
        assert_eq!(restored.settings.get("key").unwrap(), "value");
    }

    // === PresetManager 测试 ===

    #[test]
    fn test_manager_new() {
        let dir = PathBuf::from("/tmp/test_presets");
        let manager = PresetManager::new(dir.clone());
        assert!(manager.list_presets().is_empty());
    }

    #[test]
    fn test_manager_add_preset() {
        let manager = &mut PresetManager::new(PathBuf::from("/tmp/test"));
        let preset = Preset::new("P1".to_string(), String::new());
        let id = preset.id.clone();
        manager.add_preset(preset).unwrap();
        assert_eq!(manager.list_presets().len(), 1);
        assert!(manager.get_preset(&id).is_some());
    }

    #[test]
    fn test_manager_add_duplicate_fails() {
        let manager = &mut PresetManager::new(PathBuf::from("/tmp/test"));
        let preset = Preset::new("P1".to_string(), String::new());
        let id = preset.id.clone();
        manager.add_preset(preset).unwrap();

        // 用相同 ID 创建另一个并手动设置 ID
        let mut preset2 = Preset::new("P2".to_string(), String::new());
        preset2.id = id;
        let result = manager.add_preset(preset2);
        assert!(result.is_err());
    }

    #[test]
    fn test_manager_update_preset() {
        let manager = &mut PresetManager::new(PathBuf::from("/tmp/test"));
        let mut preset = Preset::new("P1".to_string(), String::new());
        let id = preset.id.clone();
        manager.add_preset(preset.clone()).unwrap();

        preset.name = "Updated".to_string();
        manager.update_preset(preset).unwrap();

        let updated = manager.get_preset(&id).unwrap();
        assert_eq!(updated.name, "Updated");
    }

    #[test]
    fn test_manager_update_nonexistent_fails() {
        let manager = &mut PresetManager::new(PathBuf::from("/tmp/test"));
        let preset = Preset::new("P1".to_string(), String::new());
        let result = manager.update_preset(preset);
        assert!(result.is_err());
    }

    #[test]
    fn test_manager_remove_preset() {
        let manager = &mut PresetManager::new(PathBuf::from("/tmp/test"));
        let preset = Preset::new("P1".to_string(), String::new());
        let id = preset.id.clone();
        manager.add_preset(preset).unwrap();

        manager.remove_preset(&id).unwrap();
        assert!(manager.list_presets().is_empty());
        assert!(manager.get_preset(&id).is_none());
    }

    #[test]
    fn test_manager_remove_nonexistent_fails() {
        let manager = &mut PresetManager::new(PathBuf::from("/tmp/test"));
        let result = manager.remove_preset("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_manager_find_by_name() {
        let manager = &mut PresetManager::new(PathBuf::from("/tmp/test"));
        let preset = Preset::new("MyPreset".to_string(), String::new());
        let id = preset.id.clone();
        manager.add_preset(preset).unwrap();

        let found = manager.find_preset_by_name("mypreset").unwrap();
        assert_eq!(found.id, id);
    }

    #[test]
    fn test_manager_find_by_name_not_found() {
        let manager = PresetManager::new(PathBuf::from("/tmp/test"));
        assert!(manager.find_preset_by_name("nonexistent").is_none());
    }

    #[test]
    fn test_manager_list_sorted_by_name() {
        let manager = &mut PresetManager::new(PathBuf::from("/tmp/test"));
        manager
            .add_preset(Preset::new("Charlie".to_string(), String::new()))
            .unwrap();
        manager
            .add_preset(Preset::new("Alpha".to_string(), String::new()))
            .unwrap();
        manager
            .add_preset(Preset::new("Bravo".to_string(), String::new()))
            .unwrap();

        let list = manager.list_presets();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].name, "Alpha");
        assert_eq!(list[1].name, "Bravo");
        assert_eq!(list[2].name, "Charlie");
    }

    #[test]
    fn test_manager_get_or_create_existing() {
        let manager = &mut PresetManager::new(PathBuf::from("/tmp/test"));
        let preset = Preset::new("Existing".to_string(), "desc".to_string());
        let id = preset.id.clone();
        manager.add_preset(preset).unwrap();

        let result = manager.get_or_create_preset("Existing");
        assert_eq!(result.id, id);
        assert_eq!(result.description, "desc");
    }

    #[test]
    fn test_manager_get_or_create_new() {
        let manager = &mut PresetManager::new(PathBuf::from("/tmp/test"));
        assert!(manager.list_presets().is_empty());

        let result = manager.get_or_create_preset("NewPreset");
        assert_eq!(result.name, "NewPreset");
        assert_eq!(manager.list_presets().len(), 1);
    }

    #[test]
    fn test_manager_create_default_presets() {
        let manager = &mut PresetManager::new(PathBuf::from("/tmp/test"));
        manager.create_default_presets().unwrap();

        let list = manager.list_presets();
        assert_eq!(list.len(), 3);

        let names: Vec<&str> = list.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"Gentle"));
        assert!(names.contains(&"Medium"));
        assert!(names.contains(&"Strong"));

        let gentle = manager.find_preset_by_name("Gentle").unwrap();
        assert_eq!(gentle.channel_a.max_power, 30);
    }

    // === PresetManager 文件 IO 测试 ===

    #[tokio::test]
    async fn test_manager_save_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let mut manager = PresetManager::new(dir.path().to_path_buf());
        manager.initialize().await.unwrap();

        // 默认应该创建了 3 个预设
        assert_eq!(manager.list_presets().len(), 3);

        // 添加自定义预设
        let preset = Preset::new("Custom".to_string(), "custom desc".to_string());
        let id = preset.id.clone();
        manager.add_preset(preset).unwrap();
        manager.save_preset(&id).await.unwrap();

        // 创建新 manager 加载同一目录
        let mut manager2 = PresetManager::new(dir.path().to_path_buf());
        manager2.load_all().await.unwrap();
        assert_eq!(manager2.list_presets().len(), 4);

        let loaded = manager2.get_preset(&id).unwrap();
        assert_eq!(loaded.name, "Custom");
        assert_eq!(loaded.description, "custom desc");
    }

    #[tokio::test]
    async fn test_manager_initialize_creates_dir() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("sub").join("presets");
        let mut manager = PresetManager::new(sub.clone());
        manager.initialize().await.unwrap();

        assert!(sub.exists());
        assert_eq!(manager.list_presets().len(), 3);
    }

    #[tokio::test]
    async fn test_manager_delete_preset_file() {
        let dir = tempfile::tempdir().unwrap();
        let mut manager = PresetManager::new(dir.path().to_path_buf());
        manager.initialize().await.unwrap();

        let preset = Preset::new("ToDelete".to_string(), String::new());
        let id = preset.id.clone();
        manager.add_preset(preset).unwrap();
        manager.save_preset(&id).await.unwrap();

        // 文件应该存在
        let file = dir.path().join(format!("{}.json", id));
        assert!(file.exists());

        // 删除文件
        manager.delete_preset_file(&id).await.unwrap();
        assert!(!file.exists());
    }

    #[tokio::test]
    async fn test_manager_save_all() {
        let dir = tempfile::tempdir().unwrap();
        let mut manager = PresetManager::new(dir.path().to_path_buf());
        tokio::fs::create_dir_all(dir.path()).await.unwrap();

        manager
            .add_preset(Preset::new("A".to_string(), String::new()))
            .unwrap();
        manager
            .add_preset(Preset::new("B".to_string(), String::new()))
            .unwrap();
        manager.save_all().await.unwrap();

        // 两个文件
        let entries: Vec<_> = std::fs::read_dir(dir.path()).unwrap().collect();
        assert_eq!(entries.len(), 2);
    }

    #[tokio::test]
    async fn test_manager_load_skips_invalid_json() {
        let dir = tempfile::tempdir().unwrap();
        // 写一个非法 JSON 文件
        std::fs::write(dir.path().join("bad.json"), "not json").unwrap();
        // 写一个合法 JSON 文件
        let preset = Preset::new("Valid".to_string(), String::new());
        let json = serde_json::to_string_pretty(&preset).unwrap();
        std::fs::write(dir.path().join("valid.json"), json).unwrap();

        let mut manager = PresetManager::new(dir.path().to_path_buf());
        manager.load_all().await.unwrap();
        assert_eq!(manager.list_presets().len(), 1);
        assert_eq!(manager.list_presets()[0].name, "Valid");
    }

    #[tokio::test]
    async fn test_manager_load_ignores_non_json() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("readme.txt"), "not a preset").unwrap();
        std::fs::write(dir.path().join("data.bin"), &[0u8; 10]).unwrap();

        let mut manager = PresetManager::new(dir.path().to_path_buf());
        manager.load_all().await.unwrap();
        assert!(manager.list_presets().is_empty());
    }
}
