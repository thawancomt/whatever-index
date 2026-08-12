use std::path::PathBuf;

use rusqlite::{Connection, Error as SqliteError};

use crate::{
    app_error::errors::{AppError, AppResult},
    paths::DATA_DIR,
};

const SCHEMA: &str = include_str!("./queries/schema.sql");

pub fn init_database() -> rusqlite::Result<()> {
    let db_path = DATA_DIR
        .get()
        .expect("Not data dir folder found")
        .join("whatever-index.db");
    let conn = rusqlite::Connection::open(db_path)?;

    conn.execute_batch(SCHEMA)?;

    Ok(())
}

pub fn get_database() -> AppResult<Connection> {
    let data_dir = DATA_DIR.get().ok_or(AppError::DataDirNotSet)?;

    let db_path = data_dir.join("whatever-index.db");

    let conn = Connection::open(db_path)?;

    Ok(conn)
}

pub fn drop_database() -> AppResult<()> {
    let conn = get_database()?;

    let result = conn.execute(DROP_FILES_SQL, [])?;

    Ok(())
}

const DROP_FILES_SQL: &str = include_str!("./queries/drop_files.sql");
