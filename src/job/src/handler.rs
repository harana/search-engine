use std::panic::RefUnwindSafe;
use harana_common::anyhow::Result;
use harana_common::async_trait::async_trait;
use harana_common::serde_json::Value;

#[async_trait]
pub trait JobHandler: Send + Sync + RefUnwindSafe {

    async fn handle(&'static self, payload: Value) -> Result<()>;

}