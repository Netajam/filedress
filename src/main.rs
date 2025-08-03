use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// Using clap's derive feature to define our CLI tool's structure
#[derive(Parser)]
#[command(name = "filedress", version = "1.0", about = "A tool to dress up your source files with path headers.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a relative path header to files
    Add(Args), 
    /// Removes the path header from files
    Remove(Args),
    /// Removes all comments from files, except for the path header
    Clean(Args),
}

#[derive(Parser)]
struct Args {
    /// The root directory to search for files in
    #[arg(required = true)]
    directory: PathBuf,

    /// File extensions to process (e.g., "ts", "svelte")
    #[arg(short, long, value_delimiter = ',', default_values_t = ["ts".to_string(), "svelte".to_string()])]
    exts: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Match on the subcommand and call the appropriate handler function
    match &cli.command {
        Commands::Add(args) => handle_add(args)?,
        Commands::Remove(args) => handle_remove(args)?,
        Commands::Clean(args) => handle_clean(args)?,
    }

    Ok(())
}

/// Handles the 'add' subcommand logic
fn handle_add(args: &Args) -> Result<()> {
    println!("Searching in: {:?}", &args.directory);
    for entry in get_files(&args.directory, &args.exts) {
        let path = entry.path();

        // Determine comment style and create the header
        let (prefix, suffix) = get_comment_style(path);
        let relative_path = path
            .strip_prefix(&args.directory)?
            .to_string_lossy()
            .replace('\\', "/"); // Normalize to forward slashes

        let header = format!("{} Path: {} {}", prefix, relative_path, suffix).trim().to_string();

        // Efficiently read just the first line
        let file = fs::File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        reader.read_line(&mut first_line)?;

        // Check if header already exists
        if first_line.trim() == header {
            println!("[SKIP] Header exists: {}", path.display());
            continue;
        }

        // Prepend header and write back to file
        let original_content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;
        
        let new_content = format!("{}\n{}", header, original_content);
        fs::write(path, new_content)
            .with_context(|| format!("Failed to write to file: {}", path.display()))?;
        
        println!("[ADDED] Header to: {}", path.display());
    }
    println!("\n'add' command finished.");
    Ok(())
}

/// Handles the 'remove' subcommand logic
fn handle_remove(args: &Args) -> Result<()> {
    println!("Searching in: {:?}", &args.directory);
    for entry in get_files(&args.directory, &args.exts) {
        let path = entry.path();
        
        let file = fs::File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        reader.read_line(&mut first_line)?;

        // Check if the first line is our specific path header
        let (prefix, _) = get_comment_style(path);
        if first_line.trim().starts_with(&format!("{} Path:", prefix)) {
            let content = fs::read_to_string(path)?;
            let new_content: String = content.lines().skip(1).collect::<Vec<&str>>().join("\n");
            fs::write(path, new_content)
                .with_context(|| format!("Failed to write to file: {}", path.display()))?;
            println!("[REMOVED] Header from: {}", path.display());
        } else {
            println!("[SKIP] No header found: {}", path.display());
        }
    }
    println!("\n'remove' command finished.");
    Ok(())
}

/// Handles the 'clean' subcommand logic
fn handle_clean(args: &Args) -> Result<()> {
    println!("Searching in: {:?}", &args.directory);
    for entry in get_files(&args.directory, &args.exts) {
        let path = entry.path();
        let original_lines: Vec<String> = fs::read_to_string(path)?.lines().map(String::from).collect();
        let mut new_lines: Vec<String> = Vec::new();

        let (comment_prefix, _) = get_comment_style(path);
        let path_header_prefix = format!("{} Path:", comment_prefix);
        let mut in_block_comment = false;

        for line in &original_lines {
            let trimmed_line = line.trim();
            
            // Rule 1: Always keep our special path header
            if trimmed_line.starts_with(&path_header_prefix) {
                new_lines.push(line.clone());
                continue;
            }

            // Simple state machine for block comments (e.g., /* ... */)
            if trimmed_line.starts_with("/*") {
                in_block_comment = true;
            }
            let is_in_block = in_block_comment;
            if trimmed_line.ends_with("*/") {
                in_block_comment = false;
            }

            // Rule 2: Remove comment lines
            if !is_in_block && !trimmed_line.starts_with(comment_prefix) {
                 new_lines.push(line.clone());
            }
        }
        
        // Only write to the file if changes were actually made
        if new_lines.len() < original_lines.len() {
            fs::write(path, new_lines.join("\n"))?;
            println!("[CLEANED] Comments from: {}", path.display());
        } else {
            println!("[SKIP] No comments to clean: {}", path.display());
        }
    }
    println!("\n'clean' command finished.");
    Ok(())
}

/// Helper function to get an iterator of valid files
fn get_files<'a>(
    dir: &'a Path,
    exts: &'a [String],
) -> impl Iterator<Item = walkdir::DirEntry> + 'a {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok()) // Ignore errors
        .filter(move |e| {
            e.file_type().is_file() && 
            e.path().extension()
                .and_then(|s| s.to_str())
                .map_or(false, |s| exts.contains(&s.to_string()))
        })
}

/// Helper function to determine comment syntax based on file extension
fn get_comment_style(path: &Path) -> (&'static str, &'static str) {
    match path.extension().and_then(|s| s.to_str()) {
        Some("ts") | Some("js") | Some("css") => ("//", ""),
        Some("svelte") | Some("html") => ("<!--", "-->"),
        _ => ("#", ""), // Default for shell scripts, etc.
    }
}
