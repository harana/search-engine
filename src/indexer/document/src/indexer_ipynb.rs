use harana_common::hashbrown::HashSet;
use std::fs;
use std::path::PathBuf;

use ipynb::{Cell, Notebook};
use markdown::*;

use harana_common::anyhow::Result;
use harana_common::serde_json;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

use crate::indexer_md::extract_keywords;

pub struct IndexerIpynb;

impl Indexer for IndexerIpynb {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let code = fs::read_to_string(path.clone())?;
        let notebook: Notebook = serde_json::from_str(code.as_str())?;

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        notebook.cells.into_iter().for_each(|c| {
            match c {
                Cell::Markdown(md) => {
                    md.source.iter().for_each(|s| {
                        if let Ok(ast) = to_mdast(s, &ParseOptions::default()) {
                            let tokens = extract_keywords(ast, false);
                            primary_tokens.extend(tokens.0);
                            secondary_tokens.extend(tokens.1);
                        }
                    })
                }
                Cell::Code(c) => {
                    primary_tokens.extend(entity_tokens(c.source.join(" ").as_str()));
                }
                Cell::Raw(r) => {
                    primary_tokens.extend(entity_tokens(r.source.join(" ").as_str()));
                    secondary_tokens.extend(r.source);
                }
            }
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
    use std::path::Path;

    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_ipynb::IndexerIpynb;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerIpynb.index("../../../test_files/Sample1.ipynb").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Secondary Tokens: {}", itertools::join(&indexed_file.secondary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}