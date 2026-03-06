use clap::Parser;

/// A simple CLI tool to shorten URLs.
#[derive(Parser)]
#[command(name = "shurl")]
#[command(long_about = "Shortens a URL using the is.gd or Tinyurl. Supports http and https URLs only.")]

pub struct Cli {
    /// The URL to shorten.
    pub url: String,

    /// Replace the domain with an embed friendly alternative (skips shortening, to allow embed  to still function)
    #[arg(short, long)]
    pub embed: bool,

    /// Strip known tracking parameters from the URL before shortening
    #[arg(short, long)]
    pub clean: bool,

    /// Do no copy the result to the clipboard
    #[arg(long)]
    pub no_copy: bool,

    /// The shortening provider to use
    #[arg(short, long, default_value = "isgd")]
    pub provider: Provider,

}

#[derive(clap::ValueEnum, Clone)]
pub enum Provider {
    Isgd,
    Tinyurl,
}
