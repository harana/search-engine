use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Deserialize;

use harana_common::anyhow::{Context, Result};
use harana_common::async_trait::async_trait;
use harana_common::itertools::Itertools;
use harana_common::log::info;
use harana_common::serde::Serialize;
use harana_common::{futures, serde_json, tokio};
use harana_common::serde_json::Value;
use harana_database::files_update_index::files_update_index;
use harana_database::manager::DatabaseManager;
use harana_indexer_core::index_result::IndexResult;
use harana_search_extensions::file::extension_details;
use harana_search_index::manager::IndexManager;
use harana_tantivy::structures::{DocumentPayload, DocumentValue, DocumentValueOptions};
use harana_job::handler::JobHandler;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct JobHandlerIndexPayload {
    pub document_id: u64,
    pub file_path: PathBuf,
}

pub struct JobHandlerIndex {
    database_manager: &'static DatabaseManager,
    index_manager: &'static IndexManager
}

impl JobHandlerIndex {
    pub fn new(database_manager: &'static DatabaseManager, index_manager: &'static IndexManager) -> Self {
        Self {
            database_manager,
            index_manager
        }
    }

    pub async fn tantivy_payload(existing_data: BTreeMap<String, DocumentValueOptions>, index_result: IndexResult) -> DocumentPayload {
        let mut new_data = BTreeMap::from([
            ("title".to_string(), DocumentValueOptions::Single(DocumentValue::Text(index_result.title.unwrap_or_default()))),
            ("description".to_string(), DocumentValueOptions::Single(DocumentValue::Text(index_result.description.unwrap_or_default()))),
            ("author".to_string(), DocumentValueOptions::Single(DocumentValue::Text(index_result.author.unwrap_or_default().to_string()))),
            ("primary_tokens".to_string(), DocumentValueOptions::Single(DocumentValue::Text(index_result.primary_tokens.iter().join(" ")))),
            ("secondary_tokens".to_string(), DocumentValueOptions::Single(DocumentValue::Text(index_result.secondary_tokens.iter().join(" ")))),
            ("metadata".to_string(), DocumentValueOptions::Single(DocumentValue::Text(index_result.metadata.to_string())))
        ]);

        new_data.extend(existing_data);
        DocumentPayload(new_data)
    }
}

#[async_trait]
impl JobHandler for JobHandlerIndex {

    async fn handle(&'static self, payload: Value) -> Result<()> {
        let payload: JobHandlerIndexPayload = serde_json::from_value(payload)?;
        let path_metadata = std::fs::metadata(payload.file_path.clone()).ok();
        let extension_details = extension_details(payload.file_path.clone(), path_metadata).await;
        let index = self.index_manager.get_index(extension_details.2);

        // Get existing document from Tantivy
        let existing_data = index.get_document(payload.document_id).await?.doc.into_iter()
            .filter(|d| d.1.is_some())
            .map(|(k, v)| (k, v.unwrap()))
            .collect();

        // Delete existing document from Tantivy
        index.delete_document(payload.document_id).await?;

        // Index document
        let file_path = payload.file_path.clone();
        let index_result = std::panic::catch_unwind(|| {
            extension_details.0.index(file_path).unwrap()
        }).unwrap();

        // Add to Tantivy
        let tantivy_payload = JobHandlerIndex::tantivy_payload(existing_data, index_result.clone()).await;
        index.add_document(payload.document_id, tantivy_payload).await?;
        index.commit().await?;

        // Log result
        let indexer_name = extension_details.0.get_name().replace("Indexer", "");
        let primary_token_count = index_result.primary_tokens.len().to_string();
        let secondary_token_count = index_result.secondary_tokens.len().to_string();
        info!("{} > {} > {}/{}", indexer_name, payload.file_path.clone().to_str().context("No payload file path")?, primary_token_count, secondary_token_count);

        // Update Database
        self.database_manager.files(move |connection| {
            files_update_index(connection, payload.file_path.to_str().context("No payload file path")?.to_string())
        }).await.expect("Failed to update index");

        Ok(())
    }
}

