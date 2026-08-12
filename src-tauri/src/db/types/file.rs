use std::{
    collections::HashMap,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
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
    let mut hasher = blake3::Hasher::new();
    let Ok(mut file) = std::fs::File::open(path) else {
        return String::new();
    };

    let mut buffer = [0u8; 65536]; // 64KB por vez
    loop {
        match std::io::Read::read(&mut file, &mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                hasher.update(&buffer[..n]);
            }
            Err(_) => break,
        }
    }

    hasher.finalize().to_string()
}

impl File {
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
        println!("File saved: {}", path.display());

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
