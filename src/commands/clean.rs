// src/commands/clean.rs

use anyhow::Result;
use std::fs;

use crate::cli::Args;
use crate::file_utils::get_comment_style;
use super::utils::{create_file_walker, resolve_extensions};

/// Handles the 'clean' subcommand logic.
pub fn clean(args: &Args) -> Result<()> {
    println!("Searching in: {:?}", &args.directory);
    let extensions = resolve_extensions(args);
    let walker = create_file_walker(&args.directory, &extensions, args.depth);

    for entry in walker {
        let path = entry.path();
        let original_lines: Vec<String> =
            fs::read_to_string(path)?.lines().map(String::from).collect();
        let mut new_lines: Vec<String> = Vec::new();

        let (comment_prefix, _) = get_comment_style(path);
        let path_header_prefix = format!("{} Path:", comment_prefix);
        let mut in_block_comment = false;

        for line in &original_lines {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with(&path_header_prefix) {
                new_lines.push(line.clone());
                continue;
            }
            if trimmed_line.starts_with("/*") && !trimmed_line.ends_with("*/") {
                in_block_comment = true;
            }
            let is_in_block = in_block_comment;
            if trimmed_line.ends_with("*/") {
                in_block_comment = false;
            }
            if !is_in_block && !trimmed_line.starts_with(comment_prefix) {
                new_lines.push(line.clone());
            }
        }

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