use rusqlite::Connection;
use rusqlite::named_params;

use harana_common::anyhow::Result;

pub const QUERY: &str = r##"
    INSERT INTO authentication_oauth(connection_id, client_id, client_secret_id, auth_url, token_url, redirect_url, scope, access_token, refresh_token, expiration)
    VALUES ($connection_id, $client_id, $client_secret_id, $auth_url, $token_url, $redirect_url, $scope, $access_token, $refresh_token, $expiration)
    ON CONFLICT(connection_id) DO UPDATE SET client_id = $client_id, client_secret_id = $client_secret_id, auth_url = $auth_url, token_url = $token_url, redirect_url = $redirect_url, scope = $scope, access_token = $access_token, refresh_token = $refresh_token, expiration = $expiration;
"##;

pub fn authentication_oauth_upsert(
    tx: &Connection,
    connection_id: String,
    client_id: String,
    client_secret_id: String,
    auth_url: String,
    token_url: String,
    redirect_url: String,
    scope: String,
    access_token: String,
    refresh_token: String,
    expiration: String,
) -> Result<()> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;

    sql_stmt.execute(named_params! {
        "$connection_id": connection_id,
        "$client_id": client_id,
        "$client_secret_id": client_secret_id,
        "$auth_url": auth_url,
        "$token_url": token_url,
        "$redirect_url": redirect_url,
        "$scope": scope,
        "$access_token": access_token,
        "$refresh_token": refresh_token,
        "$expiration": expiration,
    })?;

    Ok(())
}