use rusqlite::Connection;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    DELETE FROM files
"##;

pub fn files_delete_all(tx: &Connection) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    sql_stmt.execute([])?;
    Ok(())
}