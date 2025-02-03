use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    DELETE FROM connections
    WHERE id = $id
"##;

pub fn connections_delete(tx: &Connection, id: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$id": id,
    })?;
    Ok(())
}