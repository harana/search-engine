use harana_common::anyhow::Result;

pub trait Opener: Send + Sync {

    fn open(&self, path: &str) -> Result<()>;

}