use std::path::Path;

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::image::ImageType;
use harana_thumbnailer_core::image_decoder::ImageDecoder;
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerTga;

#[async_trait]
impl Thumbnailer for ThumbnailerTga {

    async fn thumbnail(&self, source_file: &Path, target_file: &Path, _document_id: u64, _app: &'static AppHandle<Wry>, width: u32, height: u32) -> Result<()> {
        ImageDecoder::standard(source_file, target_file, ImageType::Tga, false, None, (width, height)).await
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
    use crate::thumbnailer_tga::ThumbnailerTga;

    #[tokio::test]
    async fn test_indexing() {
        let now = Instant::now();

        let _ = ThumbnailerTga.thumbnail(
            Path::new("../../../test_files/Sample1.tga"),
            Path::new("../../../test_files/output/Sample1.png"),
            0,
            None,
            400,
            400
        ).await;

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}