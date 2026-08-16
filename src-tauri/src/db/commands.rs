use std::collections::HashMap;

use crate::{
    app_error::errors::{AppError, AppResult},
    db::database::get_database,
    use_cases::drop_indexing::drop_all,
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

#[tauri::command(async)]
pub fn get_files_by_extension(
    extension: &str,
) -> AppResult<Vec<HashMap<String, serde_json::Value>>> {
    let conn = get_database()?;

    let mut stmt = conn.prepare(
        "
        SELECT path, CAST(size_bytes AS INTEGER) FROM files
        WHERE extension = ?;
        ",
    )?;

    let query = stmt.query_map([extension], |row| {
        let path = row.get::<_, String>(0)?;
        let size_bytes = row.get::<_, i64>(1)?;

        let data = HashMap::from([
            ("path".to_string(), serde_json::Value::String(path)),
            (
                "size_bytes".to_string(),
                serde_json::Value::Number(size_bytes.into()),
            ),
        ]);

        Ok(data)
    })?;

    let data = query.filter_map(|f| f.ok()).collect();

    Ok(data)
}

#[tauri::command(async)]
pub fn get_file_content(path: &str) -> AppResult<String> {
    let file = std::fs::read_to_string(path)
        .map_err(|e| AppError::Generic(format!("Error while reading file {path}: {e}")))?;

    Ok(file)
}
