mod cli;
mod runtime;

use anyhow::{Ok, Result};
use clap::Parser;

use cli::{Cli, Commands};
use runtime::docker::DockerRuntime;
use runtime::Runtime;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let runtime = DockerRuntime;

    match cli.command {
        Commands::Create { name } => {
            runtime.create(&name)?;
        },
        Commands::Enter { name } => {
            runtime.enter(&name)?;
        },
        Commands::List => {
            runtime.list()?;
        },
        Commands::Remove { name } => {
            runtime.remove(&name)?;
        }
    }

    Ok(())
}
