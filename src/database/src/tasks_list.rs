use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;
use harana_common::time::OffsetDateTime;

use crate::HaranaTask;

pub const QUERY: &str = r##"
    SELECT * FROM harana_jobs ORDER by priority
"##;

pub fn tasks_list(tx: &Connection) -> Result<Vec<HaranaTask>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([], |row| {
            Ok(HaranaTask {
                id: row.get(0)?,
                priority: row.get(1)?,
                job_group_category: row.get(2)?,
                job_group_name: row.get(3)?,
                payload: row.get(4)?,
                attempts: row.get(5)?,
                last_attempt_date: OffsetDateTime::from_unix_timestamp(row.get(6)?).unwrap()
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}