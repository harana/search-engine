use std::fs::Metadata;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use harana_common::serde::{Deserialize, Serialize};

use harana_common::{serde_json, tokio};
use harana_common::anyhow::{Context, Result};
use harana_common::async_trait::async_trait;
use harana_common::log::{error, info};
use harana_common::serde_json::Value;
use harana_common::tauri::{AppHandle, Manager, Wry};
use harana_database::files_update_thumbnail::files_update_thumbnail;
use harana_database::manager::DatabaseManager;
use harana_search_extensions::file::extension_details;
use harana_search_index::thumbnailer::IndexThumbnailer;
use harana_job::handler::JobHandler;
use harana_job::manager::JobManager;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct JobHandlerThumbnailPayload {
    pub document_id: u64,
    pub file_path: PathBuf
}

pub struct JobHandlerThumbnail {
    app: &'static AppHandle<Wry>,
    database_manager: &'static DatabaseManager,
    index_thumbnailer: &'static IndexThumbnailer
}

impl JobHandlerThumbnail {
    pub fn new(app: &'static AppHandle<Wry>, database_manager: &'static DatabaseManager, index_thumbnailer: &'static IndexThumbnailer) -> Self {
        Self {
            app,
            database_manager,
            index_thumbnailer
        }
    }
}

#[async_trait]
impl JobHandler for JobHandlerThumbnail {

    async fn handle(&'static self, payload: Value) -> Result<()> {
        let payload: JobHandlerThumbnailPayload = serde_json::from_value(payload)?;
        let path_metadata = std::fs::metadata(payload.file_path.clone()).ok();
        let extension_details = extension_details(payload.file_path.clone(), path_metadata).await;
        let thumbnailer_name = extension_details.1.get_name().replace("Thumbnailer", "");
        let thumbnails_path = self.index_thumbnailer.thumbnails_path.to_str().context("No thumbnail path")?.to_string();
        let mut output_path = Path::new(thumbnails_path.as_str()).join(payload.document_id.to_string());
        output_path.set_extension("png");

        let thumbnailer = extension_details.1;
        let width = self.index_thumbnailer.thumbnail_width;
        let height = self.index_thumbnailer.thumbnail_height;

        extension_details.1.thumbnail(payload.file_path.as_path(), output_path.as_path(), payload.document_id, self.app, width, height).await;

        // Log result
        info!("{} > {}", thumbnailer_name, payload.file_path.to_str().context("No payload file path")?.to_string());

        // Update Database
        self.database_manager.files(move |connection| {
            files_update_thumbnail(connection, payload.file_path.to_str().context("No payload file path")?.to_string())
        }).await?;

        Ok(())
    }
}