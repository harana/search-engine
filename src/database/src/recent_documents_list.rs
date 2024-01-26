use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;

pub const QUERY: &str = r##"
    SELECT * FROM recent_documents WHERE search_query = $search_query
"##;

pub fn recent_documents_list(tx: &Connection, search_query: String) -> Result<Vec<String>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    let result = sql_stmt
        .query_row([search_query], |row| {
            let str: String = row.get(1)?;
            Ok(
                str.split(",")
                    .collect_vec()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect_vec()
            )
        })?;

    Ok(result)
}