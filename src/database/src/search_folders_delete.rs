use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    DELETE FROM search_folders
    WHERE name_or_path = $name_or_path AND is_path = 0
"##;

pub fn search_folders_delete(tx: &Connection, name_or_path: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$name_or_path": name_or_path
    })?;
    Ok(())
}