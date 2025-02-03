use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    DELETE FROM authentication_token
    WHERE connection_id = $connection_id
"##;

pub fn authentication_token_delete(tx: &Connection, connection_id: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$connection_id": connection_id,
    })?;
    Ok(())
}