use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    DELETE FROM harana_jobs
    WHERE id = $id
"##;

pub fn tasks_delete(tx: &Connection, id: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$id": id,
    })?;
    Ok(())
}