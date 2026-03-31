use colored::Colorize;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

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
    let mut matching_contents: Vec<(PathBuf, usize, String)> = Vec::new();
    let file_contents;
    let ignore_file_path = Path::new(IGNORE_FILE_PATH);
    let ignore_file_contents = fs::read_to_string(&ignore_file_path)?;
    let ignore_files: Vec<&str> = ignore_file_contents.lines().collect();

    if should_ignore(path, &ignore_files) {
        println!("Input path is ignored: {}", path.display());
        return Ok(());
    }

    if path.is_dir() {
        search_dir(path, &re, &mut matching_contents, &ignore_files)?;
    } else {
        file_contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(_) => return Ok(()),
        };
        collect_matches(&file_contents, &re, &mut matching_contents, path);
    }

    if matching_contents.is_empty() {
        println!("No matching pattern in file!");
        return Ok(());
    }

    for content in matching_contents {
        let (path, idx, line) = content;
        let line_number = idx + 1;
        let formatted_line_number = line_number.to_string().red().bold();
        let formatted_line = format_line(&line, &re);
        let formatted_path = path.display().to_string().green().bold();

        println!(
            "{}:{}: {}",
            formatted_path, formatted_line_number, formatted_line
        );
    }

    Ok(())
}

fn collect_matches(
    contents: &str,
    re: &Regex,
    matching_contents: &mut Vec<(PathBuf, usize, String)>,
    path: &Path,
) {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| re.is_match(line))
        .for_each(|(idx, line)| {
            matching_contents.push((path.to_path_buf(), idx, line.to_string()))
        });
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
    matching_contents: &mut Vec<(PathBuf, usize, String)>,
    ignore_files: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if should_ignore(&path, ignore_files) {
            println!("Skipping: {}", path.display());
            continue;
        }

        if path.is_dir() {
            search_dir(&path, &re, matching_contents, ignore_files)?;
        } else {
            let Ok(file_contents) = fs::read_to_string(&path) else {
                continue;
            };
            collect_matches(&file_contents, &re, matching_contents, &path);
        }
    }

    Ok(())
}

fn should_ignore(path: &Path, ignore_files: &[&str]) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|name| ignore_files.contains(&name))
        .unwrap_or(false)
}
