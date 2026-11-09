# opt-lto-release

> Measure LTO in release builds before enabling it

## Why It Matters

Link-Time Optimization (LTO) permits additional optimization across crate boundaries. Its effect on runtime, binary size, and build cost depends on the program and compiler; benchmark the candidate rather than assuming a percentage improvement.

## Bad

```toml
# Cargo.toml - default release profile
[profile.release]
opt-level = 3
# No LTO = missed optimization opportunities
```

## Good

```toml
# Cargo.toml - optimized release profile
[profile.release]
opt-level = 3
lto = "fat"          # Maximum optimization
codegen-units = 1    # Better optimization (single codegen unit)
panic = "abort"      # Smaller binary, no unwind tables
strip = true         # Remove symbols for smaller binary
```

## LTO Options Explained

```toml
# Default: eligible builds can use thin local LTO within one crate.
lto = false

# Thin LTO - fast compilation, most benefits
lto = "thin"

# Fat LTO - slowest compilation, maximum optimization
lto = "fat"
# Equivalent to:
lto = true

# Disable LTO, including thin local LTO.
lto = "off"
```

## Trade-offs

| Setting | Compile Time | Binary Size | Performance |
|---------|--------------|-------------|-------------|
| `lto = false` | Baseline | Measure | Measure |
| `lto = "thin"` | Can increase link work | Measure | Measure |
| `lto = "fat"` | Can increase link work further | Measure | Measure; not necessarily fastest |

## Evidence from Production

Many production crates enable fat LTO and `codegen-units = 1` in their release
profiles for maximum performance. For example, ripgrep ships a `release-lto`
profile (see the Cargo Book for profile documentation:
<https://doc.rust-lang.org/cargo/reference/profiles.html>):

```toml
# A common pattern in performance-critical crates
[profile.release]
overflow-checks = true
lto = "fat"
codegen-units = 1

# Named profile for explicit LTO builds (e.g. ripgrep's release-lto)
[profile.release-lto]
inherits = "release"
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = "symbols"
```

## Complete Optimized Profile

```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = "fat"          # Link-time optimization
codegen-units = 1    # Single codegen unit for better optimization
panic = "abort"      # Remove panic unwinding code
strip = true         # Strip symbols
debug = false        # No debug info

# For benchmarking (need some debug info for profiling)
[profile.bench]
inherits = "release"
debug = true
strip = false

# Fast dev builds with optimized dependencies
[profile.dev]
opt-level = 0
debug = true

[profile.dev.package."*"]
opt-level = 3        # Optimize dependencies even in dev
```

## When to Use Each

| Situation | LTO Setting |
|-----------|-------------|
| Development | `false` (fast compiles) |
| CI builds | `"thin"` (balance) |
| Release binaries | `"fat"` (max perf) |
| Libraries (crates.io) | `false` (users choose) |

## Measuring Impact

```bash
# Build without LTO
cargo build --release
hyperfine ./target/release/myapp

# Build with LTO
# (after adding lto = "fat" to Cargo.toml)
cargo build --release
hyperfine ./target/release/myapp

# Compare binary sizes
ls -la target/release/myapp
```

## Verified Project

<!-- verify: fixture=optimization -->

The [optimization fixture](../checks/fixtures/optimization/Cargo.toml) defines a valid thin-LTO profile. The [performance project's profile matrix](../perf-validation/README.md) executes separately built LTO candidates and compares their outputs. Per-package profile overrides cannot set `lto`; see [Cargo's profile reference](https://doc.rust-lang.org/cargo/reference/profiles.html#lto).

## See Also

- [opt-codegen-units](opt-codegen-units.md) - Use codegen-units = 1
- [opt-pgo-profile](opt-pgo-profile.md) - Profile-guided optimization
- [perf-release-profile](perf-release-profile.md) - Full release profile settings
