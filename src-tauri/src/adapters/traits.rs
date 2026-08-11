use std::{collections::HashMap, path::PathBuf};

pub trait Adapter {
    fn ingest(&self, paths: Vec<PathBuf>) -> HashMap<PathBuf, String>;
}
