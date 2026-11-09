# api-impl-into

> Consider `impl Into<T>` for useful owned-input conversions and implement `From<T>` for conversion types

## Why It Matters

An owned-input API can accept several source types through `impl Into<T>`, converting once when it needs the target representation. This trades simpler call sites for generic signatures, inference constraints, and possible monomorphization costs. Use concrete parameters when callers already have the required type; use borrowed parameters when ownership is unnecessary.

## Bad

```rust
// This reader only borrows text, but converts every literal to owned storage.
fn text_len(text: impl Into<String>) -> usize {
    text.into().len()
}

fn main() {
    assert_eq!(text_len("literal"), 7);
}
```

## Good

```rust
use std::path::PathBuf;

// A reusable configuration API accepts varied inputs and retains ownership.
struct Config {
    name: String,
    path: PathBuf,
}

impl Config {
    fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
    }
}

// A read-only operation does not require an owned conversion.
fn text_len(text: &str) -> usize {
    text.len()
}

fn main() {
    let config = Config::new("worker", "./config.toml");
    assert_eq!(config.name, "worker");
    assert_eq!(config.path, PathBuf::from("./config.toml"));
    assert_eq!(text_len("literal"), 7);
}
```

## Implement From for Infallible Conversions

```rust
#[derive(Debug, PartialEq)]
struct UserId(u64);

impl From<u64> for UserId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

fn main() {
    // The standard blanket implementation supplies Into<UserId>.
    let id: UserId = 42_u64.into();
    assert_eq!(id, UserId(42));
}
```

Use `TryFrom` when a conversion can fail. Do not implement both `From<Source>`
and `Into<Target>` for the same conversion: the blanket implementation would
conflict.

## Choose the Contract Callers Need

- A private application function receiving an existing `String` can simply
  accept `String`. A concrete parameter is not inherently an anti-pattern.
- Use `&str`, `&Path`, slices, or a justified `AsRef<T>` parameter for borrowed
  access. `Into<String>` from a string literal generally allocates.
- A generic `Into<T>` bound uses static dispatch; it does not inherently add
  virtual dispatch overhead. Consider compile time, code size, actual conversion
  work, and call-site inference instead.
- `impl Into<Box<dyn Trait>>` is syntactically valid, but only accepts source
  types with the required conversion implementation. A coercion to a trait object
  is not automatically an `Into` implementation.
- `impl Trait` in function arguments and returns is supported on stable Rust.
  It cannot be used as an arbitrary struct field type; name the field type or
  make the struct generic.

## Sources

- [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html) — accepting Into and application/library tradeoffs
- [Standard library: From](https://doc.rust-lang.org/std/convert/trait.From.html)

## See Also

- [api-impl-asref](api-impl-asref.md) - borrow a representation instead of converting ownership
- [api-from-not-into](api-from-not-into.md) - prefer the From implementation direction
- [anti-over-abstraction](anti-over-abstraction.md) - add flexibility for concrete needs
