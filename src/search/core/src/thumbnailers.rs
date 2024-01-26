use strum_macros::EnumIter;
use harana_common::enum2str::EnumStr;

#[derive(Clone, Copy, Debug, Eq, EnumStr, PartialEq, EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Thumbnailer {
    Aiff,
    Avif,
    Bmp,
    Dds,
    Epub,
    Flac,
    Gif,
    Heif,
    Ico,
    Jpeg,
    M4a,
    Mac,
    Mp3,
    Noop,
    Png,
    Pnm,
    Ppm,
    Psd,
    Qoi,
    Raw,
    Tauri,
    Tga,
    Three,
    Tiff,
    Video,
    Wav,
    Webp
}