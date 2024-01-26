use paste::paste;

#[macro_export]
macro_rules! extensions {
    {
        $(
            $(format = $format:ident)?
            extension = $extension:literal
            title = $title:literal
            category = $category:ident
            indexer = $indexer:ident
            thumbnailer = $thumbnailer:ident
            opener = $opener:ident
            viewer = $viewer:ident
            cards = $cards:tt
         )*
    } => {
        paste! {
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            #[allow(non_camel_case_types)]
            pub enum Extensions {}

            impl Extensions {

                pub fn title(format: Option<file_format::FileFormat>, extension: &str) -> String {
                    match format {
                        Some(f) =>
                            match (f, extension) {
                            $(
                            $(
                                (file_format::FileFormat::$format, $extension) => String::from($title),
                            )?
                            )*
                                _ => Self::title(None, extension)
                            }

                        None =>
                            match (extension) {
                                $(
                                    $extension => String::from($title),
                                )*
                                _ => String::from("Unknown")
                            }
                    }
                }

                pub fn category(format: Option<file_format::FileFormat>, extension: &str) -> &'static harana_search_core::categories::FileCategory {
                    match format {
                        Some(f) =>
                            match (f, extension) {
                            $(
                            $(
                                (file_format::FileFormat::$format, $extension) => &harana_search_core::categories::FileCategory::$category,
                            )?
                            )*
                                _ => &Self::category(None, extension)
                            }

                        None =>
                            match (extension) {
                                $(
                                    $extension => &harana_search_core::categories::FileCategory::$category,
                                )*
                                _ => &harana_search_core::categories::FileCategory::Other
                            }
                    }
                }

                pub fn indexer(format: Option<file_format::FileFormat>, extension: &str) -> &'static dyn harana_indexer_core::indexer::Indexer {
                    match format {
                        Some(f) =>
                            match (f, extension) {
                            $(
                            $(
                                (file_format::FileFormat::$format, $extension) => &harana_indexer::$indexer,
                            )?
                            )*
                                _ => &harana_indexer::Noop
                            }

                        None =>
                            match (extension) {
                                $(
                                    $extension => &harana_indexer::$indexer,
                                )*
                                _ => &harana_indexer::Noop
                            }
                    }
                }

                pub async fn thumbnailer(format: Option<file_format::FileFormat>, extension: &str) -> &'static dyn harana_thumbnailer_core::thumbnailer::Thumbnailer {
                    match format {
                        Some(f) =>
                            match (f, extension) {
                            $(
                            $(
                                (file_format::FileFormat::$format, $extension) => &harana_thumbnailer::$thumbnailer,
                            )?
                            )*
                                _ => &harana_thumbnailer::Noop
                            }

                        None =>
                            match (extension) {
                                $(
                                    $extension => &harana_thumbnailer::$thumbnailer,
                                )*
                                _ => &harana_thumbnailer::Noop
                            }
                    }
                }

                pub fn opener(format: Option<file_format::FileFormat>, extension: &str) -> &'static dyn harana_opener_core::opener::Opener {
                    match format {
                        Some(f) =>
                            match (f, extension) {
                            $(
                            $(
                                (file_format::FileFormat::$format, $extension) => &harana_opener::$opener,
                            )?
                            )*
                                _ => &harana_opener::Noop
                            }

                        None =>
                            match (extension) {
                                $(
                                    $extension => &harana_opener::$opener,
                                )*
                                _ => &harana_opener::Noop
                            }
                    }
                }

                pub fn viewer(format: Option<file_format::FileFormat>, extension: &str) -> &'static harana_search_core::viewers::Viewer {
                    match format {
                        Some(f) =>
                            match (f, extension) {
                            $(
                            $(
                                (file_format::FileFormat::$format, $extension) => &harana_search_core::viewers::Viewer::$viewer,
                            )?
                            )*
                                _ => &Self::viewer(None, extension)
                            }

                        None =>
                            match (extension) {
                                $(
                                    $extension => &harana_search_core::viewers::Viewer::$viewer,
                                )*
                                _ => &harana_search_core::viewers::Viewer::Noop
                            }
                    }
                }

                pub fn cards(format: Option<file_format::FileFormat>, extension: &str) -> Vec<&str> {
                    match format {
                        Some(f) =>
                            match (f, extension) {
                            $(
                            $(
                                (file_format::FileFormat::$format, $extension) => Vec::from($cards),
                            )?
                            )*
                                _ => Vec::new()
                            }

                        None =>
                            match (extension) {
                                $(
                                    $extension => Vec::from($cards),
                                )*
                                _ => Vec::new()
                            }
                    }
                }
            }
        }
    };
}