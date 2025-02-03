use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO connections
    (id, status, enabled, plugin, name, creation_date)
    VALUES
    ($id, $status, $enabled, $plugin, $name, $creation_date)
"##;

pub fn connections_add(
    tx: &Connection,
    id: String,
    status: String,
    plugin: String,
    name: String,
    authentication_type: String
) -> Result<i64> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$id": id,
        "$status": status,
        "$plugin": plugin,
        "$name": name,
        "$authentication_type": authentication_type,
    })?;

    let job_id = tx.last_insert_rowid();
    Ok(job_id)
}