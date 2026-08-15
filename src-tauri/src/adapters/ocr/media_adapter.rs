use std::collections::HashMap;

use crate::{
    adapters::{ocr, traits::Adapter},
    app_error::errors::{AppError, AppResult},
    db::types::file::File,
    paths::RESOURCE_DIR,
};
use ocrs::OcrEngineParams;
use rten::Model;

pub struct OCRAdapter {
    models_path: std::path::PathBuf,
    ocrs_engine: ocrs::OcrEngine,
}

impl OCRAdapter {
    pub fn new() -> AppResult<Self> {
        let resource_dir = RESOURCE_DIR.get().ok_or(AppError::ResourceDirNotSet)?;

        let models_path = resource_dir.join("ocr-models");

        let dt_path = models_path.join("text-detection.rten");
        let rec_path = models_path.join("text-recognition.rten");

        let dt_model = Model::load_file(dt_path).map_err(|e| AppError::Generic(e.to_string()))?;
        let rec_model = Model::load_file(rec_path).map_err(|e| AppError::Generic(e.to_string()))?;

        let handle_models = ocr::handle_models::HandleOCRModels::new()?;
        handle_models.check_model_exists()?;

        let engine = ocrs::OcrEngine::new(OcrEngineParams {
            detection_model: Some(dt_model),
            recognition_model: Some(rec_model),
            ..Default::default()
        })
        .map_err(|e| AppError::OCREngineInitializationError(e.to_string()))?;

        return Ok(Self {
            models_path,
            ocrs_engine: engine,
        });
    }

    pub fn extract_text_from_images(&self, files: Vec<File>) -> AppResult<HashMap<File, String>> {
        let mut results = HashMap::new();

        for file in files {
            println!("Processing image: {:?}", file.path);
            let img = image::open(&file.path)
                .map(|img| img.to_rgba8())
                .map_err(|e| AppError::Generic(e.to_string()))?;

            let image_source = ocrs::ImageSource::from_bytes(img.as_raw(), img.dimensions())
                .map_err(|e| {
                    AppError::OCREngineInitializationError(format!("Cannot process image {e}"))
                })?;

            let ocr_input = self.ocrs_engine.prepare_input(image_source).map_err(|e| {
                AppError::OCREngineInitializationError(format!(
                    "Cannot transform img to OCR INPUT format: {e}"
                ))
            })?;

            let text = self.ocrs_engine.get_text(&ocr_input).map_err(|e| {
                AppError::OCREngineInitializationError(format!(
                    "Cannot extract text from image: {e}"
                ))
            })?;

            println!(
                "Extracted text from image {:?}: {}",
                file.path,
                text.to_lowercase()
            );

            results.insert(file, text);
        }

        Ok(results)
    }
}

impl Adapter for OCRAdapter {
    fn ingest(&self, paths: Vec<File>) -> HashMap<File, String> {
        self.extract_text_from_images(paths)
            .unwrap_or_else(|_| HashMap::new())
    }
}
