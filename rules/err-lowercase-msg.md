# err-lowercase-msg

> Start error messages lowercase, no trailing punctuation

## Why It Matters

Error messages are often chained, logged, or displayed with additional context. Consistent formatting—lowercase start, no trailing period—allows clean composition: "failed to load config: invalid JSON: unexpected token". Mixed case and punctuation create awkward output: "Failed to load config.: Invalid JSON.: Unexpected token.".

## Bad

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Failed to read config file.")]  // Capital F, trailing period
    ReadFailed(#[from] std::io::Error),
    
    #[error("Invalid JSON format!")]  // Capital I, exclamation
    ParseFailed(#[from] serde_json::Error),
    
    #[error("The requested key was not found")]  // Reads like a sentence
    KeyNotFound(String),
}

// Chained output: "Config load error: Failed to read config file.: No such file"
// Awkward capitalization and punctuation
```

## Good

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("failed to read config file")]  // lowercase, no period
    ReadFailed(#[from] std::io::Error),
    
    #[error("invalid JSON format")]  // lowercase, no period
    ParseFailed(#[from] serde_json::Error),
    
    #[error("key not found: {0}")]  // lowercase, data at end
    KeyNotFound(String),
}

// Chained output: "config load error: failed to read config file: no such file"
// Clean, consistent
```

## Rust Standard Library Convention

Standard library error messages normally start lowercase. The exact wording can depend on the platform; these strings illustrate the style rather than a stable message contract.

```rust
let examples = [
    "entity not found",
    "permission denied",
    "connection refused",
    "invalid digit found in string",
    "invalid utf-8 sequence",
];
assert!(examples.iter().all(|message| message.starts_with(char::is_lowercase)));
```

## Formatting Guidelines

| Do | Don't |
|----|-------|
| `"failed to parse config"` | `"Failed to parse config."` |
| `"invalid input: expected number"` | `"Invalid input - expected a number!"` |
| `"connection timed out after {0}s"` | `"Connection Timed Out After {0} seconds."` |
| `"key '{0}' not found"` | `"Key Not Found: {0}"` |

## Context Addition Pattern

```rust
use anyhow::{Context, Result};

fn load_user(id: u64) -> Result<User> {
    let data = fetch(id)
        .with_context(|| format!("failed to fetch user {}", id))?;
    
    parse_user(data)
        .with_context(|| "failed to parse user data")?
}

// Output: "failed to fetch user 42: connection refused"
// All lowercase, clean chain
```

## Display vs Debug

```rust
#[derive(Error, Debug)]
#[error("invalid configuration")]  // Display: for users/logs
pub struct ConfigError {
    path: PathBuf,
    source: io::Error,
}

// Debug output (for developers) can have more detail
// Display output (for users) should be clean
```

## When to Use Capitals

Proper nouns and acronyms retain their spelling, even at the start of a message:

```rust
#[derive(Debug, thiserror::Error)]
enum RequestError {
    #[error("invalid JSON syntax")]
    InvalidJson,
    #[error("OAuth token expired")]
    ExpiredToken,
    #[error("HTTP request failed")]
    Http,
    #[error("error code E0001: invalid input")]
    InvalidInput,
}
```

## See Also

- [err-thiserror-lib](./err-thiserror-lib.md) - Error definition with thiserror
- [err-context-chain](./err-context-chain.md) - Adding context to errors
- [doc-examples-section](./doc-examples-section.md) - Documentation conventions
