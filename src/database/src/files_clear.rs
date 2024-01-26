use rusqlite::Connection;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    UPDATE files SET index_date = null AND thumbnail_date = null
"##;

pub fn files_clear(tx: &Connection) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    sql_stmt.execute([])?;
    Ok(())
}