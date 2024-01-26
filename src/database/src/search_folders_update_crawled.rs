use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    UPDATE search_folders
    SET crawled = $crawled
    WHERE name_or_path = $name_or_path AND is_path is $is_path
"##;

pub fn search_folders_update_crawled(tx: &Connection, name_or_path: String, is_path: bool, crawled: bool) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$name_or_path": name_or_path,
        "$is_path": is_path,
        "$crawled": crawled
    })?;

    Ok(())
}