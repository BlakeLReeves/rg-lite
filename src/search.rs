use std::fs;

use crate::Config;

pub fn run(Config { file_path, pattern }: &Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running rg-lite...");
    println!("pattern: {}", pattern);
    println!("file: {}", file_path);

    let file_contents = fs::read_to_string(file_path)?;

    let matching_contents = search_contents(pattern, &file_contents);

    if matching_contents.is_empty() {
        println!("No matching pattern in file!");
        return Ok(());
    }

    for content in matching_contents {
        println!("{}", content);
    }

    Ok(())
}

fn search_contents<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}
