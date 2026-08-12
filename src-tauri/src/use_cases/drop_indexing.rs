use std::fs;

use tantivy::Index;

use crate::{
    db::tantivy::TANTIVY_INDEX, paths::DATA_DIR, repositories::commands::drop_indexing,
    tantivy_indexer::tantivy_indexer::IndexerService,
};

/// Drop database + tantivy files [Called by fron-end from command]
pub fn drop_all() -> Option<()> {
    let Some(index) = TANTIVY_INDEX.get() else {
        println!("1");
        return None;
    };
    let Ok(indexer) = IndexerService::new(index.clone()) else {
        println!("2");
        return None;
    };

    let Some(data_dir) = DATA_DIR.get() else {
        println!("2");
        return None;
    };

    if let Ok(_result) = fs::remove_dir(data_dir.join("/tantivy_data")) {
        drop_indexing();
        indexer.drop();

        println!("Every index dropped");

        return Some(());
    }
    Some(())
}
