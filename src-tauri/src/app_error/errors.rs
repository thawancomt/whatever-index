use serde::Serialize;
use tantivy::TantivyError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Data dir not setted")]
    DataDirNotSet,

    #[error("Generic app error:")]
    Generic(String),

    #[error("Database Error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Error Tantivy: {0}")]
    Tantivy(#[from] TantivyError),

    #[error("Error Tantivy: {0}")]
    TantivyIndexError(String),

    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
