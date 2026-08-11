use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use tantivy::{
    doc,
    schema::{Schema, STORED, STRING, TEXT},
    Index, IndexWriter,
};

use crate::repositories::sqlite_repository::SqliteRepository;

pub fn tantivy_schema_builder() -> Schema {
    let mut schema = Schema::builder();

    schema.add_text_field("path", STRING | STORED);
    schema.add_text_field("content", TEXT);

    schema.build()
}

pub struct IndexerService {
    database_repo: SqliteRepository,
    tantivy_writer: IndexWriter,
    schema: Schema,
}

impl IndexerService {
    pub fn new(db: SqliteRepository, index: Index) -> tantivy::Result<Self> {
        let tantivy_writer = index.writer(50_000_000)?;
        let schema = tantivy_schema_builder();

        Ok(Self {
            database_repo: db,
            tantivy_writer,
            schema,
        })
    }

    pub fn index_files(&mut self, files: &HashMap<PathBuf, String>) -> Option<()> {
        let Ok(path_field) = self.schema.get_field("path") else {
            return None;
        };
        let Ok(content_field) = self.schema.get_field("content") else {
            return None;
        };

        for (file, content) in files {
            let mut doc = tantivy::TantivyDocument::default();
            doc.add_text(path_field, file.to_string_lossy());
            doc.add_text(content_field, content);
            self.tantivy_writer.add_document(doc);
        }

        self.tantivy_writer.commit();

        Some(())
    }
}

pub static TANTIVY_INDEX: OnceLock<Index> = OnceLock::new();
