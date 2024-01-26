use rusqlite::Connection;

use harana_common::anyhow::Result;
use harana_common::itertools::Itertools;
use harana_common::time::OffsetDateTime;

use crate::HaranaFile;
use crate::urlencoder::decode;

pub const QUERY: &str = r##"
    SELECT * FROM files WHERE path like ?%
"##;

pub fn files_list_prefix(tx: &Connection, path_prefix: String) -> Result<Vec<HaranaFile>> {
    let mut sql_stmt = tx.prepare_cached(QUERY)?;
    let results = sql_stmt
        .query_map([path_prefix], |row| {
            let index_date: Option<i64> = row.get(2)?;
            let thumbnail_date: Option<i64> = row.get(3)?;
            Ok(HaranaFile {
                path: decode(row.get(0)?)?,
                hash: row.get(1)?,
                index_date: index_date.map(|d| OffsetDateTime::from_unix_timestamp(d).unwrap()),
                thumbnail_date: thumbnail_date.map(|d| OffsetDateTime::from_unix_timestamp(d).unwrap()),
                document_id: row.get(4)?
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok(results)
}