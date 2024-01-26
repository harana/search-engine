use std::collections::BTreeMap;
use harana_common::serde;
use harana_common::serde::{Deserialize, Serialize};
use harana_tantivy::structures::{DocumentPayload, DocumentValue, DocumentValueOptions};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
pub struct Document {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub primary_tokens: Vec<String>,
    pub secondary_tokens: Vec<String>,
    pub is_file: bool,
    pub created: u64,
    pub modified: u64,
    pub accessed: u64,
    pub size: u64,
    pub colour: Option<String>,
    pub path: Option<String>,
    pub open_url: Option<String>,
    pub extension: Option<String>,
    pub extension_title: Option<String>,
    pub parent_folder_name: Option<String>,
    pub parent_folder_path: Option<String>,
    pub metadata: Option<String>,
    pub cards: Vec<String>
}

impl Document {

    pub fn to_tantivy_payload(&self) -> DocumentPayload {
        let map = BTreeMap::from([
            ("title".to_string(), str(self.title.clone())),
            ("description".to_string(), str(self.description.clone())),
            ("author".to_string(), str(self.author.clone())),
            ("primary_tokens".to_string(), str(Some(self.primary_tokens.clone().join(",")))),
            ("secondary_tokens".to_string(), str(Some(self.secondary_tokens.clone().join(",")))),
            ("created".to_string(), u64(self.created.clone())),
            ("modified".to_string(), u64(self.modified.clone())),
            ("accessed".to_string(), u64(self.accessed.clone())),
            ("size".to_string(), u64(self.size.clone())),
            ("colour".to_string(), str(self.colour.clone())),
            ("path".to_string(), str(self.colour.clone())),
            ("extension".to_string(), str(self.colour.clone())),
            ("extension_title".to_string(), str(self.colour.clone())),
            ("parent_folder_name".to_string(), str(self.colour.clone())),
            ("parent_folder_path".to_string(), str(self.colour.clone())),
            ("metadata".to_string(), str(self.colour.clone())),
            ("cards".to_string(), str(self.colour.clone()))
        ]);

        DocumentPayload(map)
    }
}

fn str(value: Option<String>) -> DocumentValueOptions {
    DocumentValueOptions::Single(DocumentValue::Text(value.unwrap_or_default()))
}

fn u64(value: u64) -> DocumentValueOptions {
    DocumentValueOptions::Single(DocumentValue::U64(value))
}