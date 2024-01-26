use rusqlite::Connection;

use harana_common::anyhow::{anyhow, Result};
use harana_common::itertools::Itertools;
use harana_common::log::error;
use harana_common::time::OffsetDateTime;

use crate::urlencoder::encode;

pub fn files_bulk_update_index(tx: &Connection, documents: Vec<(String, String)>) -> Result<()> {

    let result = documents.chunks(50).into_iter().enumerate().map(|chunk| {
        let mut query = String::new();
        query.push_str("BEGIN;\n");

        chunk.1.into_iter().for_each(|pair| {
            let now = OffsetDateTime::now_utc().unix_timestamp().to_string();
            query.push_str("UPDATE files ");
            query.push_str(format!("SET process_date = {}, document_id = \"{}\" ", now, pair.1).as_str());
            query.push_str(format!("WHERE path IS \"{}\"; ", encode(pair.clone().0)).as_str());
        });

        query.push_str("COMMIT;\n");
        tx.execute_batch(query.as_str()).map_err(|e| {
            error!("Database failed: {}", e.to_string());
            anyhow!(e.to_string())
        })

    }).collect_vec();

    result.into_iter().collect()
}