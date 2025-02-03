use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    DELETE FROM connections
    WHERE plugin = $plugin
"##;

pub fn connections_delete_plugin(tx: &Connection, plugin: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$plugin": plugin,
    })?;
    Ok(())
}