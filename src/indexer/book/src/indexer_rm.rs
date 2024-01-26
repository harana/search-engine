use std::fs::read;
use std::path::PathBuf;

use remarkable_lines::RemarkableFile;
use remarkable_lines::v6::block::Block;
use remarkable_lines::v6::crdt::{CrdtSequence, CrdtSequenceItem};
use remarkable_lines::v6::scene_item::text::TextItem;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

pub struct IndexerRm;

impl Indexer for IndexerRm {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let file = read(path.clone())?;
        let rm = RemarkableFile::read(&file[..])?;

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        match rm {
            RemarkableFile::V6 { tree, blocks } => {
                blocks.into_iter().for_each(|block| {
                    match block {
                        Block::TreeNode(tn) => {
                            secondary_tokens.insert(tn.group.label.value);
                        }

                        Block::SceneTextItem(sti) => {
                            match sti.item {
                                CrdtSequenceItem { value, .. } => {
                                    match value {
                                        None => {}
                                        Some(text) => {
                                            match text.items {
                                                CrdtSequence { items } => {
                                                    items.into_iter().for_each(|(k, v)| {
                                                        match v.value {
                                                            TextItem::FormatCode(_) => {}
                                                            TextItem::Text(t) => {
                                                                primary_tokens.extend(entity_tokens(t.as_str()));
                                                                secondary_tokens.insert(t);
                                                            }
                                                        };
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        Block::RootText(rt) => {
                            match rt.text.items {
                                CrdtSequence { items } => {
                                    items.into_iter().for_each(|(k, v)| {
                                        match v.value {
                                            TextItem::FormatCode(_) => {}
                                            TextItem::Text(t) => {
                                                primary_tokens.extend(entity_tokens(t.as_str()));
                                                secondary_tokens.insert(t);
                                            }
                                        };
                                    });
                                }
                            }
                        }
                        _ => {}
                    }
                })
            },
            RemarkableFile::Other { version, pages } => {}
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

    use crate::indexer_rm::IndexerRm;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerRm.index("../../../test_files/Sample1.rm").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}