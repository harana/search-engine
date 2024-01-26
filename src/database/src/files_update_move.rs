use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

use crate::urlencoder::encode;

pub const QUERY: &str = r##"
    UPDATE files
    SET path = $new_path
    WHERE path = $old_path
"##;

pub fn files_update_move(tx: &Connection, old_path: String, new_path: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$old_path": encode(old_path),
        "$new_path": encode(new_path)
    })?;
    Ok(())
}