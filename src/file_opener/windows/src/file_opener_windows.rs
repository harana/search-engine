use std::path::Path;
use harana_common::anyhow::Result;
use opener_common::file_opener::FileOpener;

pub struct FileOpenerWindows;

impl FileOpener for FileOpenerWindows {

    fn open(&self, source_file: &Path, target_dir: &Path) -> Result<()> {
        Ok(())
    }
}