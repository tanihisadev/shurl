pub mod isgd;
pub mod tinyurl;

pub trait Shortener {
    fn shorten(&self, url: &str) -> Result<String, Box<dyn std::error::Error>>;
}