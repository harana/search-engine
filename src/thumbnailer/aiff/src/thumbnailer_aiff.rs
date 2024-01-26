use std::path::Path;

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::tantivy::HasLen;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::image::{Image, ImageType};
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerAiff;

#[async_trait]
impl Thumbnailer for ThumbnailerAiff {

    async fn thumbnail(&self, source_file: &Path, target_file: &Path, _document_id: u64, _app: &'static AppHandle<Wry>, width: u32, height: u32) -> Result<()> {
        let tag = id3::Tag::read_from_aiff_path(source_file)?;
        let picture = tag.pictures().next();
        if picture.is_some() {
            let image = Image::new(picture.unwrap().clone().data, false, ImageType::Png, None);
            image?.crop(width, height, target_file, false)?
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
    use std::time::Instant;
    use crate::thumbnailer_aiff::ThumbnailerAiff;

    #[tokio::test]
    async fn test_indexing() {
        let now = Instant::now();

        let _ = ThumbnailerAiff.thumbnail(
            Path::new("../../../test_files/Sample1.aiff"),
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