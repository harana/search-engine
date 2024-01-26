use harana_common::hashbrown::HashSet;
use std::fs;
use std::path::PathBuf;

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_common::tracing::{Level, span};
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::summarise::summarise;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct TxtMetadata {
    character_count: usize,
    word_count: usize,
}

pub struct IndexerTxt;

impl Indexer for IndexerTxt {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let content = fs::read_to_string(path.clone())?;
        let tokenized = tokenize(content.as_str());

        let metadata = TxtMetadata {
            character_count: content.len(),
            word_count: tokenized.len(),
        };

        Ok(IndexResult {
            path,
            title: Some(summarise(content.as_str())),
            description: None,
            author: None,
            primary_tokens: entity_tokens(content.as_str()),
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

    use crate::indexer_txt::IndexerTxt;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerTxt.index("../../../test_files/Sample1.txt").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}