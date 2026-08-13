use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::OnceLock};

use crate::{
    app_error::errors::{AppError, AppResult},
    paths::DATA_DIR,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppSettings {
    pub auto_scan: bool,
    pub index_audio: bool,
    pub index_images: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_scan: false,
            index_audio: false,
            index_images: false,
        }
    }
}

#[derive(Debug)]
pub struct SettingsManager {
    json_path: PathBuf,
}

impl SettingsManager {
    pub fn new() -> AppResult<Self> {
        let data_dir = DATA_DIR.get().ok_or(AppError::DataDirNotSet)?;
        let settings_path = data_dir.join("settings.json");

        Ok(Self {
            json_path: settings_path,
        })
    }

    /// Lê e desserializa as configurações atuais.
    pub fn get_settings(&self) -> AppResult<AppSettings> {
        let data = fs::read_to_string(&self.json_path)?;
        let settings: AppSettings = serde_json::from_str(&data)
            .map_err(|e| AppError::Generic(format!("The settings file is corrupted: {e}")))?;
        Ok(settings)
    }

    pub fn save_settings(&self, settings: &AppSettings) -> AppResult<()> {
        let json_data = serde_json::to_string_pretty(settings)
            .map_err(|e| AppError::Generic(format!("Error serializing json: {e}")))?;
        fs::write(&self.json_path, json_data)?;
        Ok(())
    }

    pub fn update<F>(&self, mutate: F) -> AppResult<AppSettings>
    where
        F: FnOnce(&mut AppSettings),
    {
        let mut settings = self.get_settings()?;
        mutate(&mut settings);
        self.save_settings(&settings)?;
        Ok(settings)
    }

    pub fn set_auto_scan(&self, enabled: bool) -> AppResult<()> {
        self.update(|s| s.auto_scan = enabled)?;
        Ok(())
    }

    pub fn validate_settings_json(&self) -> AppResult<()> {
        self.get_settings()?;
        Ok(())
    }

    pub fn drop_settings_json(&self) -> AppResult<()> {
        if self.json_path.exists() {
            fs::remove_file(&self.json_path)?;
        }
        Ok(())
    }

    pub fn init_setting_json(&self) -> AppResult<()> {
        if self.json_path.exists() {
            if let Err(json_error) = self.validate_settings_json() {
                let _ = self.drop_settings_json();
                eprintln!("Settings file corrupted, resetting: {json_error}");
            } else {
                return Ok(());
            }
        }

        let default_settings = AppSettings::default();
        self.save_settings(&default_settings)?;
        println!("Settings initialized");

        Ok(())
    }
}

pub static SETTINGS_MANAGER: OnceLock<SettingsManager> = OnceLock::new();
