use std::{collections::HashMap, path::PathBuf};

use crate::{
    adapters::{
        document_adapter::DocumentAdapter, ocr::media_adapter::OCRAdapter,
        textable_adapters::TextableAdapter, traits::Adapter,
    },
    app_error::errors::AppResult,
    db::types::file::File,
    scanners::file_scanner::FilesByExtensionResponse,
    settings::commands::get_settings,
};

pub type ExtractedContentFromFilesResponse = HashMap<File, String>;

pub fn extract_content_from_files(
    mapped_files_by_extension: FilesByExtensionResponse,
) -> AppResult<ExtractedContentFromFilesResponse> {
    let mut content_by_path: ExtractedContentFromFilesResponse = HashMap::new();

    let settings = get_settings()?;

    let ocr = if settings.index_images {
        Some(OCRAdapter::new().unwrap_or_else(|_| {
            panic!("Failed to initialize OCRAdapter. Please ensure that the OCR models are available in the specified resource directory.");
        }))
    } else {
        None
    };

    for (extension, files) in mapped_files_by_extension {
        match extension.as_str() {
            "pdf" | "docx" => {
                content_by_path.extend(DocumentAdapter.ingest(files));
            }
            "txt" | "md" | "log" | "env" | "ini" | "conf" | "toml" | "yml" | "yaml" | "json" => {
                content_by_path.extend(TextableAdapter.ingest(files));
            }
            "png" | "jpg" | "jpeg" | "webp" => {
                if let Some(ocr_adapter) = &ocr {
                    content_by_path.extend(ocr_adapter.ingest(files));
                }
            }
            _ => {}
        }
    }

    Ok(content_by_path)
}
