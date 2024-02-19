use std::panic::RefUnwindSafe;
use std::path::Path;

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::tauri::{AppHandle, Wry};

#[async_trait]
pub trait Thumbnailer: Send + Sync + RefUnwindSafe {

    async fn thumbnail(&self,
                 source_file: &Path,
                 target_file: &Path,
                 document_id: u64,
                 app: &'static AppHandle<Wry>,
                 width: u32,
                 height: u32) -> Result<()>;

    fn should_auto_complete(&self) -> bool;

    fn get_name(&self) -> &'static str {
        std::any::type_name::<Self>().split("::").last().unwrap_or_default()
    }
}