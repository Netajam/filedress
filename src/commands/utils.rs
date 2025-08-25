// FILE: .\commands\utils.rs

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
    let absolute_target_dir = target_dir.canonicalize()
        .with_context(|| format!("Failed to canonicalize target directory: {}", target_dir.display()))?;
    let absolute_file_path = file_path.canonicalize()
        .with_context(|| format!("Failed to canonicalize file path: {}", file_path.display()))?;

    // Determine the base path from which to calculate the relative path.
    // If up_levels is 0, the base is the target_dir itself.
    // If up_levels > 0, we move up from target_dir's parent.
    let mut base_for_relative_path = absolute_target_dir.clone();

    // The 'up' logic should go up from the *effective starting point* of the relative path,
    // not necessarily from the target_dir directly.
    // The previous logic was causing paths like "config.py" instead of "project_root/config.py"
    // when `up=0` and `target_dir` was `project_root`.
    // Let's reset `base_for_relative_path` to the original `target_dir` first,
    // and then go up `up_levels`. This makes it relative to the directory chosen by `up`.

    // Calculate the effective root to strip from file_path
    let mut effective_strip_root = absolute_target_dir.clone(); // Start at target_dir

    for _ in 0..up_levels {
        if let Some(parent) = effective_strip_root.parent() {
            effective_strip_root = parent.to_path_buf();
        } else {
            // Cannot go up further, probably at filesystem root
            break;
        }
    }

    // Strip the `effective_strip_root` from the `absolute_file_path`.
    // The returned path will be relative to `effective_strip_root`.
    absolute_file_path
        .strip_prefix(&effective_strip_root)
        .map(|p| p.to_path_buf())
        .with_context(|| format!("Failed to create relative path for {} from base {}", file_path.display(), effective_strip_root.display()))
}


// This tells Rust to only compile this module when running `cargo test`
#[cfg(test)]
mod tests {
    // Import everything from the parent module (the file itself)
    use super::*;
    // Also import the necessary structs/enums from other modules
    use crate::cli::{Args, ProjectType};
    use std::fs; // Needed for tempdir and file operations in tests
    use tempfile::tempdir; // Needed for tempdir

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
        let args = mock_args();
        let exts = resolve_extensions(&args);
        assert!(exts.contains(&"rs".to_string()));
        assert!(exts.contains(&"py".to_string()));
        assert!(exts.contains(&"svelte".to_string()));
        assert!(!exts.is_empty());
    }

    // New tests for generate_display_path with canonicalized paths for robustness
    #[test]
    fn test_generate_display_path_simple() -> Result<()> {
        let temp_dir = tempdir()?;
        let project_root = temp_dir.path().join("my_project");
        fs::create_dir_all(&project_root)?;
        let src_dir = project_root.join("src");
        fs::create_dir_all(&src_dir)?;
        let file_path = src_dir.join("main.rs");
        fs::File::create(&file_path)?;

        let target_dir = project_root.clone(); // `filedress add .` (relative to my_project)
        let path = generate_display_path(&file_path, &target_dir, 0)?;
        // Expected: src/main.rs (relative to my_project, if target_dir is my_project)
        assert_eq!(path, PathBuf::from("src").join("main.rs")); 

        Ok(())
    }

    #[test]
    fn test_generate_display_path_with_up() -> Result<()> {
        let temp_dir = tempdir()?;
        let repo_root = temp_dir.path().join("repo");
        fs::create_dir_all(&repo_root)?;
        let project_root = repo_root.join("my_project");
        fs::create_dir_all(&project_root)?;
        let src_dir = project_root.join("src");
        fs::create_dir_all(&src_dir)?;
        let file_path = src_dir.join("main.rs");
        fs::File::create(&file_path)?;

        let target_dir = project_root.clone(); // `filedress add my_project -u 1` (target_dir is my_project, go up 1 level to repo)
        let path = generate_display_path(&file_path, &target_dir, 1)?; 
        // Expected: my_project/src/main.rs (relative to repo)
        assert_eq!(path, PathBuf::from("my_project").join("src").join("main.rs"));

        Ok(())
    }

    #[test]
    fn test_generate_display_path_from_deep_dir_with_up() -> Result<()> {
        let temp_dir = tempdir()?;
        let root = temp_dir.path().join("monorepo");
        fs::create_dir_all(&root)?;
        let app_dir = root.join("apps").join("frontend");
        fs::create_dir_all(&app_dir)?;
        let pages_dir = app_dir.join("pages");
        fs::create_dir_all(&pages_dir)?;
        let file_path = pages_dir.join("index.js");
        fs::File::create(&file_path)?;

        let target_dir = app_dir.clone(); // `filedress add apps/frontend --up 2` (target_dir is frontend, go up 2 levels to monorepo)
        let path = generate_display_path(&file_path, &target_dir, 2)?; 
        // Expected: apps/frontend/pages/index.js (relative to monorepo)
        assert_eq!(path, PathBuf::from("apps").join("frontend").join("pages").join("index.js"));

        Ok(())
    }
}