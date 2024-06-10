use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn main() -> io::Result<()> {
    let directory = "/home/bypablo/projects/destacame/easy-pay";
    let output_file = "output.txt";
    let excluded_files = vec!["__init__.py","package-lock.json", "django.mo"];
    let excluded_extensions = vec!["exe", "bin"];
    let excluded_folders = vec!["__pycache__",".venv","node_modules"];

    let mut output = File::create(output_file)?;

    // Write the tree structure
    let tree_structure = generate_tree_structure(directory, &excluded_folders, &excluded_files, &excluded_extensions);
    writeln!(output, "{}", tree_structure)?;

    // Write the file contents
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_entry(|e| !is_excluded(e.path(), &excluded_folders, &excluded_files, &excluded_extensions))
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            write_file_content(&mut output, path)?;
        }
    }

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

fn write_file_content(output: &mut File, path: &Path) -> io::Result<()> {
    writeln!(output, "Path: {}", path.display())?;
    writeln!(output, "Content:")?;
    writeln!(output, "```")?;
    let content = fs::read_to_string(path)?;
    writeln!(output, "{}", content)?;
    writeln!(output, "```")?;
    Ok(())
}
