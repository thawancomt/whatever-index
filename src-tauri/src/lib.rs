use std::fs;

use tantivy::Index;
use tauri::Manager;

use crate::{
    db::{
        database::init_database,
        tantivy::{tantivy_schema_builder, TANTIVY_INDEX},
    },
    paths::{DATA_DIR, RESOURCE_DIR},
};

mod adapters;
mod db;
mod extractors;
mod paths;
mod repositories;
mod scanners;
mod use_cases;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let resource_dir = app.path().resource_dir().expect("Missing resource dir");
            let data_dir = app.path().app_data_dir().expect("Missing data dir");

            RESOURCE_DIR
                .set(resource_dir.join("resources"))
                .expect("Resource dir not setted");

            DATA_DIR
                .set(data_dir.clone())
                .expect("Error while setting datadir");

            let tantivy_dir = data_dir.join("./tantivy_data");

            if !tantivy_dir.exists() {
                print!("Tantivy folder doesnt existis, creating it");
                let _ = fs::create_dir_all(&tantivy_dir);
                let _ = Index::create_in_dir(&tantivy_dir, tantivy_schema_builder());
            };

            let tantivy_index = Index::open_in_dir(tantivy_dir)
                .expect("Error while opening the tantivy data folder");

            TANTIVY_INDEX
                .set(tantivy_index)
                .expect("Error while setting tantivy during setup");

            match init_database() {
                Ok(_database) => println!("Database initilized fine"),
                Err(e) => {
                    println!("erro : {}", e)
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scanners::commands::re_scan,
            scanners::commands::search,
            utils::commands::open_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
