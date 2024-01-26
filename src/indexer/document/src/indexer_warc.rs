use std::path::PathBuf;
use webarchive::WebArchive;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::url::extract_domain;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct WebArchiveMetadata {
    domain: String,
    url: String
}
pub struct IndexerWarc;

impl Indexer for IndexerWarc {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let archive: WebArchive = webarchive::from_file(path.clone())?;
        let url = archive.main_resource.url;
        let domain = extract_domain(url.as_str());

        let metadata = WebArchiveMetadata {
            domain: domain.clone(),
            url
        };

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens: HashSet::from([domain.clone()]),
            secondary_tokens: HashSet::new(),
            metadata: serde_json::to_value(&metadata)?
        })
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_warc::IndexerWarc;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerWarc.index("../../../test_files/Sample1.warc").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Secondary Tokens: {}", itertools::join(&indexed_file.secondary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}