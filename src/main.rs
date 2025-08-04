// src/main.rs

use anyhow::Result;
use clap::Parser;
// -----------------------------------------------------------

use filedress::cli::Cli;
use filedress::commands::handle_command;
use filedress::updater::check_for_updates; 

fn main() -> Result<()> {
    // 1. Trigger the (non-blocking) update check at the start.
    check_for_updates(); 
    // 2. Parse the command-line arguments
    let cli = Cli::parse();

    // 3. Pass the parsed command to the handler from our library
    handle_command(&cli.command)?;

    Ok(())
}