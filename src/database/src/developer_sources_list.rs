use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;

use crate::HaranaDeveloperSource;

pub const QUERY: &str = r##"
    SELECT * FROM developer_sources
"##;

pub fn developer_sources_list(tx: &Connection) -> Result<Vec<HaranaDeveloperSource>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([], |row| {
            Ok(HaranaDeveloperSource {
                title: row.get(0)?,
                name: row.get(1)?,
                language: row.get(2)?,
                enabled: row.get(3)?
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}