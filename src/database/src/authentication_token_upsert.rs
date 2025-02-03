use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO authentication_token(connection_id, token_secret_id, token_type, expiration)
    VALUES ($connection_id, $token_secret_id, $token_type, $expiration)
    ON CONFLICT(connection_id) DO UPDATE SET token_secret_id = $token_secret_id, token_type = $token_type, token_type = $token_type;
"##;

pub fn authentication_http_basic_upsert(
    tx: &Connection,
    connection_id: String,
    token_secret_id: String,
    token_type: String,
    expiration: String,
) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$connection_id": connection_id,
        "$token_secret_id": token_secret_id,
        "$token_type": token_type,
        "$expiration": expiration,
    })?;

    Ok(())
}