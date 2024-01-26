use std::path::Path;

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::tauri::open_tauri_thumbnail_window;
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerMd;

#[async_trait]
impl Thumbnailer for ThumbnailerMd {

    async fn thumbnail(&self, _source_file: &Path, _target_file: &Path, document_id: u64, app: &'static AppHandle<Wry>, _width: u32, _height: u32) -> Result<()> {
        open_tauri_thumbnail_window(app, document_id)
    }

    fn should_auto_complete(&self) -> bool {
        false
    }
}