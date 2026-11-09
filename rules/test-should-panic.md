# test-should-panic

> Use `#[should_panic]` to test that code panics as expected

## Why It Matters

Some code should panic on invalid inputs or invariant violations. `#[should_panic]` verifies the panic occurs, optionally checking the panic message. This ensures defensive panics work correctly and documents expected panic conditions.

## Bad

```rust
#[test]
fn test_panic() {
    // Just calling panicking code makes test fail
    divide(1, 0);  // Test fails with panic
}

// Using catch_unwind is verbose
#[test]
fn test_panic_manual() {
    let result = std::panic::catch_unwind(|| divide(1, 0));
    assert!(result.is_err());
}
```

## Good

<!-- verify: compile -->
```rust
fn divide(dividend: i32, divisor: i32) -> i32 {
    assert_ne!(divisor, 0, "division by zero");
    dividend / divisor
}

#[cfg(test)]
mod tests {
    use super::divide;

    #[test]
    #[should_panic]
    fn divide_by_zero_panics() {
        divide(1, 0);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn divide_by_zero_panics_with_documented_message() {
        divide(1, 0);
    }

    #[test]
    fn nonzero_divisor_returns_the_quotient() {
        assert_eq!(divide(12, 3), 4);
    }
}
```

## Testing Invariants

```rust
struct NonEmpty<T>(Vec<T>);

impl<T> NonEmpty<T> {
    fn new(items: Vec<T>) -> Self {
        assert!(!items.is_empty(), "NonEmpty cannot be empty");
        NonEmpty(items)
    }
}

#[test]
#[should_panic(expected = "NonEmpty cannot be empty")]
fn non_empty_rejects_empty_vec() {
    NonEmpty::new(Vec::<i32>::new());
}

#[test]
fn non_empty_accepts_non_empty_vec() {
    let ne = NonEmpty::new(vec![1, 2, 3]);
    assert_eq!(ne.0.len(), 3);
}
```

## With expect() Messages

```rust
fn get_config_value(key: &str) -> String {
    CONFIG.get(key)
        .expect(&format!("missing required config: {}", key))
        .to_string()
}

#[test]
#[should_panic(expected = "missing required config: DATABASE_URL")]
fn missing_config_panics_with_key() {
    get_config_value("DATABASE_URL");
}
```

## When NOT to Use should_panic

```rust
// ❌ For recoverable errors - use Result
#[test]
#[should_panic]  // Wrong: this should return Err, not panic
fn invalid_input_panics() {
    parse_config("invalid");  // Should return Err, not panic
}

// ✅ Return Result and test the error
#[test]
fn invalid_input_returns_error() {
    let result = parse_config("invalid");
    assert!(result.is_err());
}
```

## Combining with Result

A test marked `#[should_panic]` must return `()`, not `Result`. If setup is
fallible, resolve its result before the operation expected to panic. Separate
setup errors from the expected panic so an unrelated setup failure cannot make
the test pass.

```rust
fn parse_dividend(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse()
}

fn divide(dividend: i32, divisor: i32) -> i32 {
    assert_ne!(divisor, 0, "division by zero");
    dividend / divisor
}

#[test]
#[should_panic(expected = "division by zero")]
fn parsed_dividend_panics_for_zero_divisor() {
    let dividend = parse_dividend("12").expect("valid literal for test setup");
    divide(dividend, 0);
}
```

## See Also

- [err-result-over-panic](./err-result-over-panic.md) - Panic vs Result
- [err-expect-bugs-only](./err-expect-bugs-only.md) - When to use expect
- [test-descriptive-names](./test-descriptive-names.md) - Test naming
