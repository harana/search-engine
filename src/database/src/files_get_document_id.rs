use std::path::PathBuf;

use rusqlite::Connection;

use harana_common::anyhow::{anyhow, Result};
use harana_common::itertools::Itertools;
use harana_common::log::error;
use harana_common::time::OffsetDateTime;

use crate::HaranaFile;
use crate::urlencoder::decode;

pub const QUERY: &str = r##"
    SELECT * FROM files WHERE document_id = ?
"##;

pub fn files_get_document_id(tx: &Connection, document_id: u64) -> Result<HaranaFile> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([document_id.to_string()], |row| {
            let path: String = row.get(0)?;
            let index_date: Option<i64> = row.get(2)?;
            let thumbnail_date: Option<i64> = row.get(3)?;
            let document_id: String = row.get(4)?;

            Ok(HaranaFile {
                path: decode(path)?,
                hash: row.get(1)?,
                index_date: index_date.map(|d| OffsetDateTime::from_unix_timestamp(d).unwrap()),
                thumbnail_date: thumbnail_date.map(|d| OffsetDateTime::from_unix_timestamp(d).unwrap()),
                document_id: document_id.parse::<u64>().unwrap()
            })
        })?
        .inspect(|r| if r.is_err() {
            error!("Failed for {} due to: {:?}", document_id, r.as_ref().unwrap_err());
        })
        .map(|r| r.unwrap())
        .collect_vec()
        .pop().ok_or(anyhow!("error or not found for document: {}", document_id));

    results
}