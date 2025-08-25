// src/commands/copy.rs

use anyhow::{Context, Result};
use arboard::Clipboard;
use std::fs;
use std::path::PathBuf;

use crate::cli::Args;
use super::utils::{create_file_walker, generate_display_path, resolve_extensions}; // THESE IMPORTS MUST BE PRESENT

/// Handles the 'copy' subcommand logic.
pub fn copy(args: &Args) -> Result<()> {
    println!("Searching for files to copy in: {:?}", &args.directory);
    let extensions = resolve_extensions(args);
    let walker = create_file_walker(&args.directory, &extensions, args.depth);

    let mut paths_to_copy: Vec<PathBuf> = walker.map(|e| e.path().to_path_buf()).collect();
    if paths_to_copy.is_empty() {
        println!("No files found matching the criteria. Clipboard remains unchanged.");
        return Ok(());
    }
    paths_to_copy.sort();

    let mut clipboard = Clipboard::new().context("Failed to initialize clipboard")?;
    let mut combined_content = String::new();
    let mut total_bytes = 0;

    for (i, path) in paths_to_copy.iter().enumerate() {
        println!("[COPYING] {}", path.display());
        let display_path = generate_display_path(path, &args.directory, args.up)?;
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        total_bytes += content.len();

        if i > 0 {
            combined_content.push_str("\n\n---\n");
        }
        
        combined_content.push_str(&format!("FILE: {}\n---\n\n", display_path.to_string_lossy()));
        combined_content.push_str(&content);
    }

    clipboard.set_text(combined_content)
        .context("Failed to copy content to clipboard")?;

    println!(
        "\n✅ Copied {} files ({} bytes) to the clipboard.",
        paths_to_copy.len(),
        total_bytes
    );

    Ok(())
}