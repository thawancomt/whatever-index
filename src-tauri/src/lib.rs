use tauri::Manager;

use crate::{
    db::{database::init_database, tantivy::init_tantivy_index},
    paths::{DATA_DIR, RESOURCE_DIR},
    settings::setting_manager::{SettingsManager, SETTINGS_MANAGER},
};

mod adapters;
mod app_error;
mod db;
mod extractors;
mod paths;
mod repositories;
mod scanners;
mod settings;
mod tantivy_indexer;
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

            let tantivy_dir = data_dir.join("tantivy_data");

            init_tantivy_index(&tantivy_dir)
                .expect("Error while initializing the Tantivy data folder");

            match init_database() {
                Ok(_database) => println!("Database initilized fine"),
                Err(e) => {
                    println!("erro : {}", e)
                }
            }

            let settings_service = SettingsManager::new()?;

            match settings_service.init_setting_json() {
                Ok(data) => println!("Settings initilized fine"),
                Err(e) => {
                    eprintln!("Error on setting manager {e}")
                }
            }

            SETTINGS_MANAGER
                .set(settings_service)
                .expect("Error while setting the settings");

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scanners::commands::re_scan,
            scanners::commands::search,
            utils::commands::open_file,
            db::commands::reset_index,
            db::commands::get_total_files_indexed,
            db::commands::get_total_by_extension,
            settings::commands::get_settings,
            settings::commands::toggle_settings,
            adapters::ocr::commands::get_ocr_models_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
