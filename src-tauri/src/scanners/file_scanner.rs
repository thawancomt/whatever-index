use rayon::prelude::*;
use std::{
    collections::HashMap,
    env::{self},
    path::Path,
};

use crate::{
    app_error::errors::{AppError, AppResult},
    db::types::file::File,
    repositories::file_cache_repository::{FileCacheRepository, FileCacheService},
    settings::commands::get_settings,
};

pub type FilesByExtensionResponse = HashMap<String, Vec<File>>;

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
    let mut condition = !is_hidden(path.as_ref()) && filter_exclusion_folders(path.as_ref());

    if let Ok(settings) = get_settings() {
        if !settings.index_images {
            let media_extensions = ["png", "jpg", "jpeg"];
            if let Some(ext) = path.as_ref().extension() {
                condition = condition
                    && !media_extensions.contains(&ext.to_string_lossy().to_lowercase().as_str());
            }
        }
    }

    condition
}

pub fn map_files_by_extension(paths: &[File]) -> FilesByExtensionResponse {
    let mut files_by_extension: FilesByExtensionResponse = HashMap::new();

    for file in paths.to_vec() {
        files_by_extension
            .entry(file.extension.to_string())
            .or_default()
            .push(file);
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

    pub fn scan_home_dir(&self) -> AppResult<impl Iterator<Item = File>> {
        let Some(home_dir) = env::home_dir() else {
            return Err(AppError::DataDirNotSet);
        };

        // all entries found on the home directory
        let founded_entries = walkdir::WalkDir::new(home_dir)
            .into_iter()
            .filter_entry(|f| exclusion_pipeline(f.path()))
            .filter_map(Result::ok)
            .filter(|f| f.path().is_file())
            .filter_map(|f| File::new(f.path()).ok().flatten());

        Ok(founded_entries)
    }

    pub fn get_modified_files(
        &self,
        files: impl Iterator<Item = File>,
    ) -> AppResult<impl Iterator<Item = File>> {
        let cached_mtime = self.cache_service.retrieve_mtime()?;

        let modified_files = files.filter(move |f| {
            let Some(key) = f.path.to_str() else {
                return false;
            };

            match cached_mtime.get(key) {
                Some(cached) => f.mtime != *cached,
                None => true,
            }
        });

        Ok(modified_files)
    }
}
