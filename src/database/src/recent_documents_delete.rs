use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    DELETE FROM recent_documents
    WHERE search_query = $search_query
"##;

pub fn recent_documents_delete(tx: &Connection, search_query: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$search_query": search_query,
    })?;
    Ok(())
}