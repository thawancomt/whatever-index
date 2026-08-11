use std::{collections::HashMap, fs, path::PathBuf};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::adapters::traits::Adapter;

pub struct TextableAdapter;

impl Adapter for TextableAdapter {
    fn ingest(&self, paths: Vec<PathBuf>) -> HashMap<PathBuf, String> {
        let content_by_path: HashMap<PathBuf, String> = paths
            .into_par_iter()
            .filter_map(|file| {
                let content = fs::read_to_string(&file);
                content.ok().map(|c| (file, c))
            })
            .collect();

        return content_by_path;
    }
}
