use std::path::Path;

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::image::{Image, ImageType};
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerM4a;

#[async_trait]
impl Thumbnailer for ThumbnailerM4a {

    async fn thumbnail(&self, source_file: &Path, target_file: &Path, _document_id: u64, _app: &'static AppHandle<Wry>, width: u32, height: u32) -> Result<()> {
        let tag = mp4ameta::Tag::read_from_path(source_file)?;
        if let Some(a) = tag.artworks().next() {
            let image_type =
                if a.fmt.is_bmp() {
                    ImageType::Bmp
                }else{
                    if a.fmt.is_jpeg() {
                        ImageType::Jpeg
                    }else{
                        ImageType::Png
                    }
                };
            let image = Image::new(a.data.to_vec(), true, image_type, None);
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
    use harana_common::time::Instant;
    use harana_common::tokio;
    use harana_thumbnailer_core::thumbnailer::Thumbnailer;
    use crate::thumbnailer_m4a::ThumbnailerM4a;

    #[tokio::test]
    async fn test_indexing() {
        let now = Instant::now();

        let _ = ThumbnailerM4a.thumbnail(
            Path::new("../../../test_files/Sample1.m4a"),
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