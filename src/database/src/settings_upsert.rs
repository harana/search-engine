use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO settings(key, value)
    VALUES ($key, $value)
    ON CONFLICT(key) DO UPDATE SET value = $value;
"##;

pub fn settings_upsert(tx: &Connection, key: String, value: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$key": key,
        "$value": value
    })?;

    Ok(())
}