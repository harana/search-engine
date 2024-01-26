use std::path::PathBuf;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

pub struct IndexerAirbyte;

impl Indexer for IndexerAirbyte {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        Ok(IndexResult {
            path: "".to_string().into(),
            title: None,
            description: None,
            author: None,
            primary_tokens: HashSet::new(),
            secondary_tokens: HashSet::new(),
            metadata: Default::default()
        })
    }
}