use std::convert::TryInto;
use std::io::Cursor;
use std::path::PathBuf;

use image::{DynamicImage, EncodableLayout, ImageBuffer, ImageFormat, RgbImage};
use pdf::content::{Op, TextDrawAdjusted};
use pdf::enc::set_jpx_decoder;
use pdf::file::FileOptions;
use pdf::object::{Resolve, XObject};
use pdf::PdfError;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::itertools::Itertools;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::image_codec::decode_jpeg2k;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::ocr;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct PdfMetadata {
    title: Option<String>,
    author: Option<String>,
    subject: Option<String>,
    keywords: Option<String>,
    creator: Option<String>,
    producer: Option<String>,
    page_count: usize
}
pub struct IndexerPdf;

impl Indexer for IndexerPdf {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let file = FileOptions::cached().open(path.clone())?;
        let resolver = file.resolver();

        let mut primary_tokens = HashSet::new();
        let mut secondary_tokens = HashSet::new();

        let mut title = None;
        let mut subject = None;
        let mut author = None;
        let mut keywords = None;
        let mut creator = None;
        let mut producer = None;

        if file.trailer.info_dict.is_some() {
            let info = file.trailer.info_dict.as_ref().unwrap();
            title = info.title.clone().map(|t| t.to_string_lossy());
            subject = info.subject.clone().map(|t| t.to_string_lossy());
            author = info.author.clone().map(|t| t.to_string_lossy());
            keywords = info.keywords.clone().map(|t| t.to_string_lossy());
            creator = info.creator.clone().map(|t| t.to_string_lossy());
            producer = info.producer.clone().map(|t| t.to_string_lossy());
        }

        let mut texts = String::new();
        let mut images: Vec<_> = vec![];

        for page in file.pages() {
            let page = page?;

            let resources = page.resources.as_ref().unwrap();
            let contents = page.contents.as_ref().unwrap();

            images.extend(resources.xobjects.iter().map(|(_name, &r)| resolver.get(r).unwrap())
                .filter(|o| matches!(**o, XObject::Image(_)))
            );

            for op in contents.operations(&resolver)?.iter() {
                match op {
                    Op::TextDraw { text } => {
                        texts.push_str(text.to_string_lossy().as_str());
                    }

                    Op::TextDrawAdjusted { array } => {
                        for data in array {
                            if let TextDrawAdjusted::Text(text) = data {
                                texts.push_str(text.to_string_lossy().as_str());
                            }
                        }
                    }

                    Op::BeginMarkedContent { tag, properties } => {
                        texts.push_str(" ")
                    }

                    Op::TextNewline => {
                        texts.push_str(" ");
                    }

                    Op::EndText => {
                        texts.push_str(" ");
                    }


                    _ => {}
                }
            }
        }

        primary_tokens.extend(entity_tokens(texts.as_str()));
        secondary_tokens.extend(tokenize(texts.as_str()));

        fn jpx_decoder(data: &[u8]) -> std::result::Result<Vec<u8>, PdfError> {
            decode_jpeg2k(data).map_err(|e| {
                PdfError::from(e.to_string())
            })
        }
        set_jpx_decoder(Box::new(jpx_decoder));


        for (i, o) in images.iter().enumerate() {
            let img = match **o {
                XObject::Image(ref im) => im,
                _ => continue
            };
            let data = img.image_data(&resolver)?;

            let mut rgb_img: RgbImage = ImageBuffer::new(img.width, img.height);

            rgb_img.copy_from_slice(data.as_bytes());

            let mut img = DynamicImage::from(rgb_img).grayscale();
            let mut grey_img = img.as_mut_luma8().unwrap();

            let mut output = Cursor::new(Vec::new());
            grey_img.write_to(&mut output, ImageFormat::Tiff)?;

            let text = ocr::text_image(output.get_ref().as_bytes(), 70)?;
            let cleaned_texts = text.split(" ").filter(|t| t.len() > 3).join(" ");
            primary_tokens.extend(entity_tokens(cleaned_texts.as_str()));
            secondary_tokens.extend(tokenize(cleaned_texts.as_str()));
        }

        let metadata = PdfMetadata {
            title: title.clone(),
            author: author.clone(),
            subject,
            keywords,
            creator,
            producer,
            page_count: file.num_pages() as usize,
        };

        Ok(IndexResult {
            path,
            title: title.clone(),
            description: None,
            author: author.clone(),
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

    use crate::indexer_pdf::IndexerPdf;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerPdf.index("../../../test_files/Sample1.pdf").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Secondary Tokens: {}", itertools::join(&indexed_file.secondary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}