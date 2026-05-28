use std::{
    process::{Command, Stdio},
    time::Duration,
};

use anyhow::{Result, bail};
use colored::Colorize;

use crate::{
    env::Environment,
    runtime::{ContainerState, container_name},
};

use indicatif::{ProgressBar, ProgressStyle};

use super::Runtime;

pub struct DockerRuntime;

enum Distro {
    Arch,
    Ubuntu,
    Alpine,
}

fn sanitize_username(username: &str) -> String {
    username
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

fn exec(container: &str, command: &str) -> Result<()> {
    let status = Command::new("docker")
        .args(["exec", "-i", container, "sh", "-c", command])
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        bail!("command failed: {}", command);
    }

    Ok(())
}

fn spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());

    pb.set_message(message.to_string());

    pb.enable_steady_tick(Duration::from_millis(100));

    pb
}

fn detect_distro(image: &str) -> Distro {
    if image.contains("arch") {
        Distro::Arch
    } else if image.contains("alpine") {
        Distro::Alpine
    } else {
        Distro::Ubuntu
    }
}

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
        let pb = spinner("Creating container...");
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
            pb.finish_with_message("✗ Failed to create container".red().to_string());
            bail!("failed to create container");
        }

        pb.finish_with_message("✓ Container created".green().to_string());

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

    fn provision(&self, env: &Environment) -> Result<()> {
        let pb = spinner("Installing basic packages...");

        let container = container_name(&env.name);
        let distro = detect_distro(&env.image);
        let username = sanitize_username(&whoami::username()?);

        let result: Result<()> = match distro {
            Distro::Arch => {
                exec(
                    &container,
                    "pacman -Sy --noconfirm \
                 git sudo curl bash neovim vim nano wget",
                )?;
                exec(
                    &container,
                    &format!("useradd -m -s /bin/bash {}", username).to_string(),
                )?;

                Ok(())
            }
            Distro::Ubuntu => {
                exec(&container, "apt update")?;
                exec(
                    &container,
                    "apt install -y \
                 git sudo curl bash neovim vim nano wget",
                )?;
                exec(
                    &container,
                    &format!("useradd -m -s /bin/bash {}", username).to_string(),
                )?;

                Ok(())
            }
            Distro::Alpine => {
                exec(
                    &container,
                    "apk add \
                 git sudo curl bash neovim vim nano wget",
                )?;

                Ok(())
            }
        };

        match result {
            Ok(_) => {
                pb.finish_with_message("✓ Installed basic packages".green().to_string());

                Ok(())
            },
            Err(err) => {
                pb.finish_with_message("✗ Installing basic packages failed");

                Err(err)
            }
        }
    }
}
