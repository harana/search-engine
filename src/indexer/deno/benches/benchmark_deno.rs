use std::path::Path;

use harana_common::criterion::*;
use harana_common::tokio::runtime::Runtime;
use indexer_airbyte::indexer_airbyte::IndexerAirbyte;

fn bench_indexing_deno(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_file_path = Path::new("../../../test_files/data.csv");
    let file = rt.block_on(test_file_path);

    c.bench_function("indexing_deno", |b| {
        b.iter(|| {
            let _indexed_document = IndexerDeno.index_file(&file).unwrap();
        });
    });
}

criterion_group!(benches, bench_indexing_airbyte,);

criterion_main!(benches);