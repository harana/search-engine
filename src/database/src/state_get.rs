use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;

use crate::HaranaState;

pub const QUERY: &str = r##"
    SELECT * FROM state WHERE key = ?
"##;

pub fn state_get(tx: &Connection, key: String) -> Result<Option<String>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([key], |row| {
            Ok(HaranaState {
                key: row.get(0)?,
                value: row.get(1)?
            })
        })?
        .map(|r| r.unwrap().value)
        .collect_vec()
        .pop();
    Ok(results)
}