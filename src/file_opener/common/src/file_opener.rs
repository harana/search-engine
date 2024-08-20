use harana_common::anyhow::Result;
use std::path::Path;
use harana_common::tauri::App;

pub trait FileOpener: Send + Sync {

    fn list_apps_associated_with_ext(_file_path: impl AsRef<Path>) -> Vec<App> {
        vec![]
    }

    fn open(&self, source_file: &Path, target_dir: &Path) -> Result<()>;
}