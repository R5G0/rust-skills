# doc-intra-links

> Use intra-doc links to reference types and items

## Why It Matters

Intra-doc links (`[TypeName]`, `[method](Self::method)`) create clickable references in generated documentation. They resolve through Rust item paths instead of hard-coded generated HTML paths. Checking links when documentation is built catches stale references after an item is renamed or moved.

## Bad

```rust
/// Returns the length of the buffer.
/// 
/// See also `capacity()` for the allocated size, and the
/// `Buffer` struct for more details.
pub fn len(&self) -> usize {
    self.data.len()
}

/// Parses the input using std::str::FromStr trait.
/// Check the Error enum for possible failures.
pub fn parse<T: FromStr>(input: &str) -> Result<T, Error> {
    // ...
}
```

## Good

<!-- verify: compile -->
```rust
/// Input accepted by [`run`].
pub struct Config {
    /// Name to greet.
    pub name: String,
}

impl Config {
    /// Construct a [`Config`] with an owned name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Greet the name supplied by [`Config::new`].
pub fn run(config: Config) -> String {
    format!("Hello, {}!", config.name)
}
```

## Link Syntax

| Syntax | Links To | Example |
|--------|----------|---------|
| `[Name]` | Item in scope | `[Vec]`, `[Option]` |
| `[path::Name]` | Fully qualified item | `[std::vec::Vec]` |
| `[Self::method]` | Method on current type | `[Self::new]` |
| `[Type::method]` | Method on other type | `[String::new]` |
| `[Type::CONST]` | Associated constant | `[usize::MAX]` |
| `[text](path)` | Custom text | `[see here](Self::len)` |

## Common Patterns

### Linking to Self Members

```rust
impl Buffer {
    /// Creates an empty buffer.
    ///
    /// Use [`with_capacity`](Self::with_capacity) if you know the size.
    pub fn new() -> Self { /* ... */ }
    
    /// Creates a buffer with pre-allocated capacity.
    ///
    /// See [`new`](Self::new) for the default constructor.
    pub fn with_capacity(cap: usize) -> Self { /* ... */ }
}
```

### Linking to Trait Methods

```rust
/// Converts to a string representation.
///
/// This is the implementation of [`Display::fmt`](std::fmt::Display::fmt).
impl Display for MyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // ...
    }
}
```

### Disambiguation

When names conflict, use suffixes:

```rust
/// See [`foo()`](fn@foo) for the function and [`foo`](mod@foo) for the module.

/// Works with [`Error`](struct@Error) struct or [`Error`](trait@Error) trait.
```

| Suffix | Item Type |
|--------|-----------|
| `fn@` | Function |
| `mod@` | Module |
| `struct@` | Struct |
| `enum@` | Enum |
| `trait@` | Trait |
| `type@` | Type alias |
| `const@` | Constant |
| `macro@` | Macro |

### Reference-Style Links

For repeated links or long paths:

```rust
/// Parses using [`serde`] with [`Deserialize`] trait.
/// Returns a [`Result`] that may contain [`Error`].
///
/// [`serde`]: https://serde.rs
/// [`Deserialize`]: serde::Deserialize
/// [`Result`]: std::result::Result
/// [`Error`]: crate::Error
```

## Verification

Enable link checking in CI:

```bash
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

This fails if any intra-doc links are broken.

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/flat-tool/src/config.rs). Config links to crate::run and its constructor links to Config. The workspace denies broken intra-doc links; cargo doc --workspace --no-deps checks those actual links.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [doc-all-public](./doc-all-public.md) - Documenting public items
- [doc-examples-section](./doc-examples-section.md) - Adding examples
- [doc-errors-section](./doc-errors-section.md) - Documenting errors
