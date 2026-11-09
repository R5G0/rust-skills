# doc-module-inner

> Use `//!` for module-level documentation

## Why It Matters

Inner doc comments (`//!`) document the module itself, not the next item. They appear at the top of module files and describe the module's purpose, contents, and usage patterns. This helps users understand what a module provides before diving into individual items.

Module docs are the first thing users see in `cargo doc` when navigating to a module.

## Bad

```rust
// This module handles authentication
// It provides JWT and session-based auth

mod auth;

pub use auth::*;
```

```rust
// auth.rs
/// Authentication utilities  // Wrong: this documents nothing useful
use std::collections::HashMap;

pub struct Session { /* ... */ }
```

## Good

<!-- verify: compile -->
```rust
//! Application configuration kept in a single module.
//!
//! [`Config::new`] accepts either a borrowed name or an owned [`String`].

/// Input supplied to the application.
pub struct Config {
    /// Name to greet.
    pub name: String,
}

impl Config {
    /// Construct a configuration with an owned name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
```

## Where to Use Inner Docs

| Location | Purpose |
|----------|---------|
| `lib.rs` | Crate-level documentation (appears on crate root) |
| `mod.rs` | Module documentation for directory modules |
| `module.rs` | Module documentation for single-file modules |

## Crate Root Example

```rust
//! # My Awesome Crate
//!
//! `my_crate` provides utilities for handling complex workflows.
//!
//! ## Quick Start
//!
//! ```rust
//! use my_crate::prelude::*;
//!
//! let workflow = Workflow::builder()
//!     .add_step(Step::new("fetch"))
//!     .add_step(Step::new("process"))
//!     .build();
//! ```
//!
//! ## Modules
//!
//! - [`workflow`] - Core workflow engine
//! - [`steps`] - Built-in workflow steps
//! - [`prelude`] - Common imports
//!
//! ## Feature Flags
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `async` | Async workflow execution |
//! | `serde` | Serialization support |

pub mod workflow;
pub mod steps;
pub mod prelude;
```

## Key Sections for Module Docs

Document the current purpose, contracts, supported features and useful usage
paths. Keep temporary design journals and agent self-reports in review records
or dedicated design documents when they are needed; durable design rationale
belongs in API docs when it helps users choose or use the API correctly.

Sources: [Microsoft M-MODULE-DOCS](https://microsoft.github.io/rust-guidelines/guidelines/docs/index.html#M-MODULE-DOCS), [M-NO-META-DESIGN-DOCUMENTATION](https://microsoft.github.io/rust-guidelines/guidelines/ai/index.html#M-NO-META-DESIGN-DOCUMENTATION).

1. **Brief description** - One-line summary
2. **Overview** - What the module provides
3. **Examples** - How to use it
4. **Feature flags** - Optional functionality
5. **See Also** - Related modules

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/flat-tool/src/config.rs). The real config.rs begins with an inner module doc comment. Its module and public Config documentation are checked by cargo doc --workspace --no-deps.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [doc-all-public](./doc-all-public.md) - Documenting public items
- [doc-examples-section](./doc-examples-section.md) - Adding examples
- [doc-cargo-metadata](./doc-cargo-metadata.md) - Crate metadata
