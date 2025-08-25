// tests/integration_tests.rs

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};

use filedress::cli::{Args, Commands};
use filedress::commands::handle_command;
// No longer needs: use filedress::commands::clean::clean as clean_command_func;

// --- Original Test Environment (kept for existing tests) ---
/// A helper struct to hold the paths of our test environment.
struct TestEnv {
    _temp_dir: TempDir, // The tempdir must be kept in scope to prevent premature deletion
    project_root: PathBuf,
    v1_dir: PathBuf,
    config_file: PathBuf,
    user_file: PathBuf,
    initial_content: String,
}

/// Helper function to create a standard, nested test environment.
fn setup_test_environment() -> Result<TestEnv> {
    let temp_dir = tempdir()?;
    let project_root_name = "project_root"; // Define the name part
    let project_root = temp_dir.path().join(project_root_name); // This is the root for the test
    fs::create_dir_all(&project_root)?;
    let src_dir = project_root.join("src").join("api").join("v1"); // Use join for platform-agnostic paths
    fs::create_dir_all(&src_dir)?;

    let config_file = project_root.join("config.py");
    let user_file = src_dir.join("user.py");
    let initial_content = "pass".to_string();

    fs::write(&config_file, &initial_content)?;
    fs::write(&user_file, &initial_content)?;

    Ok(TestEnv {
        _temp_dir: temp_dir,
        project_root,
        v1_dir: src_dir,
        config_file,
        user_file,
        initial_content,
    })
}

// --- Existing tests ---
#[test]
fn test_add_and_remove_header_simple() -> Result<()> {
    // ARRANGE
    let env = setup_test_environment()?;
    let add_args = Args {
        directory: env.project_root.clone(),
        exts: Some(vec!["py".to_string()]),
        up: 0, // Explicitly set to 0 for clarity
        ..Default::default()
    };
    
    // ACT (ADD)
    handle_command(&Commands::Add(add_args))?;

    // ASSERT (ADD)
    let content = fs::read_to_string(&env.config_file)?;
    // If up is 0 and target_dir is project_root, path should be relative to project_root, which is "config.py"
    let expected_relative_path_part = PathBuf::from("config.py"); 
    let expected_header_prefix = format!("# Path:{}", expected_relative_path_part.display()); // No trailing space after Path:
    assert!(content.starts_with(&expected_header_prefix), "Header mismatch for add simple: Expected prefix '{}', Got content starting with '{}'", expected_header_prefix, content.lines().next().unwrap_or(""));

    // ACT (REMOVE)
    let remove_args = Args {
        directory: env.project_root.clone(),
        exts: Some(vec!["py".to_string()]),
        ..Default::default()
    };
    handle_command(&Commands::Remove(remove_args))?;

    // ASSERT (REMOVE)
    let content = fs::read_to_string(&env.config_file)?;
    assert_eq!(content.trim(), env.initial_content);

    Ok(())
}


#[test]
fn test_up_parameter() -> Result<()> {
    // ARRANGE
    let env = setup_test_environment()?;
    let add_args = Args {
        directory: env.v1_dir.clone(), // Target the deepest directory: project_root/src/api/v1
        exts: Some(vec!["py".to_string()]),
        up: 2, // Go up 2 levels from v1_dir (v1_dir -> api -> src). So relative to 'src'.
        ..Default::default()
    };

    // ACT
    handle_command(&Commands::Add(add_args))?;

    // ASSERT
    let content = fs::read_to_string(&env.user_file)?;
    // File: project_root/src/api/v1/user.py
    // Target: project_root/src/api/v1
    // Up 2 levels means relative to project_root/src
    // So, expected is "api/v1/user.py"
    let expected_relative_path_part = PathBuf::from("api").join("v1").join("user.py"); 
    let expected_header = format!("# Path:{}", expected_relative_path_part.display()); // No trailing space after Path:
    let first_line = content.lines().next().unwrap_or("").trim_end().to_string(); // Trim actual first line too for exact match

    assert_eq!(
        first_line,
        expected_header.trim_end(),
        "\nHeader mismatch for --up test!\n  Expected: '{}'\n  Got:      '{}'",
        expected_header.trim_end(),
        first_line
    );

    Ok(())
}

#[test]
fn test_depth_parameter_shallow() -> Result<()> {
    // ARRANGE
    let env = setup_test_environment()?;
    let add_args = Args {
        directory: env.project_root.clone(), // Target the root of our test project
        exts: Some(vec!["py".to_string()]),
        depth: Some(1), // ONLY search in the immediate directory
        up: 0, // Explicitly set to 0
        ..Default::default()
    };

    // ACT
    handle_command(&Commands::Add(add_args))?;

    // ASSERT
    // 1. The shallow file SHOULD have a header.
    let config_content = fs::read_to_string(&env.config_file)?;
    // If up is 0, path should be relative to `directory`, i.e., "config.py"
    let expected_relative_path_config = PathBuf::from("config.py");
    let expected_header_config_prefix = format!("# Path:{}", expected_relative_path_config.display()); // No trailing space after Path:
    assert!(config_content.starts_with(&expected_header_config_prefix), "Header mismatch for depth shallow (config): Expected prefix '{}', Got content starting with '{}'", expected_header_config_prefix, config_content.lines().next().unwrap_or(""));

    // 2. The deep file SHOULD NOT have been modified.
    let user_content = fs::read_to_string(&env.user_file)?;
    assert_eq!(
        user_content.trim(),
        env.initial_content,
        "\nDepth test failed: Deep file should NOT have been modified with depth=1.\n"
    );

    Ok(())
}

#[test]
fn test_depth_parameter_deep() -> Result<()> {
    // ARRANGE
    let env = setup_test_environment()?;
    let add_args = Args {
        directory: env.project_root.clone(),
        exts: Some(vec!["py".to_string()]),
        depth: Some(4), // A depth deep enough to find user.py
        up: 0, // Explicitly set to 0
        ..Default::default()
    };

    // ACT
    handle_command(&Commands::Add(add_args))?;

    // ASSERT
    // Both files should now have headers.
    let config_content = fs::read_to_string(&env.config_file)?;
    // If up is 0, path should be relative to `directory`, i.e., "config.py"
    let expected_relative_path_config = PathBuf::from("config.py");
    let expected_header_config_prefix = format!("# Path:{}", expected_relative_path_config.display()); // No trailing space after Path:
    assert!(config_content.starts_with(&expected_header_config_prefix), "Header mismatch for depth deep (config): Expected prefix '{}', Got content starting with '{}'", expected_header_config_prefix, config_content.lines().next().unwrap_or(""));

    let user_content = fs::read_to_string(&env.user_file)?;
    // If up is 0, path should be relative to `directory`, i.e., "src/api/v1/user.py"
    let expected_relative_path_user = PathBuf::from("src").join("api").join("v1").join("user.py");
    let expected_header_user_prefix = format!("# Path:{}", expected_relative_path_user.display()); // No trailing space after Path:
    assert!(user_content.starts_with(&expected_header_user_prefix), "Header mismatch for depth deep (user): Expected prefix '{}', Got content starting with '{}'", expected_header_user_prefix, user_content.lines().next().unwrap_or(""));

    Ok(())
}


// --- NEW Clean Test Environment ---
struct CleanTestEnv {
    _temp_dir: TempDir,
    root: PathBuf,
    python_file: PathBuf,
    rust_file: PathBuf,
    css_file: PathBuf,
    html_file: PathBuf,
    file_no_comments: PathBuf,
    file_with_header_only: PathBuf,
    complex_rust_file: PathBuf,
    complex_python_file: PathBuf,
    python_file_with_strings: PathBuf,
    rust_file_with_strings: PathBuf,
    // Add original contents for direct function testing
    original_my_rust_content: String,
    original_string_python_content: String,
    original_complex_python_content: String,
}

fn setup_clean_test_files() -> Result<CleanTestEnv> {
    let temp_dir = tempdir()?;
    let root = temp_dir.path().join("clean_test_root");
    fs::create_dir_all(&root)?;

    let tests_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("test_files");

    let write_test_file_from_source = |target_dir: &Path, file_name: &str, source_file_name: &str| -> Result<PathBuf> {
        let source_path = tests_data_dir.join(source_file_name);
        let content = fs::read_to_string(&source_path)
            .with_context(|| format!("Failed to read source test file: {}", source_path.display()))?;
        let target_path = target_dir.join(file_name);
        fs::write(&target_path, content)?;
        Ok(target_path)
    };

    let original_my_rust_content = fs::read_to_string(&tests_data_dir.join("my_rust.rs.input"))?;
    let original_string_python_content = fs::read_to_string(&tests_data_dir.join("string_python.py.input"))?;
    let original_complex_python_content = fs::read_to_string(&tests_data_dir.join("complex_python.py.input"))?;


    Ok(CleanTestEnv {
        python_file: write_test_file_from_source(&root, "my_python.py", "my_python.py.input")?,
        rust_file: write_test_file_from_source(&root, "my_rust.rs", "my_rust.rs.input")?,
        css_file: write_test_file_from_source(&root, "my_style.css", "my_style.css.input")?,
        html_file: write_test_file_from_source(&root, "my_page.html", "my_page.html.input")?,
        file_no_comments: write_test_file_from_source(&root, "no_comments.rs", "no_comments.rs.input")?,
        file_with_header_only: write_test_file_from_source(&root, "only_header.py", "only_header.py.input")?,
        complex_rust_file: write_test_file_from_source(&root, "complex.rs", "complex.rs.input")?,
        complex_python_file: write_test_file_from_source(&root, "complex_python.py", "complex_python.py.input")?,
        python_file_with_strings: write_test_file_from_source(&root, "string_python.py", "string_python.py.input")?,
        rust_file_with_strings: write_test_file_from_source(&root, "string_rust.rs", "string_rust.rs.input")?,
        _temp_dir: temp_dir,
        root,
        original_my_rust_content,
        original_string_python_content,
        original_complex_python_content,
    })
}

// Helper to run the clean command (the main command handler, not the direct function)
fn run_clean_command_on_dir(directory: &PathBuf) -> Result<()> {
    let clean_args = Args {
        directory: directory.clone(),
        ..Default::default()
    };
    handle_command(&Commands::Clean(clean_args))
}

// Helper to read file and assert content
fn assert_file_content(path: &Path, expected_content_raw: &str) -> Result<()> {
    let actual_content = fs::read_to_string(path)?;
    
    // For debugging the weird concatenation issue (uncomment to see output):
    dbg!(&path);
    dbg!(&actual_content); // This will print the raw content from the file

    // Normalize line endings to LF for consistent comparison, as raw strings use LF.
    let actual_content_normalized = actual_content.replace("\r\n", "\n");

    // Trim leading/trailing newlines and other whitespace from both for comparison.
    // The .trim() on raw string literals already does most of this.
    let expected_content_trimmed = expected_content_raw.trim().to_string();
    let actual_content_trimmed = actual_content_normalized.trim().to_string();

    assert_eq!(
        actual_content_trimmed,
        expected_content_trimmed,
        "Content mismatch for file: {}\nActual:\n---\n{}\n---\nExpected:\n---\n{}\n---",
        path.display(),
        actual_content_trimmed,
        expected_content_trimmed
    );
    Ok(())
}

// --- NEW CLEAN TESTS ---

#[test]
fn test_clean_no_comments_skips_file() -> Result<()> {
    let env = setup_clean_test_files()?;
    run_clean_command_on_dir(&env.root)?;
    let expected_content = r#"
fn func() {
    let x = 1;
    return x;
}
"#.trim();
    assert_file_content(&env.file_no_comments, expected_content)?;
    Ok(())
}

#[test]
fn test_clean_removes_full_and_inline_comments_python() -> Result<()> {
    let env = setup_clean_test_files()?;
    run_clean_command_on_dir(&env.root)?;
    let expected_content = r#"
# Path: clean_test_root/my_python.py
import os
def func():
    x = 10
    print("hello")
class MyClass:
    pass
"#.trim();
    assert_file_content(&env.python_file, expected_content)?;
    Ok(())
}

#[test]
fn test_clean_removes_full_line_and_block_comments_rust() -> Result<()> {
    let env = setup_clean_test_files()?;
    
    // Write original content to the test file.
    fs::write(&env.rust_file, &env.original_my_rust_content)?;

    // Run the actual clean command on the directory, which will find and clean env.rust_file
    run_clean_command_on_dir(&env.root)?; 

    // Corrected expected output: inline // comment removed
    let expected_content = r#"
// Path: clean_test_root/my_rust.rs
fn main() {
    let x = 10;
    println!("Hello, world!");
}
"#.trim();
    assert_file_content(&env.rust_file, expected_content)?; 
    Ok(())
}

#[test]
fn test_clean_removes_block_comments_css() -> Result<()> {
    let env = setup_clean_test_files()?;
    run_clean_command_on_dir(&env.root)?;
    let expected_content = r#"
/* Path: clean_test_root/my_style.css */
body {
    margin: 0;
    padding: 0;
}
"#.trim();
    assert_file_content(&env.css_file, expected_content)?;
    Ok(())
}

#[test]
fn test_clean_removes_html_comments() -> Result<()> {
    let env = setup_clean_test_files()?;
    run_clean_command_on_dir(&env.root)?;
    let expected_content = r#"
<!-- Path: clean_test_root/my_page.html -->
<!DOCTYPE html>
<html>
<body>
    <p>Some code here </p>
    <div>Another element</div>
    <span>Final span</span>
</body>
</html>
"#.trim();
    assert_file_content(&env.html_file, expected_content)?;
    Ok(())
}

#[test]
fn test_clean_preserves_only_header() -> Result<()> {
    let env = setup_clean_test_files()?;
    run_clean_command_on_dir(&env.root)?;
    let expected_content = r#"
# Path: clean_test_root/only_header.py
"#.trim();
    assert_file_content(&env.file_with_header_only, expected_content)?;
    Ok(())
}

#[test]
fn test_clean_complex_rust_file() -> Result<()> {
    let env = setup_clean_test_files()?;
    run_clean_command_on_dir(&env.root)?;

    let expected_content = r#"
// Path: clean_test_root/complex.rs
fn do_stuff() {
    let mut s = "foo";
    s = "bar";
    let url = "http://example.com/foo.rs?param=value";
    let x = 10;
}
"#.trim();
    assert_file_content(&env.complex_rust_file, expected_content)?;
    Ok(())
}

#[test]
fn test_clean_complex_python_file() -> Result<()> {
    let env = setup_clean_test_files()?;
    run_clean_command_on_dir(&env.root)?;

    // Python docstrings (triple quotes) are treated as code and preserved.
    // All other # comments, including inline and full-line, should be removed.
    let expected_content = r#"
# Path: clean_test_root/complex_python.py
def process_data():
    """
    This is a multi-line docstring and should be preserved as code.
    It can contain # hash symbols within it.
    """
    data = {"key": "value"}
    if "key" in data:
        print(f"Data has key: {data['key']}")
    url = "https://api.example.com/#anchor";
    '''This is a single line docstring, also preserved.'''
"#.trim();
    assert_file_content(&env.complex_python_file, expected_content)?;
    Ok(())
}

#[test]
fn test_clean_preserves_comment_markers_in_strings_python() -> Result<()> {
    let env = setup_clean_test_files()?;
    run_clean_command_on_dir(&env.root)?;

    // THE FIX IS HERE:
    // The expected content now uses double quotes "" instead of triple quotes ''',
    // which matches what the program correctly produces.
    let expected_content = r#"
# Path: clean_test_root/string_python.py
my_string = "This is a string with a # hash inside."
another_string = 'Another string with // slashes.'
comment_start_literal = '''# Not a comment, it's a string literal.'''
code_with_hash = "some_value"
final_line = "value/#here_in_string"
"#.trim();
    assert_file_content(&env.python_file_with_strings, expected_content)?;
    Ok(())
}

#[test]
fn test_clean_preserves_comment_markers_in_strings_rust() -> Result<()> {
    let env = setup_clean_test_files()?;
    run_clean_command_on_dir(&env.root)?;

    let expected_content = r##"
// Path: clean_test_root/string_rust.rs
fn process() {
    let my_str = "This string contains // slashes.";
    let another_str = "A string with \"quoted\" text and // more slashes.";
    let third_str = r#"Raw string // with comments"#;
    let x = 10;
}
"##.trim();
    assert_file_content(&env.rust_file_with_strings, expected_content)?;
    Ok(())
}