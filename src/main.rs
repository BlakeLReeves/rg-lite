use rg_lite::{parse_config, run};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: rg-lite <pattern> <file_path>");
        std::process::exit(1);
    }

    let config = parse_config(&args);

    if let Err(e) = run(&config) {
        eprintln!("Application Error: {}", e);
        std::process::exit(1);
    }
}
