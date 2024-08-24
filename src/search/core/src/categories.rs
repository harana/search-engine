use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use harana_indexer_core::indexer::Indexer;

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FileCategory {
    Application,
    Archive,
    Audio,
    Book,
    Calendar,
    Certificate,
    Code,
    CodeArtifact,
    Contact,
    Data,
    DiskImage,
    Document,
    Email,
    Folder,
    Font,
    Game,
    Graphics,
    Hardware,
    Image,
    Map,
    Other,
    Science,
    Screenshot,
    Video
}

impl FileCategory {

    pub fn indexes() -> Vec<String> {
        FileCategory::iter()
            .map(|category| category.index().to_string())
            .collect()
    }

    pub fn index(&self) -> &str {
        match *self {
            FileCategory::Application => "local-application",
            FileCategory::Archive => "local-archive",
            FileCategory::Audio => "local-audio",
            FileCategory::Book => "local-book",
            FileCategory::Calendar => "local-calendar",
            FileCategory::Certificate => "local-certificate",
            FileCategory::Code => "local-code",
            FileCategory::CodeArtifact => "local-codeartifact",
            FileCategory::Contact => "local-contact",
            FileCategory::Data => "local-data",
            FileCategory::DiskImage => "local-diskimage",
            FileCategory::Document => "local-document",
            FileCategory::Email => "local-email",
            FileCategory::Folder => "local-folder",
            FileCategory::Font => "local-font",
            FileCategory::Game => "local-game",
            FileCategory::Graphics => "local-graphics",
            FileCategory::Hardware => "local-hardware",
            FileCategory::Image => "local-image",
            FileCategory::Map => "local-map",
            FileCategory::Other => "local-other",
            FileCategory::Science => "local-science",
            FileCategory::Screenshot => "local-screenshot",
            FileCategory::Video => "local-video"
        }
    }
}
