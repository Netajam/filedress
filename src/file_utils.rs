// src/file_utils.rs

//! Contains helper functions for file discovery and comment style detection.

use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Returns an iterator over files in a directory that match the given extensions.
pub fn get_files<'a>(
    dir: &'a Path,
    exts: &'a [String],
) -> impl Iterator<Item = DirEntry> + 'a {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok()) // Ignore permission errors, etc.
        .filter(move |e| {
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .map_or(false, |s| exts.contains(&s.to_string()))
        })
}

/// Determines the correct single-line and multi-line comment syntax for a file.
/// Returns a tuple of (line_comment_prefix, block_comment_suffix).
pub fn get_comment_style(path: &Path) -> (&'static str, &'static str) {
    match path.extension().and_then(|s| s.to_str()) {
        // C-style, JS-style, etc.
        Some("ts" | "js" | "jsx" | "tsx" | "c" | "cpp" | "h" | "hpp" | "cs" | "go" | "java" | "rs" | "swift" | "kt") => ("//", ""),
        
        // CSS has a different block comment style but we'll use // for the header for simplicity
        Some("css" | "scss" | "less") => ("//", ""),

        // HTML, XML, Svelte
        Some("html" | "svelte" | "vue" | "xml" | "md") => ("<!--", "-->"),

        // Python, Ruby, Shell, etc.
        Some("py" | "rb" | "sh" | "bash" | "pl" | "Dockerfile" | "yaml" | "yml" | "toml") => ("#", ""),
        
        // PowerShell
        Some("ps1") => ("#", ""),

        // Default case
        _ => ("//", ""),
    }
}