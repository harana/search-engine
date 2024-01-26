use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;

use crate::HaranaAdjustedTerm;

pub const QUERY: &str = r##"
    SELECT * FROM adjusted_terms
    WHERE user_added IS 1
"##;

pub fn adjusted_terms_list(tx: &Connection) -> Result<Vec<HaranaAdjustedTerm>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([], |row| {
            let direction: u8 = row.get(3)?;
            let user_added: u8 = row.get(4)?;

            Ok(HaranaAdjustedTerm {
                category: row.get(0)?,
                term: row.get(1)?,
                language: row.get(2)?,
                direction: direction == 1,
                user_added: user_added == 1
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}