use serde::Serialize;
use tantivy::TantivyError;

fn to_string_serializer<T: std::fmt::Display, S: serde::Serializer>(
    val: &T,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(&val.to_string())
}

#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "code", content = "data")]
pub enum AppError {
    #[error("Data dir not set")]
    DataDirNotSet,

    #[error("OCR engine initialization error")]
    OCREngineInitializationError(String),

    #[error("Resource dir not set")]
    ResourceDirNotSet,

    #[error("Generic app error: {0}")]
    Generic(String),

    #[error("OCR Model Error: {message}")]
    OCRModelMissing { status: u16, message: String },

    #[error("Database Error: {0}")]
    #[serde(serialize_with = "to_string_serializer")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Error Tantivy: {0}")]
    #[serde(serialize_with = "to_string_serializer")]
    Tantivy(#[from] TantivyError),

    #[error("Error Tantivy: {0}")]
    TantivyIndexError(String),

    #[error("I/O Error: {0}")]
    #[serde(serialize_with = "to_string_serializer")]
    Io(#[from] std::io::Error),
}

pub type AppResult<T> = Result<T, AppError>;
