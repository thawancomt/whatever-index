use std::{
    collections::{BTreeMap, HashSet},
    env, fs,
};

use crate::{repositories::traits::Persister, types::database::BinaryDatabase};
pub struct BinaryPersister;

impl Persister for BinaryPersister {
    fn load() -> BinaryDatabase {
        let path = env::temp_dir().join("whatever_index_database.bin");
        if let Ok(file) = fs::read(path) {
            let data: BinaryDatabase = match bincode::deserialize(&file) {
                Ok(data) => data,
                Err(_) => BTreeMap::new(),
            };

            return data;
        }

        BTreeMap::new()
    }

    fn save(data: &BTreeMap<String, HashSet<std::path::PathBuf>>) {
        if let Ok(bytes) = bincode::serialize(&data) {
            let path = env::temp_dir();
            match fs::write(path.join("whatever_index_database.bin"), bytes) {
                Ok(_) => print!("Data saved"),
                Err(e) => {
                    eprintln!("Error:  {}", e)
                }
            }
        }
    }
}
