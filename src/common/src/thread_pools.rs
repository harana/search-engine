use std::future::Future;
use std::sync::{Arc, OnceLock};

use crate::anyhow::{anyhow, Context, Result};
use crate::arc_swap::ArcSwap;
use futures::future;
use crate::tokio;
use crate::tokio::runtime::Runtime;
use crate::tokio::task::JoinHandle;

pub static DATABASE_POOL: OnceLock<ArcSwap<Runtime>> = OnceLock::new();
pub static FILE_CRAWLING_POOL: OnceLock<ArcSwap<Runtime>> = OnceLock::new();
pub static SEARCH_POOL: OnceLock<ArcSwap<Runtime>> = OnceLock::new();

pub fn build_default_pools() -> Result<()> {

    let database_pool = new_runtime_pool("database", 40)?;
    let file_crawling_pool = new_runtime_pool("file-crawling", 8)?;
    let search_pool = new_runtime_pool("search", 2)?;

    if let Some(existing) = DATABASE_POOL.get() {
        existing.store(Arc::new(database_pool));
    } else {
        let _ = DATABASE_POOL.set(ArcSwap::from_pointee(database_pool));
    }

    if let Some(existing) = FILE_CRAWLING_POOL.get() {
        existing.store(Arc::new(file_crawling_pool));
    } else {
        let _ = FILE_CRAWLING_POOL.set(ArcSwap::from_pointee(file_crawling_pool));
    }

    if let Some(existing) = SEARCH_POOL.get() {
        existing.store(Arc::new(search_pool));
    } else {
        let _ = SEARCH_POOL.set(ArcSwap::from_pointee(search_pool));
    }

    Ok(())
}

pub fn execute_future_operation<OP, Fut, T>(pool: &'static OnceLock<ArcSwap<Runtime>>, op: OP) -> JoinHandle<T>
    where
        T: Send + 'static,
        Fut: Future<Output = T> + Send + 'static,
        OP: FnOnce() -> Fut + Send + 'static,
{
    spawn(pool, op())
}

pub async fn execute_future_result_operation<OP, Fut, T>(pool: &'static OnceLock<ArcSwap<Runtime>>, op: OP) -> Result<T>
    where
        T: Send + 'static,
        Fut: Future<Output = Result<T>> + Send + 'static,
        OP: FnOnce() -> Fut + Send + 'static,
{
    let result = spawn(pool, op()).await?;

    match result {
        Ok(ok) => Ok(ok),
        Err(e) => Err(anyhow!(e.to_string()))
    }
}

pub async fn execute_operation<OP, T>(pool: &'static OnceLock<ArcSwap<Runtime>>, op: OP) -> Result<T>
    where
        T: Send + 'static,
        OP: FnOnce(&mut std::task::Context<'_>) -> Result<T> + Send + 'static,
{
    let future = future::lazy(op);
    let result = spawn(pool, future).await?;

    match result {
        Ok(ok) => Ok(ok),
        Err(e) => Err(anyhow!(e.to_string()))
    }
}

pub fn spawn<OP, T>(pool: &'static OnceLock<ArcSwap<Runtime>>, op: OP) -> JoinHandle<T>
    where
        T: Send + 'static,
        OP: Future<Output = T> + Send + 'static,
{
    pool
        .get()
        .expect("Executor pool should be initialised.")
        .load()
        .spawn(op)
}

fn new_runtime_pool(
    nickname: &'static str,
    num_threads: usize,
) -> Result<Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .thread_name(format!("harana-{nickname}"))
        .worker_threads(num_threads)
        .enable_all()
        .build()
        .context("Build runtime executor")
}
