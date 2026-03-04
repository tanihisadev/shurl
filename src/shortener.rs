use reqwest::blocking::Client;
use serde::Deserialize;

const API_BASE: &str = "https://is.gd/create.php";

#[derive(Deserialize)]
struct ApiResponse {
    shorturl: String,
}

pub fn shorten(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client.get(API_BASE).query(&[("format", "json"), ("url", url)]).send()?.json::<ApiResponse>()?;

    Ok(response.shorturl)
}

