use std::path::PathBuf;
use harana_common::hashbrown::HashSet;

use harana_common::anyhow::Result;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

pub struct IndexerXcodeProj;

impl Indexer for IndexerXcodeProj {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
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