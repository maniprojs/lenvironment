use std::process::Command;

use anyhow::{Ok, Result, bail};

use super::Runtime;

pub struct DockerRuntime;

impl Runtime for DockerRuntime {
    fn create(&self, name: &str) -> Result<()> {
        let status = Command::new("docker")
            .args([
                "run",
                "-dit",
                "--name",
                name,
                "ubuntu:latest",
                "bash"
            ])
            .status()?;

        if !status.success() {
            bail!("failed to create container");
        }

        println!("Created environment: {}", name);

        Ok(())
    }

    fn enter(&self, name: &str) -> Result<()> {
        let status = Command::new("docker")
            .args(["exec", "-it", name, "bash"])
            .status()?;

        if !status.success() {
            bail!("failed to enter container");
        }

        Ok(())
    }

    fn remove(&self, name: &str) -> Result<()> {
        let status = Command::new("docker")
            .args(["rm", "-f", name])
            .status()?;

        if !status.success() {
            bail!("failed to remove container")
        }

        println!("Removed environment: {}", name);
        
        Ok(())
    }

    fn list(&self) -> Result<()> {

        let status = Command::new("docker")

            .args(["ps", "-a"])

            .status()?;

        if !status.success() {

            bail!("failed to list containers");

        }

        Ok(())

    }
}
