use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO developer_sources
    (title, name, language, enabled)
    VALUES
    ($title, $name, $language, $enabled)
"##;

pub fn developer_sources_add(tx: &Connection, title: String, name: String, language: String, enabled: bool) -> Result<i64> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$title": title,
        "$name": name,
        "$language": language,
        "$enabled": enabled
    })?;

    let job_id = tx.last_insert_rowid();
    Ok(job_id)
}