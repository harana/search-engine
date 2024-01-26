use std::fs;
use std::path::PathBuf;

use psd::{ColorMode, ImageResource, Psd, PsdDepth};

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct PsdMetadata {
    width: usize,
    height: usize,
    bit_depth: usize,
    color_mode: String,
    layers: Vec<String>
}
pub struct IndexerPsd;

impl Indexer for IndexerPsd {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let bytes = fs::read(path.clone())?;
        let psd = Psd::from_bytes(bytes.as_slice())?;

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();
        let mut layers = Vec::<String>::new();

        psd.layers().iter().for_each(|l| {
            primary_tokens.insert(l.name().to_string());
            layers.push(l.name().to_string());
        });

        psd.groups().values().for_each(|g| {
            primary_tokens.insert(g.name().to_string());
        });

        psd.resources().iter().for_each(|i| {
            match i {
                ImageResource::Slices(s) => {
                    primary_tokens.insert(s.name().to_string());
                }
            }
        });

        let bit_depth = match psd.depth() {
            PsdDepth::One => 1,
            PsdDepth::Eight => 8,
            PsdDepth::Sixteen => 16,
            PsdDepth::ThirtyTwo => 32
        };

        let color_mode = match psd.color_mode() {
            ColorMode::Bitmap => "Bitmap",
            ColorMode::Grayscale => "Grayscale",
            ColorMode::Indexed => "Indexed",
            ColorMode::Rgb => "RGB",
            ColorMode::Cmyk => "CMYK",
            ColorMode::Multichannel => "Multi-Channel",
            ColorMode::Duotone => "Duotone",
            ColorMode::Lab => "Lab"
        };

        let width = psd.width() as usize;
        let height = psd.height() as usize;

        secondary_tokens.insert(color_mode.to_string());

        let metadata = PsdMetadata {
            width,
            height,
            bit_depth,
            color_mode: color_mode.to_string(),
            layers
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

    use crate::indexer_psd::IndexerPsd;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerPsd.index("../../../test_files/Sample1.psd").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}