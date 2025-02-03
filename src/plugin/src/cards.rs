use harana_common::chrono::{DateTime, Utc};
use harana_common::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(crate = "self::serde")]
#[serde(rename_all = "kebab-case")]
pub enum PluginCardBuiltin {
    Address {
        token: String,
        token_type: String,
        expiration: Option<DateTime<Utc>>,
    },
    GenericInfo {
        line_1_key: String,
        line_1_value: String,
        line_2_key: String,
        line_2_value: String,
    },
    HttpBasic {
        username: String,
        password: String,
    },
}
