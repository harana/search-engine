use rusqlite::Connection;
use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;

use crate::HaranaSearchCategory;

pub const QUERY: &str = r##"
    SELECT * FROM search_categories
"##;

pub fn search_categories_list(tx: &Connection) -> Result<Vec<HaranaSearchCategory>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([], |row| {
            Ok(HaranaSearchCategory {
                title: row.get(0)?,
                name: row.get(1)?,
                enabled: row.get(2)?,
                position: row.get(3)?
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}