use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;

use crate::HaranaRule;

pub const QUERY: &str = r##"
    SELECT * FROM rules
"##;

pub fn rules_list(tx: &Connection) -> Result<Vec<HaranaRule>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([], |row| {
            Ok(HaranaRule {
                id: row.get(0)?,
                source_type: row.get(1)?,
                qualifier_type: row.get(2)?,
                qualifier_value: row.get(3)?,
                action_type: row.get(4)?,
                action_value: row.get(5)?
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}