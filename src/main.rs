use clap::Parser;
use rg_lite::{Config, run};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Pattern to search for")]
    pattern: String,

    #[arg(short, long, help = "File to search")]
    file_path: String,

    #[arg(short, long, help = "Case insensitive search")]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();

    let config = Config {
        pattern: args.pattern,
        file_path: args.file_path,
        ignore_case: args.ignore_case,
    };

    if let Err(e) = run(&config) {
        eprintln!("Application Error: {}", e);
        std::process::exit(1);
    }
}
