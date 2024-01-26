use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::serde::Serialize;
use harana_common::serde_json;
use harana_common::serde_json::Value;
use harana_database::files_update_hash::files_update_hash;
use harana_database::manager::DatabaseManager;
use harana_job::handler::JobHandler;
use harana_search_index::manager::IndexManager;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct JobHandlerHashPayload {
    pub file_path: PathBuf,
}

pub struct JobHandlerHash {
    database_manager: &'static DatabaseManager,
    index_manager: &'static IndexManager
}

impl JobHandlerHash {
    pub fn new(database_manager: &'static DatabaseManager, index_manager: &'static IndexManager) -> Self {
        Self {
            database_manager,
            index_manager
        }
    }
}

#[async_trait]
impl JobHandler for JobHandlerHash {

    async fn handle(&'static self, payload: Value) -> Result<()> {
        let payload: JobHandlerHashPayload = serde_json::from_value(payload)?;

        let file = fs::read(payload.file_path.clone())?;
        let hash = blake3::hash(file.as_slice()).to_string();

        // Update database
        self.database_manager.files(move |c| {
            files_update_hash(c, payload.file_path.to_str().unwrap().to_string(), hash)
        }).await?;

        Ok(())
    }
}