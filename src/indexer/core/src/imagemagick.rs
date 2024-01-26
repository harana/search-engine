use std::path::Path;
use std::sync::Once;

use magick_rust::{magick_wand_genesis, MagickError, MagickWand};
use magick_rust::bindings::{ColorspaceType_GRAYColorspace, ImageType_GrayscaleType};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};

static START: Once = Once::new();

#[derive(Clone, Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct ImagemagickMetadata {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub shot_date: Option<String>,
    pub orientation: String,
    pub make: Option<String>,
    pub model: Option<String>,
    pub exposure_time: Option<f32>,
    pub f_number: Option<u32>,
    pub focal_length: Option<u32>,
    pub iso: Option<u32>,
    pub artist: Option<String>,
    pub gps_altitude: Option<(u32, u32)>,
    pub gps_latitude: Option<(u32, u32)>,
    pub gps_longitude: Option<(u32, u32)>,
    pub location_name: Option<String>,
    pub location_admin1: Option<String>,
    pub location_admin2: Option<String>,
    pub location_country_code: Option<String>,
    pub location_country: Option<String>,
    pub histogram: Vec<f64>
}

pub fn ocr_ready_image(indexable_file: &Path) -> Result<Vec<u8>>{
    START.call_once(|| { magick_wand_genesis(); });
    let mut wand = MagickWand::new();
    wand.read_image(indexable_file.to_str().unwrap())?;
    wand.set_colorspace(ColorspaceType_GRAYColorspace)?;
    wand.set_image_type(ImageType_GrayscaleType)?;
    wand.set_image_resolution(640.0, 480.0)?;
    wand.auto_level()?;
    Ok(wand.write_image_blob("tiff")?)
}

pub fn to_tiff(data: Vec<u8>) -> Result<Vec<u8>> {
    START.call_once(|| { magick_wand_genesis(); });
    let wand = MagickWand::new();
    wand.read_image_blob(data)?;
    Ok(wand.write_image_blob("tiff")?)
}

pub fn metadata(indexable_file: &Path) -> Result<ImagemagickMetadata> {
    START.call_once(|| { magick_wand_genesis(); });
    let wand = MagickWand::new();
    wand.read_image(indexable_file.to_str().unwrap())?;

    let width = wand.get_image_width();
    let height = wand.get_image_height();
    let depth = wand.get_image_depth() as usize;
    let orientation = wand.get_orientation().to_string();

    let make = wand.get_image_property("dng:make").or(wand.get_image_property("exif:LensMake")).ok();
    let model = wand.get_image_property("dng:camera.model.name").or(wand.get_image_property("exif:Model")).ok();
    let _exposure_time = wand.get_image_property("dng:exposure.time").or(wand.get_image_property("exif:ExposureTime")).ok();
    let _f_number = wand.get_image_property("dng:f.number").or(wand.get_image_property("exif:FNumber")).ok();
    let _gps_altitude = wand.get_image_property("dng:gps.altitude").or(wand.get_image_property("exif:GPSAltitude")).ok();

    let _gps_latitude = wand.get_image_property("dng:gps.latitude").or(wand.get_image_property("exif:GPSLatitude")).ok();
    let _gps_longitude = wand.get_image_property("dng:gps.longitude").or(wand.get_image_property("exif:GPSLongitude")).ok();
    let _gps_latitude_ref = wand.get_image_property("exif:GPSLatitudeRef").ok();
    let _gps_longitude_ref = wand.get_image_property("exif:GPSLongitudeRef").ok();

    let shot_date = wand.get_image_property("dng:create.date")
        .map(|date| {
            date
        })
        .or(wand.get_image_property("exif:DateTime").map(|date| {
            date
        }))
        .ok();

    let focal_length = wand.get_image_property("dng:focal.length.in.35mm.format")
        .and_then(|fl| Decimal::from_scientific(fl.as_str()).map_err(|_e| MagickError::from("")))
        .and_then(|fl| fl.to_u32().ok_or(MagickError::from("")))
        .or(to_u32(&wand, "exif:FocalLengthIn35mmFilm:"))
        .ok();

    let iso = wand.get_image_property("dng:iso.setting")
        .and_then(|iso| Decimal::from_scientific(iso.as_str()).map_err(|_e| MagickError::from("")))
        .and_then(|iso| iso.to_u32().ok_or(MagickError::from("")))
        .or(to_u32(&wand, "exif:PhotographicSensitivity"))
        .ok();

    let artist = wand.get_image_property("exif:Artist").ok();

    let mut histogram = Vec::new();
    match wand.get_image_histogram() {
        Some(pw) =>
            pw.chunks(4).for_each(|quad| {
                if quad.len() > 0 { histogram.push((quad[0].get_red() * 0.2126) + (quad[0].get_green() * 0.7152) + (quad[0].get_blue() * 0.0722)); }
                if quad.len() > 1 { histogram.push((quad[1].get_red() * 0.2126) + (quad[1].get_green() * 0.7152) + (quad[1].get_blue() * 0.0722)); }
                if quad.len() > 2 { histogram.push((quad[2].get_red() * 0.2126) + (quad[2].get_green() * 0.7152) + (quad[2].get_blue() * 0.0722)); }
                if quad.len() > 3 { histogram.push((quad[3].get_red() * 0.2126) + (quad[3].get_green() * 0.7152) + (quad[3].get_blue() * 0.0722)); }
            }),
        None => {},
    };

    Ok(
        ImagemagickMetadata {
            width,
            height,
            depth,
            shot_date,
            orientation,
            make,
            model,
            exposure_time: Some(0.0),
            f_number: Some(0),
            focal_length,
            iso,
            artist,
            gps_altitude: Some((0, 0)),
            gps_latitude: Some((0, 0)),
            gps_longitude: Some((0, 0)),
            location_name: None,
            location_admin1: None,
            location_admin2: None,
            location_country_code: None,
            location_country: None,
            histogram: downsample(histogram, 100)
        }
    )
}

fn to_u32(wand: &MagickWand, property: &str) -> Result<u32, MagickError> {
    wand.get_image_property(property)
        .and_then(|v|
            v.parse::<u32>().map_err(|_e| MagickError::from(""))
        )
}

fn downsample(items: Vec<f64>, desired_size: usize) -> Vec<f64> {
    let mut pos = 0;
    let mut res = Vec::with_capacity(desired_size);
    let step = items.len() / desired_size;

    for _ in 0..desired_size {
        res.push(items[pos]);
        pos = (pos + step) % desired_size;
    }
    res
}