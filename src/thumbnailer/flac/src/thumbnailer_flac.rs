use std::path::Path;

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerFlac;

#[async_trait]
impl Thumbnailer for ThumbnailerFlac {

    async fn thumbnail(&self, source_file: &Path, target_file: &Path, _document_id: u64, _app: &'static AppHandle<Wry>, width: u32, height: u32) -> Result<()> {
        Ok(())
    }

    fn should_auto_complete(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use harana_common::time::Instant;
    use harana_common::tokio;
    use harana_thumbnailer_core::thumbnailer::Thumbnailer;
    use crate::thumbnailer_flac::ThumbnailerFlac;

    #[tokio::test]
    async fn test_indexing() {
        let now = Instant::now();

        let _ = ThumbnailerFlac.thumbnail(
            Path::new("../../../test_files/Sample1.flac"),
            Path::new("/Users/naden/Desktop/Sample1.png"),
            0,
            None,
            400,
            400
        ).await;

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}