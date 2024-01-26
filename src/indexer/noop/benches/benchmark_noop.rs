use std::path::Path;

use harana_common::criterion::*;
use harana_common::tokio::runtime::Runtime;
use noop::indexer_noop::IndexerNoop;

fn bench_indexing_noop(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_file_path = Path::new("../../../test_files/data.csv");
    let ftp = rt.block_on(test_file_path);

    c.bench_function("indexing_noop", |b| {
        b.iter(|| {
            let _indexed_document = IndexerNoop.index_file(&ftp).unwrap();
        });
    });
}

criterion_group!(benches, bench_indexing_noop,);

criterion_main!(benches);