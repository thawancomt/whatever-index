use std::sync::OnceLock;

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

pub static TANTIVY_INDEX: OnceLock<Index> = OnceLock::new();
