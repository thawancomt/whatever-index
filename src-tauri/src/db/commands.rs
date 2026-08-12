use crate::{
    app_error::errors::AppResult, db::database::get_database, use_cases::drop_indexing::drop_all,
};

#[tauri::command(async)]
pub fn reset_index() {
    println!("Droping database and tantivy data...");
    match drop_all() {
        Ok(_result) => {
            println!("Sucessfully dropped tantvy and database index")
        }
        Err(e) => {
            eprintln!("Error while trying to drop database and or tantivy data: {e}")
        }
    }
}

#[tauri::command(async)]
pub fn get_total_files_indexed() -> AppResult<i64> {
    let conn = get_database()?;

    let total: i64 = conn.query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))?;

    Ok(total)
}
