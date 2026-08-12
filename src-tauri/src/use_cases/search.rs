use std::{collections::HashSet, path::PathBuf};

use crate::tantivy_indexer::tantivy_searcher::SearchService;

pub fn search_text(query: String) -> HashSet<PathBuf> {
    let service = SearchService::new();

    if let Ok(searcher) = service {
        let result = searcher.search(&query, 300).expect("Something goes wrong");

        let some: HashSet<PathBuf> = result
            .iter()
            .map(|f| PathBuf::from(f.path.clone()))
            .collect();

        return some;
    }

    HashSet::new()
}
