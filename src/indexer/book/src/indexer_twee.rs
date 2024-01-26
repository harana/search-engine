use std::fs;
use std::path::PathBuf;

use twee_v3::ContentNode;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct TweeMetadata {
    title: String
}

pub struct IndexerTwee;

impl Indexer for IndexerTwee {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        let path_str = fs::read_to_string(path.clone())?;
        let story = twee_v3::Story::try_from(path_str)?;

        let title = story.title().unwrap_or_default().to_string();
        primary_tokens.insert(title.clone());

        if story.start().is_some() {
            let start = story.start().unwrap();
            start.nodes().iter().for_each(|node| {
                match node {
                    ContentNode::Text(text) => {
                        primary_tokens.extend(entity_tokens(text));
                        secondary_tokens.insert(text.to_string())
                    },
                    ContentNode::Link { text, target: _ } => {
                        primary_tokens.extend(entity_tokens(text));
                        secondary_tokens.insert(text.to_string())
                    }
                };
            });
        }

        let metadata = TweeMetadata {
            title
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
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_twee::IndexerTwee;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerTwee.index("../../../test_files/Sample1.twee").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}