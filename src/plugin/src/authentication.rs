use harana_common::chrono::{DateTime, Utc};
use harana_common::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
#[serde(rename_all = "kebab-case")]
pub enum Authentication {
    Token {
        token: String,
        token_type: String,
        expiration: Option<DateTime<Utc>>,
    },
    OAuth {
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        redirect_uri: String,
        scope: Vec<String>,
        access_token: Option<String>,
        refresh_token: Option<String>,
        expiration: Option<DateTime<Utc>>,
    },
    HttpBasic {
        username: String,
        password: String,
    },
}

impl Authentication {
    fn is_expired(&self) -> bool {
        match self {
            Authentication::Token { expiration, .. } |
            Authentication::OAuth { expiration, .. } => {
                expiration
                    .map(|exp| exp < Utc::now())
                    .unwrap_or(false)
            },
            Authentication::HttpBasic { .. } => false,
        }
    }

    fn get_token(&self) -> Option<&str> {
        match self {
            Authentication::Token { token, .. } => Some(token),
            Authentication::OAuth { access_token, .. } => access_token.as_deref(),
            Authentication::HttpBasic { .. } => None,
        }
    }
}