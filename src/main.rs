mod cli;
mod env;
mod runtime;
mod doctor;

use std::fs;

use anyhow::{Ok, Result, anyhow};
use clap::Parser;

use cli::{Cli, Commands};
use runtime::Runtime;
use runtime::docker::DockerRuntime;

use crate::env::{Environment, Mount};

use colored::Colorize;

use dialoguer::Confirm;

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
                image: image.unwrap_or("ubuntu:latest".into()),
                mounts,
            };

            println!("{} {}", "Pulling".blue(), &env.image.blue());

            runtime.create(&env)?;

            env.save()?;
        }
        Commands::Enter { name } => {
            runtime.enter(&name)?;
        }
        Commands::List => {
            runtime.list()?;
        }
        Commands::Remove { name, force } => {
            if !force {
                let confirmed = Confirm::new()
                    .with_prompt(format!("Remove environment '{}'?", name))
                    .default(false)
                    .interact()?;

                if !confirmed {
                    println!("{}", "Cancelled".red());

                    return Ok(());
                }
            }

            fs::remove_file(Environment::path(&name)?)?;
            runtime.remove(&name)?;
        }
        Commands::Start { name } => {
            runtime.start(&name)?;
        }

        Commands::Stop { name } => {
            runtime.stop(&name)?;
        }

        Commands::Restart { name } => {
            runtime.restart(&name)?;
        }

        Commands::Status { name } => {
            let env = Environment::load(&name)?;
            let state = runtime.state(&name)?;

            println!("{} {}", "Name:".blue(), env.name);
            println!("{} {}", "Runtime:".blue(), env.runtime);
            println!("{} {}", "Image:".blue(), env.image);
            println!("{} {:?}", "Status:".blue(), state);
        },
        Commands::Doctor => {
            doctor::run()?;
        }
    }

    Ok(())
}
