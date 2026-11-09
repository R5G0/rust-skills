# doc-link-types

> Use intra-doc links to connect related types and functions

## Why It Matters

Rustdoc intra-doc links create clickable references between documented items. They resolve when documentation is built, so enabling broken-link warnings catches stale paths after a rename or move. Plain text references provide neither navigation nor link checking.

## Bad

```rust
/// Parses input and returns a ParseResult.
/// 
/// See also: ParseError for error types.
/// Uses the Tokenizer internally.
pub fn parse(input: &str) -> ParseResult {
    // "ParseResult", "ParseError", "Tokenizer" are not clickable
    // No verification they exist
}
```

## Good

<!-- verify: compile -->
```rust
/// Configuration accepted by [`run`].
pub struct Config {
    /// Name to greet.
    pub name: String,
}

/// Produce a greeting from a [`Config`].
///
/// Use [`String::as_str`] to borrow the resulting text.
pub fn run(config: Config) -> String {
    format!("Hello, {}!", config.name)
}
```

## Link Syntax

```rust
/// Basic link to type in same module
/// See [`MyType`] for details.

/// Link to method
/// Use [`MyType::new`] to create instances.

/// Link to associated type
/// Returns [`Iterator::Item`].

/// Link to module
/// See the [`parser`] module.

/// Link to external crate type
/// Works with [`std::collections::HashMap`].

/// Link with custom text
/// See [the parser][`parse`] for details.

/// Link to module item
/// See [`crate::utils::helper`].

/// Link to parent module item
/// See [`super::Parent`].
```

## Common Patterns

```rust
/// A configuration builder.
///
/// # Example
///
/// ```
/// use my_crate::Config;
///
/// let config = Config::builder()
///     .timeout(30)
///     .build()?;
/// ```
///
/// # Methods
///
/// - [`Config::builder`] - Create a new builder
/// - [`Config::default`] - Create with defaults
///
/// # Related Types
///
/// - [`ConfigBuilder`] - The builder returned by [`Config::builder`]
/// - [`ConfigError`] - Errors that can occur when building
pub struct Config { ... }

impl Config {
    /// Creates a new [`ConfigBuilder`].
    ///
    /// This is equivalent to [`ConfigBuilder::new`].
    pub fn builder() -> ConfigBuilder { ... }
}
```

## Linking to Trait Items

```rust
/// Implements [`Iterator`] for lazy evaluation.
///
/// [`Iterator::next`] advances the cursor; [`Iterator::nth`] skips values.
/// For parallel iteration, see [`rayon::iter::ParallelIterator`].
pub struct MyIterator {
    remaining: std::ops::Range<u32>,
}

impl MyIterator {
    pub fn new(range: std::ops::Range<u32>) -> Self {
        Self { remaining: range }
    }
}

impl Iterator for MyIterator {
    type Item = u32;

    /// Advance the cursor; see [`Iterator::nth`] for skipping elements.
    fn next(&mut self) -> Option<Self::Item> {
        self.remaining.next()
    }
}

fn main() {
    assert_eq!(MyIterator::new(1..4).collect::<Vec<_>>(), vec![1, 2, 3]);
}
```

## Broken Link Detection

```bash
# Catch broken intra-doc links
RUSTDOCFLAGS="-D warnings" cargo doc

# Or in CI
cargo doc --no-deps 2>&1 | grep "warning: unresolved link"
```

```toml
# Cargo.toml - deny broken links
[lints.rustdoc]
broken_intra_doc_links = "deny"
```

## Module-Level Documentation

```rust
//! # Parser Module
//!
//! This module provides parsing utilities.
//!
//! ## Main Types
//!
//! - [`Parser`] - The main parser struct
//! - [`Token`] - Tokens produced by tokenization
//! - [`Ast`] - The abstract syntax tree
//!
//! ## Functions
//!
//! - [`parse`] - Parse a string
//! - [`parse_file`] - Parse a file
//!
//! ## Errors
//!
//! All functions return [`ParseError`] on failure.

pub struct Parser { ... }
pub enum Token { ... }
pub struct Ast { ... }
```

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/flat-tool/src/lib.rs). The crate documentation links Config, run, parse_port, str::parse, and ParseIntError. cargo doc --workspace --no-deps resolves the links with broken_intra_doc_links denied.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [doc-examples-section](./doc-examples-section.md) - Code examples in docs
- [err-doc-errors](./err-doc-errors.md) - Documenting errors
- [lint-deny-correctness](./lint-deny-correctness.md) - Lint settings
