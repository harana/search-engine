use std::io;
use std::collections::BTreeMap;
use std::fs::Metadata;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Instant;

use filesize::PathExt;

use harana_common::tinyrand::{Rand, StdRand};
use harana_common::anyhow::Result;
use harana_common::file_format::FileFormat;
use harana_common::futures::future::{join_all, try_join_all};
use harana_common::futures::{future, TryFutureExt};
use harana_common::itertools::Itertools;
use harana_common::log::info;
use harana_common::random::random;
use harana_common::{serde_json, tokio_rayon};
use harana_common::thread_pools::{execute_future_result_operation, FILE_CRAWLING_POOL};
use harana_database::manager::DatabaseManager;
use harana_database::files_bulk_add::files_bulk_add;
use harana_database::search_folders_update_crawled::search_folders_update_crawled;
use harana_database::job_groups_add::job_groups_add;
use harana_file_crawler::FileCrawlerBuilder;
use harana_search_extensions::extensions::Extensions;
use harana_search_extensions::file::extension_details;
use harana_search_index::manager::IndexManager;
use harana_search_jobs::job_hash::JobHandlerHashPayload;
use harana_search_jobs::job_index::JobHandlerIndexPayload;
use harana_search_jobs::job_thumbnail::JobHandlerThumbnailPayload;
use harana_tantivy::structures::{DocumentPayload, DocumentValue, DocumentValueOptions};
use harana_job::manager::JobManager;
use harana_tantivy::structures::DocumentValue::*;

pub async fn tantivy_payload(path: PathBuf, metadata: Option<Metadata>, format: Option<FileFormat>) -> DocumentPayload {
    let extension = path.extension().and_then(|e| e.to_os_string().into_string().ok());
    let extension_title = extension.clone().map(|e| Extensions::title(format, e.as_str()));
    let file_name = path.file_name().and_then(|s| s.to_str().map(|s| s.to_string()));
    let title = file_name.unwrap();
    let is_file = metadata.clone().map(|m| m.is_file());
    let created = metadata.clone().and_then(|m| time_to_u64(m.created()));
    let modified = metadata.clone().and_then(|m| time_to_u64(m.modified()));
    let accessed = metadata.clone().and_then(|m| time_to_u64(m.accessed()));
    let size = metadata.clone().and_then(|m| path.size_on_disk_fast(&m).ok());

    let map = BTreeMap::from([
        ("title".to_string(), DocumentValueOptions::Single(Text(title))),
        // ("is_file".to_string(), DocumentValueOptions::Single(Boolean(is_file.unwrap_or(true)))),
        ("created".to_string(), DocumentValueOptions::Single(U64(created.unwrap_or_default()))),
        ("modified".to_string(), DocumentValueOptions::Single(U64(modified.unwrap_or_default()))),
        ("accessed".to_string(), DocumentValueOptions::Single(U64(accessed.unwrap_or_default()))),
        ("size".to_string(), DocumentValueOptions::Single(U64(size.unwrap_or_default()))),
        ("path".to_string(), DocumentValueOptions::Single(Text(path.to_str().unwrap_or_default().to_string()))),
        ("extension".to_string(), DocumentValueOptions::Single(Text(extension.unwrap_or_default().to_string()))),
        ("extension_title".to_string(), DocumentValueOptions::Single(Text(extension_title.unwrap_or_default().to_string()))),
    ]);
    DocumentPayload(map)
}

pub fn time_to_u64(time: io::Result<SystemTime>) -> Option<u64> {
    time.ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_secs())
}

pub async fn crawl(database_manager: &'static DatabaseManager, index_manager: &'static IndexManager, job_manager: &'static JobManager, name: String, start_path: PathBuf) -> Result<()> {
    info!("Crawling starting at path: {:?}.", start_path);
    let now = Instant::now();
    let file_paths: Vec<PathBuf> = FileCrawlerBuilder::default().location(start_path.clone()).build().collect();
    info!("Crawling completed in {:.2?} with {} files found.", now.elapsed(), file_paths.len());

    // Collect files
    let now = Instant::now();
    let mut rand = StdRand::default();
    let mut rand_next = rand.next_u64();

    let files =
        file_paths.clone().into_iter().enumerate().map(move |(index, path)| {
            (path, rand_next + (index as u64))
        }).collect_vec();

    let file_paths_count = file_paths.clone().len() as u32;

    let database_files = files.clone();
    execute_future_result_operation(&FILE_CRAWLING_POOL, move || async move {
        database_manager.files(move |c| {
            files_bulk_add(c, database_files)
        }).await
    }).await?;

    let tantivy_files = files.clone();
    let tantivy_name = name.clone();
    let now = Instant::now();

    let add_tasks = tantivy_files.into_iter().map(|(path, document_id)| async move {
        tokio_rayon::spawn(move || async move {
            let path_metadata = std::fs::metadata(path.clone()).ok();
            let extension_details = extension_details(path.clone(), path_metadata.clone()).await;
            let index = index_manager.get_index(extension_details.2);
            let tantivy_payload = tantivy_payload(path.clone(), path_metadata.clone(), extension_details.3).await;
            index.add_document(document_id.clone(), tantivy_payload).await
        }).await.await
    }).collect_vec();

    join_all(add_tasks).await;

    info!("Index added in {:.2?}.", now.elapsed());

    let now = Instant::now();
    index_manager.commit_all_indexes().await;
    info!("Indexes committed in {:.2?}.", now.elapsed());

    database_manager.core(move |c| {
        search_folders_update_crawled(c, tantivy_name.clone(), false, true)
    }).await?;

    let task_files = files.clone();
    let job_name = name.clone();
    execute_future_result_operation(&FILE_CRAWLING_POOL, move || async move {

        // Add task groups if needed
        let job_group_name = job_name.clone();
        database_manager.core(move |connection| {
            job_groups_add(connection, "hash".to_string(), job_group_name, file_paths_count)
        }).await.unwrap();

        let job_group_name = job_name.clone();
        database_manager.core(move |connection| {
            job_groups_add(connection, "index".to_string(), job_group_name, file_paths_count)
        }).await.unwrap();

        let job_group_name = job_name.clone();
        database_manager.core(move |connection| {
            job_groups_add(connection, "thumbnail".to_string(), job_group_name, file_paths_count)
        }).await.unwrap();

        // Add hash, index and thumbnail tasks
        let job_group_name = job_name.clone();
        let files = task_files.iter().filter(|(path, id)| path.is_file()).collect_vec();
        for file in files {
            job_manager.add("hash".to_string(), job_group_name.clone(), serde_json::to_value(
                JobHandlerHashPayload {
                    file_path: file.0.clone()
                }
            ).unwrap()).await.unwrap();

            job_manager.add("index".to_string(), job_group_name.clone(), serde_json::to_value(
                JobHandlerIndexPayload {
                    document_id: file.1.clone(),
                    file_path: file.0.clone()
                }
            ).unwrap()).await?;

            job_manager.add("thumbnail".to_string(), job_group_name.clone(), serde_json::to_value(
                JobHandlerThumbnailPayload {
                    document_id: file.1.clone(),
                    file_path: file.0.clone()
                }
            ).unwrap()).await?;
        }
        Ok(())
    }).await?;

    Ok(())
}