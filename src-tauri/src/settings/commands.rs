use crate::{
    app_error::errors::{AppError, AppResult},
    settings::setting_manager::{AppSettings, SETTINGS_MANAGER},
};

#[tauri::command(async)]
pub fn get_settings() -> AppResult<AppSettings> {
    let settings_manager = SETTINGS_MANAGER
        .get()
        .ok_or(AppError::Generic("Setting manager not initialized".into()))?;

    settings_manager.get_settings()
}

#[tauri::command(async)]
pub fn toggle_settings(patch: AppSettings) -> AppResult<AppSettings> {
    let settings_manager = SETTINGS_MANAGER
        .get()
        .ok_or(AppError::Generic("Setting manager not initialized".into()))?;

    settings_manager.update(|s| {
        s.auto_scan = patch.auto_scan;
        s.index_audio = patch.index_audio;
        s.index_images = patch.index_images
    })
}
