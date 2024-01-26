use std::path::Path;
use std::sync::Arc;
use harana_common::crossbeam_skiplist::SkipSet;
use harana_common::log::info;
use harana_common::tauri::{AppHandle, Manager, Wry};

#[derive(Clone)]
pub struct IndexThumbnailer {
    pub thumbnails_path: &'static Path,
    pub thumbnail_width: u32,
    pub thumbnail_height: u32
}

impl IndexThumbnailer {

    pub async fn new(thumbnails_path: &'static Path, thumbnail_width: u32, thumbnail_height: u32) -> Self {
        Self {
            thumbnails_path,
            thumbnail_width,
            thumbnail_height
        }
    }
}