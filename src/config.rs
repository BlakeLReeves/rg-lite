#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub file_path: String,
}

pub fn parse_config(args: &[String]) -> Config {
    let pattern = args[1].clone();
    let file_path = args[2].clone();

    Config { pattern, file_path }
}
