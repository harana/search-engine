use std::path::Path;

use harana_common::anyhow::Result;
use harana_common::tokio::fs::read;

use crate::image::{Image, ImageType};

pub struct ImageDecoder;

impl ImageDecoder {
    pub async fn standard(source_file: &Path, target_file: &Path, image_type: ImageType, alpha: bool, source_size: Option<(u32, u32)>, target_size: (u32, u32)) -> Result<()> {
        let pixels = read(source_file).await?;
        let image = Image::new(pixels, alpha, image_type, source_size);
        image?.crop(target_size.0, target_size.1, target_file, false)?;
        Ok(())
    }
}