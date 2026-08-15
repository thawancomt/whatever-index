use std::collections::HashMap;

use crate::db::types::file::File;

pub trait Adapter {
    fn ingest(&self, paths: Vec<File>) -> HashMap<File, String>;
}
