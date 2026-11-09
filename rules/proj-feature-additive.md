# proj-feature-additive

> Make reusable library features composable and declare their required capabilities

## Why It Matters

Cargo features can be unified when a dependency is shared, so another consumer may enable an additional feature. Removing APIs or changing a shared contract under that feature can break otherwise compatible consumers. Design library features to add capability and declare their dependencies rather than relying on callers to assemble a hidden combination.

## Bad

```toml
[features]
# This library feature removes baseline APIs when another crate enables it.
no_std = []
# Assumes every caller remembers to enable a separate required feature.
json = []
```

## Good

```toml
[features]
# For a library that supports no_std + alloc, std adds further capability.
default = ["std"]
std = []
json = ["std", "dep:serde", "dep:serde_json"]

[dependencies]
serde = { version = "1", optional = true, features = ["derive"] }
serde_json = { version = "1", optional = true }
```

For this library, place `#![cfg_attr(not(feature = "std"), no_std)]` at the crate
root. Import `alloc` explicitly when the baseline uses its collections:

```rust
extern crate alloc;
use alloc::vec::Vec;

// Available with or without the additive std feature.
pub fn empty_buffer() -> Vec<u8> {
    Vec::new()
}

fn main() {
    assert!(empty_buffer().is_empty());
}
```

The example requires allocation support; it does not claim to support targets
without an allocator. A library that needs `std` unconditionally does not need
to introduce a `no_std` mode.

## Feature Contracts

- Add items, implementations, or integrations without removing an existing
  promised contract. Recheck trait coherence and public type compatibility when
  feature combinations add implementations.
- A feature must enable the features and optional dependencies it needs.
  Weak dependency feature syntax such as `dependency?/feature` only forwards a
  feature when that dependency is already enabled.
- `default-features = false` on one dependency edge does not prevent another
  consumer from enabling defaults. Resolver versions isolate some build contexts;
  they do not make features exclusive configuration switches.
- Keep supported combinations building together on their applicable targets.
  Check the minimal configuration and meaningful combinations; an
  `--all-features` build alone does not exercise disabled-feature paths.
- Prefer backend selection through runtime configuration or distinct types when
  multiple backends can coexist. A documented `compile_error!` can diagnose a
  genuinely unsupported combination, but does not make it additive.
- Binary deployment choices may have narrower, explicit constraints than a
  reusable library. Do not silently impose those constraints on shared libraries.
- Use `dep:` to control optional dependency exposure deliberately.

## Sources

- [Microsoft: M-FEATURES-ADDITIVE](https://microsoft.github.io/rust-guidelines/guidelines/libs/building/index.html#M-FEATURES-ADDITIVE)
- [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Cargo feature resolver](https://doc.rust-lang.org/cargo/reference/resolver.html#features)

## See Also

- [api-serde-optional](api-serde-optional.md) - optional serialization integration
- [proj-workspace-deps](proj-workspace-deps.md) - share dependency definitions
- [lint-cfg-check](lint-cfg-check.md) - detect misspelled feature conditions
