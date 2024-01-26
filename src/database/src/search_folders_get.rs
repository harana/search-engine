use rusqlite::Connection;

use harana_common::anyhow::{anyhow, Result};
use harana_common::itertools::Itertools;

use crate::HaranaSearchFolder;

pub const QUERY: &str = r##"
    SELECT * FROM search_folders WHERE WHERE name_or_path = ? AND is_path is ?
"##;

pub fn search_folders_get(tx: &Connection, name_or_path: String, is_path: bool) -> Result<HaranaSearchFolder> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([name_or_path, String::from(if is_path { "1" } else { "0" })], |row| {
            Ok(HaranaSearchFolder {
                title: row.get(0)?,
                name_or_path: row.get(1)?,
                is_path: row.get(2)?,
                enabled: row.get(3)?,
                crawled: row.get(4)?
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec()
        .pop().ok_or(anyhow!("not found"));

    results
}