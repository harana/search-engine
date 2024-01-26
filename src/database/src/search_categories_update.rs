use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    UPDATE search_categories
    SET enabled = $enabled, position = $position
    WHERE name = $name
"##;

pub fn search_categories_update(tx: &Connection, name: String, enabled: bool, position: u32) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$name": name,
        "$enabled": enabled,
        "$position": position
    })?;

    Ok(())
}