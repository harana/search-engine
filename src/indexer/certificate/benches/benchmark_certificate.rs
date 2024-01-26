use std::path::Path;

use harana_common::criterion::*;
use harana_common::tokio::runtime::Runtime;
use indexer_certificate::indexer_certificate::IndexerCertificate;

fn bench_indexing_certificate(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_file_path = Path::new("../../../test_files/IMG_2551.jpeg");
    let ftp = rt.block_on(test_file_path);

    c.bench_function("indexing_certificate", |b| {
        b.iter(|| {
            let _indexed_document = IndexerCertificate.index_file(&ftp).unwrap();
        });
    });
}

criterion_group!(benches, bench_indexing_certificate,);

criterion_main!(benches);