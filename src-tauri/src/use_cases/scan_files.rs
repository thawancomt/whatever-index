use crate::{
    db::tantivy::TANTIVY_INDEX,
    extractors::extractor::{extract_content_from_files, ExtractedContentFromFilesResponse},
    repositories::sqlite_repository::{SQLRepository, SqliteRepository},
    scanners::file_scanner::{map_files_by_extension, scanner},
    tantivy_indexer::tantivy_indexer::IndexerService,
};

pub fn scan_files() {
    println!("Calling use case, scan");
    let files = scanner().unwrap_or_default();

    let total_files_size_bytes: i64 = files.clone().into_iter().map(|f| f.size_bytes).sum();

    println!(
        "Found: ({}) files, total size: ({} MB)",
        files.len(),
        total_files_size_bytes / (1024 * 1024)
    );

    let files_by_extension = map_files_by_extension(&files);

    println!(
        "Found {} differents extensions",
        files_by_extension.keys().len()
    );

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
    let tantivy_index = TANTIVY_INDEX
        .get()
        .expect("Tantivy was not setted but it was called in index");

    match IndexerService::new(tantivy_index.clone()) {
        Ok(mut indexer) => return indexer.index_files(data),
        Err(e) => {
            eprintln!("Error while indexing files into Tantivy: {}", e);
            return None;
        }
    }
}
