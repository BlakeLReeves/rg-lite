use colored::Colorize;
use regex::Regex;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use crate::{Config, build_regex};

const IGNORE_FILE_PATH: &str = ".ignore";

pub fn run(
    Config {
        file_path,
        pattern,
        ignore_case,
    }: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let re = build_regex(pattern, ignore_case);
    let path = Path::new(file_path);
    let cwd = std::env::current_dir()?;

    let ignore_files: HashSet<String> = fs::read_to_string(IGNORE_FILE_PATH)
        .ok()
        .map(|c| c.lines().map(String::from).collect())
        .unwrap_or_default();

    if should_ignore(path, &ignore_files) {
        println!("Input path is ignored: {}", path.display());
        return Ok(());
    }

    search_dir(path, &re, &ignore_files, &cwd)?;

    Ok(())
}

fn search_dir(
    path: &Path,
    re: &Regex,
    ignore_files: &HashSet<String>,
    cwd: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if path.is_dir() {
        let has_searchable = fs::read_dir(path)?.filter_map(Result::ok).any(|entry| {
            let p = entry.path();
            !should_ignore(&p, ignore_files) && !is_hidden_entry(&entry)
        });

        if !has_searchable {
            println!("No matches found (directory empty or all ignored/hidden)");
            return Ok(());
        }
    }

    let walker = WalkDir::new(path).into_iter();
    let mut total_matches = 0;

    for entry in walker
        .filter_entry(|e| {
            if e.depth() == 0 {
                true
            } else {
                !is_hidden(e) && !should_ignore(e.path(), ignore_files)
            }
        })
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path();

            if is_binary(path) {
                continue;
            }

            let Ok(file_contents) = fs::read_to_string(path) else {
                continue;
            };

            let relative_path = path.strip_prefix(cwd).unwrap_or(path);
            total_matches += collect_matches(&file_contents, re, relative_path);
        }
    }

    if total_matches == 0 {
        println!("No Matches found");
    }

    Ok(())
}

fn collect_matches(contents: &str, re: &Regex, path: &Path) -> usize {
    let mut count = 0;
    let formatted_path = path.to_string_lossy().to_string().green().bold();

    for (idx, line) in contents.lines().enumerate() {
        if re.is_match(line) {
            count += 1;
            let formatted_line_number = (idx + 1).to_string().red().bold();
            let formatted_line = format_line(&line, &re);

            println!(
                "{}:{}: {}",
                formatted_path, formatted_line_number, formatted_line
            );
        }
    }

    count
}

fn is_binary(path: &Path) -> bool {
    let Ok(mut file) = File::open(path) else {
        return true;
    };

    let mut buffer = [0u8; 512];

    let Ok(n) = file.read(&mut buffer) else {
        return true;
    };

    buffer[..n].contains(&0)
}

fn format_line(line: &str, re: &Regex) -> String {
    re.replace_all(line, |caps: &regex::Captures| {
        caps[0].bright_yellow().bold().to_string()
    })
    .to_string()
}

fn should_ignore(path: &Path, ignore_files: &HashSet<String>) -> bool {
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .map(|name| ignore_files.contains(name))
            .unwrap_or(false)
    })
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_hidden_entry(entry: &fs::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
