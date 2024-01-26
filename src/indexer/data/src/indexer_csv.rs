use harana_common::hashbrown::HashSet;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

use harana_common::anyhow::{Context, Result};
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct CsvMetadata {
    headers: Vec<String>
}

pub struct IndexerCsv;

impl Indexer for IndexerCsv {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let bytes = fs::read(path.clone())?;
        let mut reader = csv::Reader::from_reader(Cursor::new(bytes));

        let mut primary_tokens = HashSet::new();
        let mut headers = Vec::new();

        reader.headers().into_iter().for_each(|h| {
            h.iter().for_each(|s| {
                primary_tokens.insert(s.to_string());
                headers.push(s.to_string());
            })
        });

        let metadata = CsvMetadata {
            headers
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
    use std::path::Path;

    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_csv::IndexerCsv;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerCsv.index("../../../test_files/Sample1.csv").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}