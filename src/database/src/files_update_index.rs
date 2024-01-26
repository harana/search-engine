use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;
use harana_common::time::OffsetDateTime;

use crate::urlencoder::encode;

pub const QUERY: &str = r##"
    UPDATE files
    SET index_date = $index_date
    WHERE path = $path
"##;

pub fn files_update_index(tx: &Connection, path: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$path": encode(path),
        "$index_date": OffsetDateTime::now_utc().unix_timestamp(),
    })?;
    Ok(())
}