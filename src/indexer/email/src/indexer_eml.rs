use std::path::PathBuf;

use eml_parser::eml::{Eml, HeaderFieldValue};
use eml_parser::eml::HeaderFieldValue::*;
use eml_parser::EmlParser;

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::entity_recognition::entity_tokens;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct EmlMetadata {
    from: Vec<String>,
    to: Vec<String>,
    subject: Option<String>,
    body: Option<String>
}
pub struct IndexerEml;

impl Indexer for IndexerEml {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let eml: Eml = EmlParser::from_file(&path)?.with_body().parse()?;
        let from = people(eml.from);
        let to = people(eml.to);

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        primary_tokens.extend(from.clone());
        primary_tokens.extend(to.clone());

        if let Some(ref subject) = eml.subject {
            primary_tokens.extend(tokenize(subject.as_str()));
        }

        if let Some(ref body) = eml.body {
            primary_tokens.extend(entity_tokens(body.as_str()));
            secondary_tokens.extend(tokenize(body.as_str()));
        }

        let metadata = EmlMetadata {
            from,
            to,
            body: eml.body.clone(),
            subject: eml.subject.clone()
        };

        Ok(IndexResult {
            path,
            title: eml.subject.clone(),
            description: None,
            author: None,
            primary_tokens,
            secondary_tokens,
            metadata: serde_json::to_value(&metadata)?
        })
    }
}

fn people(hfv: Option<HeaderFieldValue>) -> Vec<String> {
    if hfv.is_none() {
        vec!()
    }else {
        hfv.map_or(vec!(), |f| {
            match f {
                SingleEmailAddress(e) => vec!(e.to_string()),
                MultipleEmailAddresses(ev) => ev.into_iter().map(|e| e.to_string()).collect(),
                Unstructured(u) => vec!(u.to_string()),
                Empty => vec!()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::indexer::Indexer;

    use crate::indexer_eml::IndexerEml;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerEml.index("../../../test_files/Sample1.eml").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}