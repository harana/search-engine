use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;
use harana_common::time::OffsetDateTime;

pub const QUERY: &str = r##"
    UPDATE connections
    SET status = $status, name = $name, authentication_type = $authentication_type
    WHERE id = $id
"##;

pub fn connections_update(tx: &Connection, id: String, status: String, name: String, authentication_type: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$id": id,
        "$status": status,
        "$name": name,
        "$authentication_type": authentication_type,
    })?;

    Ok(())
}