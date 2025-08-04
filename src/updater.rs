// src/updater.rs

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    thread,
    time::{SystemTime},
};

const CHECK_INTERVAL_HOURS: u64 = 24;
const GITHUB_REPO: &str = "Netajam/filedress";

// The structure of our simple config file.
#[derive(Serialize, Deserialize, Debug)]
struct UpdateConfig {
    last_checked: u64, // Stored as Unix timestamp (seconds)
}

// Gets the path to our config file (~/.config/filedress/update.json on Linux)
fn get_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("filedress").join("update.json"))
}

// Reads the config file from disk.
fn read_config(path: &PathBuf) -> Result<UpdateConfig> {
    let content = fs::read_to_string(path)?;
    let config: UpdateConfig = serde_json::from_str(&content)?;
    Ok(config)
}

// Writes the current timestamp to the config file.
fn write_config(path: &PathBuf) -> Result<()> {
    let config_dir = path.parent().unwrap();
    fs::create_dir_all(config_dir)?;
    let config = UpdateConfig {
        last_checked: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs(),
    };
    let content = serde_json::to_string_pretty(&config)?;
    fs::write(path, content)?;
    Ok(())
}

/// Checks if we should perform an update check based on the last checked time.
fn should_check() -> bool {
    if let Some(path) = get_config_path() {
        if let Ok(config) = read_config(&path) {
            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            let hours_since_last_check = (now - config.last_checked) / 3600;
            return hours_since_last_check >= CHECK_INTERVAL_HOURS;
        }
    }
    // If config doesn't exist or is invalid, we should check.
    true
}

/// The main function to perform the version check in the background.
pub fn check_for_updates() {
    // If it's not time to check, just return immediately.
    if !should_check() {
        return;
    }
    
    // Spawn a new thread to do the network request so we don't block the main app.
    thread::spawn(|| {
        // We ignore errors here because the update check is non-essential.
        // If it fails, the app should continue to work without issue.
        if let Ok(Some(new_version)) = fetch_latest_version() {
            let current_version = env!("CARGO_PKG_VERSION");
            
            // Using the semver crate to correctly compare versions.
            let current = semver::Version::parse(current_version).unwrap();
            let latest = semver::Version::parse(&new_version).unwrap();

            if latest > current {
                // A new version is available! Print the message.
                print_update_message(&new_version);
            }
        }
        
        // Update the config file regardless of success to reset the timer.
        if let Some(path) = get_config_path() {
            let _ = write_config(&path);
        }
    });
}

// Fetches the latest version tag from the GitHub API.
fn fetch_latest_version() -> Result<Option<String>> {
    // We need a User-Agent header to use the GitHub API.
    let client = reqwest::blocking::Client::builder()
        .user_agent("filedress-update-checker")
        .build()?;
        
    let url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);
    let response = client.get(&url).send()?;

    if response.status().is_success() {
        let release_info: serde_json::Value = response.json()?;
        if let Some(tag_name) = release_info["tag_name"].as_str() {
            // The tag name is usually like 'v1.0.2', we want to strip the 'v'.
            return Ok(Some(tag_name.trim_start_matches('v').to_string()));
        }
    }
    Ok(None)
}

// Prints the formatted update message with the correct install command.
fn print_update_message(new_version: &str) {
    let install_command = if cfg!(windows) {
        "iwr https://Netajam.github.io/filedress/install.ps1 -useb | iex"
    } else {
        "curl -sSfL https://Netajam.github.io/filedress/install.sh | sh"
    };

    let message = format!(
        "\n✨ A new version of filedress (v{}) is available!\n   To update, run: {}\n",
        new_version,
        install_command
    );
    
    // Using eprintln! prints to stderr, so it doesn't interfere with stdout
    // if the user is piping the command's output.
    eprintln!("\x1b[1;32m{}\x1b[0m", message);
}