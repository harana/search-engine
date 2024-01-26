use std::path::PathBuf;
use harana_common::hashbrown::HashSet;

use epub::doc::{EpubDoc, NavPoint};

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct EpubMetadata {
    titles: Vec<String>,
    subjects: Vec<String>,
    creators: Vec<String>,
    publishers: Vec<String>,
    descriptions: Vec<String>,
    chapters: Vec<String>,
    page_count: usize
}

pub struct IndexerEpub;

impl Indexer for IndexerEpub {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let epub = EpubDoc::new(path.clone())?;

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        let mut metadata = epub.metadata.clone();
        let titles = metadata.remove("title").unwrap_or(vec!());
        let subjects = metadata.remove("subject").unwrap_or(vec!());
        let creators = metadata.remove("creator").unwrap_or(vec!());
        let publishers = metadata.remove("publisher").unwrap_or(vec!());
        let descriptions = metadata.remove("description").unwrap_or(vec!());
        let chapters: Vec<String> = epub.toc.clone().into_iter().map(|t| t.label).collect();
        let page_count = epub.get_num_pages();

        titles.clone().into_iter().for_each(|sentence| {
            primary_tokens.extend(tokenize(sentence.as_str()));
        });

        subjects.clone().into_iter().for_each(|sentence| {
            primary_tokens.extend(tokenize(sentence.as_str()));
        });

        creators.clone().into_iter().for_each(|sentence| {
            primary_tokens.extend(tokenize(sentence.as_str()));
        });

        publishers.clone().into_iter().for_each(|sentence| {
            primary_tokens.extend(tokenize(sentence.as_str()));
        });

        descriptions.clone().into_iter().for_each(|sentence| {
            primary_tokens.extend(tokenize(sentence.as_str()));
        });

        primary_tokens.extend(extract_navpoints(epub.toc));

        let metadata = EpubMetadata {
            titles,
            subjects,
            creators,
            publishers,
            descriptions,
            chapters,
            page_count
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
fn extract_navpoints(navpoints: Vec<NavPoint>) -> HashSet::<String> {
    let mut tokens = HashSet::<String>::new();

    navpoints.into_iter().for_each(|n| {
       tokens.extend(extract_navpoints(n.children));
       tokens.extend(tokenize(n.label.as_str()));
    });

    tokens
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_common::log::error;
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_epub::IndexerEpub;

    #[tokio::test]
    async fn test_indexing() {
        match IndexerEpub.index("../../../test_files/Sample1.epub") {
            Ok(indexed_file) => {
                let items = itertools::join(&indexed_file.primary_tokens, ", ");
                assert!(indexed_file.primary_tokens.contains("naden"));
            }
            Err(e) => error!("{}", e.to_string())
        }
    }
}