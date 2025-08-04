// src/commands/mod.rs

use anyhow::Result;
use crate::cli::Commands;

// Declare all the public sub-modules for our commands.
mod add;
mod remove;
mod clean;
mod copy;
mod structure;

// Declare a private module for shared helper functions.
mod utils;

/// The main dispatcher function. It receives a command from the CLI
/// and calls the appropriate handler function from our sub-modules.
pub fn handle_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Add(args) => add::add(args)?,
        Commands::Remove(args) => remove::remove(args)?,
        Commands::Clean(args) => clean::clean(args)?,
        Commands::Copy(args) => copy::copy(args)?,
        Commands::Structure(args) => structure::structure(args)?,
    }
    Ok(())
}