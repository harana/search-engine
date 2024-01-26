use std::path::PathBuf;
use harana_common::hashbrown::HashSet;

use mobi::Mobi;

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct MobiMetadata {
    title: String,
    author: String,
    publisher: String,
    description: String,
    isbn: String,
    publication_date: String,
    contributor: String
}

pub struct IndexerMobi;

impl Indexer for IndexerMobi {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let mobi = Mobi::from_path(path.clone())?;

        let title = mobi.title();
        let author = mobi.author().unwrap_or_default();
        let publisher = mobi.publisher().unwrap_or_default();
        let description = mobi.description().unwrap_or_default();
        let contributor = mobi.contributor().unwrap_or_default();
        let publication_date = mobi.publish_date().unwrap_or_default();
        let isbn = mobi.isbn().unwrap_or_default();
        let content = mobi.content_as_string().unwrap_or_default();

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        primary_tokens.extend(tokenize(title.as_str()));
        primary_tokens.extend(tokenize(author.as_str()));
        primary_tokens.extend(entity_tokens(content.as_str()));

        secondary_tokens.extend(tokenize(publisher.as_str()));
        secondary_tokens.extend(tokenize(description.as_str()));
        secondary_tokens.extend(tokenize(contributor.as_str()));
        secondary_tokens.extend(tokenize(content.as_str()));

        let metadata = MobiMetadata {
            title,
            author,
            publisher,
            description,
            isbn,
            publication_date,
            contributor
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
    use std::path::Path;

    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_mobi::IndexerMobi;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerMobi.index("../../../test_files/Sample1.mobi").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}