use harana_common::hashbrown::HashSet;
use std::fs::File;
use std::path::PathBuf;

use dmgwiz::{DmgWiz, Verbosity};

use harana_common::anyhow::{Error, Result};
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct DmgMetadata {
    capacity: usize,
    partitions: Vec<String>
}

pub struct IndexerDmg;

impl Indexer for IndexerDmg {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let input = File::open(path.clone())?;
        let wiz = DmgWiz::from_reader(input, Verbosity::None).map_err(|e| {
            Error::msg(e.to_string())
        })?;

        let metadata = DmgMetadata {
            capacity: wiz.partitions.capacity(),
            partitions: wiz.partitions.into_iter().map(|p| p.name).collect()
        };

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens: HashSet::new(),
            secondary_tokens: HashSet::new(),
            metadata: serde_json::to_value(&metadata)?
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_dmg::IndexerDmg;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerDmg.index("../../../test_files/Sample1.dmg").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}