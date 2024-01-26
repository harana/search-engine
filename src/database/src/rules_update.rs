use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    UPDATE rules
    SET source_type = $source_type, qualifier_type = $qualifier_type, qualifier_value = $qualifier_value, action_type = $action_type, action_value = $action_value
    WHERE id = $id
"##;

pub fn rules_update(tx: &Connection, id: String, source_type: String, qualifier_type: String, qualifier_value: String, action_type: String, action_value: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$id": id,
        "$source_type": source_type,
        "$qualifier_type": qualifier_type,
        "$qualifier_value": qualifier_value,
        "$action_type": action_type,
        "$action_value": action_value
    })?;

    Ok(())
}