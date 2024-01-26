use std::path::Path;

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerNoop;

#[async_trait]
impl Thumbnailer for ThumbnailerNoop {

    async fn thumbnail(&self, _source_file: &Path, _target_dir: &Path, _document_id: u64, _app: &'static AppHandle<Wry>, _width: u32, _height: u32) -> Result<()> {
        Ok(())
    }

    fn should_auto_complete(&self) -> bool {
        true
    }
}