# api-public-dependencies

> Treat third-party types in public APIs as deliberate compatibility commitments

## Why It Matters

A dependency type in a public signature, trait bound, field, or implementation becomes part of the caller's contract. Updating that dependency can change type identity or impose compatibility work on downstream users. Prefer standard-library or crate-owned types when they express the contract well, and expose ecosystem types when their interoperability or performance benefits justify the coupling.

## Bad

```rust
// An internal client upgrade can now change the public API's types.
pub fn download(url: &str) -> Result<http::Response<bytes::Bytes>, client_impl::Error> {
    client_impl::download(url)
}

// Re-exporting those types does not remove the dependency commitment.
pub use client_impl::Error;
```

## Good

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    body: Vec<u8>,
}

impl Message {
    pub fn new(body: Vec<u8>) -> Self {
        Self { body }
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn into_body(self) -> Vec<u8> {
        self.body
    }
}

fn main() {
    let message = Message::new(vec![1, 2, 3]);
    assert_eq!(message.body(), &[1, 2, 3]);
    assert_eq!(message.into_body(), vec![1, 2, 3]);
}
```

The wrapper is useful only if it creates a meaningful API boundary. Do not add
copies or a parallel type system merely to hide an established ecosystem type.

## Review Public Dependency Exposure

- Inspect signatures, generic bounds, associated types, trait implementations,
  and re-exports. A private field can still create observable auto-trait or
  layout commitments through its containing public type.
- An ecosystem protocol type, collection, or integration trait can be the right
  public contract. Document that decision and its supported dependency versions.
- Two incompatible versions of a dependency can define distinct Rust types with
  the same spelling. A re-export provides an access path, not a conversion.
- Feature-gated integrations still become public contracts when enabled.
- Re-export own implementation modules deliberately. Re-exporting a dependency
  can be appropriate for an umbrella crate, an intentional facade, or paths used
  by exported macros; it should not happen just to save a caller's import.
- Before upgrading or replacing a public dependency, check downstream
  compatibility rather than assuming unchanged local code proves compatibility.

## Sources

- [Microsoft: M-DONT-LEAK-TYPES and M-FOREIGN-REEXPORTS](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-DONT-LEAK-TYPES)
- [Cargo SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)

## See Also

- [proj-pub-use-reexport](proj-pub-use-reexport.md) - choose deliberate public paths
- [api-serde-optional](api-serde-optional.md) - optional integration contracts
- [api-non-exhaustive](api-non-exhaustive.md) - leave room for public type evolution
