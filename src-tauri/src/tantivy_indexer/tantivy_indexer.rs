use std::{collections::HashMap, fs, path::PathBuf};

use tantivy::{schema::Schema, Index, IndexWriter};

use crate::{
    db::tantivy::{tantivy_schema_builder, TANTIVY_INDEX},
    paths::DATA_DIR,
};

pub struct IndexerService {
    tantivy_writer: IndexWriter,
    schema: Schema,
}

impl IndexerService {
    pub fn new(index: Index) -> tantivy::Result<Self> {
        let tantivy_writer = index.writer(50_000_000)?;
        let schema = tantivy_schema_builder();

        Ok(Self {
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

        let Ok(file_name_field) = self.schema.get_field("file_name") else {
            return None;
        };

        for (file, content) in files {
            let mut doc = tantivy::TantivyDocument::default();
            doc.add_text(path_field, file.to_string_lossy());
            doc.add_text(content_field, content);
            doc.add_text(
                file_name_field,
                file.file_name().unwrap_or_default().to_string_lossy(),
            );
            let _ = self.tantivy_writer.add_document(doc);
        }

        let _ = self.tantivy_writer.commit();

        Some(())
    }
    pub fn drop(&self) {
        let _ = match DATA_DIR.get() {
            Some(dir) => fs::remove_dir(dir.join("/tantivy-data")),
            None => Ok({}),
        };
    }
}
