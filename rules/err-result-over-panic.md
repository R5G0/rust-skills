# err-result-over-panic

> Return `Result<T, E>` instead of panicking for recoverable errors

## Why It Matters

Panics unwind the stack and crash the thread (or program). They're unrecoverable from the caller's perspective. `Result<T, E>` gives callers the ability to decide how to handle errors—retry, fallback, propagate, or log. Libraries should almost never panic; applications should minimize panics to truly unrecoverable situations.

## Bad

```rust
fn parse_config(path: &str) -> Config {
    let content = std::fs::read_to_string(path)
        .expect("Failed to read config");  // Crashes on missing file
    
    serde_json::from_str(&content)
        .expect("Invalid config format")   // Crashes on bad JSON
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");  // Crashes the program
    }
    a / b
}
```

Caller has no chance to recover or provide a fallback.

## Good

<!-- verify: compile -->
```rust
#[derive(Debug, PartialEq, Eq)]
enum DivideError {
    ZeroDivisor,
    Overflow,
}

fn divide(left: i32, right: i32) -> Result<i32, DivideError> {
    if right == 0 {
        return Err(DivideError::ZeroDivisor);
    }
    left.checked_div(right).ok_or(DivideError::Overflow)
}

fn main() {
    assert_eq!(divide(12, 3), Ok(4));
    assert_eq!(divide(12, 0), Err(DivideError::ZeroDivisor));
    // MIN / -1 also panics with ordinary signed division, even in release mode.
    assert_eq!(divide(i32::MIN, -1), Err(DivideError::Overflow));
}
```

## When Panic IS Appropriate

An internal invariant violation can justify a panic with context. Missing files, invalid user input, and malformed configuration remain ordinary errors even when startup cannot continue; report them at the application boundary.

```rust
use std::collections::HashMap;

struct Cache { entries: HashMap<String, u32> }

impl Cache {
    fn get_known_key(&self, key: &str) -> u32 {
        *self.entries.get(key).expect("BUG: caller verified this cache key exists")
    }
}

fn main() {
    let cache = Cache { entries: HashMap::from([("answer".into(), 42)]) };
    assert_eq!(cache.get_known_key("answer"), 42);
}
```

## Panic vs Result Decision Guide

| Situation | Use |
|-----------|-----|
| File not found | `Result` |
| Network error | `Result` |
| Invalid user input | `Result` |
| Parse error | `Result` |
| Index out of bounds (from user data) | `Result` |
| Index out of bounds (internal bug) | Panic |
| Violated internal invariant | Panic |
| Unimplemented code path | Panic (`unimplemented!()`) |
| Impossible state reached | Panic (`unreachable!()`) |

## Library vs Application

```rust
// Library: NEVER panic on user input
pub fn parse(input: &str) -> Result<Ast, ParseError> {
    // Always return Result
}

// Application: Can panic at top level for critical failures
fn main() {
    if let Err(e) = run() {
        eprintln!("Fatal error: {}", e);
        std::process::exit(1);
    }
}
```

## See Also

- [err-thiserror-lib](./err-thiserror-lib.md) - Define error types for libraries
- [err-anyhow-app](./err-anyhow-app.md) - Ergonomic errors for applications
- [err-no-unwrap-prod](./err-no-unwrap-prod.md) - Avoid unwrap in production code
- [anti-unwrap-abuse](./anti-unwrap-abuse.md) - When unwrap is acceptable
