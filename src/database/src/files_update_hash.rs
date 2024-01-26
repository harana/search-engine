use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    UPDATE files
    SET hash = $hash
    WHERE path = $path
"##;

pub fn files_update_hash(tx: &Connection, path: String, hash: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$hash": hash,
        "$path": path
    })?;
    Ok(())
}