use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;
use crate::HaranaConnectionParameter;

pub const QUERY: &str = r##"
    SELECT * FROM connections_parameters WHERE connection_id = $connection_id
"##;

pub fn connection_parameters_list(tx: &Connection, connection_id: String) -> Result<Vec<HaranaConnectionParameter>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([connection_id], |row| {
            Ok(HaranaConnectionParameter {
                connection_id: row.get(0)?,
                key: row.get(1)?,
                value: row.get(2)?,
                value_type: row.get(3)?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}