use std::fs;
use std::path::Path;

use image::EncodableLayout;
use libraw::Reader;

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::itertools::Itertools;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::image::{Image, ImageType};
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerRaw;

#[async_trait]
impl Thumbnailer for ThumbnailerRaw {

    async fn thumbnail(&self, source_file: &Path, target_file: &Path, _document_id: u64, _app: &'static AppHandle<Wry>, width: u32, height: u32) -> Result<()> {
        let buf = fs::read(source_file).expect("read in");
        let reader = Reader::new();
        let raw_image = reader.read_8bit(&buf).ok();
        if raw_image.is_some() {
            let ri = raw_image.unwrap();
            let image = Image::new(ri.to_vec(), false, ImageType::Binary, Some((ri.width(), ri.height())));
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

    use crate::thumbnailer_raw::ThumbnailerRaw;

    #[tokio::test]
    async fn test_indexing() {
        let now = Instant::now();

        let _ = ThumbnailerRaw.thumbnail(
            Path::new("../../../test_files/Sample1.cr2"),
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