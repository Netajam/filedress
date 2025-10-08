# Developer Guide for `filedress`

Welcome! Thank you for your interest in contributing to `filedress`. This guide will help you set up your development environment, understand the project structure, and follow our contribution workflow.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
  - [Building and Running](#building-and-running)
  - [Checking Code Quality](#checking-code-quality)
- [Running Tests](#running-tests)
- [Project Structure](#project-structure)
- [Submitting Changes (Pull Request Process)](#submitting-changes-pull-request-process)

## Prerequisites

Before you begin, make sure you have the following software installed on your system.

1.  **Rust Toolchain:** We use `rustup` to manage the Rust compiler and toolchain. If you don't have it, install it from [rustup.rs](https://rust-lang.org/tools/install/).

    The installation script will also install `cargo`, the Rust package manager and build tool.

2.  **Git:** A version control system for cloning the repository and managing changes.

3.  **A Code Editor:** We highly recommend using an editor with Rust support. [Visual Studio Code](https://code.visualstudio.com/) with the [**rust-analyzer**](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension provides an excellent development experience with features like autocompletion, go-to-definition, and inline error checking.

## Getting Started

1.  **Fork the Repository**
    First, create a fork of the `filedress` repository on GitHub to your own account.

2.  **Clone Your Fork**
    Next, clone your forked repository to your local machine. Replace `<your-username>` with your GitHub username.
    ```sh
    git clone https://github.com/<your-username>/filedress.git
    cd filedress
    ```

3.  **Build the Project**
    The first time you build the project, `cargo` will download all the necessary dependencies (crates) listed in `Cargo.toml` and then compile the source code.
    ```sh
    cargo build
    ```
    This command compiles a *debug* version of the application. The resulting binary will be located at `target/debug/filedress` ().

## Development Workflow

This section covers the typical day-to-day commands you will use while developing.

### Building and Running

Instead of manually building and then running the executable, you can use `cargo run` to do both in one step. This is the most common way to test your changes during development.

To pass arguments to `filedress`, add them after a `--` separator.

**Examples:**

```sh
# Run the 'add' command on the current directory for Rust files
cargo run -- add . --project rust

# Run the 'copy' command on a 'src' directory for TypeScript files, showing 2 levels of parent path
cargo run -- copy ./src --exts ts -u 2

# Check the version of your local build
cargo run -- --version
```

To build an optimized *release* version (which is slower to compile but faster to run), use the `--release` flag:

```sh
cargo build --release
```
The release binary will be at `target/release/filedress`.

### Checking Code Quality

To maintain code quality and consistency, we use the standard Rust formatting and linting tools.

1.  **Formatting (`rustfmt`)**
    Before committing your code, please format it using `rustfmt`.
    ```sh
    cargo fmt
    ```
    This command will automatically reformat all files in the project according to the standard Rust style guidelines.

2.  **Linting (`clippy`)**
    `clippy` is a powerful linter that catches common mistakes and suggests improvements to your code. It's a great way to learn idiomatic Rust.
    ```sh
    cargo clippy
    ```
    Address any warnings that `clippy` reports before submitting your changes.

## Running Tests

We have a suite of integration tests to ensure that the application's core functionality works as expected. These tests create temporary files and directories to simulate real-world usage.

To run all tests, simply execute:
```sh
cargo test
```
Make sure all tests pass before submitting a pull request. If you add new functionality, please also add corresponding tests in the `tests/` directory.

## Project Structure

Understanding the project layout will help you find the code you need to modify.

```
.
├── Cargo.toml        # Project manifest, dependencies, and metadata.
├── src/
│   ├── main.rs       # The main entry point of the application.
│   ├── cli.rs        # Defines the command-line interface structure using `clap`.
│   ├── commands/     # Core logic for each subcommand.
│   │   ├── mod.rs    # Dispatches to the correct subcommand handler.
│   │   ├── add.rs    # Logic for the 'add' command.
│   │   ├── copy.rs   # Logic for the 'copy' command.
│   │   └── ...       # Other command modules.
│   ├── file_utils.rs # Helper functions related to file types and comment styles.
│   └── updater.rs    # Logic for the non-blocking update checker.
├── tests/
│   └── integration_tests.rs # Integration tests for the CLI commands.
└── DEV.md            # This developer guide.
```

## Submitting Changes (Pull Request Process)

1.  **Create a Branch:** Create a new branch from `main` for your feature or bugfix.
    ```sh
    git checkout -b my-awesome-feature
    ```

2.  **Make Changes:** Write your code and add tests for any new functionality.

3.  **Check Your Work:** Ensure your code is formatted, passes clippy checks, and all tests succeed.
    ```sh
    cargo fmt
    cargo clippy
    cargo test
    ```

4.  **Commit Your Changes:** Write a clear and concise commit message.
    ```sh
    git add .
    git commit -m "feat: Add support for a new file type"
    ```

5.  **Push to Your Fork:**
    ```sh
    git push origin my-awesome-feature
    ```

6.  **Open a Pull Request:** Go to the original `filedress` repository on GitHub. You will see a prompt to create a pull request from your new branch. Fill out the PR template with a description of your changes.

Thank you for contributing