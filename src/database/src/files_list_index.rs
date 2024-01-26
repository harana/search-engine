use std::path::PathBuf;

use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;

use crate::urlencoder::decode;

pub const QUERY: &str = r##"
    SELECT path FROM files WHERE process_date IS NULL LIMIT ?
"##;

pub fn files_list_index(tx: &Connection, limit: usize) -> Result<Vec<PathBuf>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([limit], |row| {
            let str: String = decode(row.get(0)?)?;
            Ok(PathBuf::from(str))
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}