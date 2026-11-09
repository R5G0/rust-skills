# doc-examples-section

> Include `# Examples` with runnable code

## Why It Matters

Examples are the most valuable part of documentation. They show users exactly how to use your API. Rust's doc tests ensure examples stay correct as code evolves.

## Bad

```rust
/// Parses a string into a Foo.
pub fn parse(s: &str) -> Result<Foo, Error> {
    // No examples - users have to guess usage
}

/// A widget for doing things.
/// 
/// This widget is very useful.
pub struct Widget {
    // Still no examples
}
```

## Good

<!-- verify: compile -->
```rust
/// Parse a port number.
///
/// # Errors
///
/// Returns [`std::num::ParseIntError`] for non-numeric or out-of-range input.
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

## Use ? Not unwrap()

```rust
/// Loads configuration from a file.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use my_crate::Config;
///
/// let config = Config::load("config.toml")?;
/// println!("Port: {}", config.port);
/// # Ok(())
/// # }
/// ```
pub fn load(path: &str) -> Result<Config, Error> {
    // ...
}
```

## Hide Setup Code

```rust
/// Processes items from a database.
///
/// # Examples
///
/// ```
/// # use my_crate::{Database, Item};
/// # fn get_db() -> Database { Database::mock() }
/// let db = get_db();
/// let items = db.process_items()?;
/// assert!(!items.is_empty());
/// # Ok::<(), my_crate::Error>(())
/// ```
pub fn process_items(&self) -> Result<Vec<Item>, Error> {
    // ...
}
```

## Multiple Examples

```rust
/// Creates a new buffer with the specified capacity.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use my_crate::Buffer;
///
/// let buf = Buffer::with_capacity(1024);
/// assert_eq!(buf.capacity(), 1024);
/// ```
///
/// Zero capacity creates an empty buffer:
///
/// ```
/// use my_crate::Buffer;
///
/// let buf = Buffer::with_capacity(0);
/// assert!(buf.is_empty());
/// ```
pub fn with_capacity(cap: usize) -> Self {
    // ...
}
```

## Show Error Cases

```rust
/// Divides two numbers.
///
/// # Examples
///
/// ```
/// use my_crate::divide;
///
/// assert_eq!(divide(10, 2), Ok(5));
/// ```
///
/// Division by zero returns an error:
///
/// ```
/// use my_crate::{divide, MathError};
///
/// assert_eq!(divide(10, 0), Err(MathError::DivisionByZero));
/// ```
pub fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    // ...
}
```

## Running Doc Tests

```bash
# Run all doc tests
cargo test --doc

# Run doc tests for specific item
cargo test --doc my_function
```

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/flat-tool/src/lib.rs). The parse_port examples include a success assertion and an invalid-input assertion. cargo test --doc compiles and runs both rendered documentation examples.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [doc-question-mark](doc-question-mark.md) - Use ? in examples
- [doc-hidden-setup](doc-hidden-setup.md) - Hide setup code with #
- [doc-errors-section](doc-errors-section.md) - Document error conditions
