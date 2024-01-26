use std::path::Path;

use resvg::tiny_skia;
use resvg::tiny_skia::Transform;
use resvg::usvg::{fontdb, Options, Tree, TreeParsing, TreeTextToPath};

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::tauri::{AppHandle, Wry};
use harana_common::tokio::fs;
use harana_thumbnailer_core::image::{Image, ImageType};
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerSvg;

#[async_trait]
impl Thumbnailer for ThumbnailerSvg {

    async fn thumbnail(&self, source_file: &Path, target_file: &Path, _document_id: u64, _app: &'static AppHandle<Wry>, width: u32, height: u32) -> Result<()> {
        let buf = fs::read(source_file).await.expect("read in");
        let rtree = Tree::from_data(&buf.as_slice(), &Options::default()).map(|mut tree| {
            let mut fontdb = fontdb::Database::new();
            fontdb.load_system_fonts();
            tree.convert_text(&fontdb);
            resvg::Tree::from_usvg(&tree)
        }).unwrap();

        let size = if rtree.size.width() > rtree.size.height() {
            rtree.size.to_int_size().scale_to_width(width)
        } else {
            rtree.size.to_int_size().scale_to_height(height)
        }.unwrap();

        let transform = Transform::from_scale(
            size.width() as f32 / rtree.size.width(),
            size.height() as f32 / rtree.size.height(),
        );

        let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height()).unwrap();
        rtree.render(transform, &mut pixmap.as_mut());

        let image = Image::new(pixmap.data().to_vec(), true, ImageType::Binary, Some((pixmap.width(), pixmap.height())));
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
    use crate::thumbnailer_svg::ThumbnailerSvg;

    #[tokio::test]
    async fn test_indexing() {
        let now = Instant::now();

        let _ = ThumbnailerSvg.thumbnail(
            Path::new("../../../test_files/Sample1.svg"),
            Path::new("/Users/naden/Desktop/Sample1.png"),
            0,
            None,
            300,
            300
        ).await;

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}