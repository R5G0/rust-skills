# doc-question-mark

> Use `?` in examples, not `.unwrap()`

## Why It Matters

Doc examples should model best practices. Using `.unwrap()` teaches users to ignore errors, while `?` demonstrates proper error propagation. Examples with `?` also fail the doctest if an error occurs, catching bugs in documentation.

Use an explicit `fn main() -> Result<_, _>` or a trailing typed `Ok(())` so rustdoc can infer the result type when an example uses `?`. Hidden setup lines can supply this scaffolding without distracting readers.

## Bad

```rust
/// Reads a configuration file.
///
/// # Examples
///
/// ```
/// let config = Config::from_file("config.toml").unwrap();
/// println!("{:?}", config.database_url);
/// ```
pub fn from_file(path: &str) -> Result<Config, Error> {
    // ...
}

/// Fetches data from the API.
///
/// # Examples
///
/// ```
/// let client = Client::new();
/// let response = client.get("https://api.example.com").unwrap();
/// let data: Data = response.json().unwrap();
/// ```
pub async fn get(&self, url: &str) -> Result<Response, Error> {
    // ...
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

## Doctest Wrapper Pattern

Rust wraps doc examples in a function. You can make this explicit:

```rust
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let value: u16 = "8080".parse()?;
/// assert_eq!(value, 8080);
/// # Ok(())
/// # }
/// ```
pub fn documentation_wrapper_example() {}
```

Or let rustdoc infer the wrapper return type from a typed final expression:

```rust
/// # Examples
///
/// ```
/// # use my_crate::parse_config;
/// let value = parse_config("key=value")?;
/// assert_eq!(value.key, "value");
/// # Ok::<(), my_crate::Error>(())
/// ```
```

## When to Use `.unwrap()`

There are specific cases where `.unwrap()` is acceptable in examples:

```rust
/// # Examples
///
/// ```
/// // Static regex that is known at compile time to be valid
/// let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
///
/// // Parsing a literal that cannot fail
/// let n: i32 = "42".parse().unwrap();
/// ```
```

But still prefer `?` when demonstrating error handling patterns.

## Comparison

| Pattern | Behavior on Error | Teaches |
|---------|-------------------|---------|
| `.unwrap()` | Panics with generic message | Bad habits |
| `.expect()` | Panics with custom message | Slightly better |
| `?` | Propagates error, test fails | Best practices |

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/flat-tool/src/lib.rs). The parse_port doctest uses ? and a hidden typed Ok expression. cargo test --doc verifies the rendered example through rustdoc, rather than only compiling its outer API declaration.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [doc-examples-section](./doc-examples-section.md) - Writing examples
- [doc-hidden-setup](./doc-hidden-setup.md) - Hiding setup code
- [err-question-mark](./err-question-mark.md) - Error propagation
