use rocksdb::{ColumnFamilyDescriptor, DB, Options};

pub fn create(path: &str, column_families: Vec<ColumnFamilyDescriptor>, db_opts: Option<Options>) -> DB {
    let mut db_opts = db_opts.unwrap_or(Options::default());
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);
    db_opts.increase_parallelism(4);
    db_opts.set_db_write_buffer_size(4 * 1024 * 1024);
    db_opts.set_disable_auto_compactions(true);
    db_opts.set_max_file_opening_threads(2);
    db_opts.set_max_open_files(5000);
    db_opts.set_max_successive_merges(1000);
    db_opts.prepare_for_bulk_load();
    return DB::open_cf_descriptors(&db_opts, path, column_families).unwrap();
}

pub fn compact(db: DB) {
    db.compact_range(None::<&[u8]>, None::<&[u8]>);
}

pub fn destroy(path: String) {
    let _ = DB::destroy(&Options::default(), path);
}