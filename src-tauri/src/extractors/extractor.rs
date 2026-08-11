use std::{collections::HashMap, path::PathBuf};

use crate::{
    adapters::{
        document_adapter::DocumentAdapter, textable_adapters::TextableAdapter, traits::Adapter,
    },
    scanners::file_scanner::FilesByExtensionResponse,
};

pub type ExtractedContentFromFilesResponse = HashMap<PathBuf, String>;

pub fn extract_content_from_files(
    mapped_files_by_extension: FilesByExtensionResponse,
) -> ExtractedContentFromFilesResponse {
    let mut content_by_path: ExtractedContentFromFilesResponse = HashMap::new();

    for (extension, files) in mapped_files_by_extension {
        match extension.as_str() {
            "pdf" | "docx" => {
                content_by_path.extend(DocumentAdapter.ingest(files));
            }
            "txt" | "md" | "log" | "env" | "ini" | "conf" | "toml" | "yml" | "yaml" | "json" => {
                content_by_path.extend(TextableAdapter.ingest(files));
            }
            _ => {}
        }
    }

    content_by_path
}
