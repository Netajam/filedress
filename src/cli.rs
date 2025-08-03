// src/cli.rs

use clap::{Parser, Subcommand, ValueEnum}; // <-- Add ValueEnum
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "filedress", version = "1.0", about = "A tool to dress up your source files with path headers.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds a relative path header to files
    Add(Args),
    /// Removes the path header from files
    Remove(Args),
    /// Removes all comments from files, except for the path header
    Clean(Args),
}

// --- NEW ---
// Define the available project presets
#[derive(ValueEnum, Clone, Debug)]
pub enum ProjectType {
    Rust,
    Python,
    Web, // For JS, TS, Svelte, etc.
    Java,
    Flutter,
}
// -----------

#[derive(Parser, Debug)]
pub struct Args {
    /// The root directory to search for files in
    #[arg(required = true)]
    pub directory: PathBuf,

    // --- MODIFIED ---
    /// A preset for common project types (e.g., rust, python, web)
    #[arg(long, exclusive = true)]
    pub project: Option<ProjectType>,

    /// A custom list of file extensions to process (e.g., "ts,js,css")
    /// Cannot be used with --project
    #[arg(long, value_delimiter = ',', conflicts_with = "project")]
    pub exts: Option<Vec<String>>,
    // ----------------

    // --- NEW ---
    /// How many levels up from the target directory to include in the path
    #[arg(short, long, default_value_t = 0)]
    pub up: u32,

    /// How many levels deep to search for files
    #[arg(short, long)]
    pub depth: Option<usize>,
    // -----------
}