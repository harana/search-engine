use std::path::PathBuf;

use rusqlite::Connection;

use harana_common::anyhow::{anyhow, Result};
use harana_common::itertools::Itertools;

use crate::urlencoder::encode;

pub fn files_bulk_add(tx: &Connection, pairs: Vec<(PathBuf, u64)>) -> Result<()> {
    let result = pairs.chunks(50).into_iter().enumerate().map(|chunk| {
        let mut query = String::new();
        query.push_str("BEGIN;\n");

        chunk.1.into_iter().for_each(|pair| {
            query.push_str("INSERT OR IGNORE INTO files (path, document_id) VALUES ");
            query.push_str(format!("('{}', '{}');", encode(pair.0.to_str().unwrap().to_string()), pair.1.to_string()).as_str());
        });

        query.push_str("COMMIT;\n");
        tx.execute_batch(query.as_str()).map_err(|e| anyhow!(e.to_string()))

    }).collect_vec();

    result.into_iter().collect()
}