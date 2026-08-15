use crate::{
    app_error::errors::AppResult,
    extractors::extractor::{extract_content_from_files, ExtractedContentFromFilesResponse},
    repositories::{
        file_cache_repository::FileCacheRepository,
        sqlite_repository::{SQLRepository, SqliteRepository},
    },
    scanners::file_scanner::{map_files_by_extension, Scanner},
    tantivy_indexer::tantivy_indexer::IndexerService,
};

pub fn scan_files() -> AppResult<()> {
    let cache_service = FileCacheRepository::new()?;
    let scanner_service = Scanner::new(cache_service);

    let files = scanner_service.scan_home_dir()?;
    let mut only_modified = scanner_service.get_modified_files(files)?;

    loop {
        let files_batch = only_modified.by_ref().take(50).collect::<Vec<_>>();

        if files_batch.is_empty() {
            break;
        }

        let files_by_extension = map_files_by_extension(&files_batch);

        println!(
            "Files by extension: {:?}",
            files_by_extension
                .iter()
                .map(|(ext, files)| (ext, files.len()))
                .collect::<Vec<_>>()
        );

        let content_by_file = extract_content_from_files(files_by_extension)?;

        match index_files_to_tantivy(&content_by_file) {
            Ok(_) => println!("Files, indexed"),
            Err(e) => {
                eprintln!("{e}")
            }
        };

        match SqliteRepository::new() {
            Ok(repo) => {
                match repo.insert_files_batch(&files_batch) {
                    Ok(_) => {
                        println!("Files persisted : {}", files_batch.len())
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
    }

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
