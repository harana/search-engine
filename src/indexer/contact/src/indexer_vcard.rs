use harana_common::hashbrown::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct VcardMetadata {
    full_name: Option<String>,
    company: Option<String>,
    title: Option<String>,
    role: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    note: Option<String>
}

pub struct IndexerVcard;

impl Indexer for IndexerVcard {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let buf = BufReader::new(File::open(path.clone())?);
        let reader = ical::VcardParser::new(buf);

        let mut primary_tokens = HashSet::<String>::new();

        let mut full_name = None;
        let mut company = None;
        let mut title = None;
        let mut role = None;
        let mut email = None;
        let mut phone_number = None;
        let mut note = None;

        reader.into_iter().for_each(|vcard| {
            if vcard.is_ok() {
                let vcard = vcard.unwrap();
                vcard.properties.iter().for_each(|p| {
                    if p.value.is_some() {
                        let value = p.clone().value.unwrap();
                        match p.name.as_str() {
                            "EMAIL" => {
                                email = Some(value.clone());
                                primary_tokens.extend(tokenize(value.as_str()));
                            },
                            "FN" => {
                                full_name = Some(value.clone());
                                primary_tokens.extend(tokenize(value.as_str()));
                            },
                            "NOTE" => {
                                note = Some(value.clone());
                                primary_tokens.extend(tokenize(value.as_str()));
                            },
                            "ORG" => {
                                company = Some(value.clone());
                                primary_tokens.extend(tokenize(value.as_str()));
                            },
                            "ROLE" => {
                                role = Some(value.clone());
                                primary_tokens.extend(tokenize(value.as_str()));
                            },
                            "TEL" => phone_number = Some(value),
                            "TITLE" => {
                                title = Some(value.clone());
                                primary_tokens.extend(tokenize(value.as_str()));
                            },
                            _ => {}
                        }
                    }
                });
            }
        });

        let metadata = VcardMetadata {
            full_name: full_name.clone(),
            company,
            title,
            role,
            email,
            phone_number,
            note
        };

        Ok(IndexResult {
            path,
            title: full_name.clone(),
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
    use std::path::Path;

    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_vcard::IndexerVcard;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerVcard.index("../../../test_files/Sample1.vcf").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}