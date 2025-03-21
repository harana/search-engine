use jpeg2k::*;
use harana_common::anyhow::Result;
use harana_common::image::DynamicImage;

pub fn decode_jpeg2k(data: &[u8]) -> Result<Vec<u8>> {
    let jp2_image = Image::from_bytes(data)?;
    let img: DynamicImage = (&jp2_image).try_into()?;
    Ok(img.into_bytes())
}