use harana_common::hashbrown::HashSet;
use std::fs;
use std::path::PathBuf;
use emlx::parse_emlx;

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct EmlxMetadata {
    domain: String,
    url: String
}

pub struct IndexerEmlx;

impl Indexer for IndexerEmlx {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let contents = fs::read(path.clone())?;
        let mail = parse_emlx(contents.as_slice()).unwrap();

        let metadata = EmlxMetadata {
            domain: "".to_string(),
            url: "".to_string()
        };

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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_emlx::IndexerEmlx;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerEmlx.index("../../../test_files/Sample1.emlx").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}