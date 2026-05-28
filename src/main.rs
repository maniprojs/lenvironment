mod cli;
mod env;
mod runtime;

use std::fs;

use anyhow::{Ok, Result, anyhow};
use clap::Parser;

use cli::{Cli, Commands};
use runtime::docker::DockerRuntime;
use runtime::Runtime;

use crate::env::{Environment, Mount};

fn parse_mount(input: &str) -> Result<Mount> {
    let (host, container) = input
        .split_once(':')
        .ok_or_else(|| anyhow!("invalid mount format: '{input}'"))?;

    if host.is_empty() || container.is_empty() {
        anyhow::bail!("mount paths cannot be empty");
    }

    let expanded = shellexpand::tilde(host);

    Ok(Mount {
        host: expanded.to_string(),
        container: container.to_string(),

    })

}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let runtime = DockerRuntime;

    match cli.command {
        Commands::Create { name, image, mount } => {
            let mounts = mount
                .iter()
                .map(|m| parse_mount(m))
                .collect::<Result<Vec<_>>>()?;
            let env = Environment {
                name,
                runtime: "docker".into(),
                image:image.unwrap_or("ubuntu:latest".into()),
                mounts,
            };

            println!("Pulling {}", &env.image);

            runtime.create(&env)?;

            env.save()?;
        },
        Commands::Enter { name } => {
            runtime.enter(&name)?;
        },
        Commands::List => {
            runtime.list()?;
        },
        Commands::Remove { name } => {
            fs::remove_file(Environment::path(&name)?)?;
            runtime.remove(&name)?;
        }
    }

    Ok(())
}
