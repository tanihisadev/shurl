use url::Url;

const MAX_URL_LENGTH: usize = 2048;

// TLDs that are known to be invalid or commonly mistyped
const MIN_TLD_CHAR_LENGTH: usize = 2;

#[derive(Debug)]

pub enum ValidationError {
    EmptyInput,
    InvalidFormat(String),
    UnsupportedScheme(String),
    TooLong(usize),
    MissingHost,
    InvalidTld(String),
    BareIpAddress,
    PrivateOrLocalAddress,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::EmptyInput => {
                write!(f, "URL cannot be empty")
            }

            ValidationError::InvalidFormat(msg) => {
                write!(f, "Invalid URL format: {}", msg)
            }

            ValidationError::UnsupportedScheme(scheme) => {
                write!(f, "Unsupported scheme '{}', must be http or https", scheme)
            }

            ValidationError::TooLong(len) => {
                write!(f, "URL is too long ({} characters, maximum is {})", len, MAX_URL_LENGTH)
            }

            ValidationError::MissingHost => {
                write!(f, "URL must contain a valid hostname")
            }

            ValidationError::InvalidTld(tld) => {
                write!(f, "Invalid or missing TLD: '{}'", tld)
            }

            ValidationError::BareIpAddress => {
                write!(f, "Bare IP addresses are not supported, use a domain name")
            }

            ValidationError::PrivateOrLocalAddress => {
                write!(f, "Private and local addresses cannot be shortened")
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
        "http" | "https" => {}
        other => return Err(ValidationError::UnsupportedScheme(other.to_string())),
    }

    if trimmed.len() > MAX_URL_LENGTH {
        return Err(ValidationError::TooLong(trimmed.len()));
    }

    let host = parsed.host_str().ok_or(ValidationError::MissingHost)?.trim_start_matches("www.").to_string();

    if host.is_empty() {
        return Err(ValidationError::MissingHost);
    }

    if let Ok(addr) = host.parse::<std::net::IpAddr>() {
        // Check for private/local ranges
        if is_private_or_local(&addr) {
            return Err(ValidationError::PrivateOrLocalAddress);
        }
        return Err(ValidationError::BareIpAddress);
    }

    // Check for localhost by name
    if host == "localhost" {
        return Err(ValidationError::PrivateOrLocalAddress);
    }

    // Basic attempt to validate for TLDs
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() < 2 {
        return Err(ValidationError::InvalidTld(host.clone()));
    }

    let tld = parts.last().unwrap();
    if tld.len() < MIN_TLD_CHAR_LENGTH || !tld.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(ValidationError::InvalidTld(tld.to_string()));
    }

    Ok(trimmed.to_string())

}

fn is_private_or_local(addr: &std::net::IpAddr) -> bool {
    match addr {
        std::net::IpAddr::V4(ipv4) => {
            ipv4.is_loopback() || ipv4.is_private() || ipv4.is_link_local() || ipv4.is_unspecified() || ipv4.is_broadcast()
        }
        std::net::IpAddr::V6(ipv6) => {
            ipv6.is_loopback() || ipv6.is_unspecified()
        }
    }
}
