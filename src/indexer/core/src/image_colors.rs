use harana_common::anyhow::Result;
use image::DynamicImage;
use okolors::OklabCounts;
use palette::convert::FromColorUnclamped;
use palette::{Clamp, LinSrgb, Okhsv};
use harana_common::itertools::Itertools;

pub fn colors(img: DynamicImage, color_count: u8) -> Result<Vec<(f32, f32, f32)>> {
    let oklab = OklabCounts::try_from_image(&img, u8::MAX)?.with_lightness_weight(0.325);
    let result = okolors::run(&oklab, 1, color_count, 0.05, 64, 0);
    Ok(
        result.centroids.into_iter().map(|oklab| {
            let okhsv: Okhsv = Okhsv::from_color_unclamped(oklab);
            let clamped_okhsv = okhsv.clamp();
            let linsrgb = LinSrgb::from_color_unclamped(clamped_okhsv);
            (linsrgb.red, linsrgb.green, linsrgb.blue)
        }).collect_vec()
    )
} 