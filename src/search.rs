use colored::Colorize;
use regex::Regex;
use std::fs;

use crate::Config;

pub fn run(
    Config {
        file_path,
        pattern,
        ignore_case,
    }: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string(file_path)?;

    let matching_contents = search_contents(pattern, &file_contents, ignore_case);

    if matching_contents.is_empty() {
        println!("No matching pattern in file!");
        return Ok(());
    }

    for content in matching_contents {
        let (idx, line) = content;
        let line_number = idx + 1;
        let formatted_line = format_line(line, pattern, ignore_case);

        println!("{}: {}", line_number, formatted_line);
    }

    Ok(())
}

fn search_contents<'a>(
    pattern: &str,
    contents: &'a str,
    ignore_case: &bool,
) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            if *ignore_case {
                line.to_lowercase().contains(&pattern.to_lowercase())
            } else {
                line.contains(pattern)
            }
        })
        .collect()
}

fn format_line(line: &str, pattern: &str, ignore_case: &bool) -> String {
    let regex_pattern = if *ignore_case {
        format!("(?i){}", regex::escape(pattern))
    } else {
        regex::escape(pattern)
    };

    let re = Regex::new(&regex_pattern).unwrap();
    re.replace_all(line, |caps: &regex::Captures| {
        caps[0].bright_yellow().bold().to_string()
    })
    .to_string()
}
