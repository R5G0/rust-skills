# test-descriptive-names

> Use descriptive test names that explain what is being tested

## Why It Matters

Test names appear in test output and serve as documentation. A good test name tells you what behavior is being verified without reading the test body. When a test fails, a descriptive name immediately tells you what broke.

## Bad

```rust
#[test]
fn test1() { ... }

#[test]
fn test_parse() { ... }  // Parse what? What behavior?

#[test]
fn it_works() { ... }

#[test]
fn test_function() { ... }

// Failure output: "test test_parse ... FAILED"
// What failed? No idea.
```

## Good

<!-- verify: compile -->
```rust
fn parse_port(input: &str) -> Result<u16, std::num::ParseIntError> {
    input.parse()
}

#[cfg(test)]
mod tests {
    use super::parse_port;

    #[test]
    fn parse_port_rejects_empty_input() {
        assert!(parse_port("").is_err());
    }

    #[test]
    fn parse_port_accepts_the_largest_u16() {
        assert_eq!(parse_port("65535"), Ok(u16::MAX));
    }
}
```

## Naming Patterns

```rust
// Pattern: function_condition_expected_result
#[test]
fn parse_valid_json_returns_document() { ... }

#[test]
fn parse_invalid_json_returns_syntax_error() { ... }

// Pattern: scenario_expectation
#[test]
fn empty_cart_has_zero_total() { ... }

#[test]
fn adding_item_increases_cart_total() { ... }

// Pattern: when_given_then (BDD-style)
#[test]
fn when_user_not_found_then_returns_404() { ... }
```

## Edge Cases

```rust
#[test]
fn handles_empty_string() { ... }

#[test]
fn handles_max_length_input() { ... }

#[test]
fn handles_unicode_emoji() { ... }

#[test]
fn handles_null_bytes() { ... }

#[test]
fn handles_concurrent_access() { ... }
```

## Error Cases

```rust
#[test]
fn rejects_negative_quantity() { ... }

#[test]
fn returns_error_for_invalid_email_format() { ... }

#[test]
fn panics_on_double_initialization() { ... }

#[test]
fn timeout_returns_timeout_error() { ... }
```

## Module Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    mod parsing {
        use super::*;
        
        #[test]
        fn accepts_valid_json() { ... }
        
        #[test]
        fn rejects_trailing_comma() { ... }
    }
    
    mod validation {
        use super::*;
        
        #[test]
        fn requires_name_field() { ... }
        
        #[test]
        fn email_must_contain_at_symbol() { ... }
    }
}

// Test output:
// tests::parsing::accepts_valid_json
// tests::parsing::rejects_trailing_comma
// tests::validation::requires_name_field
```

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/flat-tool/tests/public_api.rs). Test names state concrete behavior, such as parse_port_rejects_empty_and_out_of_range_input. The workspace test command compiles and executes these named tests.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [test-arrange-act-assert](./test-arrange-act-assert.md) - Test structure
- [test-cfg-test-module](./test-cfg-test-module.md) - Test module organization
- [doc-examples-section](./doc-examples-section.md) - Documentation tests
