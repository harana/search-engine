use harana_common::anyhow::Result;
use deno_core::{
    JsRuntime, RuntimeOptions,
};
use std::fs::read_to_string;
use crate::deno_operations::*;

const RUNTIME_BOOTSTRAP: &str = r#"
globalThis.console = {
    debug: (...args) => Deno.core.opSync("op_log_debug", args.join(", "))
    error: (...args) => Deno.core.opSync("op_log_error", args.join(", "))
    info: (...args) => Deno.core.opSync("op_log_info", args.join(", "))
    log: (...args) => Deno.core.opSync("op_log_info", args.join(", "))
}
globalThis.add_document = (key, value) => (Deno.core.opSync("op_kv_set", key, JSON.stringify(value)), value)
globalThis.get = (key) => JSON.parse(Deno.core.opSync("op_kv_get", key))
"#;

fn execute_script(path: &str) -> Result<()> {
    let mut runtime = JsRuntime::new(
        RuntimeOptions {
            extensions: vec![
                deno_core::Extension::builder()
                    .ops(vec![
                        op_log_debug::decl(),
                        op_kv_set::decl(),
                        op_kv_get::decl()
                    ])
                    .js(vec![("[runtime]", RUNTIME_BOOTSTRAP)])
                    .build()
            ], ..Default::default()
    });
    runtime.execute_script(path, read_to_string(path))?
}