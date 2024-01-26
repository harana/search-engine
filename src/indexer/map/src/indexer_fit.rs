use std::fs::File;
use std::path::PathBuf;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct FitMetadata {
    fields: HashSet<String>,
    record_count: usize
}

pub struct IndexerFit;

impl Indexer for IndexerFit {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let mut file = File::open(path.clone())?;

        let mut primary_tokens = HashSet::<String>::new();
        let mut fields = HashSet::<String>::new();

        let fit = fitparser::from_reader(&mut file)?;

        fit.iter().for_each(|record| {
            record.fields().into_iter().for_each(|field| {
                fields.insert(field.name().to_string());
            })
        });

        primary_tokens.extend(fields.clone());

        let metadata = FitMetadata {
            fields,
            record_count: fit.len()
        };

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens,
            secondary_tokens: HashSet::new(),
            metadata: serde_json::to_value(&metadata)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_fit::IndexerFit;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerFit.index("../../../test_files/Sample1.fit").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}