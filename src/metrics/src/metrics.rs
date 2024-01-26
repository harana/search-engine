use lazy_static::lazy_static;
use prometheus::{register_counter_vec, register_histogram_vec};
use prometheus::exponential_buckets;
use prometheus_static_metric::{make_static_metric, register_static_counter_vec, register_static_histogram_vec};

make_static_metric! {

    pub label_enum alluxio_operations {
        check_access,
        check_consistency,
        complete_file,
        create_directory,
        create_file,
        free,
        get_file_path,
        get_mount_table,
        get_new_block_id_for_file,
        get_state_lock_holders,
        get_sync_path_list,
        get_status,
        list_status,
        mount,
        remove,
        rename,
        reverse_resolve,
        schedule_async_persistence,
        set_acl,
        set_attribute,
        start_sync,
        stop_sync,
        unmount,
        update_mount
    }

    pub label_enum hdfs_operations {
        cat,
        copy,
        copy_from_local,
        copy_to_local,
        disk_usage,
        disk_usage_total,
        list,
        make_directory,
        r#move,
        move_from_local,
        touch,
        remove_recursive,
        set_replication_factor
    }

    pub label_enum http_methods {
        delete,
        get,
        head,
        post,
        put
    }

    pub label_enum rest_operations {
        compress,
        create_directory,
        decompress,
        delete,
        download,
        duplicate,
        info,
        list,
        r#move,
        preview,
        rename,
        search,
        set_acl,
        tags,
        update_info,
        upload
    }

    pub struct AlluxioOperation: Counter {
        "operation" => alluxio_operations
    }

    pub struct HDFSOperation: Counter {
        "operation" => hdfs_operations
    }

    pub struct RESTDuration: Histogram {
        "operation" => rest_operations
    }

    pub struct RESTRequestSize: Histogram {
        "operation" => rest_operations
    }

    pub struct RESTResponseSize: Histogram {
        "operation" => rest_operations
    }

    pub struct RESTOperation: Counter {
        "operation" => rest_operations
    }
}

lazy_static! {
    pub static ref ALLUXIO_OPERATION: AlluxioOperation = register_static_counter_vec!(AlluxioOperation, "alluxio_operations", "Number of Alluxio operations", &["operation"]).unwrap();
    pub static ref HDFS_OPERATION: HDFSOperation = register_static_counter_vec!(HDFSOperation, "hdfs_operations", "Number of HDFS operations", &["operation"]).unwrap();
    pub static ref REQUEST_OPERATION: RESTOperation = register_static_counter_vec!(RESTOperation, "rest_operations", "Number of REST operations", &["operation"]).unwrap();

    pub static ref REQUEST_DURATION: RESTDuration = register_static_histogram_vec!(RESTDuration, "rest_duration", "Duration of REST request", &["operation"], exponential_buckets(0.0005, 2.0, 20).unwrap()).unwrap();
    pub static ref REQUEST_SIZE: RESTRequestSize = register_static_histogram_vec!(RESTRequestSize, "rest_request_size", "Size of REST request", &["operation"]).unwrap();
    pub static ref RESPONSE_SIZE: RESTResponseSize = register_static_histogram_vec!(RESTResponseSize, "rest_response_size", "Size of REST response", &["operation"]).unwrap();
}