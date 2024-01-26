use std::path::PathBuf;
use harana_common::hashbrown::HashSet;
use msg_parser::Outlook;

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct MsgMetadata {
    from: (String, String),
    to: Vec<(String, String)>,
    cc: Vec<(String, String)>,
    subject: String
}

pub struct IndexerMsg;

impl Indexer for IndexerMsg {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let outlook = Outlook::from_path(path.clone())?;

        let mut primary_tokens = HashSet::new();
        let mut secondary_tokens = HashSet::new();

        primary_tokens.extend(tokenize(outlook.sender.name.as_str()));
        primary_tokens.extend(tokenize(outlook.sender.email.as_str()));
        primary_tokens.extend(tokenize(outlook.subject.as_str()));

        primary_tokens.extend(tokenize(outlook.body.as_str()));
        secondary_tokens.extend(tokenize(outlook.body.as_str()));

        outlook.to.iter().for_each(|p| {
            primary_tokens.extend(tokenize(p.name.as_str()));
            primary_tokens.extend(tokenize(p.email.as_str()));
        });

        outlook.cc.iter().for_each(|p| {
            primary_tokens.extend(tokenize(p.name.as_str()));
            primary_tokens.extend(tokenize(p.email.as_str()));
        });

        let to = outlook.to.into_iter().map(|p| (p.name, p.email)).collect();
        let cc = outlook.cc.into_iter().map(|p| (p.name, p.email)).collect();

        let metadata = MsgMetadata {
            from: (outlook.sender.name, outlook.sender.email),
            to,
            cc,
            subject: outlook.subject.clone()
        };

        Ok(IndexResult {
            path,
            title: Some(outlook.subject.clone()),
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
    use std::path::Path;

    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_msg::IndexerMsg;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerMsg.index("../../../test_files/Sample1.msg").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}