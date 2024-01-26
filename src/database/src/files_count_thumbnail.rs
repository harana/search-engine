use rusqlite::Connection;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    SELECT COUNT(*) FROM files WHERE thumbnail_date IS NOT NULL
"##;

pub fn files_count_thumbnail(tx: &Connection) -> Result<usize> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    let result: usize = sql_stmt
        .query_row([], |row| {
            Ok(row.get(0)?)
        })?;

    Ok(result)
}