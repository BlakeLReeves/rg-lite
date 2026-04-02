use colored::Colorize;
use regex::Regex;
use std::fs;
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

    let ignore_files: Vec<String> = fs::read_to_string(IGNORE_FILE_PATH)
        .ok()
        .map(|c| c.lines().map(String::from).collect())
        .unwrap_or_default();

    if should_ignore(path, &ignore_files) {
        println!("Input path is ignored: {}", path.display());
        return Ok(());
    }

    search_dir(path, &re, &ignore_files)?;

    Ok(())
}

fn collect_matches(contents: &str, re: &Regex, path: &Path) -> usize {
    let matches: Vec<(usize, &str)> = contents
        .lines()
        .enumerate()
        .filter(|(_, line)| re.is_match(line))
        .collect();

    for (idx, line) in &matches {
        let line_number = idx + 1;
        let formatted_line_number = line_number.to_string().red().bold();
        let formatted_line = format_line(&line, &re);
        let formatted_path = path.display().to_string().green().bold();

        println!(
            "{}:{}: {}",
            formatted_path, formatted_line_number, formatted_line
        );
    }

    matches.len()
}

fn format_line(line: &str, re: &Regex) -> String {
    re.replace_all(line, |caps: &regex::Captures| {
        caps[0].bright_yellow().bold().to_string()
    })
    .to_string()
}

fn search_dir(
    path: &Path,
    re: &Regex,
    ignore_files: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
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
        let path = entry.path();

        if path.is_file() {
            let Ok(file_contents) = fs::read_to_string(&path) else {
                continue;
            };

            total_matches += collect_matches(&file_contents, re, path);
        }
    }

    if total_matches == 0 {
        println!("No Matches found");
    }

    Ok(())
}

fn should_ignore(path: &Path, ignore_files: &[String]) -> bool {
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .map(|name| ignore_files.iter().any(|f| f == name))
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
