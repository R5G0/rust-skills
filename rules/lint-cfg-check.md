# lint-cfg-check

> Enable `unexpected_cfgs` and declare known cfgs to catch feature-gate typos

## Why It Matters

A typo in a cfg expression — `#[cfg(feature = "serde_")]` instead of `"serde"`, or an undeclared custom cfg — can silently disable code when cfg checking is absent or its diagnostics are overlooked. The `unexpected_cfgs` lint (stabilized in Rust 1.80) flags names and values not known to the compiler. Cargo automatically registers declared feature names; declare custom names such as `tokio_unstable` via `check-cfg` in `[lints.rust]`, and make the resulting diagnostics fail CI.

## Bad

```rust
// Typo: "serde_" will never match the "serde" feature.
// Without enforcing cfg diagnostics, this disabled code can go unnoticed.
#[cfg(feature = "serde_")]
#[derive(serde::Serialize)]
pub struct MyType { pub name: String }

// Custom cfg used without declaration: Cargo reports unexpected_cfgs.
#[cfg(tokio_unstable)]
pub fn experimental() {}
```

## Good

```toml
# Cargo.toml — declare the feature, optional dependency, and custom cfg names
[dependencies]
serde = { version = "1", optional = true, features = ["derive"] }

[features]
serde = ["dep:serde"]

[lints.rust]
unexpected_cfgs = { level = "deny", check-cfg = [
    'cfg(tokio_unstable)',
    'cfg(coverage_nightly)',
] }
```

```rust
// The struct is checked in every build; optional derives are checked when enabled.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MyType { pub name: String }

#[cfg(tokio_unstable)]
pub fn experimental() -> MyType {
    MyType { name: "experimental".to_owned() }
}
```

## Workspace Projects

Configure the lint once at the workspace level and inherit it in member crates:

```toml
# Cargo.toml (workspace root)
[workspace.lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = [
    'cfg(tokio_unstable)',
] }

# member/Cargo.toml
[lints]
workspace = true
```

## Notes

- Feature names (`feature = "serde"`, `feature = "std"`) are registered automatically by Cargo — you do not need to list them in `check-cfg`.
- Declaring a custom cfg name permits it; it does not enable that branch. Enable it explicitly when testing, and compile both enabled and disabled configurations. An erased branch is not evidence that its body compiles.
- Values in `check-cfg` entries must be quoted cfg expressions: `'cfg(name)'` or `'cfg(name, values("v1", "v2"))'`.
- The lint fires at `warn` level; promote to `deny` in CI, or set `[build] warnings = "deny"` (Cargo 1.97+), if you want hard failures.

## Verified Project

<!-- verify: fixture=cfg-check -->

The [cfg-check fixture](../checks/fixtures/cfg-check/Cargo.toml) declares a `checked` feature and the custom `validation_mode` cfg with `unexpected_cfgs = "deny"`. Its [verifier](../checks/fixtures/cfg-check/verify_cfg.py) runs real tests in four configurations: default, all features, custom cfg enabled, and both enabled. It requires the branch-specific test names to execute, then compiles separate misspelled-feature and misspelled-custom-cfg callers and checks the exact `unexpected_cfgs` diagnostic and offending source line. The fixture uses no external dependencies; the serde example above illustrates a separate optional integration.

Run `python checks/fixtures/cfg-check/verify_cfg.py` with Rust 1.97.1 installed. Results are written under the fixture's ignored `target/` directory.

## See Also

- [lint-workspace-lints](lint-workspace-lints.md) - configure lints at workspace level
- [lint-warnings-deny-config](lint-warnings-deny-config.md) - deny emitted warnings in CI via Cargo config
- [proj-cfg-select](proj-cfg-select.md) - select one conditional item or expression in one place
- [proj-feature-additive](proj-feature-additive.md) - design features to be strictly additive
- [lint-warn-suspicious](lint-warn-suspicious.md) - enable suspicious lint group
