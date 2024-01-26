use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;
use harana_common::time::OffsetDateTime;

pub const QUERY: &str = r##"
    INSERT INTO ai_models
    (id, status, enabled, indexer_type, name, version, creation_date)
    VALUES
    ($id, $status, $enabled, $indexer_type, $name, $version, $creation_date)
"##;

pub fn ai_models_add(
    tx: &Connection,
    id: String,
    status: String,
    enabled: bool,
    indexer_type: String,
    name: String,
    version: String,
    creation_date: OffsetDateTime
) -> Result<i64> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$id": id,
        "$status": status,
        "$enabled": enabled,
        "$indexer_type": indexer_type,
        "$name": name,
        "$version": version,
        "$creation_date": creation_date.unix_timestamp(),
    })?;

    let job_id = tx.last_insert_rowid();
    Ok(job_id)
}