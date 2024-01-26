use harana_common::hashbrown::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use gpx::{Gpx, read, Time, Track};

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::geocoder::geocode_tokens;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct GpxMetadata {
    name: Option<String>,
    author: Option<String>,
    description: Option<String>,
    keywords: Option<String>,
    time: Option<Time>,
    tracks: Vec<Track>
}

pub struct IndexerGpx;

impl Indexer for IndexerGpx {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let file = File::open(path.clone())?;
        let reader = BufReader::new(file);
        let gpx: Gpx = read(reader)?;

        let name = gpx.metadata.clone().and_then(|m| m.name);
        let author = gpx.metadata.clone().and_then(|m| m.author).and_then(|a| a.name);
        let description = gpx.metadata.clone().and_then(|m| m.description);
        let keywords = gpx.metadata.clone().and_then(|m| m.keywords);
        let time = gpx.metadata.clone().and_then(|m| m.time);

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        primary_tokens.extend(tokenize(name.clone().unwrap_or_default().as_str()));
        primary_tokens.extend(tokenize(author.clone().unwrap_or_default().as_str()));
        primary_tokens.extend(tokenize(keywords.clone().unwrap_or_default().as_str()));

        primary_tokens.extend(entity_tokens(description.clone().unwrap_or_default().as_str()));
        secondary_tokens.extend(tokenize(description.clone().unwrap_or_default().as_str()));

        gpx.tracks.iter().for_each(|track| {
            track.segments.iter().for_each(|segment| {
                segment.points.iter().for_each(|point| {
                    primary_tokens.extend(geocode_tokens(point.point().x(), point.point().y()));
                })
            })
        });

        let metadata = GpxMetadata {
            name: name.clone(),
            author,
            description,
            keywords,
            time,
            tracks: gpx.tracks
        };

        Ok(IndexResult {
            path: path.clone(),
            title: name.clone(),
            description: None,
            author: None,
            primary_tokens: HashSet::new(),
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

    use crate::indexer_fit::IndexerFit;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerFit.index("../../../test_files/Sample1.gpx").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}