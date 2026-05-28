use std::fmt;

use anyhow::Result;

use crate::env::Environment;

pub fn container_name(name: &str) -> String {
    format!("lenv-{name}")
}

#[derive(Debug)]
pub enum ContainerState {
    Running,
    Stopped,
    Missing,
}

impl fmt::Display for ContainerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContainerState::Running => write!(f, "running"),
            ContainerState::Stopped => write!(f, "stopped"),
            ContainerState::Missing => write!(f, "missing"),
        }
    }
}

pub trait Runtime {
    fn create(&self, env: &Environment) -> Result<()>;
    fn enter(&self, name: &str) -> Result<()>;
    fn remove(&self, name: &str) -> Result<()>;
    fn stop(&self, name: &str) -> Result<()>;
    fn start(&self, name: &str) -> Result<()>;
    fn restart(&self, name: &str) -> Result<()>;
    fn state(&self, name: &str) -> Result<ContainerState>;
    fn list(&self) -> Result<()>;
    fn provision(&self, env: &Environment) -> Result<()>;
}

pub mod docker;
