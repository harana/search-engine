use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

use crate::HaranaTask;

pub const QUERY: &str = r##"
    INSERT OR IGNORE INTO harana_jobs
    (id, priority, job_group_category, job_group_name, payload, attempts, last_attempt_date)
    VALUES
    ($id, $priority, $job_group_category, $job_group_name, $payload, $attempts, $last_attempt_date)
"##;

pub fn tasks_add(tx: &Connection, task: HaranaTask) -> Result<i64> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$id": task.id,
        "$priority": task.priority,
        "$job_group_category": task.job_group_category,
        "$job_group_name": task.job_group_name,
        "$payload": task.payload,
        "$attempts": task.attempts,
        "$last_attempt_date": task.last_attempt_date.unix_timestamp()
    })?;

    let job_id = tx.last_insert_rowid();
    Ok(job_id)
}