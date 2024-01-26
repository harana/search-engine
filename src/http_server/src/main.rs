use glommio::{CpuSet, LocalExecutorPoolBuilder, PoolPlacement};
use search_common::http_server::router;
use search_common::utils::hyper_compat;

fn main() {
    println!("Starting server on port 8000 ..");
    LocalExecutorPoolBuilder::new(PoolPlacement::MaxSpread(
        num_cpus::get(),
        CpuSet::online().ok(),
    ))
        .on_all_shards(|| async move {
            let id = glommio::executor().id();
            println!("Starting executor {}", id);
            hyper_compat::serve_http(([0, 0, 0, 0], 8000), router, 1024)
                .await
                .unwrap();
        })
        .unwrap()
        .join_all();
}

