use std::path::Path;

use harana_common::criterion::*;
use harana_common::tokio::runtime::Runtime;
use indexer_application::indexer_application::IndexerApplication;

fn bench_indexing_application(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_file_path = Path::new("../../../test_files/data.csv");
    let ftp = rt.block_on(test_file_path);

    c.bench_function("indexing_application", |b| {
        b.iter(|| {
            let _indexed_document = IndexerApplication.index_file(&ftp).unwrap();
        });
    });
}

criterion_group!(benches, bench_indexing_application,);

criterion_main!(benches);