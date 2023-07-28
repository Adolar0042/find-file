use std::env;
use std::fs;
use std::fmt::Display;
use std::path::Path;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = match args.len() {
        2 => args[1].clone(),
        _ => {
            println!("Enter the name to find:");
            read_input()
        }
    };

    find_file(&name);
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn find_file(name: &str) {
    find_in_directory(Path::new("."), name);
}

fn find_in_directory(path: &Path, name: &str) {
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name();
                        let file_name_str = file_name.to_string_lossy();

                        if file_name_str.contains(name) {
                            let formatted_name = format_name(&file_name_str, &name);

                            // Use map() to convert the temporary value into a PathBuf
                            let parent_directory = entry.path().parent().map(|p| p.to_path_buf()).unwrap_or_default();
                            let path_display = parent_directory
                                .canonicalize()
                                .map(|p| p.to_string_lossy().into_owned())
                                .unwrap_or_default();

                            print_file_link(&path_display, &formatted_name);
                        }

                        if entry.path().is_dir() {
                            find_in_directory(&entry.path(), name);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory: {:?}", e),
        }
    }
}

fn format_name(name: &str, search_term: &str) -> String {
    let replaced = name.replace(search_term, &format!("{}", search_term.on_yellow().black()));
    replaced.to_string()
}

fn print_file_link(path_display: &str, formatted_name: &str) {
    let path = Path::new(path_display);
    let mut path_str = path.to_str().unwrap_or("").to_string();
    if path_str.starts_with("\\\\?\\") {
        path_str = path_str.replace("\\\\?\\", "");
    }
    let link = format_hyperlink(&path_str, Some(formatted_name));
    println!("{} {}", "".white(), link.white());
}

fn format_hyperlink<T: Display>(uri: &str, label: Option<T>) -> String {
    if cfg!(target_os = "windows") {
        match label {
            Some(l) => format!("\x1B]8;;{}\x07{}\x1B]8;;\x07", uri, l),
            None => format!("\x1B]8;;{}\x07\x1B]8;;\x07", uri),
        }
    } else {
        // Fallback for other platforms or terminals
        match label {
            Some(l) => format!("{} ({})", l, uri),
            None => uri.to_string(),
        }
    }
}

