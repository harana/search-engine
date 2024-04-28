pub mod corrections;
pub mod encrypted_dir;
pub mod encrypted_stream;
pub mod helpers;
pub mod index;
pub mod index_manager;
pub mod path_facet_converter;
pub mod query;
pub mod reader;
pub mod schema;
pub mod snapshot;
pub mod stop_words;
pub mod storage;
pub mod structures;
pub mod synonyms;
pub mod tokenizer;
pub mod writer;

use std::time::{SystemTime, UNIX_EPOCH};
use harana_common::chrono::NaiveDateTime;
use harana_common::chrono;
pub use helpers::cr32_hash;
pub use index::Index;
pub use query::DocumentId;
pub use reader::{QueryPayload, QueryResults};
pub use storage::StorageBackend;


pub fn system_time_to_rfc3389(system_time: Option<SystemTime>) -> String {
    match system_time {
        Some(time) => {
            let std_duration = time.duration_since(UNIX_EPOCH).unwrap();
            let chrono_duration = chrono::Duration::from_std(std_duration).unwrap();
            let unix = NaiveDateTime::from_timestamp_opt(0, 0).unwrap();
            let naive = unix + chrono_duration;
            naive.and_utc().to_rfc3339()
        }
        None => "".to_string()
    }
}
