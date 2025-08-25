// src/commands/clean.rs

use anyhow::Result;
use std::fs;
use std::path::Path; 

use crate::cli::Args;
use crate::file_utils::get_comment_style;
use super::utils::{create_file_walker, resolve_extensions};

/// Helper function to remove single-line and inline comments from a line,
/// ensuring that comment markers within string literals are preserved.
/// It works for single-line comment styles like `//` or `#`.
fn clean_line_of_code(line: &str, comment_prefix: &str) -> String {
    if comment_prefix.is_empty() {
        return line.trim_end().to_string(); 
    }

    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut idx = 0;

    while idx < line.len() {
        let current_char_start_idx = idx;
        let char_len = line[idx..].chars().next().map_or(0, |c| c.len_utf8());
        if char_len == 0 { break; }

        let c = line[idx..].chars().next().unwrap();

        // Handle escaped characters
        if c == '\\' {
            idx += char_len; 
            if idx < line.len() {
                idx += line[idx..].chars().next().map_or(0, |c| c.len_utf8()); 
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
            if line[current_char_start_idx..].starts_with(comment_prefix) {
                return line[..current_char_start_idx].trim_end().to_string();
            }
        }
        
        idx += char_len; 
    }

    line.trim_end().to_string()
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

        // Determine specific comment styles for the current file extension
        let (single_line_prefix_str, block_comment_start_str, block_comment_end_str) = {
            let file_ext = path.extension().and_then(|s| s.to_str());
            match file_ext {
                Some("c" | "cpp" | "h" | "hpp" | "cs" | "go" | "java" | "rs" | "swift" | "kt") => 
                    ("//", "/*", "*/"),
                Some("js" | "ts" | "jsx" | "tsx") => 
                    ("//", "/*", "*/"),
                Some("css" | "scss" | "less") => 
                    ("", "/*", "*/"), // These only use block comments
                Some("html" | "svelte" | "vue" | "xml" | "md") => 
                    ("", "<!--", "-->"), // These only use HTML-style block comments
                Some("py" | "rb" | "sh" | "bash" | "pl" | "Dockerfile" | "yaml" | "yml" | "toml" | "ps1") => 
                    ("#", "", ""), // These only use single-line comments
                _ => ("//", "", ""), // Default to C-style single-line if unknown
            }
        };

        let path_header_prefix_single_line = format!("{} Path:", single_line_prefix_str);
        let path_header_prefix_block_start = format!("{} Path:", block_comment_start_str);

        let mut in_multi_line_block_comment = false; 
        let mut in_python_triple_double_quote_string = false;
        let mut in_python_triple_single_quote_string = false;

        let is_python = path.extension().and_then(|s| s.to_str()) == Some("py");

        for line_num in 0..original_lines.len() {
            let line = &original_lines[line_num];
            let trimmed_line = line.trim();
            let mut current_processed_line_content = String::new(); 
            let mut remaining_line_segment = line.as_str(); 

            // 1. Path header always stays
            if trimmed_line.starts_with(&path_header_prefix_single_line) || trimmed_line.starts_with(&path_header_prefix_block_start) {
                new_lines.push(line.clone());
                continue;
            }

            // --- Python Triple-Quoted String/Docstring Handling ---
            if is_python {
                let num_triple_double = line.matches("\"\"\"").count();
                let num_triple_single = line.matches("'''").count();

                let was_in_multiline_string = in_python_triple_double_quote_string || in_python_triple_single_quote_string;

                // Toggle state if an odd number of delimiters is found
                if num_triple_double % 2 != 0 {
                    in_python_triple_double_quote_string = !in_python_triple_double_quote_string;
                }
                if num_triple_single % 2 != 0 {
                    in_python_triple_single_quote_string = !in_python_triple_single_quote_string;
                }

                // Preserve the line if it was or is part of a multiline string/docstring
                if was_in_multiline_string || num_triple_double > 0 || num_triple_single > 0 {
                    new_lines.push(line.clone());
                    continue;
                }
            }
            
            // --- Block Comment Handling (e.g., /* ... */, <!-- ... -->) ---
            if !block_comment_start_str.is_empty() && !block_comment_end_str.is_empty() {
                // If currently inside a multi-line block comment
                if in_multi_line_block_comment {
                    if let Some(end_idx) = remaining_line_segment.find(block_comment_end_str) {
                        current_processed_line_content.push_str(&remaining_line_segment[end_idx + block_comment_end_str.len()..]);
                        remaining_line_segment = ""; 
                        in_multi_line_block_comment = false;
                    } else {
                        continue; // Entire line is part of an ongoing multi-line block comment, skip it
                    }
                }

                // Process for any block comments (inline or new multi-line starts)
                while let Some(start_idx) = remaining_line_segment.find(block_comment_start_str) {
                    current_processed_line_content.push_str(&remaining_line_segment[..start_idx]); 
                    remaining_line_segment = &remaining_line_segment[start_idx + block_comment_start_str.len()..]; 

                    if let Some(end_idx) = remaining_line_segment.find(block_comment_end_str) {
                        remaining_line_segment = &remaining_line_segment[end_idx + block_comment_end_str.len()..];
                    } else {
                        in_multi_line_block_comment = true;
                        remaining_line_segment = ""; 
                        break; 
                    }
                }
            }
            current_processed_line_content.push_str(remaining_line_segment); 

            // --- Single-line Comment Handling (// or #) ---
            // This applies to any remaining content after docstrings and block comments.
            if !single_line_prefix_str.is_empty() { 
                let cleaned_single_line = clean_line_of_code(&current_processed_line_content, single_line_prefix_str);
                if !cleaned_single_line.is_empty() {
                    new_lines.push(cleaned_single_line);
                }
            } else {
                // If no single-line prefix for this language (e.g., HTML, CSS),
                // just push remaining content after block comment processing.
                let trimmed_final = current_processed_line_content.trim_end().to_string();
                if !trimmed_final.is_empty() {
                    new_lines.push(trimmed_final);
                }
            }
        }

        // Final content comparison and write
        let new_content_str = new_lines.join("\n");
        let original_content_str = original_lines.join("\n");
        
        let new_content_normalized = new_content_str.replace("\r\n", "\n").trim_end_matches('\n').to_string();
        let original_content_normalized = original_content_str.replace("\r\n", "\n").trim_end_matches('\n').to_string();

        let has_changed = new_content_normalized != original_content_normalized;

        if has_changed {
            let final_content = if new_content_normalized.is_empty() {
                "".to_string()
            } else {
                format!("{}\n", new_content_normalized)
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