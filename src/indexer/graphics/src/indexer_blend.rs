use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use blend::Blend;
use blend::parsers::BlendParseError;
use libflate::gzip::Decoder;

use harana_common::anyhow::{Error, Result};
use harana_common::hashbrown::HashSet;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

pub struct IndexerBlend;

impl Indexer for IndexerBlend {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let file = File::open(path.clone())?;
        let mut data: Option<Vec<u8>> = None;

        let mut primary_tokens = HashSet::new();
        let mut secondary_tokens = HashSet::new();

        if let Ok(decoded_data) = zstd::stream::decode_all(&file) {
            data = Some(decoded_data);
        };

        if data.is_none() {
            if let Ok(mut decoder) = Decoder::new(&file) {
                let mut gzip_data = Vec::new();
                decoder.read_to_end(&mut gzip_data)?;
                data = Some(gzip_data);
            }
        }

        if data.is_none() {
            if let Ok(byte_data) = fs::read(&path) {
                data = Some(byte_data);
            }
        }

        if data.is_some() {
            let blend = Blend::new(&data.unwrap()[..]).map_err(|e| {
                match e {
                    BlendParseError::NomError { .. } => { Error::msg("NomError") }
                    BlendParseError::IoError(error) => { Error::msg(error.to_string()) }
                    BlendParseError::NotEnoughData => { Error::msg("NotEnoughData") }
                    BlendParseError::UnknownBlockCode => { Error::msg("UnknownBlockCode") }
                    BlendParseError::UnsupportedCountOnPrincipalBlock => { Error::msg("UnsupportedCountOnPrincipalBlock") }
                    BlendParseError::InvalidMemoryAddress => { Error::msg("InvalidMemoryAddress") }
                    BlendParseError::NoDnaBlockFound => { Error::msg("NoDnaBlockFound") }
                    BlendParseError::CompressedFileNotSupported => { Error::msg("CompressedFileNotSupported") }
                }
            })?;

            for o in blend.root_instances() {
                let name = o.get("id").get_string("name");
                primary_tokens.extend(tokenize(name.as_str()));
            }
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

    use crate::indexer_blend::IndexerBlend;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerBlend.index("../../../test_files/Sample1.blend").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}