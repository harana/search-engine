use std::fs;
use std::path::PathBuf;

use qif_parser::parse;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct QifMetadata {
    investment_count: usize,
    transaction_count: usize,
    investment_start_date: Option<String>,
    investment_end_date: Option<String>,
    transaction_start_date: Option<String>,
    transaction_end_date: Option<String>
}

pub struct IndexerQif;

impl Indexer for IndexerQif {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let content = fs::read_to_string(path.clone())?;
        let result = parse(&content, "%m/%d/%y")?;

        let mut primary_tokens = HashSet::<String>::new();
        result.investments.iter().for_each(|i| {
            primary_tokens.insert(i.memo.to_string());
            primary_tokens.insert(i.security_name.to_string());
        });

        result.transactions.iter().for_each(|t| {
            primary_tokens.insert(t.memo.to_string());
            primary_tokens.insert(t.payee.to_string());
        });

        let metadata = QifMetadata {
            investment_count: result.investments.len(),
            transaction_count: result.transactions.len(),
            investment_start_date: result.investments.first().map(|i| i.date.to_string()),
            investment_end_date: result.investments.last().map(|i| i.date.to_string()),
            transaction_start_date: result.transactions.first().map(|i| i.date.to_string()),
            transaction_end_date: result.transactions.last().map(|i| i.date.to_string()),
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

    use crate::indexer_qif::IndexerQif;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerQif.index("../../../test_files/Sample1.qif").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}