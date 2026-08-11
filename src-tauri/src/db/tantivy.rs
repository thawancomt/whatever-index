use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use tantivy::{
    doc,
    schema::{Schema, Value, STORED, STRING, TEXT},
    Index, IndexWriter, TantivyDocument,
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
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{IndexReader, ReloadPolicy};

// Uma estrutura simples para retornar os resultados para o seu frontend ou API
#[derive(Debug)]
pub struct SearchResult {
    pub path: String,
    pub score: f32, // Relevância do resultado
}

pub struct SearchService {
    reader: IndexReader,
    schema: Schema,
    index: Index,
}

impl SearchService {
    pub fn new() -> tantivy::Result<Self> {
        // O reader cria um "snapshot" do índice.
        // O ReloadPolicy::OnCommit faz com que ele se atualize automaticamente
        // sempre que o IndexWriter fizer um .commit().
        let index = TANTIVY_INDEX
            .get()
            .expect("Error while getting tantivy in the search function");

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        Ok(Self {
            reader,
            schema: tantivy_schema_builder(),
            index: index.clone(),
        })
    }

    /// Realiza a busca full-text e retorna os caminhos dos arquivos encontrados
    pub fn search(&self, query_string: &str, limit: usize) -> tantivy::Result<Vec<SearchResult>> {
        // 1. Prepara o Searcher a partir do Reader
        let searcher = self.reader.searcher();

        // 2. Configura o QueryParser
        // Dizemos ao parser para buscar no campo "content" por padrão
        let content_field = self.schema.get_field("content").unwrap();
        let query_parser = QueryParser::for_index(&self.index, vec![content_field]);

        // 3. Interpreta a string de busca do usuário
        // Se a string for inválida (ex: erro de sintaxe do usuário), podemos retornar erro ou tratar
        let query = match query_parser.parse_query(query_string) {
            Ok(q) => q,
            Err(_) => return Ok(vec![]), // Retorna vazio se a query for malformada
        };

        // 4. Executa a busca pegando os Top N resultados baseados na relevância (score)
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit).order_by_score())?;

        let mut results = Vec::new();
        let path_field = self.schema.get_field("path").unwrap();

        // 5. Itera sobre os resultados para extrair os dados
        for (score, doc_address) in top_docs {
            // Recupera o documento real do disco/memória
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;

            // Extrai o valor de texto do campo "path"
            if let Some(path_value) = retrieved_doc.get_first(path_field) {
                if let Some(path_str) = path_value.as_str() {
                    results.push(SearchResult {
                        path: path_str.to_string(),
                        score,
                    });
                }
            }
        }

        Ok(results)
    }
}

pub static TANTIVY_INDEX: OnceLock<Index> = OnceLock::new();
