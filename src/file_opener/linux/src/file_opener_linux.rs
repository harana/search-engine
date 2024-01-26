use std::path::Path;
use harana_common::anyhow::Result;

use opener_common::opener::Opener;

pub struct FileOpenerLinux;

impl FileOpener for FileOpenerLinux {

    fn open(&self, source_file: &Path, target_dir: &Path) -> Result<()> {
        Ok(())
    }
}