use url::Url;

const EMBED_DOMAINS: &[(&str, &str)] = &[
    ("instagram.com", "kkinstagram.com"),
    ("x.com", "fxtwitter.com"),
    ("twitter.com", "fxtwitter.com"),
    ("reddit.com", "rxddit.com"),
];

#[derive(Debug)]
pub enum EmbedError {
    ParseError(String),
    UnsupportedDomain(String),
}

impl std::fmt::Display for EmbedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmbedError::ParseError(msg) => write!(f, "Failed to parse URL: {}", msg),
            EmbedError::UnsupportedDomain(domain) => {
                write!(f, "No embed alternative known for '{}'", domain)
            }
        }
    }
}

pub fn embed_url(url: &str) -> Result<String, EmbedError> {
    let mut parsed = Url::parse(url).map_err(|e| EmbedError::ParseError(e.to_string()))?;

    let host = parsed.host_str().unwrap_or("").trim_start_matches("www.").to_string();

    let replacement = EMBED_DOMAINS.iter().find(|(original, _)| *original == host).map(|(_, replacement)| *replacement).ok_or_else(|| EmbedError::UnsupportedDomain(host.clone()))?;

    parsed.set_host(Some(replacement)).map_err(|e| EmbedError::ParseError(e.to_string()))?;

    Ok(parsed.to_string())
}

