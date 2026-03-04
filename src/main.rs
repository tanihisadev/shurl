mod cli;
mod shortener;
mod validator;
mod stripper;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    let url = match validator::validate_url(&args.url) {
        Ok(valid) => valid,
        Err(e) => {
            eprintln!("Invalid URL: {}", e);
            std::process::exit(1);
        }
    };

    let url = if args.clean {
        match stripper::strip_tracking(&url) {
            Ok(cleaned) => {
                println!("Cleaned URL: {}", cleaned);
                cleaned
            }
            Err(e) => {
                eprintln!("Error cleaning URL: {}", e);
                std::process::exit(1);
            }
        }

    }
    else {
        url
    };

    match shortener::shorten(&url) {
        Ok(short) => println!("Shortened URL: {}", short),
        Err(e) => eprintln!("Error: {}", e),
    }
}

