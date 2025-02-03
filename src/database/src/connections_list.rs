use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;
use crate::HaranaConnection;

pub const QUERY: &str = r##"
    SELECT * FROM connections WHERE plugin = $plugin
"##;

pub fn connections_list(tx: &Connection, plugin: String) -> Result<Vec<HaranaConnection>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([plugin], |row| {
            Ok(HaranaConnection {
                id: row.get(0)?,
                status: row.get(1)?,
                plugin: row.get(2)?,
                name: row.get(3)?,
                authentication_type: row.get(4)?
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}