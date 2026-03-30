use colored::Colorize;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{Config, build_regex};

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

    if path.is_dir() {
        search_dir(path, &re, &mut matching_contents)?;
    } else {
        file_contents = fs::read_to_string(path)?;
        collect_matches(&file_contents, &re, &mut matching_contents, path);
    }

    if matching_contents.is_empty() {
        println!("No matching pattern in file!");
        return Ok(());
    }

    for content in matching_contents {
        let (path, idx, line) = content;
        let line_number = idx + 1;
        let formatted_line = format_line(&line, &re);

        println!("{}:{}: {}", path.display(), line_number, formatted_line);
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
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            search_dir(&path, &re, matching_contents)?;
        } else {
            let file_contents = fs::read_to_string(&path)?;
            collect_matches(&file_contents, &re, matching_contents, &path);
        }
    }

    Ok(())
}
