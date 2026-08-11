use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
};

pub type BinaryDatabase = BTreeMap<String, HashSet<PathBuf>>;
