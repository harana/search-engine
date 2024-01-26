use harana_common::hashbrown::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use ical::parser::ical::component::*;
use ical::property::Property;

use harana_common::anyhow::Result;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct IcalMetadata {
    alarms: Vec<IcalAlarm>,
    events: Vec<IcalEvent>,
    free_busys: Vec<IcalFreeBusy>,
    journals: Vec<IcalJournal>,
    properties: Vec<Property>,
    todos: Vec<IcalTodo>,
    timezones: Vec<IcalTimeZone>,
}

pub struct IndexerIcal;

impl Indexer for IndexerIcal {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let buf = BufReader::new(File::open(path.clone())?);
        let reader = ical::IcalParser::new(buf);

        let mut primary_tokens = HashSet::<String>::new();

        let mut alarms = Vec::new();
        let mut events = Vec::new();
        let mut free_busys = Vec::new();
        let mut journals = Vec::new();
        let mut properties = Vec::new();
        let mut todos = Vec::new();
        let mut timezones = Vec::new();

        reader.into_iter().for_each(|ical| {
            if ical.is_ok() {
                let ical = ical.unwrap();
                alarms.extend(ical.alarms);
                events.extend(ical.events);
                free_busys.extend(ical.free_busys);
                journals.extend(ical.journals);
                properties.extend(ical.properties);
                todos.extend(ical.todos);
                timezones.extend(ical.timezones);
            }
        });

        alarms.iter().for_each(|a| {
            a.properties.iter().for_each(|p| {
                if p.name == "DESCRIPTION" || p.name == "LOCATION" {
                    if p.value.is_some() {
                        let value = p.value.clone().unwrap();
                        primary_tokens.extend(tokenize(value.as_str()));
                    }
                }
            });
        });

        events.iter().for_each(|e| {
            e.properties.iter().for_each(|p| {
                if p.name == "DESCRIPTION" || p.name == "LOCATION" || p.name == "SUMMARY" {
                    if p.value.is_some() {
                        let value = p.value.clone().unwrap();
                        primary_tokens.extend(tokenize(value.as_str()));
                    }
                }
            });

            e.alarms.iter().for_each(|a| {
                a.properties.iter().for_each(|p| {
                    if p.name == "DESCRIPTION" || p.name == "LOCATION" {
                        if p.value.is_some() {
                            let value = p.value.clone().unwrap();
                            primary_tokens.extend(tokenize(value.as_str()));
                        }
                    }
                });
            });
        });

        let metadata = IcalMetadata {
            alarms,
            events,
            free_busys,
            journals,
            properties,
            todos,
            timezones,
        };

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens,
            secondary_tokens: HashSet::new(),
            metadata: serde_json::to_value(&metadata)?
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_ical::IndexerIcal;

    #[tokio::test]
    async fn test_indexing() {
        let indexed_file = IndexerIcal.index("../../../test_files/Sample1.ics").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}