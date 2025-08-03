// src/main.rs

use anyhow::Result;
use clap::Parser;

// Declare the modules we created
mod cli;
mod commands;
mod file_utils;

use cli::{Cli, Commands};
use commands::handle_command;

fn main() -> Result<()> {
    // 1. Parse the command-line arguments
    let cli = Cli::parse();
    
    // 2. Pass the parsed command to the handler
    handle_command(&cli.command)?;

    Ok(())
}