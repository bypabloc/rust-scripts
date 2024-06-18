use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use clap::{App, Arg};
use chrono::Local;
use x11_clipboard::Clipboard;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let matches = App::new("Tree and File Content Writer")
        .version("1.0")
        .author("Pablo Contreras")
        .about("Generates a tree structure and file contents")
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .value_name("DIRECTORY")
            .help("Sets the path to the directory")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("excluded_files")
            .long("excluded_files")
            .value_name("FILES")
            .help("Sets the files to be excluded, separated by commas")
            .takes_value(true)
            .default_value(""))
        .arg(Arg::new("excluded_extensions")
            .long("excluded_extensions")
            .value_name("EXTENSIONS")
            .help("Sets the file extensions to be excluded, separated by commas")
            .takes_value(true)
            .default_value(""))
        .arg(Arg::new("excluded_folders")
            .long("excluded_folders")
            .value_name("FOLDERS")
            .help("Sets the folders to be excluded, separated by commas")
            .takes_value(true)
            .default_value(""))
        .get_matches();

    let directory = matches.value_of("path").unwrap();
    let excluded_files: Vec<&str> = matches.value_of("excluded_files").unwrap().split(',').collect();
    let excluded_extensions: Vec<&str> = matches.value_of("excluded_extensions").unwrap().split(',').collect();
    let excluded_folders: Vec<&str> = matches.value_of("excluded_folders").unwrap().split(',').collect();

    let now = Local::now();
    let output_file = format!(".output-{}.txt", now.format("%Y-%m-%d-%H-%M-%S"));

    let mut output = File::create(&output_file)?;
    let mut clipboard_content = String::new();

    // Write the tree structure
    let tree_structure = generate_tree_structure(directory, &excluded_folders, &excluded_files, &excluded_extensions);
    writeln!(output, "{}", tree_structure)?;
    clipboard_content.push_str(&tree_structure);
    clipboard_content.push('\n');

    // Write the file contents
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_entry(|e| !is_excluded(e.path(), &excluded_folders, &excluded_files, &excluded_extensions))
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            let content = write_file_content(&mut output, path, directory)?;
            clipboard_content.push_str(&content);
            clipboard_content.push('\n');
        }
    }

    println!("Output written to {}", output_file);

    // Copy content to clipboard
    let clipboard = Clipboard::new().expect("Failed to create clipboard context");
    clipboard.store(
        clipboard.setter.atoms.clipboard,
        clipboard.setter.atoms.utf8_string,
        clipboard_content.as_bytes(),
    ).expect("Failed to store content to clipboard");

    println!("Content copied to clipboard.");

    // Keep the program running to ensure the clipboard content remains available
    thread::sleep(Duration::from_secs(1));

    Ok(())
}

fn generate_tree_structure(directory: &str, excluded_folders: &[&str], excluded_files: &[&str], excluded_extensions: &[&str]) -> String {
    let mut tree = String::new();
    let mut prefix = String::new();
    build_tree(Path::new(directory), &mut tree, &mut prefix, true, excluded_folders, excluded_files, excluded_extensions);
    tree
}

fn build_tree(path: &Path, tree: &mut String, prefix: &mut String, is_last: bool, excluded_folders: &[&str], excluded_files: &[&str], excluded_extensions: &[&str]) {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        if !is_excluded(path, excluded_folders, excluded_files, excluded_extensions) {
            tree.push_str(&prefix);
            if is_last {
                tree.push_str("└── ");
                prefix.push_str("    ");
            } else {
                tree.push_str("├── ");
                prefix.push_str("│   ");
            }

            if path.is_dir() {
                tree.push_str(&format!("📁{}\n", file_name));
                let entries: Vec<PathBuf> = fs::read_dir(path)
                    .unwrap()
                    .map(|res| res.unwrap().path())
                    .filter(|e| !is_excluded(e, excluded_folders, excluded_files, excluded_extensions))
                    .collect();
                for (i, entry) in entries.iter().enumerate() {
                    build_tree(entry, tree, prefix, i == entries.len() - 1, excluded_folders, excluded_files, excluded_extensions);
                }
            } else {
                tree.push_str(&format!("{}\n", file_name));
            }

            prefix.pop();
            prefix.pop();
            prefix.pop();
            prefix.pop();
        }
    }
}

fn is_excluded(path: &Path, excluded_folders: &[&str], excluded_files: &[&str], excluded_extensions: &[&str]) -> bool {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or_default();

    if excluded_folders.iter().any(|&folder| path.to_str().unwrap_or_default().contains(folder)) {
        return true;
    }

    if excluded_files.contains(&file_name) {
        return true;
    }

    if excluded_extensions.contains(&extension) {
        return true;
    }

    false
}

fn write_file_content(output: &mut File, path: &Path, directory: &str) -> io::Result<String> {
    let root = Path::new(directory).file_name().unwrap_or_default();
    let relative_path = path.strip_prefix(directory).unwrap_or(path);
    let short_path = format!("{}/{}", root.to_str().unwrap_or_default(), relative_path.display());

    let mut content = String::new();
    content.push_str(&format!("Path: {}\n", short_path));
    content.push_str("Content:\n");
    content.push_str("```\n");
    let file_content = fs::read_to_string(path)?;
    content.push_str(&file_content);
    content.push_str("\n```\n");

    writeln!(output, "{}", content)?;

    Ok(content)
}
