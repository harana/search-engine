use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;
use harana_common::time::OffsetDateTime;

pub const QUERY: &str = r##"
    UPDATE jobs
    SET attempts = $attempts, last_attempt_date = $last_attempt_date
    WHERE id = $id
"##;

pub fn tasks_update(tx: &Connection, id: String, attempts: u8, last_attempt_date: OffsetDateTime) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$id": id,
        "$attempts": attempts,
        "$last_attempt_date": last_attempt_date.unix_timestamp()
    })?;

    Ok(())
}