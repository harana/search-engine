use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO adjusted_terms
    (category, term, language, direction, user_added)
    VALUES
    ($category, $term, $language, $direction, $user_added)
"##;

pub fn adjusted_terms_add(
    tx: &Connection,
    category: String,
    term: String,
    language: String,
    direction: bool,
    user_added: bool
) -> Result<i64> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$category": category,
        "$term": term,
        "$language": language,
        "$direction": if direction { 1 } else { 0 },
        "$user_added": if user_added { 1 } else { 0 }
    })?;

    let job_id = tx.last_insert_rowid();
    Ok(job_id)
}