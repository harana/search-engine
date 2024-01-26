use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;

pub const QUERY: &str = r##"
    SELECT * FROM private_folders
"##;

pub fn private_folders_list(tx: &Connection) -> Result<Vec<String>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([], |row| {
            Ok(row.get(0)?)
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}