use tauri::State;
use crate::{app_error::errors::AppResult, extractors::extractor::{extract_content_from_files, ExtractedContentFromFilesResponse}, repositories::{
    file_cache_repository::FileCacheRepository,
    sqlite_repository::{SQLRepository, SqliteRepository},
}, scanners::file_scanner::{map_files_by_extension, Scanner}, tantivy_indexer::tantivy_indexer::IndexerService, AppState};
use crate::emitter::app_emitter::{AppEmitter, ResultEvent};

pub fn scan_files() -> AppResult<()> {
    let cache_service = FileCacheRepository::new()?;
    let scanner_service = Scanner::new(cache_service);

    let files = scanner_service.scan_home_dir()?;
    let mut only_modified = scanner_service.get_modified_files(files)?;

    loop {
        let files_batch = only_modified.by_ref().take(100).collect::<Vec<_>>();

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

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::{
        db::{
            database::init_database,
            tantivy::{init_tantivy_index, reset_tantivy_index},
        },
        paths::DATA_DIR,
        settings::setting_manager::{SettingsManager, SETTINGS_MANAGER},
        use_cases::{scan_files::scan_files, search::search_text},
    };

    #[test]
    fn scan_and_search_by_ngram_prefix() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock error")
            .as_nanos();
        let test_root = std::env::temp_dir().join(format!("whatever_index_scan_test_{unique}"));
        let home_dir = test_root.join("home");
        let data_dir = test_root.join("app_data");
        let docs_dir = home_dir.join("docs");
        let sample_file = docs_dir.join("sample.txt");

        fs::create_dir_all(&docs_dir).expect("failed to create docs dir");
        fs::create_dir_all(&data_dir).expect("failed to create data dir");
        fs::write(
            &sample_file,
            "A palavra cachaça deve ser localizada por prefixo.",
        )
        .expect("failed to write sample file");

        // Force scanner root so this test runs quickly and deterministically.
        std::env::set_var("HOME", &home_dir);
        std::env::set_var("USERPROFILE", &home_dir);

        let _ = DATA_DIR.get_or_init(|| data_dir.clone());

        init_database().expect("database initialization failed");
        init_tantivy_index(&data_dir.join("tantivy_data")).expect("tantivy init failed");

        let settings_manager = SETTINGS_MANAGER.get_or_init(|| {
            SettingsManager::new().expect("settings manager initialization failed")
        });
        settings_manager
            .init_setting_json()
            .expect("settings initialization failed");

        scan_files().expect("scan_files failed");

        let result = search_text("cac".to_string());
        assert!(
            result.contains(&sample_file),
            "expected 'cac' to match file containing 'cachaça'"
        );

        reset_tantivy_index();
        let _ = fs::remove_dir_all(&test_root);
    }
}
