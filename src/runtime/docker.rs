use std::process::Command;

use anyhow::{Ok, Result, bail};

use crate::env::Environment;

use super::Runtime;

pub struct DockerRuntime;

impl Runtime for DockerRuntime {
    fn create(&self, env: &Environment) -> Result<()> {
        let status = Command::new("docker")
            .args([
                "run",
                "-dit",
                "--label",
                "dev.lenv.managed=true",
                "--label",
                &format!("dev.lenv.name={}", env.name),
                "--name",
                &env.name,
                &env.image,
                "bash"
            ])
            .status()?;

        if !status.success() {
            bail!("failed to create container");
        }

        println!("Created environment: {}", &env.name);

        Ok(())
    }

    fn enter(&self, name: &str) -> Result<()> {
        let status = Command::new("docker")
            .args(["exec", "-it", name, "bash"])
            .status()?;

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

        let envs = Environment::list()?;

        for env in envs {
            println!(
                "{} ({}) [{}]",
                env.name,
                env.runtime,
                env.image
            );
        }

        Ok(())

    }
}
