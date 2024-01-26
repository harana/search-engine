use std::path::PathBuf;

use rusqlite::Connection;

use harana_common::anyhow::{anyhow, Result};
use harana_common::itertools::Itertools;
use harana_common::time::OffsetDateTime;

use crate::HaranaFile;
use crate::urlencoder::decode;

pub const QUERY: &str = r##"
    SELECT * FROM files WHERE path = ?
"##;

pub fn files_get_path(tx: &Connection, path: PathBuf) -> Result<HaranaFile> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([path.to_str().unwrap()], |row| {
            let path: String = row.get(0)?;
            let index_date: Option<i64> = row.get(2)?;
            let thumbnail_date: Option<i64> = row.get(3)?;
            Ok(HaranaFile {
                path: decode(path)?,
                hash: row.get(1)?,
                index_date: index_date.map(|d| OffsetDateTime::from_unix_timestamp(d).unwrap()),
                thumbnail_date: thumbnail_date.map(|d| OffsetDateTime::from_unix_timestamp(d).unwrap()),
                document_id: row.get(4)?
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec()
        .pop().ok_or(anyhow!("not found"));

    results
}