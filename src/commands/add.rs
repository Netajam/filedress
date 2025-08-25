// src/commands/add.rs

use anyhow::Result;
use std::fs;
use std::io::{BufRead, BufReader};

use crate::cli::Args;
use crate::file_utils::get_comment_style;
use super::utils::{create_file_walker, generate_display_path, resolve_extensions};

pub fn add(args: &Args) -> Result<()> {
    println!("Searching in: {:?}", &args.directory);
    let extensions = resolve_extensions(args);
    let walker = create_file_walker(&args.directory, &extensions, args.depth);

    for entry in walker {
        let file_path = entry.path();
        let display_path = generate_display_path(file_path, &args.directory, args.up)?;
        let (prefix, suffix) = get_comment_style(file_path);
        
        // FIX: Only add a space before the suffix if the suffix is not empty.
        let header = if suffix.is_empty() {
            format!("{} Path:{}", prefix, display_path.display()).trim().to_string()
        } else {
            format!("{} Path: {} {}", prefix, display_path.display(), suffix).trim().to_string()
        };

        let mut first_line = String::new();
        if fs::File::open(file_path).and_then(|f| BufReader::new(f).read_line(&mut first_line)).is_err() {
            continue;
        }

        let is_path_header = first_line.trim().starts_with(&format!("{} Path:", prefix));

        if is_path_header && !args.force {
            println!("[SKIP] Header exists (use --force to overwrite): {}", file_path.display());
            continue;
        }

        let original_content = if is_path_header && args.force {
            let full_content = fs::read_to_string(file_path)?;
            full_content.lines().skip(1).collect::<Vec<&str>>().join("\n")
        } else {
            fs::read_to_string(file_path)?
        };

        let new_content = format!("{}\n{}", header, original_content);
        fs::write(file_path, new_content)?;

        let action = if is_path_header && args.force { "[REPLACED]" } else { "[ADDED]" };
        println!("{} Header in: {}", action, file_path.display());
    }
    println!("\n'add' command finished.");
    Ok(())
}