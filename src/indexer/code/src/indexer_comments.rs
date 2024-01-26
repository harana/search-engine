use std::fs;
use std::path::PathBuf;

use comment_parser::CommentParser;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

pub struct IndexerComments;

impl Indexer for IndexerComments {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let code = fs::read_to_string(path.clone())?;
        let extension = path.extension().and_then(|e| e.to_str());

        let syntax = match extension.unwrap_or_default() {
            "c"     =>   "c",
            "cpp"   =>   "cpp",
            "glsl"  =>   "glsl",
            "java"  =>   "java",
            "js"    =>   "javascript",
            "json"  =>   "json",
            "jsonc" =>   "jsonc",
            "py"    =>   "python",
            "rs"    =>   "rust",
            "scss"  =>   "scss",
            "sh"    =>   "shell",
            "toml"  =>   "toml",
            "ts"    =>   "typescript",
            "yaml"  =>   "yaml",
            "yml"   =>   "yaml",
            _       =>   ""
        };

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        match comment_parser::get_syntax(syntax) {
            Some(rules) => {
                let parser = CommentParser::new(code.as_str(), rules);

                parser.into_iter().for_each(|e| {
                    primary_tokens.extend(entity_tokens(e.text()));
                    secondary_tokens.extend(tokenize(e.text()));
                });
            }
            _ => {}
        }

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
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_comments::IndexerComments;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerComments.index("../../../test_files/Sample1.java").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Secondary Tokens: {}", itertools::join(&indexed_file.secondary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}