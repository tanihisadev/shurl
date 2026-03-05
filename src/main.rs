mod cli;
mod clipboard;
mod shortener;
mod validator;

use clap::Parser;
use cli::Cli;

fn output(label: &str, url: &str, no_copy: bool) {
    println!("{}: {}", label, url);
    if !no_copy {
        match clipboard::copy_to_clipboard(url) {
            Ok(_) => println!("Copied to clipboard!"),
            Err(e) => println!("{}", e),
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

    match shortener::shorten(&url) {
        Ok(short) => output("Shortened URL:", &short, args.no_copy),
        Err(e) => eprintln!("Error: {}", e),
    }
}

