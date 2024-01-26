use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

use crate::urlencoder::encode;

pub const QUERY: &str = r##"
    INSERT OR IGNORE INTO files
    (path, hash)
    VALUES
    ($path, $hash)
"##;

pub fn files_add(tx: &Connection, path: String, hash: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$path": encode(path),
        "$hash": hash
    })?;

    Ok(())
}