use std::{collections::HashMap, fs};

use tantivy::{schema::Schema, IndexWriter, Term};

use crate::{
    app_error::errors::{AppError, AppResult},
    db::{tantivy::TANTIVY_INDEX, types::file::File},
    paths::DATA_DIR,
};

pub struct IndexerService {
    tantivy_writer: IndexWriter,
    schema: Schema,
}

impl IndexerService {
    pub fn new() -> AppResult<Self> {
        let global_indexer = TANTIVY_INDEX
            .read()
            .map_err(|_| AppError::TantivyIndexError("Tantivy index lock poisoned".into()))?
            .as_ref()
            .cloned()
            .ok_or(AppError::TantivyIndexError(
                "Tantivy not can be oppened".into(),
            ))?;

        let tantivy_writer = global_indexer.writer(50_000_000)?;

        let schema = global_indexer.schema();

        Ok(Self {
            tantivy_writer,
            schema,
        })
    }

    pub fn index_files(&mut self, files: &HashMap<File, String>) -> AppResult<()> {
        let path_field = self.schema.get_field("path")?;

        let content_field = self.schema.get_field("content")?;
        let content_ngram_field = self.schema.get_field("content_ngram")?;

        let file_name_field = self.schema.get_field("file_name")?;

        for (file, content) in files {
            let mut doc = tantivy::TantivyDocument::default();

            let term = Term::from_field_text(path_field, &file.path.to_string_lossy());

            self.tantivy_writer.delete_term(term);

            doc.add_text(path_field, file.path.to_string_lossy());
            doc.add_text(content_field, content);
            doc.add_text(content_ngram_field, content);
            doc.add_text(
                file_name_field,
                file.path.file_name().unwrap_or_default().to_string_lossy(),
            );
            self.tantivy_writer.add_document(doc)?;
        }

        self.tantivy_writer.commit()?;

        Ok(())
    }
    pub fn drop(&self) {
        let _ = match DATA_DIR.get() {
            Some(dir) => fs::remove_dir(dir.join("/tantivy-data")),
            None => Ok({}),
        };
    }
}
