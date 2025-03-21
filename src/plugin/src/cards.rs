use harana_common::chrono::{DateTime, Utc};
use harana_common::serde::{self, Deserialize, Serialize};

#[serde(crate = "self::serde")]
#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
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
