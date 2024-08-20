use std::path::Path;
use harana_common::anyhow::Result;
use harana_file_opener_common::file_opener::FileOpener;

pub struct OpenerMac;

impl FileOpener for OpenerMac {

    fn open(&self, source_file: &Path, target_dir: &Path) -> Result<()> {
        Ok(())
    }
}