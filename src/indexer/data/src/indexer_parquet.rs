use std::fs::File;
use std::path::PathBuf;

use parquet::file::reader::{FileReader, SerializedFileReader};

use harana_common::anyhow::Result;
use harana_common::hashbrown::{HashMap, HashSet};
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct ParquetMetadata {
    columns: HashMap<String, String>,
}

pub struct IndexerParquet;

impl Indexer for IndexerParquet {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let file = File::open(path.clone())?;
        let reader = SerializedFileReader::new(file)?;

        let mut columns = HashMap::new();
        let mut primary_tokens = HashSet::<String>::new();

        reader.metadata().file_metadata().schema_descr().columns().into_iter().for_each(|c| {
            columns.insert(c.name().to_string(), c.physical_type().to_string());
            primary_tokens.insert(c.name().to_string());
        });

        let metadata = ParquetMetadata {
            columns,
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

    use crate::indexer_parquet::IndexerParquet;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerParquet.index("../../../test_files/Sample1.parquet").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}