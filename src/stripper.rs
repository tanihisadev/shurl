use url::Url;

// Known tracking parameter to remove
const TRACKING_PARAMS: &[&str] = &[
    // Instagram
    "igsh", "igshid",
    // YouTube
    "is", "si",
    // Universal UTM params
    "utm_source", "utm_medium", "utm_campaign", "utm_term", "utm_content",
    // Meta
    "fbclid",
    // Google
    "gclid", "gclsrc",
    // Twitter
    "ref_src", "ref_url",
    // Generic 
    "ref",
];

pub fn strip_tracking(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut parsed = Url::parse(url)?;

    // Collect only the query params we wish to keep
    let clean_params: Vec<(String, String)> = parsed.query_pairs().filter(|(key, _)| !TRACKING_PARAMS.contains(&key.as_ref())).map(|(key, value)| (key.into_owned(), value.into_owned())).collect();

    // Rebuild the query string, or clear it if nothing remains
    if clean_params.is_empty() {
        parsed.set_query(None);
    } 
    else {
        parsed.set_query(Some(&clean_params.iter().map(|(k,v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&"),));
    }

    Ok(parsed.to_string())
}