use std::collections::HashMap;

use rusqlite::Connection;

use crate::{
    app_error::errors::{AppError, AppResult},
    db::database::get_database,
};

pub trait FileCacheService {
    fn retrieve_mtime(&self) -> AppResult<HashMap<String, i64>>;
}
pub struct FileCacheRepository {
    conn: Connection,
}

impl FileCacheRepository {
    pub fn new() -> AppResult<Self> {
        let conn = get_database()?;

        Ok(Self { conn })
    }
}

impl FileCacheService for FileCacheRepository {
    fn retrieve_mtime(&self) -> AppResult<HashMap<String, i64>> {
        let query = "SELECT path, CAST(mtime as INTEGER) FROM files";

        let mut stmt = self.conn.prepare(query).map_err(|s| AppError::Sqlite(s))?;

        let data = stmt.query_map([], |row| {
            let path = row.get::<_, String>(0)?;
            let mtime = row.get::<_, i64>(1)?;
            Ok((path, mtime))
        })?;

        let valid_rows: HashMap<String, i64> = data.filter_map(|f| f.ok()).collect();

        Ok(valid_rows)
    }
}
