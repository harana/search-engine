use rusqlite::Connection;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    DELETE FROM jobs
"##;

pub fn tasks_clear(tx: &Connection) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    sql_stmt.execute([])?;
    Ok(())
}