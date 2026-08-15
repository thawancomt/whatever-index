use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SupportedExt {
    Txt,
    Pdf,
    Log,
    Env,
    Docx,
    Json,
    Rs,
    Py,
    Go,
    Ts,
    Tsx,
    Js,
    Jsx,
    Jpg,
    Png,
    Webp,
    Jpeg,
}

impl SupportedExt {
    fn parse(path: impl AsRef<Path>) -> Option<Self> {
        let ext = path
            .as_ref()
            .extension()
            .and_then(|f| f.to_str())
            .unwrap_or("");

        match ext {
            "txt" => Some(Self::Txt),
            "pdf" => Some(Self::Pdf),
            "docx" => Some(Self::Docx),
            "log" => Some(Self::Log),
            "json" => Some(Self::Json),
            "env" => Some(Self::Env),
            "rs" => Some(Self::Rs),
            "py" => Some(Self::Py),
            "go" => Some(Self::Go),
            "ts" => Some(Self::Ts),
            "tsx" => Some(Self::Tsx),
            "png" => Some(Self::Png),
            "jpg" => Some(Self::Jpg),
            "jpeg" => Some(Self::Jpeg),
            "webp" => Some(Self::Webp),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Docx => "docx",
            Self::Log => "log",
            Self::Json => "json",
            Self::Env => "env",
            Self::Rs => "rs",
            Self::Py => "py",
            Self::Go => "go",
            Self::Ts => "ts",
            Self::Tsx => "tsx",
            Self::Js => "js",
            Self::Jsx => "jsx",
            Self::Txt => "txt",
            Self::Png => "png",
            Self::Jpg => "jpg",
            Self::Webp => "webp",
            Self::Jpeg => "jpeg",
        }
    }
}

impl std::fmt::Display for SupportedExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
pub struct File {
    pub id: Option<i64>,
    pub path: PathBuf,
    pub extension: SupportedExt,
    pub mtime: i64,
    pub content_hash: String,
    pub indexed_at: i64,
    pub size_bytes: i64,
}

fn hash_file(path: impl AsRef<Path>) -> String {
    let mut hasher = blake3::Hasher::new();

    if let Ok(mut file) = std::fs::File::open(path) {
        let _ = std::io::copy(&mut file, &mut hasher);
    }

    hasher.finalize().to_string()
}

impl File {
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Option<Self>> {
        let path = path.as_ref();

        let Some(extension) = SupportedExt::parse(path) else {
            return Ok(None);
        };

        println!("Creating File struct for path: {:?}", path);

        let metadata = path.metadata()?;

        let mtime = metadata
            .modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let size_bytes = metadata.len() as i64;

        let indexed_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let hash = hash_file(path);

        Ok(Some(Self {
            id: None,
            path: path.into(),
            extension: extension,
            content_hash: hash,
            indexed_at,
            mtime,
            size_bytes,
        }))
    }
}

impl AsRef<OsStr> for File {
    fn as_ref(&self) -> &OsStr {
        self.path.as_os_str()
    }
}
