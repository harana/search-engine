use harana_common::hashbrown::HashSet;
use std::path::{Path, PathBuf};

use archive_reader::Archive;

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct ArchiveMetadata {
    file_count: usize
}

pub struct IndexerArchive;

impl Indexer for IndexerArchive {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let mut archive = Archive::open(path.clone());

        let mut primary_tokens: HashSet<String> = HashSet::new();
        let mut file_count = 0;

        archive
            .block_size(1024 * 1024)
            .list_file_names()?
            .for_each(|path| {
                let path = path.as_ref().unwrap().as_str();
                Path::new(path).components().for_each(|c| {
                    primary_tokens.insert(c.as_os_str().to_string_lossy().to_string());
                });
                file_count += 1;
            });

        let metadata = ArchiveMetadata {
          file_count
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

    use crate::indexer_archive::IndexerArchive;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerArchive.index("../../../test_files/Sample1.zip").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}