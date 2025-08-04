// src/commands/utils.rs

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use crate::cli::{Args, ProjectType};
use crate::file_utils::get_all_supported_extensions;

/// Determines the final list of extensions based on user arguments.
pub fn resolve_extensions(args: &Args) -> Vec<String> {
    if let Some(project_type) = &args.project {
        return match project_type {
            ProjectType::Rust => vec!["rs".to_string()],
            ProjectType::Python => vec!["py".to_string()],
            ProjectType::Web => vec!["ts", "js", "jsx", "tsx", "svelte", "vue", "html", "css", "scss"]
                .iter().map(|s| s.to_string()).collect(),
            ProjectType::Java => vec!["java".to_string(), "xml".to_string()],
            ProjectType::Flutter => vec!["dart".to_string()],
        };
    } else if let Some(custom_exts) = &args.exts {
        return custom_exts.clone();
    } else {
        return get_all_supported_extensions();
    }
}

/// Creates a configured WalkDir iterator.
pub fn create_file_walker<'a>(
    dir: &'a Path,
    exts: &'a [String],
    depth: Option<usize>,
) -> impl Iterator<Item = DirEntry> + 'a {
    let mut walker_builder = WalkDir::new(dir);
    if let Some(d) = depth {
        walker_builder = walker_builder.max_depth(d);
    }

    walker_builder.into_iter().filter_map(|e| e.ok()).filter(move |e| {
        e.file_type().is_file()
            && e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map_or(false, |s| exts.contains(&s.to_string()))
    })
}

/// Generates the path to be displayed in the header based on the target directory and --up levels.
pub fn generate_display_path(file_path: &Path, target_dir: &Path, up_levels: u32) -> Result<PathBuf> {
    let mut base_path = target_dir.to_path_buf();
    if let Some(parent) = base_path.parent() {
        base_path = parent.to_path_buf();
    }
    for _ in 0..up_levels {
        if let Some(parent) = base_path.parent() {
            base_path = parent.to_path_buf();
        } else {
            break;
        }
    }
    file_path
        .strip_prefix(&base_path)
        .map(|p| p.to_path_buf())
        .with_context(|| format!("Failed to create relative path for {}", file_path.display()))
}





// This tells Rust to only compile this module when running `cargo test`
#[cfg(test)]
mod tests {
    // Import everything from the parent module (the file itself)
    use super::*;
    // Also import the necessary structs/enums from other modules
    use crate::cli::{Args, ProjectType};
    use std::path::PathBuf;

    // A helper function to create a default Args struct for testing
    fn mock_args() -> Args {
        Args {
            directory: PathBuf::from("."),
            project: None,
            exts: None,
            up: 0,
            depth: None,
            force: false,
        }
    }

    #[test]
    fn test_resolve_project_preset() {
        let mut args = mock_args();
        args.project = Some(ProjectType::Python);
        let exts = resolve_extensions(&args);
        assert_eq!(exts, vec!["py".to_string()]);
    }

    #[test]
    fn test_resolve_custom_exts() {
        let mut args = mock_args();
        args.exts = Some(vec!["toml".to_string(), "yaml".to_string()]);
        let exts = resolve_extensions(&args);
        assert_eq!(exts, vec!["toml".to_string(), "yaml".to_string()]);
    }

    #[test]
    fn test_resolve_default_to_all() {
        // No project or exts are set in mock_args by default
        let args = mock_args();
        let exts = resolve_extensions(&args);
        // Check if it contains some known extensions from the master list
        assert!(exts.contains(&"rs".to_string()));
        assert!(exts.contains(&"py".to_string()));
        assert!(exts.contains(&"svelte".to_string()));
        // Check that it's not empty
        assert!(!exts.is_empty());
    }

    #[test]
    fn test_generate_display_path_default() {
        let target = Path::new("./project/src/app");
        let file = Path::new("./project/src/app/routes/page.js");
        let path = generate_display_path(file, target, 0).unwrap();
        // The default includes the target directory name
        assert_eq!(path, PathBuf::from("app/routes/page.js"));
    }

    #[test]
    fn test_generate_display_path_with_up() {
        let target = Path::new("./project/src/app");
        let file = Path::new("./project/src/app/routes/page.js");
        let path = generate_display_path(file, target, 1).unwrap();
        // Going up 1 level includes the parent of 'app', which is 'src'
        assert_eq!(path, PathBuf::from("src/app/routes/page.js"));
    }
}