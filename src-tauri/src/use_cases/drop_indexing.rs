use std::fs;

use crate::{
    app_error::errors::{AppError, AppResult},
    db::{
        database::drop_database,
        tantivy::{init_tantivy_index, reset_tantivy_index},
    },
    paths::DATA_DIR,
    repositories::commands::drop_indexing,
};

/// Drop database + tantivy files [Called by fron-end from command]
pub fn drop_all() -> AppResult<()> {
    let data_dir = DATA_DIR.get().ok_or(AppError::DataDirNotSet)?;
    let tantivy_dir = data_dir.join("tantivy_data");

    {
        let _indexer = match crate::tantivy_indexer::tantivy_indexer::IndexerService::new() {
            Ok(indexer) => Some(indexer),
            Err(_) => None,
        };
    }

    drop_database()?;

    drop_indexing();

    if tantivy_dir.exists() {
        fs::remove_dir_all(&tantivy_dir).map_err(|e| AppError::Io(e))?;
    }

    reset_tantivy_index();
    init_tantivy_index(&tantivy_dir).map_err(|e| {
        AppError::TantivyIndexError(format!(
            "Error while reinitializing Tantivy after reset: {e}"
        ))
    })?;

    println!("Every index dropped");

    Ok(())
}
