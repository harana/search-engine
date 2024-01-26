use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    DELETE FROM job_groups
    WHERE category = $category AND name = $name
"##;

pub fn job_groups_delete(tx: &Connection, category: String, name: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$category": category,
        "$name": name,
    })?;
    Ok(())
}