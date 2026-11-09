# api-must-use

> Mark types and functions with `#[must_use]` when ignoring their results is likely a bug

## Why It Matters

An unused computed value, lazy operation, or immediately dropped guard can indicate a programming mistake. `#[must_use]` requests a diagnostic for such discarded values. `Result` and `Option` already carry this attribute; returning them does not require repeating it on the function.

## Bad

```rust
fn checksum(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0, |acc, byte| acc ^ byte)
}

fn main() {
    checksum(b"abc"); // No must_use diagnostic for this user-defined function.
}
```

## Good

```rust
#[must_use = "use the checksum to verify the data"]
fn checksum(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0, |acc, byte| acc ^ byte)
}

// Result already warns when ignored; no extra function annotation is needed.
fn parse_limit(text: &str) -> Result<u64, std::num::ParseIntError> {
    text.parse()
}

fn main() -> Result<(), std::num::ParseIntError> {
    let actual = checksum(b"abc");
    let limit = parse_limit("10")?;
    assert_eq!(actual, 96);
    assert_eq!(limit, 10);
    Ok(())
}
```

## Types, Guards, and Side Effects

Mark a custom guard type when discarding it immediately would end its intended
scope. Mark a custom lazy computation when dropping it means the intended work
never happens. For pure functions returning ordinary values, a function-level
attribute can explain the specific consequence of ignoring the result.

```rust
#[must_use = "bind the guard for the intended lock scope"]
struct LockScope<'a>(std::sync::MutexGuard<'a, ()>);

fn main() {
    let lock = std::sync::Mutex::new(());
    let _scope = LockScope(lock.lock().expect("lock should not be poisoned"));
    // The guard remains live until _scope leaves this scope.
}
```

`#[must_use]` is a diagnostic, not a guarantee: `let _ = expression` can explicitly
discard a result. For guards, `let _guard = expression` keeps the guard alive;
`let _ = expression` does not. A function with useful side effects can still
return an error that must be handled. Do not silence diagnostics merely because
the function logs or performs I/O.

## Lint Configuration

```toml
[lints.rust]
unused_must_use = "deny"

[lints.clippy]
must_use_candidate = "warn"
double_must_use = "warn"
```

`unused_must_use` is a rustc lint. An additional function attribute can be useful
for a custom message, but an unadorned attribute is redundant when the return
type already provides the diagnostic.

## Sources

- [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html) — must-use functions and types
- [Rust Reference: the must_use attribute](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute)

## See Also

- [api-builder-must-use](api-builder-must-use.md) - prevent discarded builders
- [err-result-over-panic](err-result-over-panic.md) - return errors callers can handle
- [lint-deny-correctness](lint-deny-correctness.md) - configure diagnostics by purpose
