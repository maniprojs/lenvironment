use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lenv")]
#[command(version = "0.1.0")]
#[command(about = "Linux Environment Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Create a new environment")]
    Create {
        name: String,
    },
    #[command(about = "Enter an environment")]
    Enter {
        name: String,
    },
    #[command(about = "List environments")]
    List,
    #[command(about = "Remove an environment")]
    Remove {
        name: String
    }
}