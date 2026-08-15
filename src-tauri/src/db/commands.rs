use std::collections::HashMap;

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

#[tauri::command(async)]
pub fn get_total_by_extension() -> AppResult<HashMap<String, i64>> {
    let conn = get_database()?;

    let mut stmt = conn.prepare(
        "
        SELECT extension, COUNT(path) AS amount FROM files
        GROUP BY
        extension;
        ",
    )?;

    let query = stmt.query_map([], |row| {
        let extension = row.get::<_, String>(0)?;
        let amount = row.get::<_, i64>(1)?;
        Ok((extension, amount))
    })?;

    let data: HashMap<String, i64> = query.filter_map(|f| f.ok()).collect();

    Ok(data)
}
