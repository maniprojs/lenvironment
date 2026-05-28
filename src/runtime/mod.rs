use anyhow::Result;

pub trait Runtime {
    fn create(&self, name: &str) -> Result<()>;
    fn enter(&self, name: &str) -> Result<()>;
    fn remove(&self, name: &str) -> Result<()>;
    fn list(&self) -> Result<()>;
}

pub mod docker;
