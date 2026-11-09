# api-extension-trait

> Use extension traits to add methods to external types

## Why It Matters

Rust's orphan rules prevent implementing external traits on external types. Extension traits provide a workaround: define a new trait with your methods, then implement it for the external type. This pattern is used extensively in the ecosystem (e.g., `itertools::Itertools`, `tokio::AsyncReadExt`).

## Bad

```rust
// Can't add methods directly to external types
impl Vec<u8> {
    fn as_hex(&self) -> String {
        // Error: cannot define inherent impl for a type outside this crate
    }
}

// Can't implement external trait for external type
impl SomeExternalTrait for Vec<u8> {
    // Error: orphan rules violation
}
```

## Good

```rust
// Define an extension trait
pub trait ByteSliceExt {
    fn as_hex(&self) -> String;
    fn is_ascii_printable(&self) -> bool;
}

// Implement for the external type
impl ByteSliceExt for [u8] {
    fn as_hex(&self) -> String {
        self.iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
    
    fn is_ascii_printable(&self) -> bool {
        self.iter().all(|b| b.is_ascii_graphic() || b.is_ascii_whitespace())
    }
}

// Usage: import the trait to use the methods
use my_crate::ByteSliceExt;

let data: &[u8] = b"hello";
println!("{}", data.as_hex());  // "68656c6c6f"
```

## Convention: Ext Suffix

```rust
// Standard naming: TypeExt for extending Type
pub trait OptionExt<T> {
    fn unwrap_or_log(self, msg: &str) -> Option<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn unwrap_or_log(self, msg: &str) -> Option<T> {
        if self.is_none() {
            log::warn!("{}", msg);
        }
        self
    }
}

// For generic extensions
pub trait ResultExt<T, E> {
    fn log_err(self) -> Self;
}

impl<T, E: std::fmt::Display> ResultExt<T, E> for Result<T, E> {
    fn log_err(self) -> Self {
        if let Err(ref e) = self {
            log::error!("{}", e);
        }
        self
    }
}
```

## Ecosystem Examples

```rust
// itertools::Itertools
use itertools::Itertools;
let groups = vec![1, 1, 2, 2, 3].into_iter().group_by(|x| *x);

// futures::StreamExt
use futures::StreamExt;
let next = stream.next().await;

// tokio::io::AsyncReadExt
use tokio::io::AsyncReadExt;
let mut buf = [0u8; 1024];
reader.read(&mut buf).await?;

// anyhow::Context
use anyhow::Context;
let content = std::fs::read_to_string(path)
    .with_context(|| format!("failed to read {}", path))?;
```

## Scoped Extensions

```rust
// Extension only visible where imported
mod string_utils {
    pub trait StringExt {
        /// Returns at most `max_chars` Unicode scalar values, including dots.
        fn truncate_ellipsis(&self, max_chars: usize) -> String;
    }
    
    impl StringExt for str {
        fn truncate_ellipsis(&self, max_chars: usize) -> String {
            if self.chars().nth(max_chars).is_none() {
                return self.to_owned();
            }

            let dots = max_chars.min(3);
            let prefix_chars = max_chars - dots;
            let end = self
                .char_indices()
                .nth(prefix_chars)
                .map_or(self.len(), |(offset, _)| offset);
            let mut result = self[..end].to_owned();
            result.push_str(&"..."[..dots]);
            result
        }
    }
}

// Only available when explicitly imported
use string_utils::StringExt;
fn main() {
    assert_eq!("abcdef".truncate_ellipsis(5), "ab...");
    assert_eq!("東京駅東京".truncate_ellipsis(4), "東...");
    assert_eq!("東京駅".truncate_ellipsis(3), "東京駅");
    assert_eq!("東京駅".truncate_ellipsis(2), "..");
    assert_eq!("text".truncate_ellipsis(0), "");
}
```

This contract counts Unicode scalar values, not bytes or grapheme clusters. The
owned return value can allocate, including when no truncation is needed. For
user-visible grapheme truncation, use a Unicode segmentation implementation.

Source: [Comprehensive Rust: String](https://google.github.io/comprehensive-rust/std-types/string.html).

## Generic Extensions with Bounds

```rust
pub trait VecExt<T> {
    fn push_if_unique(&mut self, item: T)
    where
        T: PartialEq;
}

impl<T> VecExt<T> for Vec<T> {
    fn push_if_unique(&mut self, item: T)
    where
        T: PartialEq,
    {
        if !self.contains(&item) {
            self.push(item);
        }
    }
}

// Works with any T: PartialEq
let mut v = vec![1, 2, 3];
v.push_if_unique(2);  // No-op
v.push_if_unique(4);  // Adds 4
```

## See Also

- [api-sealed-trait](./api-sealed-trait.md) - Controlling trait implementations
- [api-impl-into](./api-impl-into.md) - Using standard conversion traits
- [name-as-free](./name-as-free.md) - Naming conventions for conversions
- [trait-blanket-impl](./trait-blanket-impl.md) - Blanket impls for extension traits
- [type-utf8-boundaries](type-utf8-boundaries.md) - distinguish bytes, scalars, and grapheme clusters
