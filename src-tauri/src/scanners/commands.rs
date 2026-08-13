use std::{collections::HashSet, path::PathBuf};

use crate::use_cases::{scan_files, search::search_text};

#[tauri::command(async)]
pub fn re_scan() {
    println!("Get re-scan request");
    if let Err(error) = scan_files::scan_files() {
        eprintln!("Error while scanning files: {error}");
    }
}

#[tauri::command(async)]
pub fn search(search: String) -> HashSet<PathBuf> {
    search_text(search)
}
