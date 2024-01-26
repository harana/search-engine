use std::path::Path;

use harana_common::criterion::*;
use harana_common::tokio::runtime::Runtime;
use indexer_archive::indexer_archive::IndexerArchive;

fn bench_indexing_archive(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_file_path = Path::new("../../../test_files/data.csv");
    let ftp = rt.block_on(test_file_path);

    c.bench_function("indexing_archive", |b| {
        b.iter(|| {
            let _indexed_document = IndexerArchive.index_file(&ftp).unwrap();
        });
    });
}

criterion_group!(benches, bench_indexing_archive,);

criterion_main!(benches);