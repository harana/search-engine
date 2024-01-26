use rusqlite::Connection;

use harana_common::anyhow::Result;

use crate::urlencoder::encode;

pub const QUERY: &str = r##"
    DELETE FROM files WHERE path LIKE ?%
"##;

pub fn file_delete_prefix(tx: &Connection, path_prefix: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    sql_stmt.execute([encode(path_prefix)]);
    Ok(())
}