use tauri::Error;

use crate::{
    db::tantivy::{IndexerService, TANTIVY_INDEX},
    extractors::extractor::{extract_content_from_files, ExtractedContentFromFilesResponse},
    repositories::{
        binary_persister::BinaryPersister, sqlite_repository::SqliteRepository, traits::Persister,
    },
    scanners::file_scanner::{map_files_by_extension, scanner},
    tokenizers::simple_tokenizer,
    types::database::BinaryDatabase,
};

pub fn scan_files() -> BinaryDatabase {
    println!("Calling use case, scan");
    let files = scanner().unwrap_or_default();

    let files_by_extension = map_files_by_extension(files);

    let content_by_file = extract_content_from_files(files_by_extension);

    let _ = index_files_to_tantivy(&content_by_file);

    let words_by_files = simple_tokenizer::simple_tokenizer(content_by_file);

    BinaryPersister::save(&words_by_files);

    words_by_files
}

fn index_files_to_tantivy(data: &ExtractedContentFromFilesResponse) -> Result<Option<()>, Error> {
    let Ok(db) = SqliteRepository::new() else {
        return Ok(None);
    };

    let index = TANTIVY_INDEX
        .get()
        .clone()
        .expect("Tantivy was not setted but it was called in index");

    if let Ok(mut indexer) = IndexerService::new(db, index.clone()) {
        indexer.index_files(data);
    }

    Ok(Some(()))
}
