use anyhow::{Context, Result};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::cli::{Args, Commands, ProjectType};
use crate::file_utils::{get_all_supported_extensions, get_comment_style};

/// Main dispatcher function that calls the appropriate handler based on the command.
pub fn handle_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Add(args) => add(args)?,
        Commands::Remove(args) => remove(args)?,
        Commands::Clean(args) => clean(args)?,
    }
    Ok(())
}

/// Handles the 'add' subcommand logic.
fn add(args: &Args) -> Result<()> {
    println!("Searching in: {:?}", &args.directory);
    let extensions = resolve_extensions(args);
    let walker = create_file_walker(&args.directory, &extensions, args.depth);

    for entry in walker {
        let file_path = entry.path();
        let display_path = generate_display_path(file_path, &args.directory, args.up)?;
        let (prefix, suffix) = get_comment_style(file_path);
        let header = format!("{} Path: {} {}", prefix, display_path.display(), suffix)
            .trim()
            .to_string();

        let mut first_line = String::new();
        if fs::File::open(file_path)
            .and_then(|f| BufReader::new(f).read_line(&mut first_line))
            .is_err()
        {
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

/// Handles the 'remove' subcommand logic.
fn remove(args: &Args) -> Result<()> {
    println!("Searching in: {:?}", &args.directory);
    let extensions = resolve_extensions(args);
    let walker = create_file_walker(&args.directory, &extensions, args.depth);

    for entry in walker {
        let path = entry.path();
        let mut first_line = String::new();
        if fs::File::open(path)
            .and_then(|f| BufReader::new(f).read_line(&mut first_line))
            .is_err()
        {
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

/// Handles the 'clean' subcommand logic.
fn clean(args: &Args) -> Result<()> {
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

// --- Helper Functions ---

/// Determines the final list of extensions based on user arguments.
fn resolve_extensions(args: &Args) -> Vec<String> {
    if let Some(project_type) = &args.project {
        // 1. User specified --project, use its specific list.
        return match project_type {
            ProjectType::Rust => vec!["rs".to_string()],
            ProjectType::Python => vec!["py".to_string()],
            ProjectType::Web => vec!["ts", "js", "jsx", "tsx", "svelte", "vue", "html", "css", "scss"]
                .iter().map(|s| s.to_string()).collect(),
            ProjectType::Java => vec!["java".to_string(), "xml".to_string()],
            ProjectType::Flutter => vec!["dart".to_string()],
        };
    } else if let Some(custom_exts) = &args.exts {
        // 2. User specified --exts, use that custom list.
        return custom_exts.clone();
    } else {
        // 3. User specified NEITHER, so we default to ALL supported extensions.
        return get_all_supported_extensions();
    }
}

/// Creates a configured WalkDir iterator.
fn create_file_walker<'a>(
    dir: &'a Path,
    exts: &'a [String],
    depth: Option<usize>,
) -> impl Iterator<Item = walkdir::DirEntry> + 'a {
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
fn generate_display_path(file_path: &Path, target_dir: &Path, up_levels: u32) -> Result<PathBuf> {
    let mut base_path = target_dir.to_path_buf();

    // By default, the base for stripping is the PARENT of the target, including the target's name
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