use std::fmt::{Debug, Display};
use std::path::PathBuf;
use std::sync::Arc;

use effectum::{Error, Job, JobRunner, Queue, RunningJob, Worker};

use harana_common::anyhow::Result;
use harana_common::futures::future::try_join_all;
use harana_common::futures::FutureExt;
use harana_common::hashbrown::HashMap;
use harana_common::itertools::Itertools;
use harana_common::log::info;
use harana_common::once_cell::sync::OnceCell;
use harana_common::{futures, serde, tokio};
use harana_common::serde::{Deserialize, Serialize};
use harana_common::serde_json::Value;
use harana_common::sysinfo::SystemExt;
use harana_common::uuid::Uuid;
use harana_database::manager::DatabaseManager;
use harana_database::job_groups_get::job_groups_get;
use harana_database::job_groups_list::job_groups_list;
use crate::handler::JobHandler;

pub static mut PAUSED: bool = false;
pub static HANDLERS: OnceCell<HashMap<String, Box<dyn JobHandler>>> = OnceCell::new();
pub static QUEUE: OnceCell<Queue> = OnceCell::new();

pub struct JobManager {
    database_manager: &'static DatabaseManager,
    workers: Vec<Worker>
}

#[derive(Clone, Debug)]
pub struct JobContext {}

#[derive(Serialize, Deserialize)]
#[serde(crate = "self::serde")]
struct JobPayload {
    category: String,
    name: String,
    payload: Value
}

type TaskId = String;

pub async fn handler_job(job: RunningJob, context: Arc<JobContext>) -> Result<(), Error> {
    let payload: JobPayload = job.json_payload()?;
    let handler = HANDLERS.get().unwrap().get(payload.category.as_str()).unwrap();

    let mutex = std::sync::Mutex::new(payload.payload);

    std::panic::catch_unwind(|| {
        let mut payload = mutex.lock().unwrap();
        let handle = tokio::runtime::Handle::current();
        let _ = handle.enter();
        futures::executor::block_on(handler.handle(payload.take()));
    });

    Ok(())
}

impl JobManager {

    pub async fn new(path_prefix: &PathBuf, worker_count: usize, database_manager: &'static DatabaseManager, handlers: HashMap<String, Box<dyn JobHandler>>) -> Self {
        let _ = HANDLERS.set(handlers);
        let _ = QUEUE.set(Queue::new(path_prefix.join("harana-jobs.db").as_path()).await.unwrap());

        let job_runners = HANDLERS.get().unwrap().keys().into_iter().map(|name|
            JobRunner::builder(name, handler_job).build()
        ).collect_vec();

        info!("Creating {} workers to process jobs ..", worker_count);

        let workers = (1..worker_count).into_iter().map(|i| {
            let context = Arc::new(JobContext{});
            Worker::builder(QUEUE.get().unwrap(), context).max_concurrency(5).jobs(job_runners.clone()).build()
        }).collect_vec();

        Self {
            database_manager,
            workers: try_join_all(workers).await.expect("Failed to create workers")
        }
    }

    pub async fn add(&'static self, job_group_category: String, job_group_name: String, payload: Value) -> effectum::Result<Uuid> {
        Job::builder(job_group_category.clone())
            .json_payload(&JobPayload {
                category: job_group_category.to_string(),
                name: job_group_name.to_string(),
                payload
            })?
            .add_to(QUEUE.get().unwrap())
            .await
    }

    pub async fn job_groups_status(&'static self, category: String) -> Result<HashMap<String, bool>> {
        let mut map = HashMap::new();
        self.database_manager.core(move |connection| {
            job_groups_list(connection)
        }).await?
            .into_iter()
            .filter(|tg| tg.category == category)
            .for_each(|tg| { map.insert(tg.name, tg.successes + tg.failures == tg.total); });
        Ok(map)
    }

    pub async fn job_group_status(&'static self, category: String, name: String) -> Result<Option<bool>> {
        let job_group = self.database_manager.core(move |connection| {
            job_groups_get(connection, category, name)
        }).await.ok();

        Ok(job_group.map(|tg| tg.successes + tg.failures == tg.total))
    }

    pub fn pause(&'static self) {
        unsafe {
            PAUSED = true
        }
    }

    pub fn resume(&'static self) {
        unsafe {
            PAUSED = false
        }
    }

    // fn should_execute(&'static self, task: &HaranaTask) -> bool {
    //     unsafe {
    //         if PAUSED {
    //             return false;
    //         }
    //     }{
    //
    //     if task.attempts <= 10 {
    //         return true;
    //     }
    //
    //     let time_diff = OffsetDateTime::now_utc() - task.last_attempt_date;
    //     if task.attempts <= 100 && time_diff.whole_minutes() >= 60 {
    //         return true;
    //     }
    //
    //     if time_diff.whole_hours() >= 24 {
    //         return true;
    //     }
    //
    //     return false;
    // }
}