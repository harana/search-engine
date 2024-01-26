use std::path::PathBuf;
use harana_common::hashbrown::HashSet;

// use metadata::{MediaFileMetadata, StreamMetadata, Tags};
//
// use harana_common::anyhow::Result;
// use harana_common::serde::{self, Deserialize, Serialize};
//
// #[derive(Deserialize, Serialize)]
// #[serde(crate = "self::serde")]
// pub struct FfmpegMetadata {
//     title: Option<String>,
//     duration: Option<String>,
//     container_format: Option<String>,
//     artist: Option<String>,
//     album: Option<String>,
//     is_podcast: Option<bool>,
//     description: Option<String>,
//     synopsis: Option<String>,
//     genre: Option<String>,
//     rating: Option<String>,
//     video_width: Option<usize>,
//     video_height: Option<usize>,
//     video_bit_rate: Option<String>,
//     video_codec: Option<String>,
//     video_frame_rate: Option<String>,
//     video_pixel_dimensions: Option<String>,
//     video_pixel_format: Option<String>,
//     video_color_primaries: Option<String>,
//     video_color_range: Option<String>,
//     video_color_space: Option<String>,
//     video_aspect_ratio: Option<String>,
//     audio_codec: Option<String>,
//     audio_bit_rate: Option<String>,
//     audio_sample_rate: Option<String>,
// }
//
// fn _metadata(path: &str) -> Result<FfmpegMetadata> {
//     let mut meta = MediaFileMetadata::new(&PathBuf::from(path))?;
//     meta.include_checksum(true)?.include_tags(true).include_all_tags(true);
//
//     let mut primary_tokens = HashSet::<String>::new();
//
//     let artist = tag(&meta.tags, "artist");
//     let album = tag(&meta.tags, "album");
//     let is_podcast = tag(&meta.tags, "podcast").map(|p| p.as_str() == "1");
//     let description = tag(&meta.tags, "description");
//     let synopsis = tag(&meta.tags, "synopsis");
//     let genre = tag(&meta.tags, "genre");
//     let rating = tag(&meta.tags, "rating");
//
//     let mut video_width: Option<usize> = None;
//     let mut video_height: Option<usize> = None;
//     let mut video_bit_rate: Option<String> = None;
//     let mut video_codec: Option<String> = None;
//     let mut video_frame_rate: Option<String> = None;
//     let mut video_pixel_dimensions: Option<String> = None;
//     let mut video_pixel_format: Option<String> = None;
//     let mut video_color_primaries: Option<String> = None;
//     let mut video_color_range: Option<String> = None;
//     let mut video_color_space: Option<String> = None;
//     let mut video_aspect_ratio: Option<String> = None;
//
//     let mut audio_codec: Option<String> = None;
//     let mut audio_bit_rate: Option<String> = None;
//     let mut audio_sample_rate: Option<String> = None;
//
//     meta._streams_metadata.into_iter().for_each(|m| {
//         match m {
//             StreamMetadata::VideoMetadata(v) => {
//                 video_width = Some(v.width as usize);
//                 video_height = Some(v.height as usize);
//                 video_bit_rate = v.bit_rate;
//                 video_codec = Some(v.codec_desc);
//                 video_frame_rate = v.frame_rate;
//                 video_pixel_dimensions = Some(v.pixel_dimensions);
//                 video_pixel_format = v.pixel_fmt;
//                 video_color_primaries = v.color_primaries;
//                 video_color_range = v.color_range;
//                 video_color_space = v.color_space;
//                 video_aspect_ratio = Some(v.display_aspect_ratio);
//             }
//             StreamMetadata::AudioMetadata(a) => {
//                 audio_codec = Some(a.codec_desc);
//                 audio_bit_rate = a.bit_rate;
//                 audio_sample_rate = Some(a.sample_rate);
//             }
//             _ => {}
//         }
//     });
//
//     let display_frame_rate = video_frame_rate.clone().map(|fr|
//         match fr.as_str() {
//             "352x240" => "240p",
//             "480x360" => "360p",
//             "640x480" => "480p",
//             "1280x720" => "720p",
//             "1920x1080" => "1080p",
//             "3840x2160" => "4k",
//             _ => ""
//         }.to_string()
//     );
//
//     if video_codec.is_some() { primary_tokens.insert(video_codec.clone().unwrap()); }
//     if meta.title.is_some() { primary_tokens.insert(meta.title.clone().unwrap()); }
//     if artist.is_some() { primary_tokens.insert(artist.clone().unwrap()); }
//     if album.is_some() { primary_tokens.insert(album.clone().unwrap()); }
//     if description.is_some() { primary_tokens.insert(description.clone().unwrap()); }
//     if synopsis.is_some() { primary_tokens.insert(synopsis.clone().unwrap()); }
//     if genre.is_some() { primary_tokens.insert(genre.clone().unwrap()); }
//     if display_frame_rate.is_some() { primary_tokens.insert(display_frame_rate.unwrap()); }
//
//     Ok(
//         FfmpegMetadata {
//             title: meta.title,
//             duration: meta.duration,
//             container_format: Some(meta.container_format),
//             artist,
//             album,
//             is_podcast,
//             description,
//             synopsis,
//             genre,
//             rating,
//             video_width,
//             video_height,
//             video_bit_rate,
//             video_codec,
//             video_frame_rate,
//             video_pixel_dimensions,
//             video_pixel_format,
//             video_color_primaries,
//             video_color_range,
//             video_color_space,
//             video_aspect_ratio,
//             audio_codec,
//             audio_bit_rate,
//             audio_sample_rate
//         }
//     )
// }
//
// fn tag(tags: &Tags, key: &str) -> Option<String> {
//     tags
//         .into_iter()
//         .find(|(k, _v)| k.as_str() == key)
//         .map(|(_k, v)| v.to_string())
// }