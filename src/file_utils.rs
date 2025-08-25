// src/file_utils.rs

use std::path::Path; // Only necessary import at the top level

/// Determines the correct single-line and multi-line comment syntax for a file.
pub fn get_comment_style(path: &Path) -> (&'static str, &'static str) {
    match path.extension().and_then(|s| s.to_str()) {
        // C-style, JS-style, etc. (mostly // for single line, but block /**/ is common)
        Some("ts" | "js" | "jsx" | "tsx" | "c" | "cpp" | "h" | "hpp" | "cs" | "go" | "java" | "rs" | "swift" | "kt") => ("//", ""),
        
        // CSS uses /* */ for all comments
        Some("css" | "scss" | "less") => ("/*", "*/"), // Changed: now returns block comment style

        // HTML, XML, Svelte
        Some("html" | "svelte" | "vue" | "xml" | "md") => ("<!--", "-->"),

        // Python, Ruby, Shell, etc.
        Some("py" | "rb" | "sh" | "bash" | "pl" | "Dockerfile" | "yaml" | "yml" | "toml") => ("#", ""),
        
        // PowerShell
        Some("ps1") => ("#", ""),

        // Default case for unknown files
        _ => ("//", ""),
    }
}

/// Returns a master list of all file extensions supported by the application.
pub fn get_all_supported_extensions() -> Vec<String> {
    vec![
        // Web & JS
        "ts", "js", "jsx", "tsx", "svelte", "vue", "html", "css", "scss",
        // C-style languages
        "c", "cpp", "h", "hpp", "cs", "go", "java", "rs", "swift", "kt",
        // Scripting languages
        "py", "rb", "sh", "bash", "pl", "ps1",
        // Markup & Config
        "md", "xml", "yaml", "yml", "toml", "Dockerfile",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}