// tests/integration_tests.rs

// --- FIX: Cleaned up unused imports ---
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
// --------------------------------------
use tempfile::{tempdir, TempDir};

use filedress::cli::{Args, Commands};
use filedress::commands::handle_command;

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
    let project_root = temp_dir.path().join("project_root");
    let src_dir = project_root.join("src/api/v1");
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

#[test]
fn test_add_and_remove_header_simple() -> Result<()> {
    // ARRANGE
    let env = setup_test_environment()?;
    let add_args = Args {
        directory: env.project_root.clone(),
        exts: Some(vec!["py".to_string()]),
        ..Default::default()
    };
    
    // ACT (ADD)
    handle_command(&Commands::Add(add_args))?;

    // ASSERT (ADD)
    let content = fs::read_to_string(&env.config_file)?;
    assert!(content.starts_with("# Path: project_root/config.py"));

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
        directory: env.v1_dir.clone(), // Target the deepest directory
        exts: Some(vec!["py".to_string()]),
        up: 2, // We want to include 'src/api' in the path
        ..Default::default()
    };

    // ACT
    handle_command(&Commands::Add(add_args))?;

    // ASSERT
    let content = fs::read_to_string(&env.user_file)?;
    let expected_header = "# Path: src/api/v1/user.py";
    let first_line = content.lines().next().unwrap_or("");

    assert_eq!(
        first_line,
        expected_header,
        "\nHeader mismatch for --up test!\n  Expected: '{}'\n  Got:      '{}'",
        expected_header,
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
        ..Default::default()
    };

    // ACT
    handle_command(&Commands::Add(add_args))?;

    // ASSERT
    // 1. The shallow file SHOULD have a header.
    let config_content = fs::read_to_string(&env.config_file)?;
    assert!(config_content.starts_with("# Path: project_root/config.py"));

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
        ..Default::default()
    };

    // ACT
    handle_command(&Commands::Add(add_args))?;

    // ASSERT
    // Both files should now have headers.
    let config_content = fs::read_to_string(&env.config_file)?;
    assert!(config_content.starts_with("# Path: project_root/config.py"));

    let user_content = fs::read_to_string(&env.user_file)?;
    assert!(user_content.starts_with("# Path: project_root/src/api/v1/user.py"));

    Ok(())
}

// --- FIX: The `impl Default` block has been REMOVED from this file ---