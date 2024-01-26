use std::path::PathBuf;
use harana_common::hashbrown::HashSet;

use lrcat::Catalog;

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct LrcatMetadata {
    collections: Vec<String>,
    folders: Vec<String>,
    image_count: usize
}

pub struct IndexerLrcat;

impl Indexer for IndexerLrcat {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {

        let mut catalog = Catalog::new(path.clone());
        if catalog.open() && catalog.catalog_version.is_supported() {
            let mut primary_tokens = HashSet::<String>::new();
            let mut secondary_tokens = HashSet::<String>::new();

            let collections: Vec<String> = catalog.load_collections().into_iter().map(|c| c.name.to_string()).collect();
            primary_tokens.extend(collections.clone());

            let roots = catalog.load_folders().clone().roots;
            let folders: Vec<String> = roots.into_iter().map(|i| i.name).collect();
            primary_tokens.extend(folders.clone());

            let images: Vec<String> = catalog.load_images().iter()
                .filter(|i| i.copy_name.is_some())
                .into_iter()
                .map(|i| i.copy_name.clone().unwrap())
                .collect();
            secondary_tokens.extend(images);

            let metadata = LrcatMetadata {
                collections: collections.clone(),
                folders: folders.clone(),
                image_count: catalog.images().len()
            };

            Ok(IndexResult {
                path,
                title: None,
                description: None,
                author: None,
                primary_tokens,
                secondary_tokens,
                metadata: serde_json::to_value(&metadata)?
            })
        } else {
            Ok(IndexResult {
                path,
                title: None,
                description: None,
                author: None,
                primary_tokens: HashSet::new(),
                secondary_tokens: HashSet::new(),
                metadata: Default::default()
            })
        }
    }
}