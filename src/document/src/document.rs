use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
pub struct Document {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
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
    pub cards: Vec<String>
}