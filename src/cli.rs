use clap::Parser;

/// A simple CLI tool to shorten URLs.
#[derive(Parser)]
#[command(name = "shurl")]
#[command(long_about = "Shortens a URL using the is.gd service. Supports http and https URLs only.")]

pub struct Cli {
    /// The URL to shorten.
    pub url: String,

    /// Do no copy the result to the clipboard
    #[arg(long)]
    pub no_copy: bool,
}
