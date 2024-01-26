pub extern crate rusqlite;
pub extern crate rusqlite_migration;

use harana_common::serde::{Deserialize, Serialize};
use harana_common::time::OffsetDateTime;

pub mod adjusted_terms_add;
pub mod adjusted_terms_delete_all;
pub mod adjusted_terms_delete_user;
pub mod adjusted_terms_list;

pub mod ai_models_add;
pub mod ai_models_delete;
pub mod ai_models_list;
pub mod ai_models_update;

pub mod developer_sources_add;
pub mod developer_sources_list;
pub mod developer_sources_update;

pub mod files_add;
pub mod files_bulk_add;
pub mod files_bulk_update_index;
pub mod files_clear;
pub mod files_count_all;
pub mod files_count_index;
pub mod files_count_thumbnail;
pub mod files_delete;
pub mod files_delete_all;
pub mod files_delete_prefix;
pub mod files_get_document_id;
pub mod files_get_path;
pub mod files_list;
pub mod files_list_index;
pub mod files_list_prefix;
pub mod files_update_hash;
pub mod files_update_index;
pub mod files_update_move;
pub mod files_update_thumbnail;

pub mod manager;

pub mod private_folders_add;
pub mod private_folders_delete;
pub mod private_folders_list;

pub mod recent_documents_add;
pub mod recent_documents_clear;
pub mod recent_documents_delete;
pub mod recent_documents_list;

pub mod rules_add;
pub mod rules_delete;
pub mod rules_list;
pub mod rules_update;

pub mod search_categories_add;
pub mod search_categories_list;
pub mod search_categories_update;

pub mod search_folders_add;
pub mod search_folders_delete;
pub mod search_folders_get;
pub mod search_folders_list;
pub mod search_folders_update;
pub mod search_folders_update_crawled;
pub mod search_folders_update_enabled;

pub mod settings_list;
pub mod settings_get;
pub mod settings_upsert;

pub mod state_list;
pub mod state_get;
pub mod state_upsert;

pub mod tasks_add;
pub mod tasks_bulk_add;
pub mod tasks_bulk_delete;
pub mod tasks_bulk_update_failure;
pub mod tasks_delete_all;
pub mod tasks_delete;
pub mod tasks_list;
pub mod tasks_list_index;
pub mod tasks_update;
pub mod tasks_update_failure;

pub mod job_groups_add;
pub mod job_groups_complete;
pub mod job_groups_delete;
pub mod job_groups_get;
pub mod job_groups_increment_failure;
pub mod job_groups_increment_success;
pub mod job_groups_list;

mod sqlite_functions;
mod urlencoder;

#[derive(Debug, Deserialize, Serialize)]
pub struct HaranaAdjustedTerm {
    pub category: String,
    pub term: String,
    pub language: String,
    pub direction: bool,
    pub user_added: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HaranaAiModel {
    pub id: String,
    pub status: String,
    pub enabled: bool,
    pub indexer_type: String,
    pub name: String,
    pub version: String,
    pub creation_date: OffsetDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HaranaDeveloperSource {
    pub title: String,
    pub name: String,
    pub language: String,
    pub enabled: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HaranaFile {
    pub path: String,
    pub hash: Option<String>,
    pub index_date: Option<OffsetDateTime>,
    pub thumbnail_date: Option<OffsetDateTime>,
    pub document_id: u64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HaranaRecentDocuments {
    pub search_query: String,
    pub documents: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HaranaRule {
    pub id: String,
    pub source_type: String,
    pub qualifier_type: String,
    pub qualifier_value: String,
    pub action_type: String,
    pub action_value: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HaranaSearchCategory {
    pub title: String,
    pub name: String,
    pub enabled: bool,
    pub position: u32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HaranaSearchFolder {
    pub title: String,
    pub name_or_path: String,
    pub is_path: bool,
    pub enabled: bool,
    pub crawled: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HaranaSetting {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HaranaState {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HaranaTask {
    pub id: String,
    pub priority: u32,
    pub job_group_category: String,
    pub job_group_name: String,
    pub payload: String,
    pub attempts: u32,
    pub last_attempt_date: OffsetDateTime
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HaranaTaskGroup {
    pub category: String,
    pub name: String,
    pub successes: u32,
    pub failures: u32,
    pub total: u32
}
