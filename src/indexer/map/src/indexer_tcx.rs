use std::path::PathBuf;
use tcx::{Activity, read_file};

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::geocoder::geocode_tokens;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct TcxMetadata {
    activities: Vec<Activity>
}

pub struct IndexerTcx;

impl Indexer for IndexerTcx {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let result = read_file(path.to_str().unwrap())?;
        let activities = result.activities.map(|a| a.activities).unwrap_or(vec!());

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        activities.clone().into_iter().for_each(|activity| {
            primary_tokens.insert(activity.sport);
            primary_tokens.extend(entity_tokens(activity.notes.clone().unwrap_or_default().as_str()));
            secondary_tokens.insert(activity.notes.unwrap_or_default());

            for lap in activity.laps {
                primary_tokens.extend(entity_tokens(lap.notes.clone().unwrap_or_default().as_str()));
                secondary_tokens.insert(lap.notes.unwrap_or_default());

                for track in lap.tracks {
                    for trackpoint in track.trackpoints {
                        if trackpoint.position.is_some() {
                            let pos = trackpoint.position.unwrap();
                            primary_tokens.extend(geocode_tokens(pos.latitude, pos.longitude));
                        }
                    }
                }
            }
        });

        let metadata = TcxMetadata {
            activities: activities.clone()
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

    use crate::indexer_tcx::IndexerTcx;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerTcx.index("../../../test_files/Sample1.tcx").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}