# doc-errors-section

> Include `# Errors` section for fallible functions

## Why It Matters

Functions returning `Result` can fail in specific, documented ways. The `# Errors` section tells users exactly when and why a function might return an error, enabling them to handle failures appropriately without reading source code.

This is especially critical for library code where users cannot easily inspect implementation details.

## Bad

```rust
/// Opens a file and reads its contents.
pub fn read_file(path: &Path) -> Result<String, Error> {
    // Users have no idea what errors to expect
}

/// Connects to the database.
pub async fn connect(url: &str) -> Result<Connection, DbError> {
    // Multiple failure modes, none documented
}
```

## Good

<!-- verify: compile -->
```rust
use std::path::Path;

/// Read a file as UTF-8 text.
///
/// # Errors
///
/// Returns an I/O error when the file cannot be opened or read. Invalid UTF-8
/// produces an error with kind [`std::io::ErrorKind::InvalidData`].
pub fn read_file(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
```

## Error Documentation Patterns

### Simple Single Error

```rust
/// Parses a string as an integer.
///
/// # Errors
///
/// Returns [`ParseIntError`] if the string is not a valid integer.
pub fn parse_int(s: &str) -> Result<i64, ParseIntError> {
    s.parse()
}
```

### Multiple Error Variants

```rust
/// Sends an HTTP request and returns the response.
///
/// # Errors
///
/// | Error | Condition |
/// |-------|-----------|
/// | [`HttpError::Timeout`] | Request exceeded timeout duration |
/// | [`HttpError::InvalidUrl`] | URL could not be parsed |
/// | [`HttpError::ConnectionRefused`] | Server refused connection |
/// | [`HttpError::TlsError`] | TLS handshake failed |
pub fn send(request: Request) -> Result<Response, HttpError> {
    // ...
}
```

### Propagated Errors

```rust
/// Loads configuration from a file.
///
/// # Errors
///
/// Returns an error if:
/// - The configuration file cannot be read (IO error)
/// - The file contains invalid TOML syntax
/// - Required fields are missing from the configuration
///
/// The underlying error is wrapped with context about which
/// configuration file failed to load.
pub fn load_config(path: &Path) -> Result<Config, anyhow::Error> {
    // ...
}
```

## Linking to Error Types

Use intra-doc links to connect error variants to their definitions:

```rust
/// # Errors
///
/// Returns [`ValidationError::TooShort`] if the input is less than
/// the minimum length, or [`ValidationError::InvalidChars`] if it
/// contains forbidden characters.
```

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/flat-tool/src/lib.rs). The parse_port # Errors section links the concrete ParseIntError type, and public-API tests cover empty and out-of-range inputs.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [doc-panics-section](./doc-panics-section.md) - Documenting panics
- [err-doc-errors](./err-doc-errors.md) - Error documentation patterns
- [doc-intra-links](./doc-intra-links.md) - Linking to types
