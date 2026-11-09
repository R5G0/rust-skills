# test-doctest-examples

> Keep documentation examples as executable doctests

## Why It Matters

Doctests are examples in documentation that are automatically tested. They serve dual purposes: demonstrating usage to readers and verifying the examples compile and work. When your API changes, failing doctests catch outdated documentation.

## Bad

```rust
/// Parses a number from a string.
/// 
/// Example:
/// let n = parse("42");  // Not tested!
/// assert_eq!(n, 42);
pub fn parse(s: &str) -> i32 {
    s.parse().unwrap()
}

// Documentation can become outdated:
/// Adds two numbers.
/// 
/// ```
/// let sum = add(1, 2, 3);  // Wrong number of args - not caught!
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Good

```rust
/// Parse a port number.
///
/// # Examples
///
/// ```
/// # use layout_flat_tool::parse_port;
/// let port = parse_port("8080")?;
/// assert_eq!(port, 8080);
/// # Ok::<(), std::num::ParseIntError>(())
/// ```
pub fn parse_port(input: &str) -> Result<u16, std::num::ParseIntError> {
    input.parse()
}
```

## Hiding Setup Code

```rust
/// Processes data from a file.
/// 
/// # Examples
/// 
/// ```
/// # use std::io::Write;
/// # let mut file = tempfile::NamedTempFile::new().unwrap();
/// # writeln!(file, "test data").unwrap();
/// # let path = file.path();
/// use my_crate::process_file;
/// 
/// let result = process_file(path)?;
/// assert!(!result.is_empty());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn process_file(path: &Path) -> Result<String, Error> {
    std::fs::read_to_string(path).map_err(Error::from)
}
```

## Showing Error Handling

```rust
/// Parses and validates an email address.
/// 
/// # Examples
/// 
/// ```
/// use my_crate::Email;
/// 
/// let email = Email::parse("user@example.com")?;
/// assert_eq!(email.domain(), "example.com");
/// # Ok::<(), my_crate::EmailError>(())
/// ```
/// 
/// # Errors
/// 
/// Returns error for invalid format:
/// 
/// ```
/// use my_crate::Email;
/// 
/// assert!(Email::parse("not-an-email").is_err());
/// ```
pub fn parse(s: &str) -> Result<Email, EmailError> {
    // ...
}
```

## no_run and ignore

```rust
/// Starts the server.
/// 
/// ```no_run
/// use my_crate::Server;
/// 
/// // This compiles but doesn't run (would block forever)
/// Server::new().run();
/// ```
pub fn run(&self) { ... }

/// Platform-specific example.
/// 
/// ```ignore
/// // This might not compile on all platforms
/// use windows_specific::Feature;
/// ```
```

## compile_fail

```rust
/// This type is not Clone.
/// 
/// ```compile_fail
/// use my_crate::UniqueHandle;
/// 
/// let a = UniqueHandle::new();
/// let b = a.clone();  // Error: Clone not implemented
/// ```
pub struct UniqueHandle { ... }
```

## Running Doctests

```bash
# Run all tests including doctests
cargo test

# Run only doctests
cargo test --doc

# Run doctests for specific item
cargo test --doc my_function
```

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/flat-tool/src/lib.rs). Three doctests assert run and parse_port behavior. cargo test --doc executes them via rustdoc, including hidden setup and typed error propagation.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [doc-examples-section](./doc-examples-section.md) - Documentation structure
- [doc-hidden-setup](./doc-hidden-setup.md) - Hiding setup code
- [doc-question-mark](./doc-question-mark.md) - Error handling in examples
