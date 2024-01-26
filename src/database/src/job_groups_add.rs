use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT OR IGNORE INTO job_groups
    (category, name, successes, failures, total)
    VALUES
    ($category, $name, 0, 0, $total)
"##;

pub fn job_groups_add(tx: &Connection, category: String, name: String, total: u32) -> Result<i64> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$category": category,
        "$name": name,
        "$total": total
    })?;

    let job_id = tx.last_insert_rowid();
    Ok(job_id)
}