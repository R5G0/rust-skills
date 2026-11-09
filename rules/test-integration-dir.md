# test-integration-dir

> Put integration tests in the `tests/` directory

## Why It Matters

Integration tests live in `tests/` at the crate root, separate from `src/`. Each file in `tests/` is compiled as a separate crate, testing your library's public API as external users would. This separation ensures you're testing the real public interface, not implementation details.

## Structure

```
my_project/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── internal.rs
└── tests/
    ├── integration_test.rs    # Each file is a separate test binary
    ├── api_tests.rs
    └── common/                 # Shared test utilities
        └── mod.rs
```

## Bad

```rust
// src/lib.rs
// Mixing integration test logic in library code
#[test]
fn integration_test_full_workflow() {
    // This is a unit test location, not integration
}
```

## Good

```rust
// tests/public_api.rs
use layout_flat_tool::{Config, parse_port, run};

#[test]
fn run_greets_the_configured_name() {
    assert_eq!(run(Config::new("Ada")), "Hello, Ada!");
}

#[test]
fn parse_port_rejects_out_of_range_input() {
    assert!(parse_port("65536").is_err());
}
```

## Shared Test Utilities

```rust
// tests/common/mod.rs
use my_crate::Config;

pub fn test_config() -> Config {
    Config {
        timeout: Duration::from_secs(5),
        retries: 3,
        debug: true,
    }
}

pub fn setup_test_environment() {
    // Set up test fixtures
}

// tests/api_tests.rs
mod common;

use my_crate::Client;

#[test]
fn test_with_shared_config() {
    common::setup_test_environment();
    let client = Client::new(common::test_config());
    // ...
}
```

## Organizing Many Tests

```rust
// tests/api/mod.rs
mod auth;
mod users;
mod orders;

// tests/api/auth.rs
use my_crate::auth::{login, logout};

#[test]
fn test_login_success() { ... }

#[test]
fn test_login_invalid_credentials() { ... }

// tests/api/users.rs
use my_crate::users::{create_user, get_user};

#[test]
fn test_create_user() { ... }
```

## Integration vs Unit Tests

| Unit Tests | Integration Tests |
|------------|-------------------|
| In `src/` with `#[cfg(test)]` | In `tests/` directory |
| Access private items | Public API only |
| Test individual functions | Test module interactions |
| Fast, isolated | May be slower |
| `cargo test --lib` | `cargo test --test '*'` |

## Running Specific Tests

```bash
# Run all tests
cargo test

# Run only integration tests
cargo test --test '*'

# Run specific integration test file
cargo test --test integration_test

# Run tests matching pattern
cargo test --test api_tests test_login
```

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/flat-tool/tests/public_api.rs). Cargo compiles this tests/ file as a separate test crate. It imports only public Config, run, parse_port, and divide APIs and executes their success, error, and panic contracts.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [test-cfg-test-module](./test-cfg-test-module.md) - Unit test modules
- [test-descriptive-names](./test-descriptive-names.md) - Test naming
- [test-tokio-async](./test-tokio-async.md) - Async integration tests
