use std::process::{Command, Stdio};

use anyhow::{Ok, Result, bail};
use colored::Colorize;

use crate::{
    env::Environment,
    runtime::{ContainerState, container_name},
};

use super::Runtime;

pub struct DockerRuntime;

fn docker_state(name: &str) -> Result<ContainerState> {
    let output = Command::new("docker")
        .args(["inspect", "-f", "{{.State.Status}}", &container_name(name)])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        return Ok(ContainerState::Missing);
    }

    let status = String::from_utf8_lossy(&output.stdout);

    match status.trim() {
        "running" => Ok(ContainerState::Running),
        _ => Ok(ContainerState::Stopped),
    }
}

impl Runtime for DockerRuntime {
    fn create(&self, env: &Environment) -> Result<()> {
        let mut cmd = Command::new("docker");

        cmd.arg("run");
        cmd.arg("-dit");

        cmd.args(["--label", "dev.lenv.managed=true"]);

        cmd.args(["--label", &format!("dev.lenv.name={}", env.name)]);

        cmd.args(["--name", &format!("lenv-{}", env.name)]);

        for mount in &env.mounts {
            cmd.args(["-v", &format!("{}:{}", mount.host, mount.container)]);
        }

        cmd.arg(&env.image);
        cmd.arg("bash");

        let status = cmd.status()?;

        if !status.success() {
            bail!("failed to create environment")
        }

        println!("{} {}", "Created environment:".green(), &env.name);

        Ok(())
    }

    fn enter(&self, name: &str) -> Result<()> {
        let container_name = &format!("lenv-{name}").to_string();

        match self.state(name)? {
            ContainerState::Running => {}

            ContainerState::Stopped => {
                self.start(name)?;
            }

            ContainerState::Missing => {
                bail!("environment does not exist");
            }
        }

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

        println!("{} {}", "Removed environment:".green(), name);

        Ok(())
    }

    fn list(&self) -> Result<()> {
        let envs = Environment::list()?;

        println!("{:<15} {:<12} {:<25}", "NAME", "STATUS", "IMAGE");

        for env in envs {
            let state = self.state(&env.name)?;

            println!("{:<15} {:<12} {:<25}", env.name, state, env.image);
        }

        Ok(())
    }

    fn state(&self, name: &str) -> Result<ContainerState> {
        docker_state(name)
    }

    fn start(&self, name: &str) -> Result<()> {
        let status = Command::new("docker")
            .args(["start", &container_name(name)])
            .status()?;

        if !status.success() {
            bail!("failed to start container");
        }

        Ok(())
    }

    fn stop(&self, name: &str) -> Result<()> {
        let status = Command::new("docker")
            .args(["stop", &container_name(name)])
            .status()?;

        if !status.success() {
            bail!("failed to stop container");
        }

        Ok(())
    }

    fn restart(&self, name: &str) -> Result<()> {
        let status = Command::new("docker")
            .args(["restart", &container_name(name)])
            .status()?;

        if !status.success() {
            bail!("failed to restart container");
        }

        Ok(())
    }
}
