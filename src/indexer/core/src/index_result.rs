use std::path::PathBuf;
use harana_common::hashbrown::HashSet;
use harana_common::serde_json;

#[derive(Debug, Clone)]
pub struct IndexResult {
    pub path: PathBuf,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub primary_tokens: HashSet<String>,
    pub secondary_tokens: HashSet<String>,
    pub metadata: serde_json::Value
}