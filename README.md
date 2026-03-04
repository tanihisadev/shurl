# What is Shurl?

Shurl (Short-Url) is a simple command-line URL shortener written in Rust, using the [is.gd](https://is.gd) service.

Built as a learning project to explore Rust and CLI tooling.

## Requirements

- [Rust](https://rustup.rs) (installed via `rustup`)

## Installation

Clone the repository and install via Cargo:

```bash
git clone git@github.com:tanihisadev/shurl.git
cd shurl
cargo install --path .
```

This will build a release binary and place it in `~/.cargo/bin/`, making it available system-wide.

## Usage

```bash
shurl <URL>
```

### Example

```bash
shurl https://www.example.com
# Shortened URL: https://is.gd/abc123
```

### Arguments

| Argument | Description                  | Required |
|----------|------------------------------|----------|
| `URL`    | The full URL to shorten      | Yes      |

### Flags

| Flag         | Description                                                                        |
|--------------|------------------------------------------------------------------------------------|
| `-h, --help` | Print help text                                                                    |
| `--embed`    | Uses known alternative domains to give a URL you can embed in places like Discord  |

## Validation

The shurl command validates input before hitting the API. The URL must:

- Be non-empty
- Include a scheme (`http://` or `https://`)
- Be a valid, parseable URL

## Project Structure

```
src/
├── main.rs        # Entry point
├── cli.rs         # CLI argument definitions (clap)
├── shortener.rs   # HTTP logic and API call
├── embedders.rs   # Embed domains
└── validator.rs   # URL validation
```

## Dependencies

| Crate         | Purpose                              |
|---------------|--------------------------------------|
| `clap`        | CLI argument parsing                 |
| `reqwest`     | HTTP client                          |
| `serde`       | Serialization / deserialization      |
| `serde_json`  | JSON support                         |
| `url`         | URL parsing and validation           |

## License

MIT
