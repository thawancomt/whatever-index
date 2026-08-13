use std::fs;

use crate::{
    app_error::errors::{AppError, AppResult},
    db::database::drop_database,
    paths::DATA_DIR,
    repositories::commands::drop_indexing,
    tantivy_indexer::tantivy_indexer::IndexerService,
};

/// Drop database + tantivy files [Called by fron-end from command]
pub fn drop_all() -> AppResult<()> {
    let data_dir = DATA_DIR.get().ok_or(AppError::DataDirNotSet)?;

    let indexer = IndexerService::new()?;

    let tantivy_dir = data_dir.join("tantivy_data");

    if tantivy_dir.exists() {
        let _ = fs::remove_dir_all(tantivy_dir)
            .map_err(|e| format!("Error while deleting the tantivy folder on drop use case: {e}"));
    }

    drop_database()?;

    drop_indexing();
    indexer.drop();

    println!("Every index dropped");

    Ok(())
}
