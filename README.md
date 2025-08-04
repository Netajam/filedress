# filedress

![Build Status](https://img.shields.io/github/actions/workflow/status/Netajam/filedress/rust.yml?branch=main&style=flat-square)

A fast, cross-platform command-line tool to manage file headers, copy code for LLMs, and scaffold new project structures. Built in Rust for developers who value speed and efficiency.

## Why use `filedress`?

In large projects, especially those using modern frameworks (like SvelteKit, Next.js, etc.), you often end up with many files having the same name. `filedress` helps you manage your codebase with a suite of powerful tools.

-   **Add Context:** Add a simple, machine-readable path comment to the top of each file, so you always know which file you're editing.
-   **Feed your LLM:** Aggregate the content of multiple files into your clipboard, perfectly formatted to be pasted into large language models like GPT-4, Claude, or Gemini.
-   **Scaffold Projects:** Instantly create complex directory and file structures from a simple text-based template.

```typescript
// Path: src/routes/dashboard/settings/profile.ts
import { ... }
```

## Installation

### For Linux & macOS (in Bash or Zsh)

You can install `filedress` with a single command. This script will automatically detect your operating system, download the correct binary from the latest GitHub release, and install it to `~/.local/bin`.

```sh
curl -sSfL https://Netajam.github.io/filedress/install.sh | sh
```
> **Note:** If the `filedress` command isn't available after installation, you may need to open a new terminal or add `~/.local/bin` to your shell's `PATH` by adding `export PATH="$HOME/.local/bin:$PATH"` to your `~/.bashrc` or `~/.zshrc` file.

---

### For Windows (in PowerShell)

Open PowerShell and run the following command. This will download and install the latest `filedress.exe` to a user-specific directory and add it to your PATH.

```powershell
iwr https://Netajam.github.io/filedress/install.ps1 -useb | iex
```
> **Note:** You must open a **new** PowerShell or Command Prompt window after the installation is complete for the `filedress` command to be available.

---

### Other Installation Methods

#### From Release Binaries (Manual)

If you prefer to install manually:
1.  Go to the [**Releases page**](https://github.com/Netajam/filedress/releases).
2.  Download the appropriate `.zip` or `.tar.gz` file for your system.
3.  Unpack the archive and place the `filedress` (or `filedress.exe`) executable in a directory that is included in your system's `PATH`.

#### From Source (for developers)

If you have the Rust toolchain installed, you can build `filedress` from source:
1.  **Clone the repository:** `git clone https://github.com/Netajam/filedress.git`
2.  **Navigate into the directory:** `cd filedress`
3.  **Build the release binary:** `cargo build --release`
4.  The executable will be located at `target/release/filedress`.

## Key Features

- **Add/Remove Path Headers**: Add or remove a special `Path:` header to files for context.
- **Copy for LLMs**: Aggregate and format the content of multiple files into your clipboard, ready for pasting into AI models.
- **Scaffold Structures**: Instantly create complex file and directory layouts from a simple text template.
- **Update Notifier**: Automatically checks for new versions and lets you know when an update is available.
- **Smart Path Control**: Finely control the generated path with the `--up` (`-u`) flag.
- **Project Presets**: Use `--project` for common tech stacks (`rust`, `web`, `python`, etc.).
- **Configurable Search**: Limit search depth with the `--depth` (`-d`) flag.
- **Cross-Platform**: A single, compiled binary that runs on Windows, macOS, and Linux.

## Usage

### Commands

| Command     | Description                                                                    |
| :---------- | :----------------------------------------------------------------------------- |
| `add`       | Adds a `Path:` header to the top of files.                                     |
| `remove`    | Removes the specific `Path:` header from files.                                |
| `clean`     | Removes all other comments from files, preserving the `Path:` header.          |
| `copy`      | Copies the contents of multiple files into the clipboard for use with LLMs.   |
| `structure` | Creates a file and directory structure from a text-based template.             |

### `add` / `remove` / `clean` / `copy` Options

These commands share the same set of file discovery options.

| Option             | Alias | Description                                                               |
| :----------------- | :---- | :------------------------------------------------------------------------ |
| `<DIRECTORY>`      |       | **(Required)** The root directory to start searching from.                |
| `--project <TYPE>` |       | Use a preset group of file extensions (e.g., `rust`, `web`, `python`).   |
| `--exts <EXTS>`    |       | Provide a custom, comma-separated list of extensions (e.g., `ts,py`).     |
| `--up <LEVELS>`    | `-u`  | How many levels up from the target directory to include in the path.      |
| `--depth <LEVELS>` | `-d`  | How many levels deep to search for files from the target directory.         |
| `--force`          | `-f`  | Overwrite an existing `Path:` header during an `add` operation.           |

### `structure` Options

| Option             | Alias | Description                                                                 |
| :----------------- | :---- | :-------------------------------------------------------------------------- |
| `--file <FILE>`    | `-f`  | The input file with the tree structure. Reads from stdin if not provided. |
| `--directory <DIR>`| `-d`  | The root directory where the structure will be created. Defaults to `.`.    |
| `--indent <WIDTH>` | `-i`  | The number of spaces that represent one level of indentation.             |

---

## Examples

### Adding and Managing Headers

```sh
# Add headers to all supported files in a project
filedress add ./my-project

# Add headers with more path context (2 levels up) to only Python files
filedress add ./src/app -u 2 --project python

# Overwrite existing headers with a new path format
filedress add ./src/app -u 3 --project python -f
```

### Copying Code for an LLM

```sh
# Copy all TypeScript files from the 'src/utils' directory to the clipboard
filedress copy ./src/utils --exts ts

# The clipboard will contain:
# FILE: src/utils/api.ts
# ---
#
# // content of api.ts
#
# ---
# FILE: src/utils/helpers.ts
# ---
#
# // content of helpers.ts
```

### Scaffolding a New Project

Given a file `template.txt` with the following content:
```txt
my_app/
    src/
        main.rs
        lib.rs
    tests/
    .gitignore
    Cargo.toml
```
You can run:
```sh
# Create the structure in the current directory
filedress structure -f template.txt

# Or create it inside a 'build' folder
filedress structure -f template.txt -d ./build
```

## Contributing

Contributions are welcome! Feel free to open an issue for bug reports or feature requests, or submit a pull request.

## License

This project is licensed under the MIT License.