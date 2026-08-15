use std::path::PathBuf;

use crate::{
    app_error::errors::{AppError, AppResult},
    paths::RESOURCE_DIR,
};

pub struct HandleOCRModels {
    models_path: PathBuf,
}

// static DETECTION_MODEL: &str =
//     "https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten";
//
// static RECOGNITION_MODEL: &str =
//     "https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten";

impl HandleOCRModels {
    pub fn new() -> AppResult<Self> {
        let resource_dir = RESOURCE_DIR.get().ok_or(AppError::DataDirNotSet)?;

        let models_path = resource_dir.join("ocr-models");

        Ok(Self { models_path })
    }

    pub fn check_model_exists(&self) -> AppResult<()> {
        let detection_model_path = self.models_path.join("text-detection.rten");
        let recognition_model_path = self.models_path.join("text-recognition.rten");

        if !detection_model_path.exists() || !recognition_model_path.exists() {
            return Err(AppError::OCRModelMissing {
                status: 404,
                message: "Missing OCR models".to_string(),
            });
        }
        Ok(())
    }
}
