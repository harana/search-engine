use rusqlite::Connection;

use harana_common::anyhow::{anyhow, Result};
use harana_common::itertools::Itertools;
use harana_common::log::error;

pub fn tasks_bulk_delete(tx: &Connection, ids: Vec<&str>) -> Result<()> {

    let result = ids.chunks(50).into_iter().enumerate().map(|chunk| {
        let mut query = String::new();
        query.push_str("BEGIN;\n");

        chunk.1.into_iter().for_each(|id| {
            query.push_str(format!("DELETE FROM harana_jobs WHERE id = \"{}\"; ", id).as_str());
        });

        query.push_str("COMMIT;\n");
        tx.execute_batch(query.as_str()).map_err(|e| {
            error!("Database failed: {}", e.to_string());
            anyhow!(e.to_string())
        })

    }).collect_vec();

    result.into_iter().collect()
}