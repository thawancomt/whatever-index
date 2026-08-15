use std::{collections::HashMap, fs};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{adapters::traits::Adapter, db::types::file::File};

pub struct TextableAdapter;

impl Adapter for TextableAdapter {
    fn ingest(&self, paths: Vec<File>) -> HashMap<File, String> {
        let content_by_path: HashMap<File, String> = paths
            .into_par_iter()
            .filter_map(|file| {
                let content = fs::read_to_string(&file.path);
                content.ok().map(|c| (file, c))
            })
            .collect();

        return content_by_path;
    }
}
