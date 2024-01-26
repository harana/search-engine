use std::path::PathBuf;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

pub struct IndexerImagemagick;

impl Indexer for IndexerImagemagick {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        // let metadata = imagemagick::metadata(&PathBuf::from(path))?;
        // let ocr_image = imagemagick::ocr_ready_image(&PathBuf::from(path))?;
        //
        // match ocr::text_image(ocr_image.as_slice()) {
        //     Ok(text) => {
        //         let mut phrase = String::new();
        //         let cleaned = text.replace(|c: char| !c.is_ascii_alphabetic(), " ");
        //         cleaned.split(" ").for_each(|word| {
        //             if word.len() > 4 {
        //                 phrase.push_str(word.to_lowercase().as_str());
        //                 phrase.push_str(" ");
        //             }
        //         });
        //     }
        //     Err(e) => error!("Failed to OCR image: {}", e.to_string())
        // }
        //
        // let mut primary_tokens = HashSet::new();
        //
        // let artist = metadata.artist.clone();
        // let make = metadata.make.clone();
        // let model = metadata.model.clone();
        // let admin1 = metadata.location_admin1.clone();
        // let admin2 = metadata.location_admin2.clone();
        // let location = metadata.location_name.clone();
        // let country = metadata.location_country.clone();
        // let histogram = metadata.histogram.clone();
        //
        // if artist.is_some() { primary_tokens.insert(artist.unwrap()); }
        // if make.is_some() { primary_tokens.insert(make.unwrap()); }
        // if model.is_some() { primary_tokens.insert(model.unwrap()); }
        // if admin1.is_some() { primary_tokens.insert(admin1.unwrap()); }
        // if admin2.is_some() { primary_tokens.insert(admin2.unwrap()); }
        // if location.is_some() { primary_tokens.insert(location.unwrap()); }
        // if country.is_some() { primary_tokens.insert(country.unwrap()); }

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens: HashSet::new(),
            // primary_tokens,
            secondary_tokens: HashSet::new(),
            metadata: Default::default()
            // metadata: serde_json::to_value(&metadata)?
        })
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_imagemagick::IndexerImagemagick;

    #[tokio::test]
    async fn test_indexing() {
        let file_types = vec!("cr2", "dng", "heic", "jpg", "png");

        file_types.into_iter().for_each(|ft| {
            let file_name = format!("Sample1.{}", ft);
            let file_path = format!("../../../test_files/Sample1.{}", ft);

            let indexed_file = IndexerImagemagick.index(&file_path).unwrap();
            println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
            println!("Metadata: {}", indexed_file.metadata.to_string());
        });
    }
}