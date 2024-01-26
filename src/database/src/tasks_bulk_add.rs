use rusqlite::Connection;

use harana_common::anyhow::{anyhow, Result};
use harana_common::itertools::Itertools;

use crate::HaranaTask;

pub fn tasks_bulk_add(tx: &Connection, tasks: Vec<HaranaTask>) -> Result<()> {

    let result = tasks.chunks(500).into_iter().enumerate().map(|chunk| {
        let mut query = String::new();
        query.push_str("BEGIN;\n");

        chunk.1.into_iter().for_each(|task| {
            query.push_str("INSERT OR IGNORE INTO harana_jobs (id, priority, job_group_category, job_group_name, payload, attempts, last_attempt_date) VALUES ");
            query.push_str(format!("('{}', '{}', '{}', '{}', '{}', '{}', '{}'); ", task.id, task.priority, task.job_group_category, task.job_group_name, task.payload, task.attempts, task.last_attempt_date.unix_timestamp()).as_str())
        });

        query.push_str("COMMIT;\n");
        tx.execute_batch(query.as_str()).map_err(|e| anyhow!(e.to_string()))

    }).collect_vec();

    result.into_iter().collect()
}