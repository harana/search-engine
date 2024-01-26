use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    DELETE FROM private_folders
    WHERE path = $path
"##;

pub fn private_folders_delete(tx: &Connection, path: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$path": path,
    })?;
    Ok(())
}