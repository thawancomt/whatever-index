use std::{fs, path::Path, sync::RwLock};

use tantivy::{
    schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions, STORED, STRING, TEXT},
    tokenizer::{LowerCaser, NgramTokenizer, TextAnalyzer},
    Index,
};

pub fn tantivy_schema_builder() -> Schema {
    let mut schema = Schema::builder();

    schema.add_text_field("path", STRING | STORED);
    schema.add_text_field("file_name", STRING | STORED);
    schema.add_text_field("content", TEXT);

    let ngram_indexing = TextFieldIndexing::default()
        .set_tokenizer("ngram3")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let ngram_options = TextOptions::default().set_indexing_options(ngram_indexing);

    schema.add_text_field("content_ngram", ngram_options);

    schema.build()
}

pub static TANTIVY_INDEX: RwLock<Option<Index>> = RwLock::new(None);

pub fn reset_tantivy_index() {
    let mut index = TANTIVY_INDEX.write().expect("Tantivy index lock poisoned");
    *index = None;
}

pub fn init_tantivy_index(index_dir: &Path) -> Result<(), String> {
    if !index_dir.exists() {
        fs::create_dir_all(index_dir)
            .map_err(|e| format!("Error while creating the Tantivy folder: {e}"))?;
    }

    let index = match Index::open_in_dir(index_dir) {
        Ok(index) => index,
        Err(_) => Index::create_in_dir(index_dir, tantivy_schema_builder())
            .map_err(|e| format!("Error while creating the Tantivy index: {e}"))?,
    };

    // 1. Criamos um TextAnalyzer que aplica o Ngram e depois transforma tudo em minúsculo
    let ngram_analyzer = TextAnalyzer::builder(NgramTokenizer::new(2, 10, false).unwrap())
        .filter(LowerCaser)
        .build();

    // 2. Registramos o nosso analyzer com o LowerCaser
    index.tokenizers().register("ngram3", ngram_analyzer);

    reset_tantivy_index();
    let mut active = TANTIVY_INDEX.write().expect("Tantivy index lock poisoned");
    *active = Some(index);

    Ok(())
}
