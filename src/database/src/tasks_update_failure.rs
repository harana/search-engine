use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;
use harana_common::time::OffsetDateTime;

pub const QUERY: &str = r##"
    UPDATE harana_jobs
    SET attempts = attempts + 1, last_attempt_date = $last_attempt_date
    WHERE id = $id
"##;

pub fn tasks_update_failure(tx: &Connection, id: String, last_attempt_date: OffsetDateTime) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$id": id,
        "$last_attempt_date": last_attempt_date.unix_timestamp()
    })?;

    Ok(())
}