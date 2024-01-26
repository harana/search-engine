use rusqlite::Connection;
use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;
use harana_common::time::OffsetDateTime;

use crate::HaranaAiModel;

pub const QUERY: &str = r##"
    SELECT * FROM ai_models
"##;

pub fn ai_models_list(tx: &Connection) -> Result<Vec<HaranaAiModel>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([], |row| {
            Ok(HaranaAiModel {
                id: row.get(0)?,
                status: row.get(1)?,
                enabled: row.get(2)?,
                indexer_type: row.get(3)?,
                name: row.get(4)?,
                version: row.get(5)?,
                creation_date: OffsetDateTime::from_unix_timestamp(row.get(6)?).unwrap()
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}