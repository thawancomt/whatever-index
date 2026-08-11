use rusqlite::Connection;

use crate::paths::DATA_DIR;

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

pub fn get_database() -> rusqlite::Result<Connection> {
    let db_path = DATA_DIR
        .get()
        .expect("Not data dir folder found")
        .join("whatever-index.db");
    rusqlite::Connection::open(db_path)
}
