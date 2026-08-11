use std::{
    collections::HashMap,
    fs,
    io::Error,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tauri::path::BaseDirectory::Cache;
#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub id: Option<i64>,
    pub path: PathBuf,
    pub extension: String,
    pub mtime: i64,
    pub content_hash: String,
    pub indexed_at: i64,
    pub size_bytes: i64,
}

fn hash_file(path: &PathBuf) -> String {
    let content = fs::read(path).unwrap_or_default();
    blake3::hash(&content).to_string()
}

impl File {
    pub fn from_path(path: &PathBuf) -> std::io::Result<Self> {
        let metadata = path.metadata()?;

        let extension = path
            .extension()
            .and_then(|f| f.to_str())
            .unwrap_or_default()
            .to_lowercase()
            .to_string();

        let size_bytes = metadata.len() as i64;

        let indexed_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let mtime = metadata
            .modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let hash = hash_file(path);

        Ok(Self {
            id: None,
            path: path.into(),
            extension,
            indexed_at,
            mtime,
            size_bytes,
            content_hash: hash,
        })
    }

    pub fn from_path_using_cache(
        path: &PathBuf,
        cache: &HashMap<PathBuf, i64>,
    ) -> std::io::Result<Option<Self>> {
        let metadata = path.metadata()?;

        let mtime = metadata
            .modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        if let Some(&cached_mtime) = cache.get(path) {
            if mtime <= cached_mtime {
                // File hasn't changed. Skip it entirely.
                return Ok(None);
            }
        }

        let extension = path
            .extension()
            .and_then(|f| f.to_str())
            .unwrap_or_default()
            .to_lowercase()
            .to_string();

        let size_bytes = metadata.len() as i64;

        let indexed_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let hash = hash_file(path);

        Ok(Some(Self {
            id: None,
            path: path.into(),
            extension,
            indexed_at,
            mtime,
            size_bytes,
            content_hash: hash,
        }))
    }
}
