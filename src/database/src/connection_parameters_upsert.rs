use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO connection_parameters(connection_id, key, value)
    VALUES ($connection_id, $key, $value)
    ON CONFLICT(connection_id, key) DO UPDATE SET value = $value;
"##;

pub fn connection_parameters_upsert(
    tx: &Connection,
    connection_id: String,
    key: String,
    value: String,
) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$connection_id": connection_id,
        "$key": key,
        "$value": value,
    })?;

    Ok(())
}