use harana_common::hashbrown::HashSet;
use std::fs;
use std::path::PathBuf;

use midi_reader_writer::midly_0_5::exports::{MetaMessage, TrackEventKind};
use midi_reader_writer::midly_0_5::exports::Smf;

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct MidiMetadata {
    tracks: HashSet<String>,
    instruments: HashSet<String>
}

pub struct IndexerMidi;

impl Indexer for IndexerMidi {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let bytes = fs::read(path.clone())?;
        let midi = Smf::parse(&bytes)?;

        let mut primary_tokens = HashSet::<String>::new();
        let mut tracks = HashSet::<String>::new();
        let mut instruments = HashSet::<String>::new();

        midi.tracks.into_iter().for_each(|track| {
            track.into_iter().for_each(|t| {
                match t.kind {
                    TrackEventKind::Meta(m) => {
                        match m {
                            MetaMessage::Text(t) => {
                                match std::str::from_utf8(t) {
                                    Ok(text) => primary_tokens.extend(tokenize(text)),
                                    Err(_) => {}
                                }
                            }
                            MetaMessage::Copyright(t) => {
                                match std::str::from_utf8(t) {
                                    Ok(text) => primary_tokens.extend(tokenize(text)),
                                    Err(_) => {}
                                }
                            }
                            MetaMessage::TrackName(t) => {
                                match std::str::from_utf8(t) {
                                    Ok(text) => {
                                        primary_tokens.extend(tokenize(text));
                                        tracks.insert(text.to_string());
                                    },
                                    Err(_) => {}
                                }
                            }
                            MetaMessage::InstrumentName(i) => {
                                match std::str::from_utf8(i) {
                                    Ok(text) => {
                                        primary_tokens.extend(tokenize(text));
                                        instruments.insert(text.to_string());
                                    },
                                    Err(_) => {}
                                }
                            }
                            MetaMessage::Lyric(l) => {
                                match std::str::from_utf8(l) {
                                    Ok(text) => primary_tokens.extend(tokenize(text)),
                                    Err(_) => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            })
        });

        let metadata = MidiMetadata {
            tracks,
            instruments
        };

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens,
            secondary_tokens: HashSet::new(),
            metadata: serde_json::to_value(&metadata)?
        })
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_midi::IndexerMidi;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerMidi.index("../../../test_files/Sample1.midi").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}