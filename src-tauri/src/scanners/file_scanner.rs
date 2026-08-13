use rayon::prelude::*;
use std::{
    collections::HashMap,
    env::{self},
    path::{Path, PathBuf},
};
use walkdir::DirEntry;

use crate::{
    app_error::errors::{AppError, AppResult},
    db::{database::get_database, types::file::File},
    repositories::file_cache_repository::{FileCacheRepository, FileCacheService},
};

pub type FilesByExtensionResponse = HashMap<String, Vec<PathBuf>>;

fn is_hidden(path: impl AsRef<Path>) -> bool {
    path.as_ref()
        .file_name()
        .map_or(false, |f| f.to_string_lossy().starts_with('.'))
}

#[cfg(windows)]
fn filter_exclusion_folders(path: impl AsRef<Path>) -> bool {
    if let Some(root) = env::home_dir() {
        let excluded = [root.join("AppData")];
        if excluded.iter().any(|e| path.as_ref().starts_with(e)) {
            return false;
        }
    }
    true
}
#[cfg(not(windows))]
fn filter_exclusion_folders(_path: &Path) -> bool {
    true
}

fn exclusion_pipeline(path: impl AsRef<Path>) -> bool {
    !is_hidden(path.as_ref()) || filter_exclusion_folders(path.as_ref())
}

fn load_mtime_cache() -> Option<HashMap<PathBuf, i64>> {
    let conn = get_database().ok()?;
    let mut stmt = conn
        .prepare("SELECT path, CAST(mtime AS INTEGER) FROM files")
        .ok()?;

    let rows = stmt
        .query_map([], |row| {
            let path_str: String = row.get(0)?;
            let mtime: i64 = row.get(1)?;
            Ok((PathBuf::from(path_str), mtime))
        })
        .ok()?;

    // Ignora linhas com erro silenciosamente mantendo o código enxuto
    Some(rows.filter_map(Result::ok).collect())
}

pub fn scanner() -> Option<Vec<File>> {
    let root = env::home_dir()?;
    let cache = load_mtime_cache()?;

    println!("Root path: {}", root.display());
    println!("Loaded mtime cache: {}", cache.len());

    let files: Vec<File> = walkdir::WalkDir::new(root)
        .into_iter()
        .filter_entry(|f| !is_hidden(f.path()))
        .filter_map(Result::ok)
        .par_bridge()
        .filter(|f| f.path().is_file())
        .filter_map(|f| {
            File::from_path_using_cache(&f.into_path(), &cache)
                .ok()
                .flatten()
        })
        .collect();

    Some(files)
}

pub fn map_files_by_extension(paths: &[File]) -> FilesByExtensionResponse {
    let mut files_by_extension: FilesByExtensionResponse = HashMap::new();

    for file in paths.to_vec() {
        files_by_extension
            .entry(file.extension.to_string())
            .or_default()
            .push(file.path);
    }

    files_by_extension
}

pub struct Scanner {
    cache_service: FileCacheRepository,
}

impl Scanner {
    pub fn new(file_cache_service: FileCacheRepository) -> Self {
        return Self {
            cache_service: file_cache_service,
        };
    }
    pub fn scan_home_dir(&self) -> AppResult<Vec<File>> {
        let Some(home_dir) = env::home_dir() else {
            return Err(AppError::DataDirNotSet);
        };

        // all entries found on the home directory
        let founded_entries: Vec<DirEntry> = walkdir::WalkDir::new(home_dir)
            .into_iter()
            .filter_entry(|f| !is_hidden(f.path()))
            .filter_map(Result::ok)
            .collect();

        let valid_files: Vec<File> = founded_entries
            .into_iter()
            .par_bridge()
            .filter(|f| f.path().is_file())
            .filter_map(|f| File::new(f.path()).ok().flatten())
            .collect();

        Ok(valid_files)
    }

    pub fn get_modified_files(&self, files: Vec<&File>) -> AppResult<Vec<File>> {
        let cached_mtime = self.cache_service.retrieve_mtime()?;

        let modified_files: Vec<File> = files
            .into_iter()
            .filter_map(|f| {
                let key = f.path.to_str()?;
                let Some(mtime) = cached_mtime.get(key) else {
                    return None;
                };

                if *mtime != f.mtime {
                    return Some(f.clone());
                }

                None
            })
            .collect();

        Ok(modified_files)
    }
}
