use std::{
    fs::File,
    io::prelude::*,
    ops::Deref,
    slice,
};
use std::ffi::{CStr, CString};
use std::path::Path;

use ffi::{AVCodecID_AV_CODEC_ID_MJPEG, AVMediaType_AVMEDIA_TYPE_VIDEO, SWS_FAST_BILINEAR, SWS_PRINT_INFO};
use rsmpeg::{avcodec::*, avformat::*, avutil::*, error::RsmpegError, ffi, swscale::*};

use harana_common::anyhow::{Context, Result};
use harana_common::async_trait::async_trait;
use harana_common::tauri::{AppHandle, Wry};
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

pub struct ThumbnailerVideo;

#[async_trait]
impl Thumbnailer for ThumbnailerVideo {

    async fn thumbnail(&self, source_file: &Path, target_file: &Path, _document_id: u64, _app: &'static AppHandle<Wry>, width: u32, height: u32) -> Result<()> {
        let c_string = CString::new(source_file.to_str().unwrap()).expect("Failed to create CString");
        let c_str_ptr = c_string.as_ptr();
        let c_str = unsafe { CStr::from_ptr(c_str_ptr) };

        let mut input_format_context = AVFormatContextInput::open(c_str)?;

        let (video_stream_index, mut decode_context) = {
            let (stream_index, decoder) = input_format_context
                .find_best_stream(AVMediaType_AVMEDIA_TYPE_VIDEO)?.unwrap();

            let stream = input_format_context.streams().get(stream_index).unwrap();

            let mut decode_context = AVCodecContext::new(&decoder);
            decode_context.apply_codecpar(&stream.codecpar())?;
            decode_context.open(None)?;

            (stream_index, decode_context)
        };

        let cover_frame = loop {
            let cover_packet = loop {
                match input_format_context.read_packet()? {
                    Some(x) if x.stream_index != video_stream_index as i32 => {}
                    x => break x,
                }
            };

            decode_context.send_packet(cover_packet.as_ref())?;
            // repeatedly send packet until a frame can be extracted
            match decode_context.receive_frame() {
                Ok(x) => break x,
                Err(RsmpegError::DecoderDrainError) => {}
                Err(e) => {},
            }
        };

        println!("Cover frame info: {:#?}", cover_frame);

        let mut encode_context = {
            let encoder = AVCodec::find_encoder(AVCodecID_AV_CODEC_ID_MJPEG).context("Encoder not found")?;
            let mut encode_context = AVCodecContext::new(&encoder);

            encode_context.set_bit_rate(decode_context.bit_rate);
            encode_context.set_width(width as i32);
            encode_context.set_height(height as i32);
            encode_context.set_time_base(av_inv_q(decode_context.framerate));
            encode_context.set_pix_fmt(if let Some(pix_fmts) = encoder.pix_fmts() {
                pix_fmts[0]
            } else {
                decode_context.pix_fmt
            });
            encode_context.open(None)?;
            encode_context
        };

        let scaled_cover_packet = {
            let mut sws_context = SwsContext::get_context(
                decode_context.width,
                decode_context.height,
                decode_context.pix_fmt,
                encode_context.width,
                encode_context.height,
                encode_context.pix_fmt,
                SWS_FAST_BILINEAR | SWS_PRINT_INFO,
            )
                .context("Invalid swscontext parameter.")?;

            let image_buffer = AVImage::new(
                encode_context.pix_fmt,
                encode_context.width,
                encode_context.height,
                1,
            ).unwrap();

            let mut scaled_cover_frame = AVFrameWithImage::new(image_buffer);

            sws_context.scale_frame(
                &cover_frame,
                0,
                decode_context.height,
                &mut scaled_cover_frame,
            )?;

            println!("{:#?}", scaled_cover_frame.deref());

            encode_context.send_frame(Some(&scaled_cover_frame))?;
            encode_context.receive_packet()?
        };

        let mut file = File::create(target_file).unwrap();
        let data = unsafe {
            slice::from_raw_parts(scaled_cover_packet.data, scaled_cover_packet.size as usize)
        };
        file.write_all(data)?;

        Ok(())
    }

    fn should_auto_complete(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use harana_common::time::Instant;
    use harana_common::tokio;
    use harana_thumbnailer_core::thumbnailer::Thumbnailer;

    use crate::thumbnailer_video::ThumbnailerVideo;

    #[tokio::test]
    async fn test_indexing() {
        let now = Instant::now();

        let _ = ThumbnailerVideo.thumbnail(
            Path::new("../../../test_files/Sample1.mp4"),
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