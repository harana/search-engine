use std::path::PathBuf;
use std::sync::Arc;

use deadpool_sqlite::{Config, Pool};
use rusqlite::config::DbConfig;
use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};

use harana_common::anyhow::{anyhow, Result};
use harana_common::futures::TryFutureExt;
use harana_common::log::{debug, error, info};
use harana_common::thread_pools::{DATABASE_POOL, execute_future_result_operation};

const CORE_MIGRATIONS: [&str; 4] = [
    include_str!("../migrations/core-00001-init.sql"),
    include_str!("../migrations/core-00002-developer-sources.sql"),
    include_str!("../migrations/core-00003-search-categories.sql"),
    include_str!("../migrations/core-00004-search-folders.sql"),
];

const FILES_MIGRATIONS: [&str; 1] = [
    include_str!("../migrations/files-00001-init.sql")
];

pub struct DatabaseManager {
    core_pool: Arc<Pool>,
    files_pool: Arc<Pool>,
}

impl DatabaseManager {
    pub async fn new(database_path: &PathBuf) -> Self {
        let core_pool: Pool = Config::new(database_path.join("harana-core.db"))
            .builder(deadpool_sqlite::Runtime::Tokio1)
            .unwrap()
            .max_size(16)
            .build()
            .expect("Failed to build core pool");

        let files_pool: Pool = Config::new(database_path.join("harana-files.db"))
            .builder(deadpool_sqlite::Runtime::Tokio1)
            .unwrap()
            .max_size(16)
            .build()
            .expect("Failed to build files pool");

        let core_manager = core_pool.get().await.unwrap();
        core_manager.interact(move |mut c| {
            info!("Migrating core database ..");
            let core_migrations = Migrations::new(CORE_MIGRATIONS.iter().map(|m| M::up(m)).collect::<Vec<_>>());
            core_migrations.to_latest(&mut c).unwrap()
        }).await.expect("Failed to migrate core database");

        let files_manager = files_pool.get().await.unwrap();
        files_manager.interact(move |mut c| {
            info!("Migrating files database ..");
            let files_migrations = Migrations::new(FILES_MIGRATIONS.iter().map(|m| M::up(m)).collect::<Vec<_>>());
            files_migrations.to_latest(&mut c).unwrap()
        }).await.expect("Failed to migrate files database");

        Self {
            core_pool: Arc::new(core_pool),
            files_pool: Arc::new(files_pool)
        }
    }

    pub async fn core<F, R>(&'static self, f: F) -> Result<R> where
        F: FnOnce(&Connection) -> Result<R> + Send + 'static,
        R: Send + 'static
    {
        execute_future_result_operation(&DATABASE_POOL, move || async move {
            let connection = self.core_pool.get().await.unwrap();
            connection
                .interact(move |conn| {
                    conn.trace(Some(|msg|{ debug!("{}", msg); }));
                    f(conn)
                })
                .map_err(|e| {
                    anyhow!(e.to_string())
                })
                .await?
        }).await
    }

    pub async fn files<F, R>(&'static self, f: F) -> Result<R> where
        F: FnOnce(&Connection) -> Result<R> + Send + 'static,
        R: Send + 'static
    {
        execute_future_result_operation(&DATABASE_POOL, move || async move {
            let connection = self.files_pool.get().await.unwrap();
            connection
                .interact(move |conn| {
                    conn.trace(Some(|msg|{ debug!("{}", msg); }));
                    f(conn)
                })
                .map_err(|e| {
                    error!("{}", e.to_string());
                    anyhow!(e.to_string())
                })
                .await?
        }).await
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use harana_common::tokio;

    use crate::manager::DatabaseManager;

    #[tokio::test]
    async fn test() {
        let manager = DatabaseManager::new(&PathBuf::from("/tmp")).await;
    }
}