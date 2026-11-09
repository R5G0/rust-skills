# err-doc-errors

> Document error conditions with `# Errors` section in doc comments

## Why It Matters

Users of your API need to know what can go wrong and why. The `# Errors` documentation section is the standard Rust convention for describing when a function returns `Err`. Good error documentation helps callers handle errors appropriately and understand the contract of your API.

## Bad

```rust
/// Loads a configuration from the specified path.
pub fn load_config(path: &Path) -> Result<Config, ConfigError> {
    // No documentation of error conditions
    // Caller must read source code to understand what can fail
}

/// Parses and validates the input string.
/// 
/// Returns the parsed value.  // What about errors?
pub fn parse_input(input: &str) -> Result<Value, ParseError> {
    // ...
}
```

## Good

<!-- verify: compile -->
```rust
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    InvalidFormat,
    Overflow,
    NotPositive,
}

/// Parses an integer strictly greater than zero.
///
/// # Errors
///
/// Returns [`ParseError::Empty`] for empty input, [`ParseError::InvalidFormat`]
/// for invalid integer syntax, [`ParseError::Overflow`] for an integer outside
/// the `i64` range, and [`ParseError::NotPositive`] for zero or negative values.
pub fn parse_positive_int(input: &str) -> Result<i64, ParseError> {
    if input.is_empty() {
        return Err(ParseError::Empty);
    }
    let value = input.parse::<i64>().map_err(|error| match error.kind() {
        std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => ParseError::Overflow,
        _ => ParseError::InvalidFormat,
    })?;
    if value <= 0 {
        return Err(ParseError::NotPositive);
    }
    Ok(value)
}

fn main() {
    assert_eq!(parse_positive_int("42"), Ok(42));
    assert_eq!(parse_positive_int(""), Err(ParseError::Empty));
    assert_eq!(parse_positive_int("bad"), Err(ParseError::InvalidFormat));
    assert_eq!(parse_positive_int("0"), Err(ParseError::NotPositive));
    assert_eq!(parse_positive_int("9223372036854775808"), Err(ParseError::Overflow));
}
```

## Linking to Error Variants

```rust
/// Attempts to connect to the database.
///
/// # Errors
///
/// This function will return an error if:
///
/// - [`DbError::ConnectionFailed`] - The database server is unreachable
/// - [`DbError::AuthenticationFailed`] - Invalid credentials
/// - [`DbError::Timeout`] - Connection attempt exceeded timeout
/// - [`DbError::TlsError`] - TLS handshake failed
///
/// See [`DbError`] for more details on each variant.
pub fn connect(config: &DbConfig) -> Result<Connection, DbError> {
    // ...
}
```

## Panic vs Error Documentation

```rust
/// Divides two numbers.
///
/// # Errors
///
/// Returns [`MathError::DivisionByZero`] if `divisor` is zero.
///
/// # Panics
///
/// Panics if called from a non-main thread (debug builds only).
pub fn divide(dividend: i64, divisor: i64) -> Result<i64, MathError> {
    // ...
}
```

## Error Section Format Options

```rust
// Style 1: Bullet list (good for multiple conditions)
/// # Errors
///
/// Returns an error if:
/// - The file does not exist
/// - The file cannot be read
/// - The content is invalid UTF-8

// Style 2: Returns statements (good for mapping to variants)
/// # Errors
///
/// Returns [`Error::NotFound`] if the item doesn't exist.
/// Returns [`Error::PermissionDenied`] if access is forbidden.

// Style 3: Prose (good for complex conditions)
/// # Errors
///
/// This function returns an error when the input fails validation.
/// Validation includes checking that all required fields are present,
/// that numeric fields are within allowed ranges, and that string
/// fields match their expected formats.
```

## Clippy Lint

```toml
# Cargo.toml - require error documentation
[lints.clippy]
missing_errors_doc = "warn"
```

```rust
// This will warn without # Errors section
pub fn might_fail() -> Result<(), Error> { Ok(()) }
```

## See Also

- [doc-examples-section](./doc-examples-section.md) - Examples in documentation
- [err-thiserror-lib](./err-thiserror-lib.md) - Defining error types
- [api-must-use](./api-must-use.md) - Marking Results as must_use
