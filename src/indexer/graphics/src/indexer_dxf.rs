use std::path::PathBuf;
use harana_common::hashbrown::HashSet;
use harana_common::anyhow::Result;
use harana_indexer_core::indexer::Indexer;
use dxf::Drawing;
use dxf::entities::EntityType;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::geocoder::{geocode, geocode_tokens, Location};
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct DfxMetadata {
    project_name: String,
    location: Option<Location>,
    last_saved_by: String
}

pub struct IndexerDxf;

impl Indexer for IndexerDxf {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let drawing = Drawing::load_file(path.clone())?;

        let mut primary_tokens: HashSet<String> = HashSet::default();
        primary_tokens.extend(tokenize(drawing.header.project_name.as_str()));
        primary_tokens.extend(tokenize(drawing.header.last_saved_by.as_str()));

        let mut location = None;
        if drawing.header.latitude != 37.7950 {
            primary_tokens.extend(geocode_tokens(drawing.header.latitude, drawing.header.longitude));
            let geocoded_location = geocode(drawing.header.latitude, drawing.header.longitude)?;
            location = Some(geocoded_location);
        }

        for e in drawing.entities() {
            match e.specific {
                EntityType::Text(ref text) => {
                    primary_tokens.extend(tokenize(text.value.as_str()));
                }

                EntityType::MText(ref text) => {
                    primary_tokens.extend(tokenize(text.text.as_str()));
                }

                EntityType::RText(ref text) => {
                    primary_tokens.extend(entity_tokens(text.contents.as_str()));
                }

                _ => (),
            }
        }

        let metadata = DfxMetadata {
            project_name: drawing.header.project_name.clone(),
            location,
            last_saved_by: drawing.header.last_saved_by,
        };

        Ok(IndexResult {
            path,
            title: Some(drawing.header.project_name.clone()),
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
    use std::path::Path;

    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_dxf::IndexerDxf;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerDxf.index("../../../test_files/Sample1.dxf").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());

    }
}