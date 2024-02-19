use std::panic::RefUnwindSafe;
use std::path::PathBuf;
use harana_common::anyhow::Result;
use crate::index_result::IndexResult;

pub trait Indexer: Send + Sync + RefUnwindSafe {

    fn index(&self, path: PathBuf) -> Result<IndexResult>;

    fn get_name(&self) -> &'static str {
        std::any::type_name::<Self>().split("::").last().unwrap_or_default()
    }
}