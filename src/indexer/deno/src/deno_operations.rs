use deno_core::op2;
use harana_common::log::*;

#[op2(fast)]
fn op_log_debug(#[string] msg: &str) {
    debug!("{}", msg)
}

#[op2(fast)]
fn op_log_info(#[string] msg: &str) {
    info!("{}", msg)
}

#[op2(fast)]
fn op_log_error(#[string] msg: &str) {
    error!("{}", msg)
}

#[op2(fast)]
#[serde]
fn op_get_connection() -> Result<Connection> {
}

#[op2(fast)]
fn op_add_document(#[serde] document: Document) {

}