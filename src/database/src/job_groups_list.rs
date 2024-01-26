use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;

use crate::HaranaTaskGroup;

pub const QUERY: &str = r##"
    SELECT * FROM job_groups
"##;

pub fn job_groups_list(tx: &Connection) -> Result<Vec<HaranaTaskGroup>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([], |row| {
            Ok(HaranaTaskGroup {
                category: row.get(0)?,
                name: row.get(1)?,
                successes: row.get(2)?,
                failures: row.get(3)?,
                total: row.get(4)?
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}