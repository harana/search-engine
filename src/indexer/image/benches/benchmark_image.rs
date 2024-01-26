use std::path::Path;

use harana_common::criterion::*;
use harana_common::tokio::runtime::Runtime;
use indexer_image::indexer_imagemagick::Indexer;

fn bench_indexing_image(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_file_path = Path::new("../../../test_files/data.csv");
    let ftp = rt.block_on(test_file_path);

    c.bench_function("indexing_image", |b| {
        b.iter(|| {
            let _indexed_document = Indexer.index_file(&ftp).unwrap();
        });
    });
}

criterion_group!(benches, bench_indexing_image,);

criterion_main!(benches);