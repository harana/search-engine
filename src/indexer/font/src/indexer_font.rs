use std::path::PathBuf;
use font::{File, Names};
use font::opentype::truetype::tables::names::NameID;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct FontMetadata {
    fonts: Vec<Font>,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct Font {
    family: Option<String>,
    sub_family: Option<String>,
    manufacturer: Option<String>,
    designer: Option<String>
}

pub struct IndexerFont;

impl Indexer for IndexerFont {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let font = File::open(path.clone())?;

        let mut primary_tokens = HashSet::new();

        let font_names: Vec<Names> = font.fonts.into_iter().map(|mut f| f.names().unwrap()).collect();
        let mut fonts = Vec::new();

        font_names.iter().for_each(|names| {

            let mut family = None;
            let mut sub_family = None;
            let mut manufacturer = None;
            let mut designer = None;

            names.iter().for_each(|n| {

                match n.0.0 {
                    NameID::FontFamilyName => {
                        if n.0.1.is_some() {
                            family = Some(n.1.unwrap());
                        }
                    }
                    NameID::FontSubfamilyName => {
                        if n.0.1.is_some() {
                            sub_family = Some(n.1.unwrap());
                        }
                    }
                    NameID::ManufacturerName => {
                        if n.0.1.is_some() {
                            manufacturer = Some(n.1.unwrap());
                        }
                    }
                    NameID::DesignerName => {
                        if n.0.1.is_some() {
                            designer = Some(n.1.unwrap());
                        }
                    }
                    _ => {}
                }
            });

            primary_tokens.extend(tokenize(family.clone().unwrap_or_default().as_str()));
            primary_tokens.extend(tokenize(sub_family.clone().unwrap_or_default().as_str()));
            primary_tokens.extend(tokenize(manufacturer.clone().unwrap_or_default().as_str()));
            primary_tokens.extend(tokenize(designer.clone().unwrap_or_default().as_str()));

            fonts.push(
                Font {
                    family,
                    sub_family,
                    manufacturer,
                    designer,
                }
            );
        });

        let metadata = FontMetadata {
            fonts
        };

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens,
            secondary_tokens: HashSet::new(),
            metadata: serde_json::to_value(&metadata)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_font::IndexerFont;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerFont.index("../../../test_files/Sample1.ttf").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Secondary Tokens: {}", itertools::join(&indexed_file.secondary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}