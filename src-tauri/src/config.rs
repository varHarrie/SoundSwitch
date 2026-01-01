use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub excluded_device_ids: Vec<String>,
    pub hotkey: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            excluded_device_ids: Vec::new(),
            hotkey: Some("CommandOrControl+Shift+A".to_string()),
        }
    }
}

// Global config store
// In a production app, we might pass this via Tauri's State management, but a lazy static or mutex is fine for this scale.
// Actually, Tauri State is better. Implementing standard struct logic first.

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new(app_handle: &tauri::AppHandle) -> Self {
        let config_dir = app_handle.path().app_config_dir().unwrap_or_else(|_| {
            // Fallback
            PathBuf::from("config")
        });

        if let Err(_) = fs::create_dir_all(&config_dir) {
            // Ignore error or log it
        }

        let config_path = config_dir.join("config.json");
        Self { config_path }
    }

    pub fn load(&self) -> Config {
        if let Ok(content) = fs::read_to_string(&self.config_path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
        Config::default()
    }

    pub fn save(&self, config: &Config) -> Result<(), String> {
        let content = serde_json::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&self.config_path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }
}
