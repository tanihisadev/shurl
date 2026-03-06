use reqwest::blocking::Client;
use serde::Deserialize;
use crate::providers::Shortener;

const API_BASE: &str = "https://is.gd/create.php";

#[derive(Deserialize)]
struct ApiResponse {
    shorturl: String,
}

pub struct IsgdShortener;

impl Shortener for IsgdShortener {
    fn shorten(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();

        let response = client.get(API_BASE).query(&[("format", "json"), ("url", url)]).send()?.json::<ApiResponse>()?;

        Ok(response.shorturl)
    }
}
