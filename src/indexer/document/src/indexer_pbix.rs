use std::path::PathBuf;
use harana_common::anyhow::{Error, Result};
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct PbixMetadata {
    page_count: usize,
    page_names: Vec<String>
}

pub struct IndexerPbix;

impl Indexer for IndexerPbix {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let pbix = pbix::parse_file(path.clone()).map_err(|e| Error::msg(e.to_string()))?;


        let mut primary_tokens = HashSet::<String>::new();

        let page_names: Vec<String> = pbix.pages.iter().map(|p| p.name.to_string()).collect();
        primary_tokens.extend(page_names.clone());

        let page_count = pbix.pages.len();

        let metadata = PbixMetadata {
            page_count,
            page_names,
        };

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens,
            secondary_tokens: HashSet::new(),
            metadata: serde_json::to_value(&metadata)?
        })
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_pbix::IndexerPbix;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerPbix.index("../../../test_files/Sample1.pbix").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Secondary Tokens: {}", itertools::join(&indexed_file.secondary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}