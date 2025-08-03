// src/main.rs

// --- FIX: Add the necessary imports for the binary to work ---
use anyhow::Result;
use clap::Parser;
// -----------------------------------------------------------

// Use the library crate we just defined. The name 'filedress' comes from `[package]` in Cargo.toml.
use filedress::cli::Cli;
use filedress::commands::handle_command;

fn main() -> Result<()> {
    // 1. Parse the command-line arguments
    // This now works because the `clap::Parser` trait is in scope.
    let cli = Cli::parse();

    // 2. Pass the parsed command to the handler from our library
    handle_command(&cli.command)?;

    // This now works because `anyhow::Result` is in scope, which provides the error type.
    Ok(())
}