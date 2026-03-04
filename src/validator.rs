use url::Url;

#[derive(Debug)]

pub enum ValidationError {
    EmptyInput,
    InvalidFormat(String),
    UnsupportedScheme(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::EmptyInput => write!(f, "URL cannot be empty"),
            ValidationError::InvalidFormat(msg) => write!(f, "Invalid URL format: {}", msg),
            ValidationError::UnsupportedScheme(scheme) => {
                write!(f, "Unsupported scheme '{}', must be http or https", scheme)
            }
        }
    }
}

pub fn validate_url(input: &str) -> Result<String, ValidationError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    let parsed = Url::parse(trimmed).map_err(|e| ValidationError::InvalidFormat(e.to_string()))?;

    match parsed.scheme() {
        "http" | "https" => Ok(trimmed.to_string()),
        other => Err(ValidationError::UnsupportedScheme(other.to_string())),
    }
}


