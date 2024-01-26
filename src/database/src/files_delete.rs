use rusqlite::Connection;

use harana_common::anyhow::Result;

use crate::urlencoder::encode;

pub const QUERY: &str = r##"
    DELETE FROM files WHERE path = ?
"##;

pub fn file_delete(tx: &Connection, path: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    sql_stmt.execute([encode(path)])?;
    Ok(())
}