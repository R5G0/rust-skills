# type-named-fields

> Use named fields for values whose positions would obscure their meaning

## Why It Matters

A tuple of similar primitive values can make argument order and field access ambiguous. Named fields make the domain meaning visible at construction and use sites. Small local tuples, standard tuple conventions, and single-field newtypes remain useful when their meaning is clear.

## Bad

```rust
// Which value is the retry limit, and which is the timeout in milliseconds?
fn connection_options() -> (u32, u32) {
    (3, 500)
}

fn main() {
    let options = connection_options();
    println!("timeout: {}", options.0); // Compiles, but reads the retry limit.
}
```

## Good

```rust
use std::time::Duration;

struct ConnectionOptions {
    retry_limit: u32,
    timeout: Duration,
}

fn connection_options() -> ConnectionOptions {
    ConnectionOptions {
        retry_limit: 3,
        timeout: Duration::from_millis(500),
    }
}

fn main() {
    let options = connection_options();
    assert_eq!(options.retry_limit, 3);
    assert_eq!(options.timeout, Duration::from_millis(500));
}
```

## Scope the Recommendation

A pair immediately destructured into clear local names may be simpler than a new
struct. Iterator pairs such as `(index, item)` from `enumerate` already have a
well-known meaning. Single-field tuple structs remain a natural representation
for newtypes; use them to prevent semantically different identifiers from mixing.

## Sources

- [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html) — named fields, local tuples, and newtypes

## See Also

- [api-newtype-safety](api-newtype-safety.md) - distinguish semantically different values
- [type-newtype-ids](type-newtype-ids.md) - keep resource identities separate
- [api-builder-pattern](api-builder-pattern.md) - construct values with many independent options
