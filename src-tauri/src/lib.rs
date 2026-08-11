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
mod tokenizers;
mod types;
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
                fs::create_dir_all(&tantivy_dir);
                Index::create_in_dir(&tantivy_dir, tantivy_schema_builder());
            };

            let tantivy_index = Index::open_in_dir(tantivy_dir)
                .expect("Error while opening the tantivy data folder");

            TANTIVY_INDEX
                .set(tantivy_index)
                .expect("Error while setting tantivy during setup");

            match init_database() {
                Ok(database) => println!("Database initilized fine"),
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

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Schema;
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
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        let path_field = self.schema.get_field("path").unwrap();

        // 5. Itera sobre os resultados para extrair os dados
        for (score, doc_address) in top_docs {
            // Recupera o documento real do disco/memória
            let retrieved_doc = searcher.doc(doc_address)?;

            // Extrai o valor de texto do campo "path"
            if let Some(path_value) = retrieved_doc.get_first(path_field) {
                if let Some(path_str) = path_value.as_text() {
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
