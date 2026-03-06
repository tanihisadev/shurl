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
shurl <URL> [FLAGS]
```

### Example

```bash
shurl https://www.example.com
# Shortened URL: https://is.gd/abc123
# Copied to clipboard!
```

### Arguments

| Argument | Description             | Required |
|----------|-------------------------|----------|
| `URL`    | The full URL to shorten | Yes      |

### Flags

| Flag            | Description                                                               |
|-----------------|---------------------------------------------------------------------------|
| `-e, --embed`   | Replaces the domain with an embed-friendly alternative (skips shortening) |
| `-c, --clean`   | Strips known tracking parameters from the URL before shortening           |
| `--no-copy`     | Disables automatic copying of the result to the clipboard                 |
| `-V, --version` | Print version                                                             |
| `-h, --help`    | Print help text                                                           |
| `-p, --provider`| Choose provider API to use for link shortening

## Clipboard

By default, Shurl automatically copies the result to your clipboard after shortening or embedding. To disable this behaviour, use the `--no-copy` flag:

```bash
shurl --no-copy https://www.example.com
```

## Embed Domains

The `--embed` flag replaces known domains with embed-friendly alternatives, useful for platforms like Discord where certain links do not preview correctly.

| Original        | Replacement      |
|-----------------|------------------|
| `instagram.com` | `kkinstagram.com`|
| `x.com`         | `fxtwitter.com`  |
| `twitter.com`   | `fxtwitter.com`  |
| `reddit.com`    | `rxddit.com`     |

Note: shortening is skipped when `--embed` is used, as shortened URLs prevent platforms from generating a preview.

# Providers

The `--providers` allows for choosing the API to use for to process and shorten the link.

| Parameter        | 
|------------------|
| `isgd` (Default) |
| `tinyurl`        |

## Tracking Parameters

The `--clean` flag strips known tracking parameters from the URL before shortening. Recognised parameters include:

| Parameter                                                             | Source          |
|-----------------------------------------------------------------------|-----------------|
| `igsh`, `igshid`                                                      | Instagram       |
| `si`, `is`                                                            | YouTube         |
| `utm_source`, `utm_medium`, `utm_campaign`, `utm_term`, `utm_content` | Universal UTM   |
| `fbclid`                                                              | Meta / Facebook |
| `gclid`, `gclsrc`                                                     | Google          |
| `ref_src`, `ref_url`                                                  | Twitter         |
| `ref`                                                                 | Generic         |

## Project Structure

```
src/
├── main.rs           # Entry point, wires everything together
├── cli.rs            # CLI argument definitions (clap)
├── embedder.rs       # Embed-friendly domain substitution (--embed)
├── stripper.rs       # Tracking parameter removal (--clean)
├── clipboard.rs      # Clipboard support
├── validator.rs      # URL validation
└── providers/
    ├── mod.rs        # Shortener trait definition (--provider)
    ├── isgd.rs       # is.gd implementation
    └── tinyurl.rs    # TinyURL implementation
```

## Dependencies

| Crate        | Purpose                         |
|--------------|---------------------------------|
| `clap`       | CLI argument parsing            |
| `reqwest`    | HTTP client                     |
| `serde`      | Serialization / deserialization |
| `serde_json` | JSON support                    |
| `url`        | URL parsing and validation      |
| `arboard`    | Clipboard support               |

## License

MIT