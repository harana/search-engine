use std::path::PathBuf;
use kml::{Kml, KmlReader};
use kml::types::{Element, Geometry};

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::geocoder::geocode_tokens;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct KmlMetadata {
    placemarks: Vec<String>
}

pub struct IndexerKml;

impl Indexer for IndexerKml {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let mut kml_reader = KmlReader::<_, f64>::from_path(path.clone())?;
        let kml = kml_reader.read()?;
        let kml_data = extract_kml(kml);

        let metadata = KmlMetadata {
            placemarks: kml_data.2
        };

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens: kml_data.0,
            secondary_tokens: kml_data.1,
            metadata: serde_json::to_value(&metadata)?
        })
    }
}

fn extract_elements(element: Element) -> (HashSet::<String>, HashSet::<String>) {
    let mut primary_tokens = HashSet::<String>::new();
    let mut secondary_tokens = HashSet::<String>::new();

    primary_tokens.extend(tokenize(element.name.as_str()));
    primary_tokens.extend(entity_tokens(element.content.clone().unwrap_or_default().as_str()));
    secondary_tokens.extend(tokenize(element.content.clone().unwrap_or_default().as_str()));

    element.children.into_iter().for_each(|e| {
       let tokens = extract_elements(e);
        primary_tokens.extend(tokens.0);
        secondary_tokens.extend(tokens.1);
    });

    (primary_tokens, secondary_tokens)
}

fn extract_kml(kml: Kml<f64>) -> (HashSet::<String>, HashSet::<String>, Vec<String>) {
    let mut primary_tokens = HashSet::<String>::new();
    let mut secondary_tokens = HashSet::<String>::new();
    let mut placemarks = Vec::<String>::new();

    match kml {
        Kml::KmlDocument(d) => {
            d.elements.into_iter().for_each(|k| {
               let tokens = extract_kml(k);
                primary_tokens.extend(tokens.0);
                secondary_tokens.extend(tokens.1);
            });
        }
        Kml::Location(l) => {
            primary_tokens.extend(geocode_tokens(l.latitude, l.longitude));
        }
        Kml::Placemark(p) => {
            primary_tokens.extend(tokenize(p.name.clone().unwrap_or_default().as_str()));
            primary_tokens.extend(entity_tokens(p.description.clone().unwrap_or_default().as_str()));
            secondary_tokens.extend(tokenize(p.description.clone().unwrap_or_default().as_str()));
            p.children.into_iter().for_each(|e| {
                let tokens = extract_elements(e);
                primary_tokens.extend(tokens.0);
                secondary_tokens.extend(tokens.1);
            });
            match p.geometry {
                Some(Geometry::Point(p)) => {
                    primary_tokens.extend(geocode_tokens(p.coord.x, p.coord.y));
                }
                _ => {}
            }
            if p.name.is_some() {
                placemarks.push(p.name.clone().unwrap());
            }
        }
        Kml::Point(p) => {
            primary_tokens.extend(geocode_tokens(p.coord.x, p.coord.y));
        },
        Kml::Element(e) => {
            let tokens = extract_elements(e);
            primary_tokens.extend(tokens.0);
            secondary_tokens.extend(tokens.1);
        }
        _ => {}
    }

    (primary_tokens, secondary_tokens, placemarks)
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_kml::IndexerKml;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerKml.index("../../../test_files/Sample1.kml").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}