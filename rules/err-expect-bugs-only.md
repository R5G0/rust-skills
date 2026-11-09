# err-expect-bugs-only

> Use `expect()` only for invariants that indicate bugs, not user errors

## Why It Matters

`expect()` is better than `unwrap()` because it provides context, but it still panics. Reserve it for situations where failure indicates a bug in your code—a violated invariant, not a user error or external failure. The message should explain why the invariant should hold, helping future developers understand and fix the bug.

## Bad

```rust
// User input can legitimately fail - don't expect
fn parse_user_input(input: &str) -> Config {
    serde_json::from_str(input)
        .expect("Invalid JSON")  // User error, not a bug!
}

// Network can fail - don't expect
fn fetch_data(url: &str) -> Data {
    reqwest::get(url)
        .expect("Network request failed")  // External failure!
        .json()
        .expect("Invalid response")
}

// File might not exist - don't expect
fn load_config() -> Config {
    let content = fs::read_to_string("config.json")
        .expect("Config file missing");  // Environment issue!
}
```

## Good

<!-- verify: compile -->
```rust
use std::collections::HashMap;

#[derive(Default)]
struct Cache {
    entries: HashMap<String, String>,
}

impl Cache {
    fn insert_and_get(&mut self, key: String, value: String) -> &str {
        self.entries.insert(key.clone(), value);
        self.entries.get(&key).expect("BUG: key must exist immediately after insert")
    }
}

fn main() {
    let mut cache = Cache::default();
    assert_eq!(cache.insert_and_get("key".into(), "value".into()), "value");
}
```

## expect() Message Guidelines

Messages should explain why the invariant holds and give enough context to diagnose a bug. A `BUG:` prefix is one useful convention, not a language requirement.

```rust
let value = Some(42);
// Too vague to explain the invariant:
let _ = value.expect("failed");

// Explain the established invariant:
let _ = value.expect("BUG: value was populated immediately before this lookup");
```

## Pattern: Validate Once, expect() After

```rust
struct ValidatedEmail(String);

impl ValidatedEmail {
    pub fn new(email: &str) -> Result<Self, EmailError> {
        // Validation happens here, returns Result
        if !is_valid_email(email) {
            return Err(EmailError::Invalid);
        }
        Ok(ValidatedEmail(email.to_string()))
    }
    
    pub fn domain(&self) -> &str {
        // After validation, expect() is fine
        self.0.split('@').nth(1)
            .expect("BUG: ValidatedEmail must contain @")
    }
}
```

## Alternatives When expect() Is Wrong

```rust
// Don't: expect on user data
let port: u16 = input.parse().expect("Invalid port");

// Do: Return Result
let port: u16 = input.parse().map_err(|_| ConfigError::InvalidPort)?;

// Do: Provide default
let port: u16 = input.parse().unwrap_or(8080);

// Do: Handle explicitly
let port: u16 = match input.parse() {
    Ok(p) => p,
    Err(_) => {
        log::warn!("Invalid port '{}', using default", input);
        8080
    }
};
```

## See Also

- [err-no-unwrap-prod](./err-no-unwrap-prod.md) - Avoiding unwrap in production
- [err-result-over-panic](./err-result-over-panic.md) - When to return Result
- [api-parse-dont-validate](./api-parse-dont-validate.md) - Type-driven validation
