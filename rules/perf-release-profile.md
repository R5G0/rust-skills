# perf-release-profile

> Optimize release profile settings

## Why It Matters

Cargo's release defaults are already optimized. LTO and codegen settings can change runtime performance, binary size, and build cost, but benefits depend on the workload and compiler. Measure candidate settings rather than assuming a fixed speedup.

## Default Profile

```toml
[profile.release]
opt-level = 3
debug = false
lto = false
codegen-units = 16
```

## Candidate Profile

```toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = "fat"            # Full link-time optimization
codegen-units = 1      # Better optimization, slower compile
panic = "abort"        # Changes failure behavior: no unwinding or panic cleanup
strip = true           # Remove symbols

[profile.release.package."*"]
# Keep dependencies optimized even if main crate changes
opt-level = 3
```

## Profile Options

| Option | Values | Effect |
|--------|--------|--------|
| `opt-level` | 0-3, "s", "z" | Optimization level |
| `lto` | false, "thin", "fat" | Link-time optimization |
| `codegen-units` | 1-256 | Parallel compilation units |
| `panic` | "unwind", "abort" | Panic behavior |
| `strip` | true, false, "symbols", "debuginfo" | Binary stripping |
| `debug` | true, false, 0-2 | Debug info level |

## Optimization Levels

| Level | Description | Use Case |
|-------|-------------|----------|
| `0` | No optimization | Debug builds |
| `1` | Basic optimization | Fast compile |
| `2` | Most optimizations | Balanced |
| `3` | All optimizations | Maximum performance |
| `"s"` | Optimize for size | Embedded |
| `"z"` | Minimize size | Smallest binary |

## LTO Options

| Option | Compile Time | Performance | Binary Size |
|--------|--------------|-------------|-------------|
| `false` | Baseline; local thin LTO may occur | Measure | Measure |
| `"thin"` | Can add link work | Measure | Measure |
| `"fat"` | Can add more link work | Measure; not necessarily best | Measure |

## Custom Profiles

```toml
# Fast release builds for development
[profile.release-dev]
inherits = "release"
lto = false
codegen-units = 16

# Maximum performance for production
[profile.release-prod]
inherits = "release"
lto = "fat"
codegen-units = 1
strip = true

# Profiling with symbols
[profile.profiling]
inherits = "release"
debug = true
strip = false
```

Use with: `cargo build --profile release-prod`

## Dev Dependencies Optimization

Speed up tests and dev builds:

```toml
[profile.dev]
opt-level = 0

# Optimize dependencies even in dev
[profile.dev.package."*"]
opt-level = 3
```

## Benchmarking Profile

```toml
[profile.bench]
inherits = "release"
debug = true      # For profiling
strip = false     # Keep symbols for flamegraphs
lto = "fat"       # Consistent with release-prod
```

## Size vs Speed Trade-offs

```toml
# Smallest binary
[profile.min-size]
inherits = "release"
opt-level = "z"
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true

# Balance size and speed
[profile.balanced]
inherits = "release"
opt-level = "s"
lto = "thin"
```

## Workspace Configuration

```toml
# In workspace Cargo.toml
[profile.release]
lto = "fat"
codegen-units = 1

# Override for specific package
[profile.release.package.fast-compile-lib]
codegen-units = 16
```

`lto`, `panic`, and `rpath` cannot be set in per-package profile overrides. Set those at the profile level instead. `lto = false` permits thin local LTO in eligible builds; `lto = "off"` disables LTO.

## Verified Project

<!-- verify: fixture=performance -->

The [formal validation project](../perf-validation/Cargo.toml) builds and executes default, thin-LTO, single-codegen-unit, and combined release profiles. It compares checksums and verifies destructor behavior during a process-level panic. Standard `cargo test --release` uses the test harness's unwind strategy and does not prove an aborting production binary's behavior. [Recorded measurements](../perf-validation/results/summary.md) are local observations, not universal speed or size rankings.

## See Also

- [opt-lto-release](./opt-lto-release.md) - LTO details
- [opt-codegen-units](./opt-codegen-units.md) - Codegen units
- [opt-pgo-profile](./opt-pgo-profile.md) - Profile-guided optimization
