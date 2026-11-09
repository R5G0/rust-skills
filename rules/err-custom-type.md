# err-custom-type

> Define custom error types for domain-specific failures

## Why It Matters

Generic errors like `String`, `Box<dyn Error>`, or catch-all enums obscure what can actually go wrong. Custom error types document failure modes in the type system, enable pattern matching for specific handling, and provide clear API contracts. They make your code self-documenting and help callers handle errors appropriately.

## Bad

```rust
// Generic string errors - no structure
fn validate_user(user: &User) -> Result<(), String> {
    if user.name.is_empty() {
        return Err("Name is empty".to_string());
    }
    if user.age > 150 {
        return Err("Age is invalid".to_string());
    }
    Ok(())
}

// Caller can't match on specific errors
match validate_user(&user) {
    Ok(()) => save(user),
    Err(msg) => {
        // Can only do string comparison - fragile!
        if msg.contains("Name") {
            prompt_for_name()
        }
    }
}
```

## Good

<!-- verify: compile -->
```rust
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
enum ValidationError {
    #[error("name cannot be empty")]
    EmptyName,
    #[error("name contains {actual} characters; maximum is {max}")]
    NameTooLong { max: usize, actual: usize },
    #[error("invalid age {0}: maximum is 150")]
    InvalidAge(u8),
}

fn validate_user(name: &str, age: u8) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::EmptyName);
    }
    let actual = name.chars().count();
    if actual > 100 {
        return Err(ValidationError::NameTooLong { max: 100, actual });
    }
    if age > 150 {
        return Err(ValidationError::InvalidAge(age));
    }
    Ok(())
}

fn main() {
    assert_eq!(validate_user("Alice", 42), Ok(()));
    assert_eq!(validate_user("", 42), Err(ValidationError::EmptyName));
    assert_eq!(validate_user("Alice", 151), Err(ValidationError::InvalidAge(151)));
}
```

## Error Type Design Guidelines

```rust
// 1. Group related errors in domain-specific enums
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("account locked after {attempts} failed attempts")]
    AccountLocked { attempts: u32 },
    #[error("token expired")]
    TokenExpired,
}

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("insufficient funds: need {required}, have {available}")]
    InsufficientFunds { required: Decimal, available: Decimal },
    #[error("card declined: {reason}")]
    CardDeclined { reason: String },
}

// 2. Include relevant data for error handling/display
#[derive(Error, Debug)]
pub enum FileError {
    #[error("file not found: {path}")]
    NotFound { path: PathBuf },
    #[error("permission denied for {path}")]
    PermissionDenied { path: PathBuf },
}

// 3. Consider #[non_exhaustive] for public APIs
#[derive(Error, Debug)]
#[non_exhaustive]  // Allows adding variants without breaking changes
pub enum ApiError {
    #[error("rate limited")]
    RateLimited,
    #[error("not found")]
    NotFound,
}
```

## When to Use What

| Error Pattern | Use Case |
|---------------|----------|
| Custom enum | Library with specific failure modes |
| `thiserror` | Libraries needing `std::error::Error` |
| `anyhow::Error` | Applications, prototypes |
| Struct with source | Single error type with wrapped cause |

## Struct-Based Errors

For single error types with rich context:

```rust
#[derive(Error, Debug)]
#[error("query failed for table '{table}' with filter '{filter}'")]
pub struct QueryError {
    pub table: String,
    pub filter: String,
    #[source]
    pub source: DatabaseError,
}
```

## See Also

- [err-thiserror-lib](./err-thiserror-lib.md) - thiserror for error definitions
- [err-anyhow-app](./err-anyhow-app.md) - When to use anyhow instead
- [api-non-exhaustive](./api-non-exhaustive.md) - Forward-compatible enums
