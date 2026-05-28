use std::process::Command;

use anyhow::{Ok, Result, bail};

use crate::env::Environment;

use super::Runtime;

pub struct DockerRuntime;

impl Runtime for DockerRuntime {
    fn create(&self, env: &Environment) -> Result<()> {
        let mut cmd = Command::new("docker");

        cmd.arg("run");
        cmd.arg("-dit");

        cmd.args([
            "--label",
            "dev.lenv.managed=true",
        ]);

        cmd.args([
            "--label",
            &format!("dev.lenv.name={}", env.name),
        ]);

        cmd.args([
            "--name",
            &format!("lenv-{}", env.name)
        ]);

        for mount in &env.mounts {
            cmd.args([
                "-v",
                &format!("{}:{}", mount.host, mount.container),
            ]);
        }

        cmd.arg(&env.image);
        cmd.arg("bash");
        
        let status = cmd.status()?;

        if !status.success() {
            bail!("failed to create environment")
        }

        println!("Created environment: {}", &env.name);

        Ok(())
    }

    fn enter(&self, name: &str) -> Result<()> {
        let container_name = &format!("lenv-{name}").to_string();

        let _ = Command::new("docker")
            .args(["exec", "-it", container_name, "bash"])
            .status()?;

        Ok(())
    }

    fn remove(&self, name: &str) -> Result<()> {
        let container_name = &format!("lenv-{name}").to_string();

        let status = Command::new("docker")
            .args(["rm", "-f", container_name])
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
