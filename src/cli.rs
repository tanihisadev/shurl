use clap::Parser;

/// A simple CLI tool to shorten URLs.
#[derive(Parser)]
#[command(name = "shurl")]
#[command(long_about = "Shortens a URL using the is.gd service. Supports http and https URLs only.")]

pub struct Cli {
    /// The URL to shorten.
    pub url: String,

    /// Strip known tracking parameters from the URL before shortening
    #[arg(short, long)]
    pub clean: bool,
}