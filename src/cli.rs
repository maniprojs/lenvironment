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
        #[arg(long)]
        image: Option<String>,
        #[arg(long)]
        mount: Vec<String>
    },
    #[command(about = "Enter an environment")]
    Enter {
        name: String,
    },
    #[command(about = "List environments")]
    List,
    #[command(about = "Remove an environment")]
    Remove {
        name: String,
        #[arg(short, long)]
        force: bool,
    },
    #[command(about = "Start an environment")]
    Start {
        name: String,
    },
    #[command(about = "Stop an environment")]
    Stop {
        name: String,
    },
    #[command(about = "Restart an environment")]
    Restart {
        name: String,
    },
    #[command(about = "Show the status of an environment")]
    Status {
        name: String,
    },
    #[command(about = "Verify runtime dependencies and environment health")]
    Doctor,
}