mod cli;
mod embedder;
mod shortener;
mod validator;

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

    let url = if args.embed {
        match embedder::embed_url(&url) {
            Ok(embedded) => {
                println!("Embed URL: {}", embedded);
                embedded
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
    else {
        url
    };

    if args.embed {
        return;
    }

    match shortener::shorten(&url) {
        Ok(short) => println!("Shortened URL: {}", short),
        Err(e) => eprintln!("Error: {}", e),
    }
}

