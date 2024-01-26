use std::fs::Metadata;
use std::path::PathBuf;

use harana_common::file_format::FileFormat;

use harana_indexer_core::indexer::Indexer;
use harana_thumbnailer_core::thumbnailer::Thumbnailer;

use crate::extensions::Extensions;

pub async fn extension_details(path: PathBuf, path_metadata: Option<Metadata>) -> (&'static dyn Indexer, &'static dyn Thumbnailer, &'static str, Option<FileFormat>) {
    let is_file = path_metadata.map(|m| m.is_file()).unwrap_or(true);

    let extension = path.extension().and_then(|e| e.to_str());
    let format = FileFormat::from_file(path.clone()).ok();
    //let format = None;

    if is_file {
        if extension.is_some() {
            let indexer = Extensions::indexer(None, extension.unwrap().to_lowercase().as_str());
            let category = Extensions::category(None, extension.unwrap().to_lowercase().as_str());
            let thumbnailer_extension = extension.clone().unwrap().to_lowercase();
            let thumbnailer = Extensions::thumbnailer(None, thumbnailer_extension.as_str()).await;
            (indexer, thumbnailer, category.index(), format)
        } else {
            let indexer = Extensions::indexer(format, "");
            let category = Extensions::category(format, "");
            let thumbnailer = Extensions::thumbnailer(format, "").await;
            (indexer, thumbnailer, category.index(), format)
        }
    }else{
        (&harana_indexer::Noop, &harana_thumbnailer::Noop, "local-folder", None)
    }
}