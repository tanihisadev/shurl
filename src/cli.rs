use clap::Parser;

/// A simple CLI tool to shorten URLs.
#[derive(Parser)]
#[command(name = "shurl")]
#[command(long_about = "Shortens a URL using the is.gd service. Supports http and https URLs only.")]

pub struct Cli {
    /// The URL to shorten.
    pub url: String,

    /// Replace the domain with an embed friendly alternative (skips shortening, to allow embed  to still function)
    #[arg(short, long)]
    pub embed: bool,
}

