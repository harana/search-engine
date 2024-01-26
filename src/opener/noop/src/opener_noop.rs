use harana_common::anyhow::Result;
use harana_opener_core::opener::Opener;

pub struct OpenerNoop;

impl Opener for OpenerNoop {

    fn open(&self, _path: &str) -> Result<()> {
        Ok(())
    }
}