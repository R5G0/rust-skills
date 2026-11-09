# err-from-impl

> Implement `From<E>` for error conversions to enable `?` operator

## Why It Matters

The `?` operator automatically converts errors using `From` trait. By implementing `From<SourceError> for YourError`, you enable seamless error propagation without explicit `.map_err()` calls. This makes error handling code cleaner and ensures consistent error wrapping throughout your codebase.

## Bad

```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(serde_json::Error),
    Database(diesel::result::Error),
}

fn load_config(path: &str) -> Result<Config, AppError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| AppError::Io(e))?;  // Manual conversion everywhere
    
    let config: Config = serde_json::from_str(&content)
        .map_err(|e| AppError::Parse(e))?;  // Repeated boilerplate
    
    save_to_db(&config)
        .map_err(|e| AppError::Database(e))?;  // Gets tedious
    
    Ok(config)
}
```

## Good

<!-- verify: compile -->
```rust
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("failed to read configuration: {0}")]
    Io(std::io::Error),
    #[error("invalid JSON configuration: {0}")]
    Parse(serde_json::Error),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self { Self::Io(error) }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self { Self::Parse(error) }
}

fn read_config(mut reader: impl Read) -> Result<serde_json::Value, AppError> {
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(serde_json::from_str(&content)?)
}

fn main() {
    let config = read_config(&b"{\"retries\":3}"[..]).expect("fixture is JSON");
    assert_eq!(config["retries"], 3);
    assert!(matches!(read_config(&b"invalid"[..]), Err(AppError::Parse(_))));
}
```

## Use thiserror for Automatic From

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),  // Auto-generates From impl
    
    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),  // #[from] does the work
    
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
}

// Now ? just works
fn load_config(path: &str) -> Result<Config, AppError> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    save_to_db(&config)?;
    Ok(config)
}
```

## From with Context

Use an explicit conversion when the destination error also needs operation-specific fields:

```rust
use anyhow::Context;
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("failed to read config from {path}: {source}")]
    ReadFailed { path: String, #[source] source: std::io::Error },
}

fn read_with_typed_context(path: &str) -> Result<String, ConfigError> {
    std::fs::read_to_string(path).map_err(|source| ConfigError::ReadFailed {
        path: path.to_owned(), source,
    })
}

fn read_with_anyhow_context(path: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("failed to read config from {path}"))
}
```

## Blanket From Implementations

A blanket conversion can overlap with `From<T> for T` when the destination itself implements `Error`:

```rust,compile_fail
#[derive(Debug, thiserror::Error)]
#[error("application error: {0}")]
struct AppError(String);

impl<E: std::error::Error> From<E> for AppError {
    fn from(error: E) -> Self { Self(error.to_string()) }
}
```

Prefer concrete source types:

```rust
#[derive(Debug)]
enum AppError { Io(std::io::Error), Parse(std::num::ParseIntError) }

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self { Self::Io(error) }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self { Self::Parse(error) }
}
```

## See Also

- [err-thiserror-lib](./err-thiserror-lib.md) - Using thiserror for libraries
- [err-source-chain](./err-source-chain.md) - Preserving error chains
- [err-question-mark](./err-question-mark.md) - The ? operator
- [conv-tryfrom-fallible](./conv-tryfrom-fallible.md) - TryFrom for fallible conversions
