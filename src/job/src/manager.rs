use std::cmp;
use std::fmt::{Debug, Display};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::thread;

use effectum::{Error, Job, JobRunner, Queue, RunningJob, Worker};

use harana_common::anyhow::Result;
use harana_common::battery::Manager;
use harana_common::futures::future::{ok, try_join_all};
use harana_common::futures::FutureExt;
use harana_common::hashbrown::HashMap;
use harana_common::itertools::Itertools;
use harana_common::log::info;
use harana_common::once_cell::sync::OnceCell;
use harana_common::{num_cpus, serde, systemstat};
use harana_common::serde::{Deserialize, Serialize};
use harana_common::serde_json::Value;
use harana_common::sysinfo::*;
use harana_common::systemstat::Platform;
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
pub struct JobContext {
    ac_power_required: bool,
    battery_life_remaining_enabled: bool,
    battery_life_remaining_value: u8,
    cpu_maximum_usage_enabled: bool,
    cpu_maximum_usage_value: u8,
    cpu_maximum_temperature_enabled: bool,
    cpu_maximum_temperature_value: u8,
    hours_between_enabled: bool,
    hours_between_start: u8,
    hours_between_end: u8
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "self::serde")]
struct JobPayload {
    category: String,
    name: String,
    payload: Value
}

type TaskId = String;

impl JobManager {

    pub async fn new(path_prefix: &PathBuf,
                     worker_count: usize,
                     worker_concurrency: usize,
                     database_manager: &'static DatabaseManager,
                     handlers: HashMap<String, Box<dyn JobHandler>>,
                     ac_power_required: bool,
                     battery_life_remaining_enabled: bool,
                     battery_life_remaining_value: u8,
                     cpu_maximum_usage_enabled: bool,
                     cpu_maximum_usage_value: u8,
                     cpu_maximum_temperature_enabled: bool,
                     cpu_maximum_temperature_value: u8,
                     hours_between_enabled: bool,
                     hours_between_start: u8,
                     hours_between_end: u8) -> Self {
        let _ = HANDLERS.set(handlers);
        let _ = QUEUE.set(Queue::new(path_prefix.join("harana-jobs.db").as_path()).await.unwrap());

        let job_runners = HANDLERS.get().unwrap().keys().into_iter().map(|name|
            JobRunner::builder(name, handler_job).build()
        ).collect_vec();

        info!("Creating {} workers to process jobs ..", worker_count);

        let workers = (1..worker_count).into_iter().map(|i| {
            let context = Arc::new(JobContext{
                ac_power_required,
                battery_life_remaining_enabled,
                battery_life_remaining_value,
                cpu_maximum_usage_enabled,
                cpu_maximum_usage_value,
                cpu_maximum_temperature_enabled,
                cpu_maximum_temperature_value,
                hours_between_enabled,
                hours_between_start,
                hours_between_end
            });
            Worker::builder(QUEUE.get().unwrap(), context).max_concurrency(worker_concurrency as u16).jobs(job_runners.clone()).build()
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
}

pub async fn handler_job(job: RunningJob, context: Arc<JobContext>) -> Result<(), Error> {
    let payload: JobPayload = job.json_payload()?;
    let handler = HANDLERS.get().unwrap().get(payload.category.as_str()).unwrap();

    if (should_execute(context)) {
        handler.handle(payload.payload).await;
    }else{
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn should_execute(context: Arc<JobContext>) -> bool {
    let mut sys = System::new_all();

    let valid_ac =
        if context.ac_power_required {
            systemstat::System::new().on_ac_power().unwrap_or(true)
        } else {
            true
        };

    let valid_battery =
        if context.battery_life_remaining_enabled {
            Manager::new()
                .and_then(|b| b.batteries())
                .and_then(|b| b.last().unwrap())
                .and_then(|b| Ok((b.state_of_charge().value * 100.0) as u8 > context.battery_life_remaining_value))
                .unwrap_or(true)
        } else {
            true
        };

    let valid_cpu_temperature =
        if context.cpu_maximum_temperature_enabled {
            let components = Components::new_with_refreshed_list();
            let mut max = 0.0;

            for component in &components {
                if component.temperature() > max {
                    max = component.temperature();
                }
            }
            (max as u8) < context.cpu_maximum_temperature_value
        } else {
            true
        };

    let valid_cpu_usage =
        if context.cpu_maximum_usage_enabled {
            let mut total = 0.0;
            sys.refresh_cpu_all();

            for cpu in sys.cpus() {
                total += cpu.cpu_usage()
            }

            let average = total / sys.cpus().len() as f32;
            (average as u8) < context.cpu_maximum_usage_value
        } else {
            true
        };

    valid_ac && valid_battery && valid_cpu_temperature && valid_cpu_usage
}