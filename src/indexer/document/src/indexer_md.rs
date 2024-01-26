use std::fs;
use std::path::PathBuf;

use markdown::*;
use markdown::mdast::Node;
use nanohtml2text::html2text;

use harana_common::anyhow::{Error, Result};
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct MdMetadata {
    title: Option<String>
}

pub struct IndexerMd;

impl Indexer for IndexerMd {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let md = fs::read_to_string(path.clone())?;
        let ast = to_mdast(&md, &ParseOptions::default()).map_err(|e| Error::msg(e))?;
        let tokens = extract_keywords(ast.clone(), false);

        let mut title = None;
        if ast.children().is_some() {
            ast.children().unwrap().iter().for_each(|node| {
                match node {
                    Node::Heading(heading) => {
                        if heading.depth == 1 {
                            heading.children.iter().for_each(|child| {
                                match child {
                                    Node::Text(t) => title = Some(t.to_owned().value),
                                    _ => {}
                                }
                            });
                        }
                    }
                    _ => {}
                }
            });
        }

        let metadata = MdMetadata {
            title
        };
        
        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens: tokens.0,
            secondary_tokens: tokens.1,
            metadata: serde_json::to_value(&metadata)?
        })
    }
}


pub fn extract_keywords(node: Node, within_heading: bool) -> (HashSet<String>, HashSet<String>) {
    let mut primary_tokens = HashSet::<String>::new();
    let mut secondary_tokens = HashSet::<String>::new();

    match node {
        Node::Root(root) => {
            root.children.into_iter().for_each(|c| {
                let tokens = extract_keywords(c, false);
                primary_tokens.extend(tokens.0);
                secondary_tokens.extend(tokens.1);
            });
        }

        Node::Code(code) => {
            primary_tokens.extend(entity_tokens(code.value.as_str()));
        }

        Node::Paragraph(paragraph) => {
            paragraph.children.into_iter().for_each(|c| {
                let tokens = extract_keywords(c, false);
                primary_tokens.extend(tokens.0);
                secondary_tokens.extend(tokens.1);
            });
        }

        Node::Html(html) => {
            let html = html2text(html.value.as_str());
            primary_tokens.extend(entity_tokens(html.as_str()));
            secondary_tokens.extend(tokenize(html.as_str()));
        }

        Node::Text(text) => {
            if within_heading {
                primary_tokens.extend(tokenize(text.value.as_str()));
            }else{
                primary_tokens.extend(entity_tokens(text.value.as_str()));
            }
        }

        Node::Heading(heading) => {
            heading.children.into_iter().for_each(|c| {
                let tokens = extract_keywords(c, true);
                primary_tokens.extend(tokens.0);
                secondary_tokens.extend(tokens.1);
            });
        }

        Node::Image(image) => {
            secondary_tokens.extend(tokenize(image.title.unwrap_or_default().as_str()));
        }
        Node::ImageReference(reference) => {
            secondary_tokens.extend(tokenize(reference.label.unwrap_or_default().as_str()));
        }
        Node::Link(link) => {
            secondary_tokens.extend(tokenize(link.title.unwrap_or_default().as_str()));
        }
        Node::LinkReference(link_reference) => {
            secondary_tokens.extend(tokenize(link_reference.label.unwrap_or_default().as_str()));
        }
        Node::Definition(definition) => {
            secondary_tokens.extend(tokenize(definition.label.unwrap_or_default().as_str()));
            secondary_tokens.extend(tokenize(definition.title.unwrap_or_default().as_str()));
        }
        _ => {}
    }

    (primary_tokens, secondary_tokens)
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_md::IndexerMd;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerMd.index("../../../test_files/Sample1.md").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Secondary Tokens: {}", itertools::join(&indexed_file.secondary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}