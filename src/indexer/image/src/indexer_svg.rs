use std::fs;
use std::path::PathBuf;

use svg::parser::Event;
use svg_metadata::Metadata;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct SvgMetadata {
    width: Option<usize>,
    height: Option<usize>,
    viewbox_width: Option<usize>,
    viewbox_height: Option<usize>,
    viewbox_min_x: Option<usize>,
    viewbox_min_y: Option<usize>
}
pub struct IndexerSvg;

impl Indexer for IndexerSvg {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let bytes = fs::read_to_string(path.clone())?;
        let svg_metadata = Metadata::parse(bytes.as_str())?;

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        let mut content = String::new();
        svg::open(path.clone(), &mut content)?.for_each(|event| {
            match event {
                Event::Tag(t,  ..) => {
                    primary_tokens.extend(entity_tokens(t));
                    secondary_tokens.extend(tokenize(t));
                }

                Event::Text(t) => {
                    primary_tokens.extend(entity_tokens(t));
                    secondary_tokens.extend(tokenize(t));
                }
                Event::Comment(c) => {
                    primary_tokens.extend(entity_tokens(c));
                    secondary_tokens.extend(tokenize(c));
                }
                Event::Declaration(d) => {
                    primary_tokens.extend(entity_tokens(d));
                    secondary_tokens.extend(tokenize(d));
                }
                Event::Instruction(i) => {
                    primary_tokens.extend(entity_tokens(i));
                    secondary_tokens.extend(tokenize(i));
                }
                _ => {}
            }
        });

        let width = svg_metadata.width().map(|w| w as usize);
        let height = svg_metadata.height().map(|h| h as usize);

        let metadata = SvgMetadata {
            width,
            height,
            viewbox_width: svg_metadata.view_box().map(|v| v.width as usize),
            viewbox_height: svg_metadata.view_box().map(|v| v.height as usize),
            viewbox_min_x: svg_metadata.view_box().map(|v| v.min_x as usize),
            viewbox_min_y: svg_metadata.view_box().map(|v| v.min_y as usize),
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
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_svg::IndexerSvg;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerSvg.index("../../../test_files/Sample1.svg").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}