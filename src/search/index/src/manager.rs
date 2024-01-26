use std::fs;
use std::path::PathBuf;
use rust_embed::RustEmbed;

use harana_common::{serde_json, tokio_rayon};
use harana_common::anyhow::Result;
use harana_common::futures::future;
use harana_common::futures::future::join_all;
use harana_common::hashbrown::HashMap;
use harana_common::itertools::Itertools;
use harana_common::log::info;
use harana_search_core::categories::FileCategory;
use harana_tantivy::Index;
use harana_tantivy::index_manager::IndexManager as CoreIndexManager;
use harana_tantivy::schema::{BaseFieldOptions, CalculatedIntOptions, FieldDeclaration};
use harana_tantivy::structures::IndexDeclaration;

#[derive(RustEmbed)]
#[folder = "index_declarations"]
struct IndexDeclarations;

#[derive(Clone)]
pub struct IndexManager {
    index_manager: &'static CoreIndexManager,
    index_path: &'static PathBuf,
    index_declarations_path: &'static PathBuf
}

impl IndexManager {

    pub async fn new(index_path: &'static PathBuf, index_declarations_path: &'static PathBuf) -> Self {
        let index_manager = Box::leak(Box::new(CoreIndexManager::default()));

        Self {
            index_manager,
            index_path,
            index_declarations_path
        }
    }

    pub async fn create_indexes(&self) {
        // Built in indexes
        join_all(IndexDeclarations::iter().map(|index_decl_name| {
            let index_decl_file = IndexDeclarations::get(index_decl_name.as_ref());
            let index_decl_bytes = index_decl_file.unwrap().data.into_owned();
            let index_decl = std::str::from_utf8(index_decl_bytes.as_slice()).unwrap();
            self.setup_index(index_decl.to_string())
        }).collect_vec()).await;

        // Remote indexes
        let remote_indexes = fs::read_dir(self.index_declarations_path).unwrap().collect_vec();
        join_all(remote_indexes.into_iter().map(|index| {
            let index_declaration = fs::read_to_string(index.unwrap().path()).unwrap();
            self.setup_index(index_declaration)
        }).collect_vec()).await;
    }

    pub fn index_manager(&self) -> &'static CoreIndexManager {
        self.index_manager
    }

    pub async fn print_index_summary(&self) {
        info!("Index Summary ..");
        join_all(
            FileCategory::indexes().iter().map(move |name| async move {
                let count = self.get_index(name).get_documents_count().await.unwrap();
                println!("{}\t\t{} docs", name, count);
            }).collect_vec()
        ).await;
    }

    pub async fn shutdown(&self) {
        self.index_manager.shutdown().await.unwrap();
    }

    async fn setup_index(&self, index_declaration: String) {
        let mut index_decl: IndexDeclaration = serde_json::from_str(index_declaration.as_str()).unwrap();
        let int_options = CalculatedIntOptions { indexed: false, fieldnorms: None, fast: false, base: BaseFieldOptions { stored: true, multi: false, required: false }};

        let file_fields = HashMap::from([
            ("title".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: true, multi: false, required: true } }),
            ("description".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: true, multi: false, required: false } }),
            ("tags".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: true, multi: false, required: false } }),
            ("author".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: true, multi: false, required: false } }),
            ("primary_tokens".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: false, multi: true, required: false } }),
            ("secondary_tokens".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: false, multi: true, required: false } }),
            ("is_file".to_string(), FieldDeclaration::Boolean { opts: int_options }),
            ("created".to_string(), FieldDeclaration::U64 { opts: int_options }),
            ("modified".to_string(), FieldDeclaration::U64 { opts: int_options }),
            ("accessed".to_string(), FieldDeclaration::U64 { opts: int_options }),
            ("size".to_string(), FieldDeclaration::U64 { opts: int_options }),
            ("colour".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: true, multi: false, required: false } }),
            ("icon_url".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: false, multi: true, required: false } }),
            ("open_url".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: false, multi: true, required: false } }),
            ("path".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: true, multi: false, required: false } }),
            ("extension".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: true, multi: false, required: false } }),
            ("extension_title".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: true, multi: false, required: false } }),
            ("group_name".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: true, multi: false, required: false } }),
            ("metadata".to_string(), FieldDeclaration::String { opts: BaseFieldOptions { stored: true, multi: false, required: false } }),
        ]);

        index_decl.add_fields(file_fields);

        self.index_manager.add_index(self.index_path.as_path(), index_decl.clone(), false).await.unwrap();
    }

    pub fn get_index(&self, name: &str) -> Index {
        let index = self.index_manager.get_index(name);
        assert!(index.is_some(), "Failed to find index: {}", name);
        index.unwrap()
    }

    pub async fn log_indexes(&'static self) {
        let tasks = self.index_manager.get_all_indexes().into_iter().map(|index| async move {
            tokio_rayon::spawn(move || async move {
                println!("Index: {} ({})", index.name(), index.get_documents_count().await?);
                info!("Index: {} ({})", index.name(), index.get_documents_count().await?);
                Ok(())
            }).await.await
        }).collect_vec();

        let _: Result<Vec<()>> = future::try_join_all(tasks).await;
    }

    pub async fn commit_all_indexes(&'static self) {
        self.index_manager.commit_all_indexes().await.unwrap();
    }
}