use harana_common::hashbrown::{HashMap, HashSet};
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json::Value;

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
pub struct Document {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub tags: HashSet<String>,
    pub author: Option<String>,
    pub primary_tokens: HashSet<String>,
    pub secondary_tokens: HashSet<String>,
    pub is_file: bool,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub accessed: Option<String>,
    pub size: Option<String>,
    pub colour: Option<String>,
    pub icon_url: Option<String>,
    pub open_url: Option<String>,
    pub path: Option<String>,
    pub extension: Option<String>,
    pub extension_title: Option<String>,
    pub group_name: Option<String>,
    pub parent_folder_name: Option<String>,
    pub parent_folder_path: Option<String>,
    pub metadata: Option<String>,
    pub cards: Vec<String>,
    pub cards_data: HashMap<String, String>
}