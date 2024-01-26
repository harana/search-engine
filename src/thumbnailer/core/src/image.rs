use std::io::{BufWriter, Write};
use std::io::Cursor;
use std::num::NonZeroU32;
use std::path::Path;

use fast_image_resize as fir;
use fast_image_resize::{FilterType, Image as FirImage, ResizeAlg};
use fast_image_resize::PixelType::{U8x3, U8x4};
use image::{ImageEncoder, ImageFormat};
use image::codecs::png::PngEncoder;
use image::ColorType::{Rgb8, Rgba8};
use image::io::Reader as ImageReader;
use zune_image::codecs::bmp::BmpDecoder;
use zune_image::codecs::jpeg::JpegDecoder;
use zune_image::codecs::png::PngDecoder;
use zune_image::codecs::ppm::PPMDecoder;
use zune_image::codecs::psd::PSDDecoder;
use zune_image::codecs::qoi::QoiDecoder;

use harana_common::anyhow::{anyhow, Context, Result};

pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub struct Image {
    pub src_image: fir::Image<'static>,
    pub src_width: u32,
    pub src_height: u32,
}

pub enum ImageType {
    Avif, Binary, Bmp, Dds, Farbfeld, Gif, Hdr, Ico, Jpeg,
    OpenExr, Png, Pnm, Ppm, Psd, Qoi, Tga, Tiff, Webp
}

impl Image {

    pub fn new(bytes: Vec<u8>, alpha: bool, mime_type: ImageType, size: Option<(u32 ,u32)>) -> Result<Self> {
        let decoded_bytes = match mime_type {
            ImageType::Avif         => Image::decode_image(bytes.clone(), ImageFormat::Avif)?,
            ImageType::Binary       => bytes.clone(),
            ImageType::Bmp         => Image::decode_image(bytes.clone(), ImageFormat::Bmp)?,
            ImageType::Dds          => Image::decode_image(bytes.clone(), ImageFormat::Dds)?,
            // ImageType::Farbfeld     => FarbFeldDecoder::new(bytes.clone()).decode().unwrap(),
            ImageType::Gif          => Image::decode_image(bytes.clone(), ImageFormat::Gif)?,
            // ImageType::Hdr          => HdrDecoder::new(bytes.clone()).decode().unwrap(),
            ImageType::Ico          => Image::decode_image(bytes.clone(), ImageFormat::Ico)?,
            ImageType::Jpeg         => Image::decode_image(bytes.clone(), ImageFormat::Jpeg)?,
            ImageType::OpenExr      => Image::decode_image(bytes.clone(), ImageFormat::OpenExr)?,
            ImageType::Png          => Image::decode_image(bytes.clone(), ImageFormat::Png)?,
            ImageType::Pnm          => Image::decode_image(bytes.clone(), ImageFormat::Pnm)?,
            ImageType::Ppm          => PPMDecoder::new(bytes.clone()).decode().map_err(|e| anyhow!("{:?}", e))?.u8().context("Not u8")?,
            ImageType::Psd          => PSDDecoder::new(bytes.clone()).decode().map_err(|e| anyhow!("{:?}", e))?.u8().context("Not u8")?,
            ImageType::Qoi          => QoiDecoder::new(bytes.clone()).decode().map_err(|e| anyhow!("{:?}", e))?,
            ImageType::Tga          => Image::decode_image(bytes.clone(), ImageFormat::Tga)?,
            ImageType::Tiff         => Image::decode_image(bytes.clone(), ImageFormat::Tiff)?,
            ImageType::Webp         => Image::decode_image(bytes.clone(), ImageFormat::WebP)?,
            _                       => bytes.clone()
        };

        let calculated_size = match size {
            Some(d) => (d.0, d.1),
            None => Image::detect_size(mime_type, bytes)?
        };

        let fir_image = FirImage::from_vec_u8(
            NonZeroU32::try_from(calculated_size.0)?,
            NonZeroU32::try_from(calculated_size.1)?,
            decoded_bytes,
            if alpha { U8x4 } else { U8x3 }
        );

        if fir_image.is_ok() {
            Ok(Image {
                src_image: fir_image?,
                src_width: calculated_size.0,
                src_height: calculated_size.1
            })
        }else{
            Err(anyhow!("Failed to generate image due to: {}", fir_image.unwrap_err().to_string()))
        }
    }

    pub fn crop(&self, width: u32, height: u32, output_file: &Path, white_crop: bool) -> Result<()> {
        let mut resizer = fir::Resizer::new(ResizeAlg::Convolution(FilterType::Lanczos3));
        let width = NonZeroU32::new(width).context("Width is not > 0")?;
        let height = NonZeroU32::new(height).context("Height is not > 0")?;
        let mut src_view = self.src_image.view();
        src_view.set_crop_box_to_fit_dst_size(width, height, Some((0.5, 0.5)));
        let mut dst_image = FirImage::new(width, height, self.src_image.pixel_type());
        let mut dst_view = dst_image.view_mut();
        resizer.resize(&src_view, &mut dst_view)?;

        self.internal_save(dst_image.buffer(), width.get(), height.get(), output_file)?;

        Ok(())
    }

    pub fn save(&self, output_file: &Path, width: u32, height: u32) -> Result<()> {
        self.internal_save(self.src_image.buffer(), width, height, output_file)
    }

    fn internal_save(&self, data: &[u8], width: u32, height: u32, output_file: &Path) -> Result<()> {
        let result_file = std::fs::File::create(output_file)?;
        let mut result_buf = BufWriter::new(result_file);
        let color_type = if self.src_image.pixel_type() == U8x3 { Rgb8 } else { Rgba8 };
        PngEncoder::new(&mut result_buf).write_image(data, width, height, color_type)?;
        result_buf.flush()?;
        Ok(())
    }

    fn decode_image(bytes: Vec<u8>, format: ImageFormat) -> Result<Vec<u8>> {
        let cursor = Cursor::new(bytes.clone());
        let image= ImageReader::with_format(cursor, format).decode()?;
        Ok(image.into_bytes())
    }

    fn detect_size(mime_type: ImageType, bytes: Vec<u8>) -> Result<(u32, u32)> {
        let cursor = Cursor::new(bytes.clone());

        match mime_type {
            ImageType::Psd => PSDDecoder::new(bytes.clone()).get_dimensions().context("No dimensions for PSD").map(|d| (d.0 as u32, d.1 as u32)),
            _ => {
                let img = ImageReader::new(cursor).with_guessed_format()?.decode()?;
                Ok((img.width(), img.height()))
            }
        }
    }

        // pub fn crop(src_bytes: Vec<u8>,
    //             src_width: u32,
    //             src_height: u32,
    //             src_filename: String,
    //             white_crop: bool,
    //             target_path: &Path,
    //             target_width: u32,
    //             target_height: u32) -> Result<()> {
    //
    //     let this = Image::new(src_bytes, src_width, src_height, src_filename)?;
    //     // this.original.save(output_path.join(format!("{}_orig.png", file_name.clone())))?;
    //
    //     // White Crop
    //     let src_image = if white_crop {
    //         this.white_crop(thumbnail_width, thumbnail_height)?;
    //     } else {
    //         FirImage::from_vec_u8(NonZeroU32::new(src_width).unwrap(), NonZeroU32::new(src_height).unwrap(),
    //             white_cropped_dynamic.to_rgba8().into_raw(),
    //             fr::PixelType::U8x4
    //         )?;
    //     }
    //
    //     // Resize
    //     let resized_size = this.proportional_scale((white_crop.1.0, white_crop.1.1), (thumbnail_width, thumbnail_height));
    //     let mut resized_image = Image::resize_fir(white_crop.0, resized_size.0, resized_size.1)?;
    //
    //     // Crop
    //     let cropped_image = resized_image?.crop(0, 0, thumbnail_width, thumbnail_height);
    //
    //     // Write destination image as PNG-file
    //     Image::save(output_path, file_name, cropped_image)
    // }
    //

    // fn white_crop(&self, _width: u32, _height: u32) -> Result<(FirImage, (u32, u32))> {
    //     let (top_left_corner, bottom_right_corner) = self.calculate_corners();
    //
    //     let top_left_x = top_left_corner.x;
    //     let top_left_y = top_left_corner.y;
    //     let bottom_right_x = bottom_right_corner.x;
    //     let bottom_right_y = bottom_right_corner.y;
    //
        // println!("Original Size: ({} x {})", self.original.width(), self.original.height());
        // println!("----------------------------------------------------------------------------");
        //
        // let mut white_cropped_width = bottom_right_x - top_left_x;
        // let mut white_cropped_height = bottom_right_y - top_left_y;
        //
        // println!("White Coordinates: ({} x {}) -> ({} x {})", top_left_x, top_left_y, bottom_right_x, bottom_right_y);
        // println!("White Cropped Size: ({} x {})", white_cropped_width, white_cropped_height);
        // println!("Minimum Size: ({} x {})", width, height);
        // println!("----------------------------------------------------------------------------");
        //
        // if white_cropped_width > width {
        //     let mut diff = (white_cropped_width - width) as f32;
        //     let left_diff = top_left_x as f32;
        //     let right_diff = (self.original.width() - bottom_right_x) as f32;
        //     println!("Width Diff: {} ---- Left: {} ---- Right: {}", diff, left_diff, right_diff);
        //
        //     top_left_x -= (diff * left_diff / (left_diff + right_diff)) as u32;
        //     bottom_right_x += (diff * right_diff / (left_diff + right_diff)) as u32;
        // }
        //
        // if white_cropped_height > height {
        //     let mut diff = (white_cropped_height - height) as f32;
        //     let top_diff = top_left_y as f32;
        //     let bottom_diff = (self.original.height() - bottom_right_y) as f32;
        //
        //     let top_adjust = (diff * top_diff / (top_diff + bottom_diff)) as u32;
        //     let bottom_adjust = (diff * bottom_diff / (top_diff + bottom_diff)) as u32;
        //
        //     println!("Height Diff: {} ---- Top: {} ---- Bottom: {} --- Top Adjust: {} ----- Bottom Adjust: {}", diff, top_diff, bottom_diff, top_adjust, bottom_adjust);
        //
        //     top_left_y -= (diff * top_diff / (top_diff + bottom_diff)) as u32;
        //     bottom_right_y += (diff * bottom_diff / (top_diff + bottom_diff)) as u32;
        // }

    //     let white_cropped_width = bottom_right_x - top_left_x;
    //     let white_cropped_height = bottom_right_y - top_left_y;
    //
    //     println!("Cropping: ({} x {}) @ ({} x {})", top_left_x, top_left_y, white_cropped_width, white_cropped_height);
    //     let white_cropped_dynamic = self.original.clone().crop(top_left_x, top_left_y, white_cropped_width, white_cropped_height);
    //     let white_cropped_image = FirImage::from_vec_u8(
    //         NonZeroU32::new(white_cropped_width).unwrap(),
    //         NonZeroU32::new(white_cropped_height).unwrap(),
    //         white_cropped_dynamic.to_rgba8().into_raw(),
    //         fr::PixelType::U8x4
    //     )?;
    //
    //     Ok((white_cropped_image, (white_cropped_width, white_cropped_height)))
    // }
    //
    // fn calculate_corners(&self) -> (Point, Point) {
    //     (self.top_left_corner(), self.bottom_right_corner())
    // }
    //
    // fn is_white(pixel: Rgba<u8>) -> bool {
    //     pixel[0] != 255 &&
    //         pixel[1] != 255 &&
    //         pixel[2] != 255
    // }
    //
    // fn top_left_corner(&self) -> Point {
    //     Point {
    //         x: self.top_left_corner_x(),
    //         y: self.top_left_corner_y(),
    //     }
    // }
    //
    // fn top_left_corner_x(&self) -> u32 {
    //     for x in 0..(self.original.width()) {
    //         for y in 0..(self.original.height()) {
    //             let pixel = self.original.get_pixel(Vec2:;from(x as usize, y as usize));
    //             if Self::is_white(pixel) {
    //                 return x;
    //             }
    //         }
    //     }
    //     unreachable!();
    // }
    //
    // fn top_left_corner_y(&self) -> u32 {
    //     for y in 0..(self.src_image.height()) {
    //         for x in 0..(self.src_image.width()) {
    //             let pixel = self.src_image.get_pixel(x, y);
    //             if Self::is_white(pixel) {
    //                 return y;
    //             }
    //         }
    //     }
    //     unreachable!();
    // }
    //
    // fn bottom_right_corner(&self) -> Point {
    //     Point {
    //         x: self.bottom_right_corner_x(),
    //         y: self.bottom_right_corner_y(),
    //     }
    // }
    //
    // fn bottom_right_corner_x(&self) -> u32 {
    //     let mut x = self.original.width() as i32 - 1;
    //     // Using while loop as currently there is no reliable built-in
    //     // way to use custom negative steps when specifying range
    //     while x >= 0 {
    //         let mut y = self.original.height() as i32 - 1;
    //         while y >= 0 {
    //             let pixel = self.original.get_pixel(x as u32, y as u32);
    //             if Self::is_white(pixel) {
    //                 return x as u32 + 1;
    //             }
    //             y -= 1;
    //         }
    //         x -= 1;
    //     }
    //     unreachable!();
    // }
    //
    // fn bottom_right_corner_y(&self) -> u32 {
    //     let mut y = self.original.height() as i32 - 1;
    //     // Using while loop as currently there is no reliable built-in
    //     // way to use custom negative steps when specifying range
    //     while y >= 0 {
    //         let mut x = self.original.width() as i32 - 1;
    //         while x >= 0 {
    //             let pixel = self.original.get_pixel(x as u32, y as u32);
    //             if Self::is_white(pixel) {
    //                 return y as u32 + 1;
    //             }
    //             x -= 1;
    //         }
    //         y -= 1;
    //     }
    //     unreachable!();
    // }

}