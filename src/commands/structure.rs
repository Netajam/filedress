// src/commands/structure.rs

use anyhow::{Context, Result};
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use crate::cli::StructureArgs;

/// Represents a file or directory in the structure tree.
#[derive(Debug)]
struct Node {
    name: String,
    level: usize,
    relative_path: PathBuf,
    children: Vec<Node>,
}

impl Node {
    fn new_root() -> Self {
        Node {
            name: "ROOT".to_string(),
            level: usize::MAX,
            relative_path: PathBuf::new(),
            children: Vec::new(),
        }
    }
}

/// Parses a single line from the input file into its indentation level and name.
fn parse_line(line: &str, indent_width: u32) -> Option<(usize, String)> {
    if line.trim().is_empty() {
        return None;
    }
    let indent_chars = line
        .chars()
        .take_while(|c| c.is_whitespace() || "│├└─".contains(*c))
        .count();
    let level = (indent_chars as u32 / indent_width) as usize;
    let name = line[indent_chars..].trim_start().to_string();
    Some((level, name))
}

/// Builds a tree of Node objects from the input lines.
fn build_tree(lines: Vec<String>, indent_width: u32) -> Node {
    let mut root = Node::new_root();
    let mut stack: Vec<*mut Node> = vec![&mut root];

    for line in lines {
        if let Some((level, name)) = parse_line(&line, indent_width) {
            unsafe {
                // --- THIS IS THE FIX ---
                // Add `stack.len() > 1` to ensure we never pop the root element.
                while stack.len() > 1 && (**stack.last().unwrap()).level >= level {
                    stack.pop();
                }
                // -----------------------

                let parent = &mut **stack.last().unwrap();
                let relative_path = parent.relative_path.join(&name);
                
                let new_node = Node {
                    name,
                    level,
                    relative_path,
                    children: Vec::new(),
                };
                
                parent.children.push(new_node);
                stack.push(parent.children.last_mut().unwrap() as *mut Node);
            }
        }
    }
    root
}

/// Recursively traverses the Node tree and creates the file/folder structure on disk.
fn create_structure_from_tree(node: &Node, base_path: &Path) -> Result<()> {
    for child in &node.children {
        let full_path = base_path.join(&child.relative_path);
        let is_dir = child.name.ends_with('/') || !child.children.is_empty();

        if is_dir {
            fs::create_dir_all(&full_path)
                .with_context(|| format!("Failed to create directory: {:?}", full_path))?;
            println!("[CREATED DIR]  {}", full_path.display());
        } else {
            if let Some(parent_dir) = full_path.parent() {
                fs::create_dir_all(parent_dir)
                    .with_context(|| format!("Failed to create parent directory for file: {:?}", full_path))?;
            }
            fs::File::create(&full_path)
                .with_context(|| format!("Failed to create file: {:?}", full_path))?;
            println!("[CREATED FILE] {}", full_path.display());
        }

        if !child.children.is_empty() {
            create_structure_from_tree(child, base_path)?;
        }
    }
    Ok(())
}

/// Handles the 'structure' subcommand logic.
pub fn structure(args: &StructureArgs) -> Result<()> {
    let lines: Vec<String> = if let Some(file_path) = &args.file {
        println!("Reading structure from file: {}", file_path.display());
        let file = fs::File::open(file_path)
            .with_context(|| format!("Failed to open file: {:?}", file_path))?;
        io::BufReader::new(file).lines().collect::<Result<_, _>>()?
    } else {
        if atty::is(atty::Stream::Stdin) {
            anyhow::bail!("No input file provided and no data piped to stdin. Use -f <file> or pipe input.");
        }
        println!("Reading structure from stdin...");
        io::stdin().lock().lines().collect::<Result<_, _>>()?
    };

    let output_dir = args.directory.clone().unwrap_or_else(|| PathBuf::from("."));
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create base directory: {:?}", output_dir))?;
    let absolute_output_dir = output_dir.canonicalize()?;

    println!("Building structure in: {}", absolute_output_dir.display());

    let tree = build_tree(lines, args.indent);
    
    create_structure_from_tree(&tree, &absolute_output_dir)?;

    println!("\n✅ Structure created successfully.");
    Ok(())
}