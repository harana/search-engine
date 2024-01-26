use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO recent_documents
    (query, documents)
    VALUES
    ($query, $documents)
"##;

pub fn recent_documents_add(
    tx: &Connection, query: String, documents: Vec<&str>) -> Result<i64> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$query": query,
        "$documents": documents.join(",")
    })?;

    let job_id = tx.last_insert_rowid();
    Ok(job_id)
}