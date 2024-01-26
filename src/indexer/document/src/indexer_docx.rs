use harana_common::hashbrown::HashSet;
use std::ops::Deref;
use std::path::PathBuf;

use docx_rust::DocxFile;

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct DocxMetadata {
    application: Option<String>,
    character_count: Option<usize>,
    company: Option<String>,
    line_count: Option<usize>,
    page_count: Option<usize>,
    paragraph_count: Option<usize>,
    total_time: Option<usize>,
    word_count: Option<usize>,

    subject: Option<String>,
    description: Option<String>,
    title: Option<String>,
    creator: Option<String>,
    keywords: Option<String>,
    last_modified_by: Option<String>,
    revision: Option<String>
}

pub struct IndexerDocx;

impl Indexer for IndexerDocx {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let docx = DocxFile::from_file(path.clone()).unwrap();
        let mut docx = docx.parse().unwrap();

        let mut primary_tokens = HashSet::new();
        let mut secondary_tokens = HashSet::new();

        let application = docx.app.clone().and_then(|a| a.application).map(|s| s.to_string());
        let character_count = docx.app.clone().and_then(|a| a.characters).and_then(|s| s.parse::<usize>().ok());
        let company = docx.app.clone().and_then(|a| a.company).map(|s| s.to_string());
        let line_count = docx.app.clone().and_then(|a| a.lines).and_then(|s| s.parse::<usize>().ok());
        let page_count = docx.app.clone().and_then(|a| a.pages).and_then(|s| s.parse::<usize>().ok());
        let paragraph_count = docx.app.clone().and_then(|a| a.paragraphs).and_then(|s| s.parse::<usize>().ok());
        let total_time = docx.app.clone().and_then(|a| a.total_time).and_then(|s| s.parse::<usize>().ok());
        let word_count = docx.app.clone().and_then(|a| a.words).and_then(|s| s.parse::<usize>().ok());

        let subject = docx.core.clone().and_then(|a| a.subject).map(|s| s.to_string());
        let description = docx.core.clone().and_then(|a| a.description).map(|s| s.to_string());
        let title = docx.core.clone().and_then(|a| a.title).map(|s| s.to_string());
        let creator = docx.core.clone().and_then(|a| a.creator).map(|s| s.to_string());
        let keywords = docx.core.clone().and_then(|a| a.keywords).map(|s| s.to_string());
        let last_modified_by = docx.core.clone().and_then(|a| a.last_modified_by).map(|s| s.to_string());
        let revision = docx.core.clone().and_then(|a| a.revision).map(|s| s.to_string());

        if subject.is_some() { primary_tokens.extend(tokenize(subject.clone().unwrap().as_str())); }
        if description.is_some() { primary_tokens.extend(tokenize(description.clone().unwrap().as_str())); }
        if title.is_some() { primary_tokens.extend(tokenize(title.clone().unwrap().as_str())); }
        if creator.is_some() { primary_tokens.extend(tokenize(creator.clone().unwrap().as_str())); }
        if keywords.is_some() { primary_tokens.extend(tokenize(keywords.clone().unwrap().as_str())); }
        if last_modified_by.is_some() { primary_tokens.extend(tokenize(last_modified_by.clone().unwrap().as_str())); }

        if docx.comments.is_some() {
            docx.comments.unwrap().comments.iter().for_each(|c| {
                primary_tokens.extend(tokenize(c.author.deref()));
                primary_tokens.extend(entity_tokens(c.content.text().as_str()));
                secondary_tokens.extend(tokenize(c.content.text().as_str()));
            });
        }

        primary_tokens.extend(entity_tokens(docx.document.body.text().as_str()));
        secondary_tokens.extend(tokenize(docx.document.body.text().as_str()));

        let metadata = DocxMetadata {
            application,
            character_count,
            company,
            line_count,
            page_count,
            paragraph_count,
            total_time,
            word_count,
            subject,
            description: description.clone(),
            title: title.clone(),
            creator: creator.clone(),
            keywords,
            last_modified_by,
            revision
        };

        Ok(IndexResult {
            path,
            title: title.clone(),
            description: description.clone(),
            author: creator.clone(),
            primary_tokens,
            secondary_tokens,
            metadata: serde_json::to_value(&metadata)?
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_docx::IndexerDocx;

    #[tokio::test]
    async fn test_indexing() {
        let file = IndexResult {
            file_name: "Sample1.docx".to_string(),
            path: Path::new("../../../test_files/Sample1.docx").to_path_buf(),
            title: None,
            description: None,
            author: None,
            primary_tokens: Default::default(),
            secondary_tokens: Default::default(),
            metadata: Default::default(),
        };

        let indexed_file = IndexerDocx.index(&file).unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Secondary Tokens: {}", itertools::join(&indexed_file.secondary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}