use rusqlite::Connection;

use crate::db::{database::get_database, types::file::File};

pub trait SQLRepository {
    fn insert_files_batch(&self, files: &[File]) -> rusqlite::Result<()>;
}

pub struct SqliteRepository {
    conn: Connection,
}

impl SqliteRepository {
    pub fn new() -> rusqlite::Result<Self> {
        let conn = get_database().expect("Erro while getting database");
        Ok(Self { conn })
    }
}

impl SQLRepository for SqliteRepository {
    fn insert_files_batch(&self, files: &[File]) -> rusqlite::Result<()> {
        if files.is_empty() {
            return Ok(());
        }

        let payload = serde_json::to_string(&files)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        self.conn
            .execute(INSERT_SCHEMA_BATCH, rusqlite::params![payload])?;

        Ok(())
    }
}

// const INSERT_SCHEMA: &str = include_str!("../db/queries/insert_file.sql");
const INSERT_SCHEMA_BATCH: &str = include_str!("../db/queries/insert_files_batch.sql");
