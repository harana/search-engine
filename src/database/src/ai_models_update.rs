use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    UPDATE ai_models
    SET enabled = $enabled
    WHERE id IS $id
"##;

pub fn ai_models_update(tx: &Connection, id: String, enabled: bool) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$id": id,
        "$enabled": enabled
    })?;

    Ok(())
}