use std::{collections::HashSet, path::PathBuf};

use crate::repositories::{binary_persister::BinaryPersister, traits::Persister};

pub fn search_text(query: String) -> HashSet<PathBuf> {
    let database = BinaryPersister::load();

    let trimmed = query.trim().to_lowercase();
    let mut queries = trimmed.split_whitespace().map(|word| {
        word.trim_matches(|c: char| !c.is_alphanumeric())
            .to_string()
    });

    let mut tree: HashSet<PathBuf> = HashSet::new();

    if let Some(first_term) = queries.next() {
        for (word, files) in database.range(first_term.to_string()..) {
            if !word.starts_with(&first_term) {
                continue;
            }
            tree.extend(files.clone());
        }
    }

    for term in queries {
        let mut current_files: HashSet<PathBuf> = HashSet::new();
        for (word, files) in database.range(term.to_string()..) {
            if !word.starts_with(&term) {
                continue;
            }

            current_files.extend(files.clone());
        }

        tree.retain(|file| current_files.contains(file));
    }

    tree
}
