// src/commands/remove.rs

use anyhow::Result;
use std::fs;
use std::io::{BufRead, BufReader};

use crate::cli::Args;
use crate::file_utils::get_comment_style;
use super::utils::{create_file_walker, resolve_extensions}; // THESE IMPORTS MUST BE PRESENT

pub fn remove(args: &Args) -> Result<()> {
    println!("Searching in: {:?}", &args.directory);
    let extensions = resolve_extensions(args);
    let walker = create_file_walker(&args.directory, &extensions, args.depth);

    for entry in walker {
        let path = entry.path();
        let mut first_line = String::new();
        if fs::File::open(path).and_then(|f| BufReader::new(f).read_line(&mut first_line)).is_err() {
            continue;
        }

        let (prefix, _) = get_comment_style(path);
        if first_line.trim().starts_with(&format!("{} Path:", prefix)) {
            let content = fs::read_to_string(path)?;
            let new_content: String = content.lines().skip(1).collect::<Vec<&str>>().join("\n");
            fs::write(path, new_content)?;
            println!("[REMOVED] Header from: {}", path.display());
        } else {
            println!("[SKIP] No header found: {}", path.display());
        }
    }
    println!("\n'remove' command finished.");
    Ok(())
}