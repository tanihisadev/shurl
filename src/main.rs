mod cli;
mod clipboard;
mod embedder;
mod shortener;
mod stripper;
mod validator;

use clap::Parser;
use cli::Cli;

fn output(label: &str, url: &str, no_copy: bool) {
    println!("{}: {}", label, url);
    if !no_copy {
        match clipboard::copy_to_clipboard(url) {
            Ok(_) => println!("Copied to clipboard!"),
            Err(e) => eprintln!("{}", e),
        }
    }
}

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
    } else {
        url
    };

    let url = if args.embed {
        match embedder::embed_url(&url) {
            Ok(embedded) => embedded,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        url
    };

    if args.embed {
        output("Embed URL copied to clipboard", &url, args.no_copy);
        return;
    }

    match shortener::shorten(&url) {
        Ok(short) => output("Shortened URL", &short, args.no_copy),
        Err(e) => eprintln!("Error: {}", e),
    }
}