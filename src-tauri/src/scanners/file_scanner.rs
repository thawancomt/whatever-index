use rayon::prelude::*;
use std::{collections::HashMap, env, path::PathBuf};
use walkdir::DirEntry;

use crate::db::{database::get_database, types::file::File};

pub type ScanResponse = Vec<File>;
pub type FilesByExtensionResponse = HashMap<String, Vec<PathBuf>>;

fn is_hidden(path: &DirEntry) -> bool {
    path.file_name().to_string_lossy().starts_with('.')
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

pub fn scanner() -> Option<ScanResponse> {
    let root = env::home_dir()?;
    let cache = load_mtime_cache()?;

    println!("Loaded mtime cache: {}", cache.len());

    let files: ScanResponse = walkdir::WalkDir::new(root)
        .into_iter()
        .filter_entry(|f| !is_hidden(f))
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
            .entry(file.extension)
            .or_default()
            .push(file.path);
    }

    files_by_extension
}
