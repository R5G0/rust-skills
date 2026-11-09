# lint-warnings-deny-config

> Use Cargo `build.warnings` for a warning-free CI policy without changing compiler flags

## Why It Matters

Changing `RUSTFLAGS` changes Cargo's compiler inputs and can trigger unnecessary rebuilds when switching between local and CI warning policies. Cargo 1.97 stabilized `build.warnings`, which makes warnings from local packages fail the command without changing those compiler flags. Clippy also supports this setting; enable the desired lint groups separately.

## Bad

```bash
# Applies to every rustc invocation and invalidates cached artifacts
export RUSTFLAGS="-Dwarnings"
cargo test --workspace
```

```yaml
# .github/workflows/ci.yml
- name: Build
  env:
    RUSTFLAGS: "-Dwarnings"
  run: cargo test --workspace
```

## Good

```toml
# .cargo/config.toml — Cargo 1.97+
[build]
warnings = "deny"
```

```yaml
# .github/workflows/ci.yml
- name: Build
  run: cargo test --workspace

- name: Clippy
  run: cargo clippy --workspace --all-targets
```

## Notes

- `build.warnings` accepts `"warn"` (default), `"deny"`, and `"allow"`. Use `"deny"` in CI so new rustc warnings fail the build.
- The setting changes the treatment of emitted warnings, not which allow-by-default lints are enabled. Keep selecting appropriate lints in `[workspace.lints]`.
- Prefer a checked-in `.cargo/config.toml` (or a CI `--config 'build.warnings="deny"'` override) over a global `RUSTFLAGS` export.
- On older Cargo versions, `cargo clippy -- -D warnings` remains a supported fallback. Do not raise a project's MSRV solely to change warning policy.

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/.cargo/config.toml). The fixture sets build.warnings = "deny". Its checks run from this directory so Cargo actually discovers that configuration; both rustc and Clippy commands must finish without emitted warnings.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [lint-workspace-lints](lint-workspace-lints.md) - configure rustc and Clippy lints in Cargo.toml
- [lint-deny-correctness](lint-deny-correctness.md) - deny Clippy correctness lints
- [lint-cfg-check](lint-cfg-check.md) - catch cfg typos before they become silent dead code

## Sources

- [Cargo build.warnings](https://doc.rust-lang.org/cargo/reference/config.html#buildwarnings)
- [Clippy warning policy](https://doc.rust-lang.org/clippy/usage.html#elevating-warning-to-errors)
