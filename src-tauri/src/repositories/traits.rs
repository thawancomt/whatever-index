use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
};

use crate::types::database::BinaryDatabase;

pub trait Persister {
    fn load() -> BinaryDatabase;
    fn save(data: &BTreeMap<String, HashSet<PathBuf>>);
}
