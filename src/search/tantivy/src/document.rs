use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use humansize::{DECIMAL, format_size};
use timeago::Formatter;

use harana_common::hashbrown::{HashMap, HashSet};
use harana_common::serde_json::{from_str, Value};
use harana_document::document::Document;
use harana_tantivy::helpers::{hit_bool, hit_string, hit_u64};
use harana_tantivy::structures::DocumentHit;

use harana_search_extensions::extensions::Extensions;

pub struct TantivyDocument {}

impl TantivyDocument {
    pub fn from_hit(hit: DocumentHit) -> Document {
        let mut tags = HashSet::new();
        hit_string(&hit, "tags").unwrap_or_default().split(",").map(|s| s.to_string()).for_each(|s| {
            tags.insert(s);
        });

        let mut primary_tokens = HashSet::new();
        hit_string(&hit, "primary_tokens").unwrap_or_default().split(" ").map(|s| s.to_string()).for_each(|s| {
            primary_tokens.insert(s);
        });

        let mut secondary_tokens = HashSet::new();
        hit_string(&hit, "secondary_tokens").unwrap_or_default().split(" ").map(|s| s.to_string()).for_each(|s| {
            secondary_tokens.insert(s);
        });

        let path = hit_string(&hit, "path");
        let path_buf = path.clone().map(|p| PathBuf::from(p));
        let parent_folder = path_buf.as_ref().and_then(|p| p.parent());
        let parent_folder_name = parent_folder.and_then(|p| p.file_stem()).and_then(|n| n.to_str()).map(|s| s.to_string());
        let parent_folder_path = parent_folder.and_then(|p| p.to_str()).map(|s| s.to_string());

        let extension = hit_string(&hit, "extension");

        let mut cards_data: HashMap<String, String> = HashMap::new();
        let cards_data_json: Option<Value> = hit_string(&hit, "cards_data").and_then(|s| from_str(s.as_str()).ok());
        if cards_data_json.is_some() {
            if let Value::Object(obj) = cards_data_json.unwrap() {
                for (key, value) in obj {
                    cards_data.insert(key, value.to_string());
                }
            }
        }

        Document {
            id: hit.document_id.to_string(),
            title: hit_string(&hit, "title").unwrap_or(String::new()),
            subtitle: hit_string(&hit, "subtitle"),
            description: hit_string(&hit, "description"),
            tags,
            author: hit_string(&hit, "author"),
            primary_tokens,
            secondary_tokens,
            is_file: hit_bool(&hit, "is_file").unwrap_or(true),
            created: readable_date(hit_u64(&hit, "created")),
            modified: readable_date(hit_u64(&hit, "modified")),
            accessed: readable_date(hit_u64(&hit, "accessed")),
            size: hit_u64(&hit, "size").map(|u| format_size(u, DECIMAL)),
            colour: hit_string(&hit, "colour"),
            icon_url: hit_string(&hit, "icon_url"),
            open_url: hit_string(&hit, "open_url"),
            path: path.clone(),
            extension: extension.clone(),
            extension_title: hit_string(&hit, "extension_title"),
            group_name: hit_string(&hit, "group_name"),
            parent_folder_name,
            parent_folder_path,
            metadata: hit_string(&hit, "metadata"),
            cards: if extension.is_some() { Extensions::cards(None, extension.unwrap().as_str()).iter().map(|s| s.to_string()).collect() } else { vec!() },
            cards_data
        }
    }
}

pub fn readable_date(date: Option<u64>) -> Option<String> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok().map(|duration| duration.as_secs());
    now.and_then(|now_secs| {
        date.map(|date_secs| {
            let duration = Duration::from_secs(now_secs - date_secs);
            Formatter::new().convert(duration)
        })
    })
}