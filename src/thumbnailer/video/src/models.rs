use std::{
    error::Error,
    fmt, io,
    path::Path,
    process::{Command, Stdio},
    str,
    str::FromStr,
};

use clap::arg_enum;
use image;
use rustfft::{FFTplanner, num_complex::Complex, num_traits::Zero};
use harana_common::serde::Deserialize;
use structopt::StructOpt;

use crate::constants::*;
use crate::errors::VcsrError;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Grid {
    pub x: u32,
    pub y: u32,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

impl FromStr for Grid {
    type Err = VcsrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mxn_result: Result<Vec<u32>, _> = s.split("x").map(|m| m.parse::<u32>()).collect();
        let mxn = mxn_result?;
        if mxn.len() > 2 {
            Err(VcsrError::GridShape)
        } else {
            Ok(Grid {
                x: mxn[0],
                y: mxn[1],
            })
        }
    }
}

#[derive(Clone, Debug)]
pub struct Frame {
    pub avg_colour: f32,
    pub blurriness: f32,
    pub file_name: String,
    pub timestamp: f32,
}

#[derive(Clone, Debug, Default)]
pub struct MediaInfo {
    pub ffprobe: Ffprobe,
    pub media_attributes: Option<MediaAttributes>,
}

#[derive(Clone, Debug, Default)]
pub struct MediaAttributes {
    pub dimensions: Dimensions,
    pub display_aspect_ratio: Option<String>,
    pub duration: String,
    pub duration_seconds: f32,
    pub file_name: String,
    pub frame_rate: u32,
    pub sample_aspect_ratio: Option<String>,
    pub size_bytes: f64,
    pub size: String,
    pub video_codec: Option<String>,
    pub video_codec_long: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Dimensions {
    pub display_height: Option<u32>,
    pub display_width: Option<u32>,
    pub sample_height: Option<u32>,
    pub sample_width: Option<u32>,
}

impl MediaInfo {
    pub fn new(path: &Path, _verbose: bool) -> Result<MediaInfo, VcsrError> {
        let ffprobe = Self::probe_media(path)?;
        let media_attributes = Self::create_media_attributes(&ffprobe)?;
        Ok(MediaInfo {
            ffprobe,
            media_attributes: Some(media_attributes),
        })
    }

    pub fn probe_media(path: &Path) -> Result<Ffprobe, VcsrError> {
        if path.exists() {
            let output = Command::new("ffprobe")
                .arg("-v")
                .arg("quiet")
                .arg("-print_format")
                .arg("json")
                .arg("-show_format")
                .arg("-show_streams")
                .arg(path)
                .output()?;

            if let Ok(stdout) = str::from_utf8(&output.stdout) {
                let f: Ffprobe =
                    serde_json::from_str(stdout).map_err(|e| VcsrError::StreamError(e))?;
                Ok(f)
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "ffprobe crashed unexpectedly").into())
            }
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "cannot find requested video file").into())
        }
    }

    pub fn human_readable_size(mut num: f64) -> String {
        let suffix = "B";
        let mut size = String::from("");
        for unit in vec!["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi"].iter() {
            if num.abs() < 1024.0 {
                size = format!("{:3.1} {}{}", num, unit, suffix);
                break;
            }
            num = num / 1024.0;
        }
        size
    }

    pub fn find_video_stream(ffprobe: &Ffprobe) -> Option<&Stream> {
        ffprobe.streams.iter().find(|stream| match stream {
            Stream::VideoStream(_) => true,
            _ => false,
        })
    }

    pub fn compute_display_resolution(ffprobe: &Ffprobe) -> Result<Dimensions, VcsrError> {
        let video_stream = Self::find_video_stream(ffprobe).unwrap().clone();
        if let Stream::VideoStream(video_stream) = video_stream {
            let mut display_height: Option<u32>;
            let mut display_width: Option<u32>;
            let mut sample_height: Option<u32>;
            let mut sample_width: Option<u32>;
            sample_width = video_stream.width;
            sample_height = video_stream.height;
            if let Some(rotation) = video_stream.tags.rotate {
                // Swap width and height
                if rotation == 90 {
                    std::mem::swap(&mut sample_width, &mut sample_height);
                }
            }

            let sample_aspect_ratio = video_stream
                .sample_aspect_ratio
                .ok_or(VcsrError::NoneError)?;
            if sample_aspect_ratio == "1:1" {
                display_width = sample_width;
                display_height = sample_height;
            } else {
                let mut sample_split = sample_aspect_ratio.split(":").into_iter();
                let sw = sample_split
                    .next()
                    .unwrap()
                    .to_string()
                    .parse::<u32>()
                    .unwrap();
                let sh = sample_split
                    .next()
                    .unwrap()
                    .to_string()
                    .parse::<u32>()
                    .unwrap();

                let new_sample_width = sample_width.unwrap() * sw / sh;
                display_width = Some(new_sample_width);
                display_height = sample_height;
            }

            if let Some(option_display_width) = display_width {
                if option_display_width == 0 {
                    display_width = sample_width;
                }
            }
            if let Some(option_display_height) = display_height {
                if option_display_height == 0 {
                    display_height = sample_height;
                }
            }
            return Ok(Dimensions {
                display_height,
                display_width,
                sample_height,
                sample_width,
            });
        }
        Err(VcsrError::VideoStreamError)
    }

    pub fn compute_duration(ffprobe: &Ffprobe) -> Option<(f32, String)> {
        let video_stream = Self::find_video_stream(ffprobe).unwrap();
        if let Stream::VideoStream(video_stream) = video_stream {
            let duration = match &video_stream.duration {
                Some(duration) => duration,
                None => &ffprobe.format.duration,
            }
                .to_string();
            let duration_seconds = duration.parse::<f32>().unwrap();
            let duration = MediaInfo::pretty_duration(duration.parse::<f32>().unwrap(), true, true);
            return Some((duration_seconds, duration));
        }
        None
    }

    // Compute duration, size and retrieve file_name
    pub fn compute_file_name(ffprobe: &Ffprobe) -> String {
        Path::new(&ffprobe.format.file_name)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    }

    pub fn compute_size(ffprobe: &Ffprobe) -> Result<(f64, String), Box<dyn Error>> {
        let size_bytes = ffprobe.format.size.parse::<f64>()?;
        let size = MediaInfo::human_readable_size(size_bytes);
        Ok((size_bytes, size))
    }

    // Converts seconds to a human readable time format
    pub fn pretty_duration(seconds: f32, show_centis: bool, show_millis: bool) -> String {
        let hours = (seconds / 3600.0).floor();
        let remaining_seconds = seconds - 3600.0 * hours;

        let minutes = (remaining_seconds / 60.0).floor();
        let remaining_seconds = remaining_seconds - 60.0 * minutes;
        let mut duration = "".to_string();

        if hours > 0.0 {
            duration = format!("{}:", hours);
        }

        duration = format!("{}{:0>2}:{:0>2}", duration, minutes, remaining_seconds.floor());

        if show_centis || show_millis {
            let mut coeff = 100.0;
            let mut digits = 2;
            if show_millis {
                coeff = 1000.0;
                digits = 3;
            }

            let centis = ((remaining_seconds - remaining_seconds.floor()) * coeff).floor();
            duration = format!("{}.{:0>digits$}", duration, centis, digits = digits);
        }

        duration
    }

    pub fn pretty_to_seconds(pretty_duration: &str) -> Result<f32, VcsrError> {
        let millis_split: Vec<&str> = pretty_duration.split(".").collect();
        let mut millis = 0.0;
        let left;
        if millis_split.len() == 2 {
            millis = millis_split[1].parse()?;
            left = millis_split[0];
        } else {
            left = pretty_duration;
        }
        let left_split: Vec<&str> = left.split(":").collect();
        let hours;
        let minutes;
        let seconds;
        if left_split.len() < 3 {
            hours = 0.0;
            minutes = left_split[0].parse::<f32>()?;
            seconds = left_split[1].parse::<f32>()?;
        } else {
            hours = left_split[0].parse::<f32>()?;
            minutes = left_split[1].parse::<f32>()?;
            seconds = left_split[2].parse::<f32>()?;
        }
        Ok((millis / 1000.0) + seconds + minutes * 60.0 + hours * 3600.0)
    }

    pub fn parse_duration(seconds: f32) -> Time {
        let hours = (seconds / 3600.0).floor();
        let remaining_seconds = seconds - 3600.0 * hours;

        let minutes = (remaining_seconds / 60.0).floor();
        let remaining_seconds = remaining_seconds - 60.0 * minutes;
        let seconds = remaining_seconds.floor();

        let millis = ((remaining_seconds - remaining_seconds.floor()) * 1000.0).floor();
        let centis = ((remaining_seconds - remaining_seconds.floor()) * 100.0).floor();

        Time {
            hours,
            minutes,
            seconds,
            centis,
            millis,
        }
    }

    pub fn desired_size(dimensions: &Dimensions, width: Option<u32>) -> Grid {
        let new_width = match width {
            Some(w) => w,
            None => DEFAULT_CONTACT_SHEET_WIDTH,
        };
        let ratio = new_width as f64 / f64::from(dimensions.display_width.unwrap());
        let desired_height = (dimensions.display_height.unwrap() as f64 * ratio).floor();
        Grid {
            x: new_width,
            y: desired_height as u32,
        }
    }

    // Parse multiple media attributes
    pub fn create_media_attributes(ffprobe: &Ffprobe) -> Result<MediaAttributes, VcsrError> {
        let dimensions = Self::compute_display_resolution(&ffprobe)?;
        let (duration_seconds, duration) = Self::compute_duration(&ffprobe).unwrap();
        let file_name = Self::compute_file_name(&ffprobe);
        let (size_bytes, size) = Self::compute_size(&ffprobe).unwrap();
        let mut video_codec = None;
        let mut video_codec_long = None;
        let mut sample_aspect_ratio = None;
        let mut display_aspect_ratio = None;
        let mut frame_rate = 0;

        // video
        let video_stream = Self::find_video_stream(&ffprobe).unwrap().clone();
        if let Stream::VideoStream(video_stream) = video_stream {
            video_codec = video_stream.codec_name;
            video_codec_long = video_stream.codec_long_name;
            sample_aspect_ratio = video_stream.sample_aspect_ratio;
            display_aspect_ratio = video_stream.display_aspect_ratio;
            if let Some(avg_frame_rate) = video_stream.avg_frame_rate {
                let splits: Vec<&str> = avg_frame_rate.split("/").collect();
                if splits.len() == 2 {
                    frame_rate = (splits[0]).parse::<u32>().unwrap() / splits[1].parse::<u32>().unwrap();
                } else {
                    frame_rate = avg_frame_rate.parse::<u32>().unwrap();
                }

                frame_rate = frame_rate;
            }
        }

        Ok(MediaAttributes {
            dimensions,
            display_aspect_ratio,
            duration,
            duration_seconds,
            file_name,
            frame_rate,
            sample_aspect_ratio,
            size,
            size_bytes,
            video_codec,
            video_codec_long,
        })
    }
}

pub struct MediaCapture {
    path: String,
    accurate: bool,
    skip_delay_seconds: f32,
    frame_type: Option<String>,
}

impl MediaCapture {
    pub fn new(
        path: String,
        accurate: bool,
        skip_delay_seconds: f32,
        frame_type: Option<String>,
    ) -> MediaCapture {
        MediaCapture {
            path,
            accurate,
            skip_delay_seconds,
            frame_type,
        }
    }

    /// Capture a frame at given time with given width and height
    /// using ffmpeg.
    pub fn make_capture(
        &self,
        time: &str,
        width: u32,
        height: u32,
        out_path: Option<&str>,
    ) -> Result<(), VcsrError> {
        let skip_delay = MediaInfo::pretty_duration(self.skip_delay_seconds, false, true);
        let out_path = match out_path {
            Some(o) => o,
            None => "out.jpg",
        };

        let frame_type_string: String;
        let mut select_args = match &self.frame_type {
            Some(frame_type) => {
                if frame_type == "key" {
                    vec!["-vf", "select=key"]
                } else {
                    frame_type_string = format!("\'select=eq(frame_type\\,{})\'", frame_type);
                    vec!["-vf", &frame_type_string]
                }
            }
            None => Vec::new(),
        };

        let time_seconds;
        let skip_time_seconds;
        let skip_time;
        let mut args = if !self.accurate {
            vec!["-ss", time, "-i", &self.path]
        } else {
            time_seconds = MediaInfo::pretty_to_seconds(time)?;
            skip_time_seconds = time_seconds - self.skip_delay_seconds;
            skip_time = MediaInfo::pretty_duration(skip_time_seconds, false, true);
            if skip_time_seconds < 0.0 {
                vec!["-ss", time, "-i", &self.path]
            } else {
                vec!["-ss", &skip_time, "-i", &self.path, "-ss", &skip_delay]
            }
        };

        let width_x_height = format!("{}x{}", width, height);
        args.append(&mut vec!["-vframes", "1", "-s", &width_x_height]);
        args.append(&mut select_args);
        args.append(&mut vec!["-y", out_path]);

        let output = Command::new("ffmpeg")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .args(args)
            .output()?;
        if !output.status.success() {
            error!("ffmpeg error: {}", str::from_utf8(&output.stderr).unwrap());
        }
        Ok(())
    }

    pub fn compute_avg_colour(image_path: &str) -> Result<f32, VcsrError> {
        if Path::new(image_path).exists() {
            let image = image::open(image_path).unwrap().to_rgba8();
            let rgbs: (f32, f32, f32) =
                image
                    .enumerate_pixels()
                    .fold((0.0, 0.0, 0.0), |acc, (_, _, p)| match p {
                        image::Rgba(rgba) => (
                            acc.0 + rgba[0] as f32,
                            acc.1 + rgba[1] as f32,
                            acc.2 + rgba[2] as f32,
                        ),
                    });
            let size = image.width() as f32 * image.height() as f32;
            Ok((rgbs.0 / size + rgbs.1 / size + rgbs.2 / size) / 3.0)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Cannot compute average colour as capture file was not created",
            )
                .into())
        }
    }

    pub fn compute_blurrines(image_path: &str) -> Result<f32, VcsrError> {
        if Path::new(image_path).exists() {
            let f = std::fs::File::open(image_path).unwrap();
            drop(f);

            let image = image::open(image_path).unwrap().to_luma8();
            let mut input: Vec<Complex<f32>> = image
                .enumerate_pixels()
                .map(|(_, _, p)| match p {
                    image::Luma(g) => Complex {
                        re: g[0] as f32,
                        im: 0.0,
                    },
                })
                .collect();

            let mut output: Vec<Complex<f32>> = vec![Zero::zero(); input.len()];
            let mut planner = FFTplanner::new(false);
            let fft = planner.plan_fft(input.len());
            fft.process(&mut input, &mut output);

            let mut collected: Vec<f32> = output
                .into_iter()
                .map(|c| match c {
                    Complex { re, im: _ } => (re).abs(),
                })
                .collect();
            collected.sort_by(|a, b| b.partial_cmp(&a).unwrap());
            collected.dedup();
            let max_freq = MediaCapture::avg9x(collected, None);
            return if max_freq > 0.0 {
                Ok(1.0 / max_freq)
            } else {
                Ok(1.0)
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Cannot compute blurriness as capture file was not created",
            )
                .into())
        }
    }

    pub fn avg9x(matrix: Vec<f32>, percentage: Option<f32>) -> f32 {
        let percentage = match percentage {
            Some(percentage) => percentage,
            None => 0.05,
        };

        let length = (percentage * matrix.len() as f32).floor() as usize;
        if length >= 2 {
            let matrix_subset = &matrix[0..length];
            if length % 2 == 0 {
                (matrix_subset[length / 2 - 1] + matrix_subset[length / 2]) / 2.0
            } else {
                matrix_subset[(length - 1)] / 2.0
            }
        } else {
            0.0
        }
    }

    fn max_req(matrix: Vec<f32>) -> f32 {
        *matrix.first().unwrap()
    }
}

#[derive(Debug)]
pub struct Time {
    hours: f32,
    minutes: f32,
    seconds: f32,
    centis: f32,
    millis: f32,
}

#[derive(Clone, Default, Debug, Deserialize)]
struct Disposition {
    attached_pic: u32,
    clean_effects: u32,
    comment: u32,
    default: u32,
    dub: u32,
    forced: u32,
    hearing_impaired: u32,
    karaoke: u32,
    lyrics: u32,
    original: u32,
    timed_thumbnails: u32,
    visual_impaired: u32,
}

#[derive(Clone, Default, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
struct StreamTags {
    creation_time: Option<String>,
    duration: Option<String>,
    handler_name: Option<String>,
    language: Option<String>,
    rotate: Option<u32>,
    #[serde(rename = "BPS-eng")]
    bps_eng: String,
    #[serde(rename = "DURATION-eng")]
    duration_eng: String,
    #[serde(rename = "NUMBER_OF_BYTES-eng")]
    number_of_bytes_eng: String,
    #[serde(rename = "NUMBER_OF_FRAMES-eng")]
    number_of_frames_eng: String,
    #[serde(rename = "_STATISTICS_TAGS-eng")]
    statistics_tags_eng: String,
    #[serde(rename = "_STATISTICS_WRITING_APP-eng")]
    statistics_writing_app_eng: String,
    #[serde(rename = "_STATISTICS_WRITING_DATE_UTC-eng")]
    statistics_writing_date_utc_eng: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct GenericStream {}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "codec_type")]
pub enum Stream {
    #[serde(rename = "video")]
    VideoStream(StreamStruct),
    #[serde(rename = "audio")]
    AudioStream(StreamStruct),
    #[serde(rename = "subtitle")]
    SubtitleStream(StreamStruct),
    #[serde(rename = "data")]
    DataStream(StreamStruct),
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct StreamStruct {
    avg_frame_rate: Option<String>,
    bits_per_raw_sample: Option<String>,
    bit_rate: Option<String>,
    chroma_location: Option<String>,
    codec_long_name: Option<String>,
    pub codec_name: Option<String>,
    codec_tag: Option<String>,
    codec_tag_string: Option<String>,
    codec_time_base: Option<String>,
    codec_type: Option<String>,
    coded_height: Option<u32>,
    coded_width: Option<u32>,
    color_primaries: Option<String>,
    color_range: Option<String>,
    color_space: Option<String>,
    color_transfer: Option<String>,
    display_aspect_ratio: Option<String>,
    #[serde(skip)]
    disposition: Disposition,
    duration_ts: Option<u64>,
    duration: Option<String>,
    field_order: Option<String>,
    has_b_frames: Option<u32>,
    height: Option<u32>,
    index: Option<u32>,
    is_avc: Option<String>,
    level: Option<u32>,
    nal_length_size: Option<String>,
    pix_fmt: Option<String>,
    profile: Option<String>,
    r_frame_rate: Option<String>,
    refs: Option<u32>,
    #[serde(default = "default_sample_aspect_ratio")]
    sample_aspect_ratio: Option<String>,
    sample_rate: Option<String>,
    start_pts: Option<u32>,
    start_time: Option<String>,
    #[serde(skip)]
    tags: StreamTags,
    time_base: Option<String>,
    width: Option<u32>,
}

#[derive(Clone, Default, Debug, Deserialize)]
struct SideDataType {
    side_data_type: String,
}

fn default_sample_aspect_ratio() -> Option<String> {
    Some("1:1".to_string())
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Format {
    bit_rate: Option<String>,
    pub duration: String,
    file_name: String,
    format_long_name: String,
    format_name: String,
    nb_programs: i32,
    nb_streams: i32,
    probe_score: i32,
    size: String,
    start_time: String,
    #[serde(skip)]
    tags: FormatTags,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct FormatTags {
    creation_time: Option<String>,
    compatible_brands: Option<String>,
    encoder: Option<String>,
    major_brand: Option<String>,
    minor_version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Ffprobe {
    streams: Vec<Stream>,
    pub format: Format,
}

arg_enum! {
    #[derive(Clone, Debug, StructOpt)]
    pub enum MetadataPosition {
        Top,
        Bottom,
        Hidden
    }
}

arg_enum! {
    #[derive(Clone, Debug, StructOpt)]
    pub enum TimestampPosition {
        North,
        South,
        East,
        West,
        NE,
        NW,
        SE,
        SW,
        Center,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_from_string() {
        let g = Grid::from_str("2x2");
        assert_eq!(g.unwrap(), Grid { x: 2, y: 2 });
    }

    #[test]
    fn grid_from_invalid_string_length() {
        // specific errors would be better
        // but I don't feel like implementing
        // PartialEq for VcsrError right now.
        let g = Grid::from_str("2x3x4");
        assert!(g.is_err());
    }

    #[test]
    fn grid_from_invalid_string_characters() {
        let g = Grid::from_str("2x3m");
        assert!(g.is_err());
    }

    #[test]
    fn test_human_readable_size() {
        let mut size = 1000f64;
        for hrs in vec!["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi"]
            .iter()
            .map(|s| format!("1000.0 {}{}", s, "B"))
        {
            assert_eq!(hrs, MediaInfo::human_readable_size(size));
            size = size * 1024.0;
        }
    }
}