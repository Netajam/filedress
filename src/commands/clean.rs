// src/commands/clean.rs

use anyhow::Result;
use std::fs;
use std::path::Path; // Keep this import as `clean` itself needs `Path`

use crate::cli::Args;
use crate::file_utils::get_comment_style;
use super::utils::{create_file_walker, resolve_extensions};

/// Helper function to remove single-line and inline comments from a line,
/// ensuring that comment markers within string literals are preserved.
/// It works for single-line comment styles like `//` or `#`.
fn clean_line_of_code(line: &str, comment_prefix: &str) -> String {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut comment_start_idx: Option<usize> = None;

    let mut chars = line.chars().enumerate().peekable();

    while let Some((i, c)) = chars.next() {
        // Handle escaped characters
        if c == '\\' {
            // If it's an escape, skip the next character as well.
            // This is a basic form, might not cover all edge cases (e.g., unicode escapes)
            if chars.peek().is_some() {
                chars.next(); 
            }
            continue;
        }

        // Toggle string state
        if c == '\'' && !in_double_quote {
            in_single_quote = !in_single_quote;
        } else if c == '"' && !in_single_quote {
            in_double_quote = !in_double_quote;
        }

        // Check for comment prefix only if NOT inside a string literal
        if !in_single_quote && !in_double_quote {
            if line[i..].starts_with(comment_prefix) {
                comment_start_idx = Some(i);
                break; // Found the start of a comment outside a string
            }
        }
    }

    match comment_start_idx {
        Some(idx) => line[..idx].trim_end().to_string(),
        None => line.trim_end().to_string(), // No comment found or comment was inside a string
    }
}

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

        let (comment_prefix, comment_suffix) = get_comment_style(path);
        let path_header_prefix = format!("{} Path:", comment_prefix);
        let mut in_block_comment = false;
        let mut in_python_docstring = false;

        let is_python = path.extension().and_then(|s| s.to_str()) == Some("py");

        for line in &original_lines {
            let trimmed_line = line.trim();
            let mut processed_line_content = String::new(); // Accumulates non-comment parts of this line

            // 1. Path header always stays
            if trimmed_line.starts_with(&path_header_prefix) {
                new_lines.push(line.clone());
                continue;
            }

            // 2. Python docstring handling (highest priority for Python files)
            if is_python {
                let starts_triple_double = trimmed_line.starts_with("\"\"\"");
                let starts_triple_single = trimmed_line.starts_with("'''");
                let ends_triple_double = trimmed_line.ends_with("\"\"\"");
                let ends_triple_single = trimmed_line.ends_with("'''");

                if (starts_triple_double || starts_triple_single) && !in_python_docstring {
                    in_python_docstring = true;
                    new_lines.push(line.clone());
                    continue;
                } else if in_python_docstring {
                    new_lines.push(line.clone());
                    if (ends_triple_double && trimmed_line.matches("\"\"\"").count() >= 2) || // Ensure it's not just a quote inside
                       (ends_triple_single && trimmed_line.matches("'''").count() >= 2) {
                        in_python_docstring = false;
                    }
                    continue;
                }
            }
            // If we are currently in a Python docstring, and it didn't start/end on this line, skip it.
            if in_python_docstring {
                continue; 
            }

            // 3. Handle generic block comments (e.g., /* ... */, <!-- ... -->)
            let mut current_segment = line.as_str(); // Use slices for efficient processing

            // If we're already inside a multi-line block comment
            if in_block_comment {
                if let Some(end_idx) = current_segment.find(comment_suffix) {
                    current_segment = &current_segment[end_idx + comment_suffix.len()..];
                    in_block_comment = false;
                } else {
                    // Entire line is part of an ongoing block comment, skip it
                    continue;
                }
            }

            // Process line for inline block comments or start of new blocks
            while let Some(start_idx) = current_segment.find(comment_prefix) {
                processed_line_content.push_str(&current_segment[..start_idx]); // Add code before comment
                current_segment = &current_segment[start_idx..]; // Move past the code part

                if let Some(end_idx) = current_segment.find(comment_suffix) {
                    // Found an inline block comment, remove it
                    current_segment = &current_segment[end_idx + comment_suffix.len()..];
                } else {
                    // Block comment starts but doesn't end on this line
                    in_block_comment = true;
                    current_segment = ""; // Remove rest of the line as it's part of the block comment
                    break; // Stop processing this line for comments
                }
            }
            processed_line_content.push_str(current_segment); // Add any remaining code after block comments

            // 4. Handle single-line and inline comments (for // and # files)
            // Apply clean_line_of_code to the content *after* block comments have been processed.
            if comment_suffix.is_empty() { 
                let cleaned_single_line = clean_line_of_code(&processed_line_content, comment_prefix);
                if !cleaned_single_line.is_empty() {
                    new_lines.push(cleaned_single_line);
                }
            } else {
                // For block-comment languages, after block comment removal, push remaining code.
                let trimmed_final = processed_line_content.trim_end().to_string();
                if !trimmed_final.is_empty() {
                    new_lines.push(trimmed_final);
                }
            }
        }

        // Only write if there's a change in content (after normalizing line endings for comparison)
        let new_content_str = new_lines.join("\n");
        let original_content_str = original_lines.join("\n");
        
        let new_content_normalized = new_content_str.replace("\r\n", "\n").trim_end_matches('\n').to_string();
        let original_content_normalized = original_content_str.replace("\r\n", "\n").trim_end_matches('\n').to_string();

        let has_changed = new_content_normalized != original_content_normalized;

        if has_changed {
            let final_content = if new_content_normalized.is_empty() {
                "".to_string()
            } else {
                format!("{}\n", new_content_normalized) // Add a single trailing newline for consistency
            };
            fs::write(path, final_content)?;
            println!("[CLEANED] Comments from: {}", path.display());
        } else {
            println!("[SKIP] No comments to clean: {}", path.display());
        }
    }
    println!("\n'clean' command finished.");
    Ok(())
}