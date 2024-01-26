use std::path::PathBuf;

use srt_parser::SubRipFile;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

pub struct IndexerSrt;

impl Indexer for IndexerSrt {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let subs = SubRipFile::new(path.clone())?;

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        subs.subtitles().iter().for_each(|s| {
            primary_tokens.extend(entity_tokens(s.text()));
            secondary_tokens.extend(tokenize(s.text()));
        });

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens,
            secondary_tokens,
            metadata: Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_srt::IndexerSrt;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerSrt.index("../../../test_files/Sample1.srt").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}