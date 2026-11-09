# lint-clippy-nursery-selected

> Enable high-value `clippy::nursery` lints selectively, not the whole group

## Why It Matters

The `clippy::nursery` group contains lints that are correct and useful but still being refined — their suggestions may be noisy or have edge cases that haven't been polished yet. Enabling the entire group (`#![warn(clippy::nursery)]`) floods you with false positives and creates churn as nursery lints graduate or change. Cherry-picking individual lints gives you the signal without the noise. Several nursery lints are especially valuable: `significant_drop_tightening` catches lock guards held across `.await` or longer than necessary, `redundant_clone` flags clones that could be moves, and `use_self` keeps type names DRY inside impl blocks.

## Bad

```toml
# Cargo.toml — enables every nursery lint, including noisy ones
[lints.clippy]
nursery = "warn"
```

## Good

```toml
# Cargo.toml — selectively enable high-value nursery lints
[lints.clippy]
# Catches lock/guard held longer than necessary (overlaps with async issues)
significant_drop_tightening = "warn"
# Flags .clone() calls that could be avoided by moving
redundant_clone = "warn"
# Replace TypeName with Self inside impl blocks
use_self = "warn"
# Avoid redundant else after a diverging if
redundant_else = "warn"
# Prefer or_default() over or(Default::default())
or_fun_call = "warn"
```

<!-- verify: compile -->
```rust
use std::sync::Mutex;

// Keep the guard in a small scope before invoking unrelated work.
fn process(state: &Mutex<Vec<u32>>, after_read: impl FnOnce()) -> Result<usize, String> {
    let len = {
        let guard = state.lock().map_err(|error| error.to_string())?;
        guard.len()
    };
    after_read();
    Ok(len)
}

struct Counter {
    value: u32,
}

impl Counter {
    // use_self accepts Self in the return type and construction expression.
    fn new() -> Self {
        Self { value: 0 }
    }
}
```

## Suggested Starter Set

| Lint | What it catches |
|------|----------------|
| `significant_drop_tightening` | Guards/locks held longer than needed |
| `redundant_clone` | `.clone()` where a move suffices |
| `use_self` | Type name repeated inside `impl` block |
| `redundant_else` | `else` after diverging `if` branch |
| `or_fun_call` | `or(Default::default())` → `or_default()` |

Start with this set. Add more only after reviewing what they flag in your codebase.

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/Cargo.toml). workspace.lints.clippy selects use_self, redundant_clone, and or_fun_call individually; member manifests inherit them. The gate runs Clippy on all targets.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [lint-pedantic-selective](lint-pedantic-selective.md) - same strategy for clippy::pedantic
- [lint-warn-perf](lint-warn-perf.md) - enable the performance lint group
- [anti-lock-across-await](anti-lock-across-await.md) - don't hold locks across `.await`
