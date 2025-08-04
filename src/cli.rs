// src/cli.rs

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "filedress", version = "1.0.0", about = "A tool to dress up your source files with path headers.", long_about = None)]
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
    /// Copies the content of multiple files to the clipboard
    Copy(Args),
    /// Creates a file/folder structure from a text file
    Structure(StructureArgs),
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ProjectType {
    Rust,
    Python,
    Web,
    Java,
    Flutter,
}

#[derive(Parser, Debug)]
pub struct Args {
    /// The root directory to search for files in
    #[arg(required = true)]
    pub directory: PathBuf,
    /// A preset for common project types (e.g., rust, python, web)
    #[arg(long, exclusive = true)]
    pub project: Option<ProjectType>,
    /// A custom list of file extensions to process (e.g., "ts,js,css")
    #[arg(long, value_delimiter = ',', conflicts_with = "project")]
    pub exts: Option<Vec<String>>,
    /// How many levels up from the target directory to include in the path
    #[arg(short, long, default_value_t = 0)]
    pub up: u32,
    /// How many levels deep to search for files
    #[arg(short, long)]
    pub depth: Option<usize>,
    /// Overwrites an existing path header if one is found
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
}

#[derive(Parser, Debug)]
pub struct StructureArgs {
    /// The input file with the tree structure. Reads from stdin if not provided.
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    /// The root directory where the structure will be created. Defaults to the current directory.
    #[arg(short, long)]
    pub directory: Option<PathBuf>,
    /// The number of spaces that represent one level of indentation.
    #[arg(short, long, default_value_t = 4)]
    pub indent: u32,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            directory: PathBuf::new(),
            project: None,
            exts: None,
            up: 0,
            depth: None,
            force: false,
        }
    }
}