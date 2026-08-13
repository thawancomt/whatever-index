use std::{
    fs,
    path::Path,
    sync::RwLock,
};

use tantivy::{
    schema::{Schema, STORED, STRING, TEXT},
    Index,
};

pub fn tantivy_schema_builder() -> Schema {
    let mut schema = Schema::builder();

    schema.add_text_field("path", STRING | STORED);
    schema.add_text_field("content", TEXT);
    schema.add_text_field("file_name", STRING | STORED);

    schema.build()
}

pub static TANTIVY_INDEX: RwLock<Option<Index>> = RwLock::new(None);

pub fn reset_tantivy_index() {
    let mut index = TANTIVY_INDEX.write().expect("Tantivy index lock poisoned");
    *index = None;
}

pub fn init_tantivy_index(index_dir: &Path) -> Result<(), String> {
    if !index_dir.exists() {
        fs::create_dir_all(index_dir).map_err(|e| format!("Error while creating the Tantivy folder: {e}"))?;
    }

    let index = match Index::open_in_dir(index_dir) {
        Ok(index) => index,
        Err(_) => Index::create_in_dir(index_dir, tantivy_schema_builder())
            .map_err(|e| format!("Error while creating the Tantivy index: {e}"))?,
    };

    reset_tantivy_index();
    let mut active = TANTIVY_INDEX.write().expect("Tantivy index lock poisoned");
    *active = Some(index);

    Ok(())
}
