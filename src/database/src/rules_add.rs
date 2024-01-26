use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO rules
    (id, source_type, qualifier_type, qualifier_value, action_type, action_value)
    VALUES
    ($id, $source_type, $qualifier_type, $qualifier_value, $action_type, $action_value)
"##;

pub fn rules_add(
    tx: &Connection,
    id: String,
    source_type: String,
    qualifier_type: String,
    qualifier_value: String,
    action_type: bool,
    action_value: bool
) -> Result<i64> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$id": id,
        "$source_type": source_type,
        "$qualifier_type": qualifier_type,
        "$qualifier_value": qualifier_value,
        "$action_type": action_type,
        "$action_value": action_value
    })?;

    let job_id = tx.last_insert_rowid();
    Ok(job_id)
}