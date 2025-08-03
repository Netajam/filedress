# filedress

![Crates.io](https://img.shields.io/crates/v/filedress.svg?style=flat-square)
![Build Status](https://img.shields.io/github/actions/workflow/status/your-username/filedress/rust.yml?branch=main&style=flat-square)

A fast, cross-platform command-line tool written in Rust to manage file headers and comments in your source code.

## Why use `filedress`?

In large projects, especially those using modern frameworks (like SvelteKit, Next.js, etc.), you often end up with many files having the same name (e.g., `+page.svelte`, `index.js`, `__init__.py`). When you have several of these files open, it can be hard to know which one you're looking at.

`filedress` solves this by adding a simple, machine-readable path comment to the top of each file, giving you immediate context.

```typescript
// Path: src/routes/dashboard/settings/profile.ts
import { ... }
```

## Key Features

- **Add/Remove Path Headers**: Add or remove a special `Path:` header to files.
- **Clean Comments**: Strip all other comments from files while preserving the special path header.
- **Cross-Platform**: A single, compiled binary that runs on Windows, macOS, and Linux.
- **Extremely Fast**: Built in Rust for maximum performance, even on large codebases.
- **Smart Path Control**: Finely control the generated path with the `--up` (`-u`) flag.
- **Project Presets**: Use `--project` for common tech stacks (`rust`, `web`, `python`, etc.).
- **Configurable Search**: Limit search depth with the `--depth` (`-d`) flag.

## Installation

### From Crates.io (Recommended)

Once the project is published to crates.io, you can install it easily with `cargo`:

```sh
cargo install filedress
```

### From Source

If you want to build it from the source code:

1.  **Clone the repository:**
    ```sh
    git clone https://github.com/your-username/filedress.git
    cd filedress
    ```
2.  **Build the release binary:**
    ```sh
    cargo build --release
    ```
3.  The executable will be located at `target/release/filedress`. You can move this binary to a directory in your system's `PATH` to make it available globally.

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
| `--help`            | `-h`  | Show the help message.                                                    |
| `--version`         | `-V`  | Show the version of the tool.                                             |

### Examples

#### 1. Add Headers Using a Project Preset

This is the easiest way to get started. It will process all files matching the "web" preset (`.ts`, `.js`, `.svelte`, etc.).

```sh
filedress add ./my-web-project --project web
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

#### 3. Limiting Search Depth with `--depth`

To process files *only* in the target directory and not its subdirectories:

```sh
# This will process 'project/config.py' but not 'project/db/models.py'
filedress add ./project -d 1 --project python
```

#### 4. Removing All Headers

To clean up a project and remove all `Path:` headers:

```sh
filedress remove ./my-project --project web
```

## Available Project Presets

The `--project` flag uses one of the following presets:

| Preset Name | Included Extensions                                    |
| :---------- | :----------------------------------------------------- |
| `web`       | `ts`, `js`, `jsx`, `tsx`, `svelte`, `vue`, `html`, `css`, `scss` |
| `rust`      | `rs`                                                   |
| `python`    | `py`                                                   |
| `java`      | `java`, `xml`                                          |
| `flutter`   | `dart`                                                 |

## Contributing

Contributions are welcome! Feel free to open an issue for bug reports or feature requests, or submit a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
