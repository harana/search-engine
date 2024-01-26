use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{EventKind, RecursiveMode, Watcher};
use notify_debouncer_full::{DebounceEventResult, new_debouncer};
use walkdir::{DirEntry, WalkDir};

use harana_common::futures::future::join_all;
use harana_common::log::{error, info};
use harana_common::tokio;

pub async fn start_watcher<A, B, C, FutA, FutB, FutC>(
    directories: Vec<&Path>,
    on_create: A,
    on_update: B,
    on_remove: C
) where
    A: Fn(PathBuf) -> FutA + Copy,
    FutA: Future<Output = ()> + Send + 'static,
    B: Fn(PathBuf) -> FutB + Copy,
    FutB: Future<Output = ()> + Send + 'static,
    C: Fn(PathBuf) -> FutC + Copy,
    FutC: Future<Output = ()> + Send + 'static,
{
    info!("Started file watcher thread on: {:?}", directories);
    let (watcher_tx, watcher_rx) = channel();

    let mut debouncer = new_debouncer(Duration::from_secs(2), None, move |result: DebounceEventResult| {
        match result {
            Ok(events) => {
                let watcher_tx = watcher_tx.clone();
                events.into_iter().for_each(|event| watcher_tx.send(event).unwrap())
            },
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        }
    }).unwrap();

    directories.into_iter().for_each(|dir| {
        debouncer.watcher().watch(dir, RecursiveMode::Recursive).unwrap();
    });

    loop {
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let watcher_event = watcher_rx.recv();
        match watcher_event {
            Ok(event) => {
                info!("Received watcher event: {:?}", event);

                let mut affected_paths = Vec::new();
                event.paths.iter().for_each(|path|
                    if path.is_dir() {
                        let walker = WalkDir::new(path).into_iter();
                        for entry in walker.filter_entry(|e| !is_hidden(e)) {
                            let entry = entry.unwrap();
                            affected_paths.push(entry.into_path());
                        }
                    } else {
                        affected_paths.push(path.to_path_buf());
                    }
                );

                let kind = event.kind.clone();

                join_all(
                    affected_paths.iter().map(move |path| async move {
                        match kind {
                            EventKind::Create(_e) => on_create(path.to_path_buf()).await,
                            EventKind::Modify(_e) => on_update(path.to_path_buf()).await,
                            EventKind::Remove(_e) => on_remove(path.to_path_buf()).await,
                            _ => {}
                        }
                    })
                ).await;
            }
            Err(e) => error!("Watch error: {:?}", e),
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}