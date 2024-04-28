use std::path::Path;
use std::sync::Arc;

use harana_common::anyhow::{Error, Result};
use harana_common::arc_swap::ArcSwap;
use harana_common::futures::future::try_join_all;
use harana_common::parking_lot::Mutex;
use harana_common::hashbrown::HashMap;
use harana_common::itertools::Itertools;
use crate::structures::IndexDeclaration;
pub use crate::{
    structures,
    DocumentId,
    Index,
    QueryPayload,
    QueryResults,
    StorageBackend,
};

/// A manager around a set of indexes.
#[derive(Clone)]
pub struct IndexManager {
    declarations: Arc<Mutex<HashMap<String, IndexDeclaration>>>,
    indexes: Arc<ArcSwap<HashMap<String, Index>>>,
}

/// Creates a new unpopulated index manager.
impl Default for IndexManager {
    fn default() -> Self {
        Self {
            declarations: Arc::new(Mutex::new(HashMap::new())),
            indexes: Arc::new(ArcSwap::from_pointee(HashMap::new())),
        }
    }
}

impl IndexManager {
    /// Adds an index to the index from a given declaration.
    ///
    /// This duplicates the current indexes and swaps the clone, in general
    /// this is a very heavy operation and shouldn't be ran often / arbitrarily.
    pub async fn add_index(
        &self,
        index_path: &Path,
        declaration: IndexDeclaration,
        override_if_exists: bool,
        passphrase: String
    ) -> Result<()> {
        let mut indexes;
        {
            let guard = self.indexes.load();
            indexes = guard.as_ref().clone();
        }

        if !override_if_exists & indexes.get(declaration.name()).is_some() {
            return Err(Error::msg("index already exists."));
        }

        // remove the index if it exists
        if override_if_exists {
            self.remove_index(index_path, declaration.name()).await?;
        }

        let ctx = declaration.create_context(index_path, passphrase)?;
        let name = ctx.name();

        let built_index = Index::create(index_path, ctx, true).await?;

        indexes.insert(name, built_index);
        self.indexes.store(Arc::new(indexes));
        {
            self.declarations
                .lock()
                .insert(declaration.name().to_string(), declaration);
        }

        Ok(())
    }

    /// Removes an index to the index from the engine with a given name.
    /// This internally calls `Index.destroy()` to cleanup writers.
    pub async fn remove_index(&self, index_path: &Path, name: &str) -> Result<()> {
        let indexes = {
            let indexes = self.indexes.load();

            let mut indexes = indexes.as_ref().clone();
            if let Some(old) = indexes.remove(name) {
                old.destroy(index_path).await?;
            };

            indexes
        };

        self.indexes.store(Arc::new(indexes));

        {
            self.declarations.lock().remove(name);
        }

        Ok(())
    }

    /// Gets an index from the engine with the a given name.
    /// An error will be returned if the index does not exist.
    pub fn get_index(&self, index: &str) -> Option<Index> {
        let guard = self.indexes.load();
        let index = guard.get(index)?;

        Some(index.clone())
    }

    pub fn get_all_indexes(&self) -> Vec<Index> {
        let guard = self.indexes.load();
        guard.values().cloned().collect_vec()
    }

    pub fn get_all_declarations(&self) -> Vec<IndexDeclaration> {
        let guard = self.declarations.lock();
        guard.values().cloned().collect()
    }

    pub async fn commit_all_indexes(&self) -> Result<()> {
        let guard = self.indexes.load();
        try_join_all(
            guard.iter().map(|index| async move {
                index.1.commit().await
            }).collect_vec()
        ).await.map(|_| ())
    }

    pub async fn shutdown(&self) -> Result<()> {
        let guard = self.indexes.load();
        for (_, index) in guard.iter() {
            index.shutdown().await?;
        }
        Ok(())
    }
}