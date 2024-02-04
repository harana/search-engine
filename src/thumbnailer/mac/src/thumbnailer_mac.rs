use std::path::Path;

use base64_simd::Base64;

#[cfg(target_os = "macos")]
use swift_rs::{SRString, swift};

use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::lazy_static::lazy_static;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::image::{Image, ImageType};
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerMac;

lazy_static! {
	pub static ref BASE64: Base64 = base64_simd::STANDARD;
}

swift!(
	pub fn file_thumbnail(source_path: &SRString) -> SRString
);

swift!(
	pub fn quicklook_preview(source_path: &SRString) -> SRString
);

#[async_trait]
impl Thumbnailer for ThumbnailerMac {

    async fn thumbnail(&self, source_file: &Path, target_file: &Path, _document_id: u64, _app: &'static AppHandle<Wry>, width: u32, height: u32) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            let source_path = SRString::from(source_file.to_str().unwrap());
            let preview_base64 = unsafe { quicklook_preview(&source_path) };
            let decoded = BASE64.decode_to_vec(preview_base64)?;

            let image = Image::new(decoded, false, ImageType::Tiff, None);
            image?.crop(width, height, target_file, false)
        }

        #[cfg(not(target_os = "macos"))]
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
    use crate::thumbnailer_mac::ThumbnailerMac;

    #[tokio::test]
    async fn test_indexing() {
        let now = Instant::now();

        let _ = ThumbnailerMac.thumbnail(
            Path::new("../../../test_files/Sample1.key"),
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