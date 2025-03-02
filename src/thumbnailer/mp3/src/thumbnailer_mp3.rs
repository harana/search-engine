use std::path::Path;

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::image::{Image, ImageType};
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerMp3;

#[async_trait]
impl Thumbnailer for ThumbnailerMp3 {

    async fn thumbnail(&self, source_file: &Path, target_file: &Path, _document_id: u64, _app: &'static AppHandle<Wry>, width: u32, height: u32) -> Result<()> {
        let tag = id3::Tag::read_from_path(source_file)?;
        if let Some(p) = tag.pictures().next() {
            let image_type = match p.mime_type.as_str() {
                "image/bmp"     => ImageType::Bmp,
                "image/jpeg"    => ImageType::Jpeg,
                "image/png"     => ImageType::Png,
                _               => ImageType::Binary
            };

            let image = Image::new(p.clone().data, true, image_type, None);
            image?.crop(width, height, target_file, false)?;
        }
        Ok(())
    }

    fn should_auto_complete(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use harana_common::tokio;
    use harana_thumbnailer_core::thumbnailer::Thumbnailer;
    use harana_common::time::Instant;
    use crate::thumbnailer_mp3::ThumbnailerMp3;

    #[tokio::test]
    async fn test_indexing() {
        let now = Instant::now();

        let _ = ThumbnailerMp3.thumbnail(
            Path::new("../../../test_files/Sample1.mp3"),
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