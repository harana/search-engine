use rusqlite::Connection;

use harana_common::anyhow::{anyhow, Result};
use harana_common::itertools::Itertools;

use crate::HaranaTaskGroup;

pub const QUERY: &str = r##"
    SELECT * FROM job_groups WHERE category = ? and name = ?
"##;

pub fn job_groups_get(tx: &Connection, category: String, name: String) -> Result<HaranaTaskGroup> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([category, name], |row| {
            Ok(HaranaTaskGroup {
                category: row.get(0)?,
                name: row.get(1)?,
                successes: row.get(2)?,
                failures: row.get(3)?,
                total: row.get(4)?
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec()
        .pop().ok_or(anyhow!("not found"));

    results
}