use std::path::PathBuf;

use jni::JNIEnv;
use jni::objects::{JClass, JString};

use harana_common::itertools::Itertools;
use harana_common::serde_json;
use harana_common::serde_json::Value;
use harana_common::tinyrand::{Rand, StdRand};
use harana_common::tokio::runtime::Runtime;
use harana_search_index::manager::IndexManager;

use crate::document::Document;

mod document;

#[no_mangle]
pub extern "system" fn Java_Search_debug<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    index_path: JString<'local>,
    index_declarations_path: JString<'local>,
    passphrase: JString<'local>
) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let index_path_ref = Box::leak(Box::new(PathBuf::from(String::from(env.get_string(&index_path).expect("Couldn't get index_path")))));
        let index_declarations_path_ref = Box::leak(Box::new(PathBuf::from(String::from(env.get_string(&index_declarations_path).expect("Couldn't get index_declarations_path")))));

        let index_manager = Box::leak(Box::new(IndexManager::new(index_path_ref, index_declarations_path_ref).await));
        index_manager.create_indexes(passphrase.).await;
        index_manager.log_indexes().await;
    });
}

#[no_mangle]
pub extern "system" fn Java_Search_add_documents<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    index_path: JString<'local>,
    index_declarations_path: JString<'local>,
    index_name: JString<'local>,
    document_payloads_json: JString<'local>
) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut rand = StdRand::default();
        let mut rand_next = rand.next_u64();

        let index_path_ref = Box::leak(Box::new(PathBuf::from(String::from(env.get_string(&index_path).expect("Couldn't get index_path")))));
        let index_declarations_path_ref = Box::leak(Box::new(PathBuf::from(String::from(env.get_string(&index_declarations_path).expect("Couldn't get index_declarations_path")))));
        let index_name = String::from(env.get_string(&index_name).expect("Couldn't get index_name"));
        let index_manager = IndexManager::new(index_path_ref, index_declarations_path_ref).await;
        let index = index_manager.get_index(index_name.as_str());

        let document_payloads_value: Value = serde_json::from_str(String::from(env.get_string(&document_payloads_json).expect("Couldn't get document_payloads")).as_str()).unwrap();
        match document_payloads_value.clone().get("documents").unwrap() {
            Value::Array(values) => {
                let documents = values.clone().into_iter().enumerate().map(|(index, value)| {
                    let document_id = rand_next + (index as u64);
                    let document: Document = serde_json::from_value(value).unwrap();
                    (document_id, document.to_tantivy_payload())
                }).collect_vec();
                index.add_documents(documents).await.expect("Failed to add documents");
            }
            _ => {}
        }
        index.commit().await.expect("Failed to commit");
    });
}