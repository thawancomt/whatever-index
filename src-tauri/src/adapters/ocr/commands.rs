use crate::{adapters::ocr::handle_models::HandleOCRModels, app_error::errors::AppResult};

#[tauri::command(async)]
pub fn get_ocr_models_status() -> AppResult<bool> {
    let handle_models = HandleOCRModels::new()?;

    handle_models.check_model_exists()?;

    Ok(true)
}
