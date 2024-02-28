use std::io::Seek;
use std::path::Path;

use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::image::{Image, ImageType};
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerHeif;

#[async_trait]
impl Thumbnailer for ThumbnailerHeif {

    async fn thumbnail(&self, source_file: &Path, target_file: &Path, _document_id: u64, _app: &'static AppHandle<Wry>, width: u32, height: u32) -> Result<()> {
        let lib_heif = LibHeif::new();
        let ctx = HeifContext::read_from_file(source_file.to_str().unwrap())?;
        let handle = ctx.primary_image_handle()?;
        let src_image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgba), None)?;

        let planes = src_image.planes();
        let interleaved = planes.interleaved.unwrap();

        let src_bytes = interleaved.data;
        let src_width = interleaved.width;
        let src_height = interleaved.height;
        let src_stride = interleaved.stride;

        // rgba
        let mut res: Vec<u8> = Vec::new();
        for y in 0..src_height {
            let mut step = y as usize * src_stride;

            for _ in 0..src_width {
                res.extend_from_slice(&[
                    src_bytes[step],
                    src_bytes[step + 1],
                    src_bytes[step + 2],
                    src_bytes[step + 3],
                ]);
                step += 4;
            }
        }

        let image = Image::new(src_bytes.to_vec(), true, ImageType::Binary, Some((src_image.width(), src_image.height())));
        image?.crop(width, height, target_file, false)?;
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
    use crate::thumbnailer_heif::ThumbnailerHeif;

    #[tokio::test]
    async fn test_indexing() {
        let now = Instant::now();

        let _ = ThumbnailerHeif.thumbnail(
            Path::new("../../../test_files/Sample1.heic"),
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