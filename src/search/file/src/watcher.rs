use std::fs;
use std::path::{Path, PathBuf};

use partial_application::partial;

use harana_common::anyhow::{anyhow, Result};
use harana_database::manager::DatabaseManager;
use harana_database::files_add::files_add;
use harana_database::files_delete::file_delete;
use harana_file_watcher::start_watcher;

pub async fn start_watching(database_manager: &'static DatabaseManager, path: &Path) -> Result<()> {
    let on_create = partial!(watch_create => database_manager, _);
    let on_update = partial!(watch_update => database_manager, _);
    let on_delete = partial!(watch_delete => database_manager, _);

    start_watcher(vec!(path), on_create, on_update, on_delete).await;
    Ok(())
}

async fn watch_create(database_manager: &'static DatabaseManager, path: PathBuf) {
    if path.is_file() {
        database_manager.files(move |c| {
            let file = fs::read(path.clone()).map_err(|e| anyhow!(e.to_string()))?;
            let hash = blake3::hash(file.as_slice()).to_string();
            files_add(c, path.clone().display().to_string(), hash)
        }).await.unwrap()
    }
}

async fn watch_update(database_manager: &'static DatabaseManager, path: PathBuf) {
    database_manager.files(move |c| {
        file_delete(c, path.display().to_string())
    }).await.unwrap()
}

async fn watch_delete(database_manager: &'static DatabaseManager, path: PathBuf) {
    database_manager.files(move |c| {
        file_delete(c, path.display().to_string())
    }).await.unwrap()
}