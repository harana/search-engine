use harana_common::anyhow::Result;
use curl::easy::{Easy, List};
use std::io::Read;
use once_cell::sync::OnceCell;
use flume::{Receiver, Sender};

static RECENT_SEARCHES: OnceCell<(Sender<String>, Receiver<String>)> = OnceCell::new();

pub fn recent_searches() -> Result<Vec<String>> {
    let receiver = RECENT_SEARCHES.get_or_init(|| { flume::unbounded() }).to_owned().1;
    Ok(receiver.iter().collect::<Vec<String>>())
}

pub fn search(query: String, indexes: Vec<String>, meilisearch_url: String, meilisearch_key: String) -> Result<String> {
    let sender = RECENT_SEARCHES.get_or_init(|| { flume::unbounded() }).to_owned().0;
    sender.send(query.to_owned())?;

    let queries = indexes
        .iter()
        .map(|index|
            format!("{{ \"indexUid\": {}, \"q\": {}, \"limit\": {}}}", String::fromindex, query, 1)
        )
        .collect::<Vec<String>>()
        .join(",");
    let all_queries = format!("\"queries\": [{}]", queries.to_string());
    let mut data = all_queries.as_bytes();

    let mut http_client = Easy::new();

    let mut headers = List::new();
    http_client.url(&meilisearch_url)?;
    headers.append(format!("Authorization: Basic {}", meilisearch_key).as_str())?;
    headers.append("Content-Type: application/json")?;
    http_client.http_headers(headers)?;

    http_client.post(true)?;
    http_client.post_field_size(data.len() as u64)?;

    let mut out_buffer = Vec::new();
    {
        let mut transfer = http_client.transfer();
        transfer.read_function(|buf| { Ok(data.read(buf).unwrap_or(0)) })?;
        transfer.write_function(|data| { out_buffer.extend_from_slice(data); Ok(data.len()) })?;
        transfer.perform()?;
        drop(transfer);
    }

    Ok(String::from_utf8(out_buffer)?)
}