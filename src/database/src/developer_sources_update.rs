use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    UPDATE developer_sources
    SET enabled = $enabled
    WHERE name = $name
"##;

pub fn developer_sources_update(tx: &Connection, name: String, enabled: bool) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$name": name,
        "$enabled": enabled
    })?;

    Ok(())
}