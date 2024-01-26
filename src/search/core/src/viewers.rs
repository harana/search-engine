use strum_macros::EnumIter;
use harana_common::enum2str::EnumStr;

#[derive(Clone, Copy, Debug, Eq, EnumStr, PartialEq, EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Viewer {
    Bim,
    Calendar,
    Certificate,
    Code,
    Comic,
    Cornerstone,
    Docx,
    Epub,
    Font,
    Html,
    Image,
    Ipynb,
    Json,
    Latex,
    Leaflet,
    Markdown,
    Model,
    Niivue,
    Noop,
    Pdf,
    SeqViz,
    SheetJS,
    Three,
    Thumbnail,
    Text,
    Video
}