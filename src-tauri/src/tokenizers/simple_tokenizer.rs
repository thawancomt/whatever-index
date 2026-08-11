use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::PathBuf,
};

pub fn simple_tokenizer(data: HashMap<PathBuf, String>) -> BTreeMap<String, HashSet<PathBuf>> {
    let mut paths_by_word: BTreeMap<String, HashSet<PathBuf>> = BTreeMap::new();

    for (path, text) in data {
        let words: Vec<String> = text
            .trim()
            .split_whitespace()
            // Remove non alphanumerics
            .map(|word| {
                word.trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase()
            })
            .collect();

        for word in words {
            paths_by_word
                .entry(word.into())
                .or_insert_with(HashSet::new)
                .insert(path.clone());
        }
    }
    paths_by_word
}
