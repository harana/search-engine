use std::path::Path;

use harana_common::anyhow::{anyhow, Result};
use harana_common::directories::UserDirs;
use harana_database::manager::DatabaseManager;
use harana_database::job_groups_add::job_groups_add;
use harana_search_index::manager::IndexManager;
use harana_job::manager::JobManager;

use crate::{crawler, watcher};

pub struct FileManager {
    database_manager: &'static DatabaseManager,
    index_manager: &'static IndexManager,
    job_manager: &'static JobManager
}

impl FileManager {

    pub async fn new(database_manager: &'static DatabaseManager, index_manager: &'static IndexManager, job_manager: &'static JobManager) -> Self {
        Self {
            database_manager,
            index_manager,
            job_manager
        }
    }

    pub async fn crawl(&'static self, name: String) -> Result<()> {
        let user_dirs = UserDirs::new().ok_or(anyhow!("Failed to find user-dirs"))?;
        let start_path = match name.as_str() {
            "application" => Path::new("/Applications"),
            "audio" => user_dirs.audio_dir().ok_or(anyhow!("Audio directory does not exist"))?,
            "desktop" => user_dirs.desktop_dir().ok_or(anyhow!("Desktop directory does not exist"))?,
            "document" => user_dirs.document_dir().ok_or(anyhow!("Document directory does not exist"))?,
            "download" => user_dirs.download_dir().ok_or(anyhow!("Download directory does not exist"))?,
            "font" => user_dirs.font_dir().ok_or(anyhow!("Font directory does not exist"))?,
            "home" => user_dirs.home_dir(),
            "picture" => user_dirs.picture_dir().ok_or(anyhow!("Picture directory does not exist"))?,
            "public" => user_dirs.public_dir().ok_or(anyhow!("Public directory does not exist"))?,
            "video" => user_dirs.video_dir().ok_or(anyhow!("Video directory does not exist"))?,
            _ => Path::new("/tmp")
        };

        self.crawl_path(name, start_path).await
    }

    pub async fn crawl_path(&'static self, name: String, start_path: &Path) -> Result<()> {
        let job_group_name = name.clone();
        self.database_manager.core(move |connection| {
            job_groups_add(connection, "file".to_string(), job_group_name, 1)
        }).await?;

        // Crawl path
        crawler::crawl(
            self.database_manager,
            self.index_manager,
            self.job_manager,
            name.clone(),
            start_path.to_path_buf()
        ).await?;

        // Start watching
        watcher::start_watching(self.database_manager, start_path).await
    }
}