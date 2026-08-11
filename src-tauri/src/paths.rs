use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub static RESOURCE_DIR: OnceLock<PathBuf> = OnceLock::new();
pub static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn get_resource_dir() -> &'static Path {
    RESOURCE_DIR
        .get()
        .expect("resource dir não inicializado — chame init_resource_dir() no setup()")
}

pub fn get_data_dir() -> &'static Path {
    DATA_DIR
        .get()
        .expect("resource dir não inicializado — chame init_resource_dir() no setup()")
}
