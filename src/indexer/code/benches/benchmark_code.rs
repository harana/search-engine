use std::path::Path;

use harana_common::criterion::*;
use harana_common::tokio::runtime::Runtime;
use indexer_code::indexer_code::IndexerCode;

fn bench_indexing_code(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_file_path = Path::new("../../../test_files/IMG_2551.jpeg");
    let ftp = rt.block_on(test_file_path);

    c.bench_function("indexing_code", |b| {
        b.iter(|| {
            let _indexed_document = IndexerCode.index_file(&ftp).unwrap();
        });
    });
}

criterion_group!(benches, bench_indexing_code,);

criterion_main!(benches);