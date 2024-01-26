use std::path::PathBuf;

use msoffice_pptx::document::PPTXDocument;
use msoffice_pptx::pml::ShapeGroup;
use msoffice_shared::drawingml::TextRun;

use harana_common::{anyhow, serde_json};
use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::tracing::{Level, span};
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct PptxMetadata {
    title: Option<String>,
    revision: Option<i32>,
    created_time: Option<String>,
    creator: Option<String>,
    modified_time: Option<String>,
    last_modifier: Option<String>,
    app_name: Option<String>,
    app_version: Option<String>,
    slide_count: usize
}

pub struct IndexerPptx;

impl Indexer for IndexerPptx {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let mut primary_tokens = HashSet::new();

        let document =
            match PPTXDocument::from_file(&PathBuf::from(path.clone())) {
                Ok(doc) => Ok(doc),
                Err(e) => Err(anyhow::anyhow!(format!("pptx_indexer: Failed to open PPTX Document from file at path: {:?} with error {:?}", path, e)))
            }?;


        let mut title = None;
        let mut revision = None;
        let mut created_time = None;
        let mut creator = None;
        let mut modified_time = None;
        let mut last_modifier = None;

        let mut app_name = None;
        let mut app_version = None;

        document.core.iter().for_each(|core| {
            title = core.title.as_ref().cloned();
            revision = core.revision.as_ref().cloned();
            created_time = core.created_time.as_ref().cloned();
            creator = core.creator.as_ref().cloned();
            modified_time = core.modified_time.as_ref().cloned();
            last_modifier = core.last_modified_by.as_ref().cloned();
        });

        document.app.iter().for_each(|app| {
            app_name = app.app_name.as_ref().cloned();
            app_version = app.app_version.as_ref().cloned();
        });

        let slide_count = document.slide_map.len();

        for slide in document.slide_map.values() {
            let shape_group = &(*(*slide.common_slide_data).shape_tree).shape_array;
            for s_g in shape_group {
                if let Some(res_text) = extract_text(s_g) {
                    primary_tokens.extend(tokenize(res_text.as_str()));
                }
            }
        }

        let metadata = PptxMetadata {
            title: title.clone(),
            revision,
            created_time,
            creator: creator.clone(),
            modified_time,
            last_modifier,
            app_name,
            app_version,
            slide_count
        };

        Ok(IndexResult {
            path,
            title: title.clone(),
            description: None,
            author: creator.clone(),
            primary_tokens,
            secondary_tokens: HashSet::new(),
            metadata: serde_json::to_value(&metadata)?
        })
    }
}

fn extract_text(shape_group: &ShapeGroup) -> Option<String> {
    let mut total_text = String::new();
    match shape_group {
        ShapeGroup::Shape(shape) => {
            if let Some(text_body) = &shape.text_body {
                for paragraph in &text_body.paragraph_array {
                    for text_run in &paragraph.text_run_list {
                        if let TextRun::RegularTextRun(regular_text_run) = text_run {
                            total_text.push_str(&regular_text_run.text);
                            total_text.push_str(" ");
                        }
                    }
                }
            }
        }
        ShapeGroup::GroupShape(group_shape) => {
            let res_text = group_shape
                .shape_array
                .iter()
                .map(|s_g| extract_text(s_g))
                .filter_map(|r_t| r_t)
                .fold(String::new(), |mut acc, x| {
                    acc.push_str(&x);
                    acc.push_str(" ");
                    acc
                });

            total_text.push_str(&res_text);
        }
        _ => {}
    }
    if total_text != String::new() {
        Some(total_text)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_pptx::IndexerPptx;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerPptx.index("../../../test_files/Sample1.pptx").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Secondary Tokens: {}", itertools::join(&indexed_file.secondary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}