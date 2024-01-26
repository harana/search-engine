use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO search_folders
    (title, name_or_path, is_path, enabled)
    VALUES
    ($title, $name_or_path, $is_path, $enabled)
"##;

pub fn search_folders_add(tx: &Connection, title: String, name_or_path: String, is_path: bool, enabled: bool) -> Result<i64> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$title": title,
        "$name_or_path": name_or_path,
        "$is_path": is_path,
        "$enabled": enabled
    })?;

    let job_id = tx.last_insert_rowid();
    Ok(job_id)
}