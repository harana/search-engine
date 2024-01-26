use std::path::Path;

use harana_common::criterion::*;
use harana_common::tokio::runtime::Runtime;
use indexer_game::indexer_game::IndexerGame;

fn bench_indexing_game(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let test_file_path = Path::new("../../../test_files/data.csv");
    let ftp = rt.block_on(test_file_path);

    c.bench_function("indexing_game", |b| {
        b.iter(|| {
            let _indexed_document = IndexerGame.index_file(&ftp).unwrap();
        });
    });
}

criterion_group!(benches, bench_indexing_game,);

criterion_main!(benches);