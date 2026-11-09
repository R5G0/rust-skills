# const-vs-static

> Use `const` for an inlined value and `static` for a single addressed instance

## Why It Matters

`const` defines a compile-time value without promising one unique address; its uses can still occupy storage, and the compiler can merge or duplicate that storage. `static` defines one addressed instance per linked definition. Choose according to value semantics and identity requirements, not an assumed universal allocation saving. Mutable shared state needs synchronization; LazyLock and OnceLock control initialization, not the consistency of every later operation.

## Bad

```rust
// large table as `const` — potentially duplicated at every use site
const LOOKUP: [u8; 256] = [0u8; 256];

// A static mut declaration is allowed, but safe concurrent access is not.
// References to static mut are denied by default in edition 2024.
static mut COUNTER: u64 = 0;

// `static` for a tiny value — needlessly takes an address
static TIMEOUT_MS: u64 = 5000;
```

## Good

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, OnceLock};

// small value: const expresses a value without a unique-address contract
const MAX_RETRIES: u32 = 3;
const TIMEOUT_MS: u64 = 5_000;
const FLAG_MASK: u8 = 0b0000_1111;

// large data: `static` — one copy in the binary, shareable as `&'static`
static LOOKUP: [u8; 256] = [0u8; 256];

fn process(byte: u8) -> u8 {
    LOOKUP[byte as usize]
}

// String literals and promoted constants can also provide static lifetimes.
static APP_NAME: &str = "my-app";

// mutable global state — use atomics, not `static mut`
static REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);

fn record_request() {
    REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
}

// lazily initialized global — `LazyLock` (stable since 1.80)
static CONFIG_PATH: LazyLock<String> = LazyLock::new(|| {
    std::env::var("CONFIG_PATH").unwrap_or_else(|_| "/etc/app/config.toml".to_owned())
});

// single-assignment global — `OnceLock`
static GREETING: OnceLock<String> = OnceLock::new();

fn set_greeting(name: &str) {
    let _ = GREETING.set(format!("hello, {name}"));
}
```

## When to Prefer Which

| Situation | Use |
|-----------|-----|
| Small constant (number, bool, tiny array) | `const` |
| String literal | `const` or `static` (both work; prefer `const`) |
| Large lookup table | `static` |
| Need one stable addressed instance | `static` |
| Mutable counter / flag | `static AtomicXxx` |
| Lazily initialized value | `static LazyLock<T>` |
| Single-writer initialization | `static OnceLock<T>` |

## Policy Values and Shared Identity

Name non-obvious policy and protocol constants with their units and purpose. Explain why a timeout, capacity, or limit was chosen and what external contract depends on it; repeating the numeric value in a comment adds little.

A static in one library instance is not an authority shared across separately linked versions or independently loaded plugins. Pass an explicitly owned registry or runtime context when consumers must agree on one identity. Immutable lookup tables and caches whose duplication affects only performance can still use statics.

Sources: [Microsoft M-DOCUMENTED-MAGIC](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-DOCUMENTED-MAGIC), [M-AVOID-STATICS](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-AVOID-STATICS), [Rust Reference: constant items](https://doc.rust-lang.org/reference/items/constant-items.html).

## See Also

- [name-consts-screaming](name-consts-screaming.md) - naming `SCREAMING_SNAKE_CASE` for `const` and `static`
- [own-mutex-interior](own-mutex-interior.md) - use `Mutex<T>` for interior mutability in multi-threaded code
- [api-explicit-state](api-explicit-state.md) - give correctness-sensitive state an explicit owner
