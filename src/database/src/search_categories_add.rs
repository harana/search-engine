use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO search_categories
    (title, name, enabled, position)
    VALUES
    ($title, $name, $enabled, $position)
"##;

pub fn search_categories_add(tx: &Connection, title: String, name: String, enabled: bool, position: u32) -> Result<i64> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$title": title,
        "$name": name,
        "$enabled": enabled,
        "$position": position
    })?;

    let job_id = tx.last_insert_rowid();
    Ok(job_id)
}