use colored::Colorize;
use regex::Regex;
use std::fs;

use crate::{Config, build_regex};

pub fn run(
    Config {
        file_path,
        pattern,
        ignore_case,
    }: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string(file_path)?;
    let re = build_regex(pattern, ignore_case);

    let matching_contents = search_contents(&file_contents, &re);

    if matching_contents.is_empty() {
        println!("No matching pattern in file!");
        return Ok(());
    }

    for content in matching_contents {
        let (idx, line) = content;
        let line_number = idx + 1;
        let formatted_line = format_line(line, &re);

        println!("{}: {}", line_number, formatted_line);
    }

    Ok(())
}

fn search_contents<'a>(contents: &'a str, re: &Regex) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| re.is_match(line))
        .collect()
}

fn format_line(line: &str, re: &Regex) -> String {
    re.replace_all(line, |caps: &regex::Captures| {
        caps[0].bright_yellow().bold().to_string()
    })
    .to_string()
}
