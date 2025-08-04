// src/commands/mod.rs

use anyhow::Result;
use crate::cli::Commands;

// 1. Declare the sub-modules
mod add;
mod remove;
mod clean;
mod copy;
mod utils; // utils is a private module for shared functions

// 2. The main dispatcher function
pub fn handle_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Add(args) => add::add(args)?,
        Commands::Remove(args) => remove::remove(args)?,
        Commands::Clean(args) => clean::clean(args)?,
        Commands::Copy(args) => copy::copy(args)?,
    }
    Ok(())
}