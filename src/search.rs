use ansi_term::Color::Yellow;
use std::fs;

use crate::Config;

pub fn run(
    Config {
        file_path,
        pattern,
        ignore_case,
    }: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", ignore_case);
    let file_contents = fs::read_to_string(file_path)?;

    let matching_contents = search_contents(pattern, &file_contents);

    if matching_contents.is_empty() {
        println!("No matching pattern in file!");
        return Ok(());
    }

    for content in matching_contents {
        let (idx, line) = content;
        let line_number = idx + 1;
        let formatted_line = format_line(line, pattern);

        println!("{}: {}", line_number, formatted_line);
    }

    Ok(())
}

fn search_contents<'a>(pattern: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(pattern))
        .collect()
}

fn format_line(line: &str, pattern: &str) -> String {
    let highlighted = Yellow.bold().paint(pattern).to_string();
    line.replace(pattern, &highlighted)
}
