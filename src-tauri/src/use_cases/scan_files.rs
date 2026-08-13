use crate::{
    app_error::errors::AppResult,
    extractors::extractor::{extract_content_from_files, ExtractedContentFromFilesResponse},
    repositories::{
        file_cache_repository::FileCacheRepository,
        sqlite_repository::{SQLRepository, SqliteRepository},
    },
    scanners::file_scanner::{map_files_by_extension, scanner, Scanner},
    tantivy_indexer::tantivy_indexer::IndexerService,
};

pub fn scan_files() -> AppResult<()> {
    let cache_service = FileCacheRepository::new()?;
    let scanner_service = Scanner::new(cache_service);

    let files = scanner_service.scan_home_dir()?;

    let only_modified = scanner_service.get_modified_files(files)?;

    let _total_files_size_bytes: i64 = only_modified.clone().into_iter().map(|f| f.size_bytes).sum();

    let files_by_extension = map_files_by_extension(&only_modified);

    let content_by_file = extract_content_from_files(files_by_extension);

    match index_files_to_tantivy(&content_by_file) {
        Ok(_) => println!("Files, indexed"),
        Err(e) => {
            eprintln!("{e}")
        }
    };

    match SqliteRepository::new() {
        Ok(repo) => {
            match repo.insert_files_batch(&only_modified) {
                Ok(_) => {
                    println!("Files persisted : {}", only_modified.len())
                }
                Err(e) => {
                    eprintln!("Error while persisting files, {e}")
                }
            };
        }
        Err(e) => {
            eprintln!("Error while instanciating SqliteRepo {}", e);
        }
    };

    Ok(())
}

fn index_files_to_tantivy(data: &ExtractedContentFromFilesResponse) -> AppResult<()> {
    let mut index_service = IndexerService::new()?;

    match index_service.index_files(data) {
        Ok(_) => {
            println!("Files inserted: {}", data.len())
        }
        Err(e) => {
            eprintln!("Error on Indexing Tantivy Files: {}", e)
        }
    }

    Ok(())
}
