use std::path::PathBuf;
use calamine::{open_workbook, DataType, Reader, Xlsx};

use harana_common::{anyhow, serde_json};
use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::itertools::Itertools;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct XlsxMetadata {
    sheets: Vec<String>
}

pub struct IndexerXlsx;

impl Indexer for IndexerXlsx {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let mut workbook: Xlsx<_> =
            match open_workbook(path.clone()) {
                Ok(workbook) => Ok(workbook),
                Err(e) => Err(anyhow::anyhow!(format!(
                    "indexer_xlsx: Failed to open workbook at path: {} with error {:?}", path.to_str().unwrap(), e
                )))
            }?;

        let mut primary_tokens = HashSet::<String>::new();
        workbook
            .sheet_names()
            .to_vec()
            .iter()
            .map(|sheet_name| workbook.worksheet_range(sheet_name))
            .filter_map(Result::ok)
            .map(|range| {
                range
                    .used_cells()
                    .filter(|(_, _, cell)| cell.is_string())
                    .filter_map(|(_, _, cell)| cell.get_string())
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
            })
            .flatten()
            .fold(String::new(), |mut acc, x| {
                primary_tokens.extend(tokenize(x.as_str()));
                acc
            });

        let metadata = XlsxMetadata {
            sheets: workbook.sheet_names()
        };

        Ok(IndexResult {
            path: path.clone(),
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

    use crate::indexer_xlsx::IndexerXlsx;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerXlsx.index("../../../test_files/Sample1.xlsx").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Secondary Tokens: {}", itertools::join(&indexed_file.secondary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}