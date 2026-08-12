use crate::use_cases::drop_indexing::drop_all;

#[tauri::command(async)]
pub fn reset_index() {
    println!("Every index dropped2");
    match drop_all() {
        Some(_) => {}
        None => {}
    }
}
