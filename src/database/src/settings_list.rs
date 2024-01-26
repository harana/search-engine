use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;

use crate::HaranaSetting;

pub const QUERY: &str = r##"
    SELECT * FROM settings
"##;

pub fn settings_list(tx: &Connection) -> Result<Vec<HaranaSetting>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([], |row| {
            Ok(HaranaSetting {
                key: row.get(0)?,
                value: row.get(1)?
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}