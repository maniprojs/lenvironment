use anyhow::Result;

use crate::env::Environment;

pub trait Runtime {
    fn create(&self, env: &Environment) -> Result<()>;
    fn enter(&self, name: &str) -> Result<()>;
    fn remove(&self, name: &str) -> Result<()>;
    fn list(&self) -> Result<()>;
}

pub mod docker;
