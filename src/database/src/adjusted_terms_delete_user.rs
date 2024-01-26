use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    DELETE FROM adjusted_term
    WHERE term = $term and language = $language
"##;

pub fn adjusted_terms_delete_user(tx: &Connection, term: String, language: String) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$term": term,
        "$language": language,
    })?;
    Ok(())
}