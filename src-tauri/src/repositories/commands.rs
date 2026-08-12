use crate::db::database::get_database;

pub fn drop_indexing() {
    let Ok(conn) = get_database() else {
        eprintln!("Database at this point should be working");
        return;
    };

    match conn.execute(DROP_FILES_SQL, []) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error while droping database {}", e);
            return;
        }
    }
}

const DROP_FILES_SQL: &str = include_str!("../db/queries/drop_files.sql");
