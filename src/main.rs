mod cli;
mod env;
mod runtime;

use std::fs;

use anyhow::{Ok, Result};
use clap::Parser;

use cli::{Cli, Commands};
use runtime::docker::DockerRuntime;
use runtime::Runtime;

use crate::env::Environment;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let runtime = DockerRuntime;

    match cli.command {
        Commands::Create { name } => {
            let env = Environment {
                name,
                runtime: "docker".into(),
                image: "ubuntu:latest".into(),
            };

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
