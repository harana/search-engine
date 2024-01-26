use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;

use crate::HaranaSearchFolder;

pub const QUERY: &str = r##"
    SELECT * FROM search_folders
"##;

pub fn search_folders_list(tx: &Connection) -> Result<Vec<HaranaSearchFolder>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([], |row| {
            Ok(HaranaSearchFolder {
                title: row.get(0)?,
                name_or_path: row.get(1)?,
                is_path: row.get(2)?,
                enabled: row.get(3)?,
                crawled: row.get(4)?
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}