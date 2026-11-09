# api-explicit-state

> Give correctness-sensitive shared state an explicit owner and identity scope

## Why It Matters

Cargo can link multiple versions of a library, each with its own statics. Independently loaded libraries may also have separate runtime and thread-local state. When several consumers must share one registry or counter for correctness, a hidden global inside a reusable library does not establish that identity contract.

## Bad

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

// A separate linked version of this library has a separate counter.
pub fn process_wide_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
```

The name promises process-wide uniqueness that the library cannot provide across versions; wrapping at the integer limit would also repeat identifiers.

## Good

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Registry {
    next_id: AtomicU64,
}

impl Registry {
    pub fn new() -> Self {
        Self { next_id: AtomicU64::new(0) }
    }

    pub fn next_id(&self) -> Option<u64> {
        self.next_id
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .ok()
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Client {
    registry: Arc<Registry>,
}

impl Client {
    pub fn new(registry: Arc<Registry>) -> Self {
        Self { registry }
    }

    pub fn next_id(&self) -> Option<u64> {
        self.registry.next_id()
    }
}
```

The composition root creates one Registry and shares that instance among consumers that require the same ID scope. Separate registries intentionally create separate scopes; persistence or distributed uniqueness needs an appropriate external authority.

## Key Points

- State the scope: object, thread, runtime, process, deployment, or persistent namespace. Choose ownership and storage that actually enforce it.
- Do not create a fresh registry for each consumer while promising shared identity. Across incompatible crate versions or FFI, use a host-owned protocol instead of assuming their Rust types are interchangeable.
- Constants, immutable lookup tables, local caches, and deliberately application-owned singletons can remain static when duplicated state does not violate their contract.
- LazyLock and OnceLock control initialization of one static instance; they do not merge separately linked crate versions.
- Keep reusable state initialization explicit enough to configure, isolate, and test it. Avoid adding a factory or trait when passing a concrete state object suffices.
- Atomic arithmetic needs an exhaustion policy; Relaxed ordering is enough here because the counter does not publish other data.

## See Also

- [const-vs-static](const-vs-static.md) - distinguish constant values from addressed static instances
- [own-arc-shared](own-arc-shared.md) - share one actual instance when required
- [unsafe-ffi-boundary](unsafe-ffi-boundary.md) - keep cross-library state behind its owning protocol
- [conc-atomic-update](conc-atomic-update.md) - update counters with an explicit failure path

## Sources

- [Microsoft M-AVOID-STATICS](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-AVOID-STATICS)
- [Cargo version compatibility](https://doc.rust-lang.org/cargo/reference/resolver.html#version-incompatibility-hazards)
