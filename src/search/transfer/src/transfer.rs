use std::path::PathBuf;

use harana_common::itertools::Itertools;
use harana_common::serde_json;
use harana_common::serde_json::Value;
use harana_common::tinyrand::{Rand, StdRand};
use harana_common::tokio::runtime::Runtime;
use harana_search_index::manager::IndexManager;

use crate::transfer_document::TransferDocument;

pub struct Transfer {}

impl Transfer {
    pub fn from_external(index_path: String, index_declarations_path: String, index_name: String, document_payloads_json: String) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut rand = StdRand::default();
            let mut rand_next = rand.next_u64();

            let index_path_ref = Box::leak(Box::new(PathBuf::from(index_path)));
            let index_declarations_path_ref = Box::leak(Box::new(PathBuf::from(index_declarations_path)));

            let index_manager = IndexManager::new(index_path_ref, index_declarations_path_ref).await;
            let index = index_manager.get_index(index_name.as_str());

            let document_payloads_value: Value = serde_json::from_str(document_payloads_json.as_str()).unwrap();
            match document_payloads_value.clone().get("documents").unwrap() {
                Value::Array(values) => {
                    let documents = values.clone().into_iter().enumerate().map(|(index, value)| {
                        let document_id = rand_next + (index as u64);
                        let document: TransferDocument = serde_json::from_value(value).unwrap();
                        (document_id, document.to_tantivy_payload())
                    }).collect_vec();
                    index.add_documents(documents).await.expect("Failed to add documents");
                }
                _ => {}
            }
            index.commit().await.expect("Failed to commit");
        });
    }
}