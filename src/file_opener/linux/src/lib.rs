mod app_info;
mod env;

use harana_common::anyhow::Result;

use std::path::Path;
use opener_common::file_opener::FileOpener;
use crate::app_info::App;

pub struct FileOpenerLinux;

impl FileOpener for FileOpenerLinux {

    async fn list_apps_associated_with_ext(file_path: impl AsRef<Path>) -> Vec<App> {
        app_info::list_apps_associated_with_ext(file_path)
    }

    fn open(&self, source_file: &Path, target_dir: &Path) -> Result<()> {
        todo!()
    }
}
