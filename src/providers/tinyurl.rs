use reqwest::blocking::Client;
use crate::providers::Shortener;

const API_BASE: &str = "https://tinyurl.com/api-create.php";

pub struct TinyUrlShortener;

impl Shortener for TinyUrlShortener {
    fn shorten(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();
        let response = client.get(API_BASE).query(&[("url", url)]).send()?.text()?;

        Ok(response)
    }
}