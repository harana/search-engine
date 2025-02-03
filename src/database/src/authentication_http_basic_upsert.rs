use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO authentication_http_basic(connection_id, username_secret_id, password_secret_id)
    VALUES ($connection_id, $username_secret_id, $password_secret_id)
    ON CONFLICT(connection_id) DO UPDATE SET username_secret_id = $username_secret_id, password_secret_id = $password_secret_id;
"##;

pub fn authentication_http_basic_upsert(
    tx: &Connection,
    connection_id: String,
    username_secret_id: String,
    password_secret_id: String
) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$connection_id": connection_id,
        "$username_secret_id": username_secret_id,
        "$password_secret_id": password_secret_id
    })?;

    Ok(())
}