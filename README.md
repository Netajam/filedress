# filedress

![Build Status](https://img.shields.io/github/actions/workflow/status/your-username/filedress/rust.yml?branch=main&style=flat-square)

A fast, cross-platform command-line tool written in Rust to manage file headers and comments in your source code.

## Why use `filedress`?

In large projects, especially those using modern frameworks (like SvelteKit, Next.js, etc.), you often end up with many files having the same name (e.g., `+page.svelte`, `index.js`, `__init__.py`). When you have several of these files open, it can be hard to know which one you're looking at.

`filedress` solves this by adding a simple, machine-readable path comment to the top of each file, giving you immediate context.

```typescript
// Path: src/routes/dashboard/settings/profile.ts
import { ... }
```

## Installation

### One-Liner Install (Linux & macOS)

You can install `filedress` with a single command. This script will automatically detect your operating system, download the correct binary from the latest GitHub release, and install it to `~/.local/bin`.

```sh
curl -sSfL https://your-username.github.io/filedress/install.sh | sh
```
> **Note:** If the `filedress` command isn't available after installation, you may need to open a new terminal or add `~/.local/bin` to your shell's `PATH` by adding `export PATH="$HOME/.local/bin:$PATH"` to your `~/.bashrc` or `~/.zshrc` file.

---

### Other Installation Methods

#### From Release Binaries (Windows, Linux, macOS)

If you prefer to install manually:
1.  Go to the [**Releases page**](https://github.com/your-username/filedress/releases).
2.  Download the appropriate `.zip` or `.tar.gz` file for your system.
3.  Unpack the archive and place the `filedress` (or `filedress.exe`) executable in a directory that is included in your system's `PATH`.

#### From Source (for developers)

If you have the Rust toolchain installed, you can build `filedress` from source:
1.  **Clone the repository:** `git clone https://github.com/your-username/filedress.git`
2.  **Navigate into the directory:** `cd filedress`
3.  **Build the release binary:** `cargo build --release`
4.  The executable will be located at `target/release/filedress`.

## Key Features

- **Add/Remove Path Headers**: Add or remove a special `Path:` header to files.
- **Force Overwrite**: Re-apply headers with new settings using the `--force` flag.
- **Clean Comments**: Strip all other comments from files while preserving the special path header.
- **Cross-Platform**: A single, compiled binary that runs on Windows, macOS, and Linux.
- **Extremely Fast**: Built in Rust for maximum performance, even on large codebases.
- **Smart Path Control**: Finely control the generated path with the `--up` (`-u`) flag.
- **Project Presets**: Use `--project` for common tech stacks (`rust`, `web`, `python`, etc.).
- **Configurable Search**: Limit search depth with the `--depth` (`-d`) flag.

## Usage

The basic structure of a `filedress` command is:

```sh
filedress <COMMAND> <DIRECTORY> [OPTIONS]
```

### Commands

| Command  | Description                                                         |
| :------- | :------------------------------------------------------------------ |
| `add`    | Adds a `Path:` header to the top of files.                          |
| `remove` | Removes the specific `Path:` header from files.                     |
| `clean`  | Removes all other comments from files, preserving the `Path:` header. |

### Options

| Option              | Alias | Description                                                               |
| :------------------ | :---- | :------------------------------------------------------------------------ |
| `<DIRECTORY>`       |       | **(Required)** The root directory to start searching from.                |
| `--project <TYPE>`  |       | Use a preset group of file extensions (e.g., `rust`, `web`, `python`).   |
| `--exts <EXTS>`     |       | Provide a custom, comma-separated list of extensions (e.g., `ts,py`).     |
| `--up <LEVELS>`     | `-u`  | How many levels up from the target directory to include in the path.      |
| `--depth <LEVELS>`  | `-d`  | How many levels deep to search for files from the target directory.         |
| `--force`           | `-f`  | Overwrite an existing `Path:` header during an `add` operation.           |
| `--help`            | `-h`  | Show the help message.                                                    |
| `--version`         | `-V`  | Show the version of the tool.                                             |

### Examples

#### 1. Add Headers to All Supported Files

This is the simplest use case. It will process all known file types.

```sh
filedress add ./my-project
```

#### 2. Advanced Path Control with `--up`

Imagine a file at `project/services/api/v1/user.py`.

- **Default behavior** (target directory is `v1`):
  ```sh
  filedress add ./project/services/api/v1 --project python
  ```
  Resulting path in `user.py`: `# Path: v1/user.py`

- **Including parent directories** (target `v1`, up 2 levels):
  ```sh
  filedress add ./project/services/api/v1 --project python -u 2
  ```
  Resulting path in `user.py`: `# Path: services/api/v1/user.py`

#### 3. Overwriting Headers with `--force`

If you change your mind and want a different path structure, use `-f`.

```sh
# First, add with -u 1
filedress add ./project/services/api/v1 -u 1 --project python

# Now, overwrite with -u 2
filedress add ./project/services/api/v1 -u 2 --project python -f
```

## Contributing

Contributions are welcome! Feel free to open an issue for bug reports or feature requests, or submit a pull request.

## License

This project is licensed under the MIT License.