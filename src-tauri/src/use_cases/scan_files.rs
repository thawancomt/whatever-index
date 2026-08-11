use crate::{
    db::tantivy::{IndexerService, TANTIVY_INDEX},
    extractors::extractor::{extract_content_from_files, ExtractedContentFromFilesResponse},
    repositories::sqlite_repository::{SQLRepository, SqliteRepository},
    scanners::file_scanner::{map_files_by_extension, scanner},
};

pub fn scan_files() {
    println!("Calling use case, scan");
    let files = scanner().unwrap_or_default();

    let files_by_extension = map_files_by_extension(&files);

    let content_by_file = extract_content_from_files(files_by_extension);

    let _ = index_files_to_tantivy(&content_by_file);

    match SqliteRepository::new() {
        Ok(repo) => {
            let _ = repo.insert_files_batch(&files);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    };
}

fn index_files_to_tantivy(data: &ExtractedContentFromFilesResponse) -> Option<()> {
    let Ok(_db) = SqliteRepository::new() else {
        return None;
    };

    let index = TANTIVY_INDEX
        .get()
        .clone()
        .expect("Tantivy was not setted but it was called in index");

    if let Ok(mut indexer) = IndexerService::new(index.clone()) {
        indexer.index_files(data);
        return Some(());
    }

    None
}
