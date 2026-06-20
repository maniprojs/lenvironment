use std::{env::consts::OS, fs, os, process::Command, time::Duration};

use anyhow::Result;
use colored::Colorize;
use directories::ProjectDirs;
use indicatif::{ProgressBar, ProgressStyle};

use crate::runtime::docker;

fn spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());

    pb.set_message(message.to_string());

    pb.enable_steady_tick(Duration::from_millis(100));

    pb
}

fn check_docker_installed() -> Result<()> {
    let pb = spinner("Checking Docker installation...");

    let docker_installed = Command::new("docker").arg("--version").output();

    if docker_installed.is_ok() {
        pb.finish_with_message("✓ Docker installed".green().to_string());
    } else {
        pb.finish_with_message("✗ Docker not installed".red().to_string());
        println!(
            "  {}",
            "You must install Docker, either by installing Docker Desktop and configuring it or you can use Colima (Container on Lima (Linux Machines)) on macOS to start Docker daemon.".blue()
        );
    }

    Ok(())
}

pub fn run() -> Result<()> {
    // Check if Docker installed
    check_docker_installed()?;
    let pb = spinner("Checking Docker daemon...");

    let docker_running = Command::new("docker")
        .arg("info")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if docker_running {
        pb.finish_with_message("✓ Docker daemon running".green().to_string());
    } else {
        pb.finish_with_message("✗ Docker daemon not running".red().to_string());

        println!(
            "    {}",
            "Docker daemon must be running. If it's not, Open Docker Desktop".blue()
        );
        if OS == "macos" {
            println!("    {}", "It appears you are running macOS. You can also use Colima to start a Docker context without the size of Docker Desktop".blue());
        } else if OS == "linux" {
            println!("    {}", "It appears you are running Linux. You can also use the Docker from your package manager, though it's not recommended".blue());
        }
    }

    // Metadata directory
    let pb = spinner("Checking metadata directory...");

    let metadata_ok = if let Some(proj_dirs) = ProjectDirs::from("", "", "lenv") {
        let dir = proj_dirs.data_dir();
        fs::create_dir_all(dir).is_ok()
    } else {
        false
    };
    if metadata_ok {
        pb.finish_with_message("✓ Metadata directory writable".green().to_string());
    } else {
        pb.finish_with_message("✗ Metadata directory not writable".red().to_string());
    }

    Ok(())
}
