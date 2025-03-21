use harana_common::anyhow::Result;
use harana_common::image::DynamicImage;
use okolors::Okolors;
use palette::Srgb;

pub fn colors(img: DynamicImage, color_count: u8) -> Result<Vec<Srgb<u8>>> {
    Ok(
        Okolors::try_from(&img.to_rgb8())?
        .palette_size(color_count)
        .lightness_weight(0.5)
        .sampling_factor(0.25)
        .parallel(true)
        .sort_by_frequency(true)
        .srgb8_palette()
    )
}