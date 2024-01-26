use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO private_folders
    (path)
    VALUES
    ($path)
"##;

pub fn private_folders_add(tx: &Connection, path: String) -> Result<i64> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$path": path
    })?;

    let job_id = tx.last_insert_rowid();
    Ok(job_id)
}