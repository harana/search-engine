use std::io::Read;
use std::path::PathBuf;

use c3dio::C3d;

use harana_common::anyhow::{Error, Result};
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct C3dMetadata {
    manufacturer_company: Option<String>,
    manufacturer_software: Option<String>,
    manufacturer_version: Option<String>,
}

pub struct IndexerC3d;

impl Indexer for IndexerC3d {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let c3d = C3d::load_parameters(path.to_str().unwrap()).map_err(|e| {
            Error::msg(e.to_string())
        })?;

        let mut primary_tokens = HashSet::new();
        let mut secondary_tokens = HashSet::new();

        // let analog_format = match c3d.parameters.analog.format {
        //     AnalogFormat::Signed => "Signed",
        //     AnalogFormat::Unsigned => "Unsigned"
        // };
        //
        // c3d.parameters.analog.descriptions.iter().for_each(|d| {
        //     secondary_tokens.extend(tokenize(d.as_str()));
        // });
        //
        // c3d.parameters.event.descriptions.unwrap_or_default().iter().for_each(|u| {
        //    println!("Event Desc: {}", u);
        // });
        //
        // let metadata = C3dMetadata {
        //     manufacturer_company: c3d.parameters.manufacturer.company,
        //     manufacturer_software: c3d.parameters.manufacturer.software,
        //     manufacturer_version: c3d.parameters.manufacturer.version.map(|v| {
        //         match v {
        //             ManufacturerVersion::String(s) => { s }
        //             ManufacturerVersion::Float(f) => { f.to_string() }
        //             ManufacturerVersion::Array(a) => { "".to_string() }
        //         }
        //     })
        // };

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens,
            secondary_tokens,
            metadata: Default::default()
            // metadata: serde_json::to_value(&metadata)?
        })
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_c3d::IndexerC3d;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerC3d.index("../../../test_files/Sample1.c3d").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}