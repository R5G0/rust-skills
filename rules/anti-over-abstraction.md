# anti-over-abstraction

> Introduce traits and generics for a real abstraction boundary rather than speculative flexibility

## Why It Matters

Traits support polymorphism, generic algorithms, and extension methods, but also create contracts that must be understood and maintained. An inherent implementation is often clearer for behavior belonging to one concrete type. Choose an abstraction because it serves callers or separates a real boundary, not to meet an arbitrary count of implementations.

## Bad

```rust
struct Counter(u32);

// No callers need a trait bound, trait object, or extension method here.
trait CounterOperations {
    fn value(&self) -> u32;
}

impl CounterOperations for Counter {
    fn value(&self) -> u32 {
        self.0
    }
}

fn main() {
    let counter = Counter(3);
    assert_eq!(counter.value(), 3);
}
```

## Good

```rust
struct Counter(u32);

impl Counter {
    fn value(&self) -> u32 {
        self.0
    }
}

fn main() {
    let counter = Counter(3);
    assert_eq!(counter.value(), 3);
}
```

A library algorithm can justify a trait bound directly through what it does:

```rust
use std::io::{self, Read, Write};

fn copy_header(input: impl Read, mut output: impl Write) -> io::Result<u64> {
    io::copy(&mut input.take(8), &mut output)
}

fn main() -> io::Result<()> {
    let mut output = Vec::new();
    let copied = copy_header(&b"header12body"[..], &mut output)?;
    assert_eq!(copied, 8);
    assert_eq!(output, b"header12");
    Ok(())
}
```

## Decide from the Boundary

Traits are useful for generic algorithms, dynamic dispatch, extension methods,
and substitutable implementations at an actual integration or testing boundary.
One current implementation can still justify a trait when the caller needs one
of those capabilities. Marker traits and associated-type contracts can also be
intentional, even without methods.

Repeated concrete implementations can reveal an abstraction, but neither two
nor three implementations is a prerequisite. Similar-looking code need not
share a domain contract. Avoid traits used only to divide one type's inherent
methods into arbitrary groups.

Reusable libraries may anticipate credible caller needs. Application and private
code can usually stay concrete until broader flexibility improves an actual use
case. A public function alone does not require a generic parameter, and private
code can still legitimately need polymorphism.

## Sources

- [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html) — API context and trait versus inherent implementation
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) — deliberate public contracts

## See Also

- [type-generic-bounds](type-generic-bounds.md) - require only capabilities an algorithm uses
- [api-extension-trait](api-extension-trait.md) - add methods to types from another crate
- [trait-dyn-vs-generic](trait-dyn-vs-generic.md) - choose the necessary dispatch mechanism
