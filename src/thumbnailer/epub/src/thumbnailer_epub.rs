use std::path::Path;

use epub::doc::EpubDoc;

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::image::{Image, ImageType};
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerEpub;

#[async_trait]
impl Thumbnailer for ThumbnailerEpub {

    async fn thumbnail(&self, source_file: &Path, target_file: &Path, _document_id: u64, app: &'static AppHandle<Wry>, width: u32, height: u32) -> Result<()> {
        let mut doc = EpubDoc::new(source_file)?;

        let epub_resources = doc.resources.clone();
        let cover = epub_resources
            .iter()
            .find(|(k, (v1, v2))| k.contains("cover") && v2.starts_with("image/"))
            .and_then(|(k, (v1, v2))| doc.get_resource(k));

        if cover.is_some() {
            let image = Image::new(cover.unwrap().0, true, ImageType::Png, None);
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
    use crate::thumbnailer_epub::ThumbnailerEpub;

    #[tokio::test]
    async fn test_indexing() {
        let now = Instant::now();

        let _ = ThumbnailerEpub.thumbnail(
            Path::new("../../../test_files/Sample1.epub"),
            Path::new("/Users/naden/Desktop/Sample1.png"),
            0,
            None,
            600,
            600
        ).await;

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}