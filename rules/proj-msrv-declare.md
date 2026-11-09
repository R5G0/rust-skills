# proj-msrv-declare

> Declare `rust-version` (MSRV) in Cargo.toml and test it in CI

## Why It Matters

Setting `package.rust-version` declares the minimum compiler supported by your crate. Resolver 3 defaults to preferring dependencies compatible with declared Rust versions, but this is a best-effort preference: incompatible versions can still be selected if no compatible candidate fits. Check the actual supported feature combinations with the minimum toolchain. Edition 2024 itself requires at least Rust 1.85.

## Bad

```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"
# no rust-version — users get cryptic errors on old toolchains,
# and nothing prevents a dep bump from silently raising the floor
```

## Good

```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"  # edition 2024 minimum; raise if used APIs require it

[workspace]
resolver = "3"  # default for edition 2024; enables MSRV-aware dep resolution
```

CI job pinning the MSRV toolchain (GitHub Actions example):

```yaml
# .github/workflows/msrv.yml
- name: Install MSRV toolchain
  uses: dtolnay/rust-toolchain@master
  with:
    toolchain: "1.85"

- name: Check MSRV
  run: cargo check --all-features
```

## Choosing and Maintaining MSRV

- Pick the oldest stable toolchain your users are realistically running (check distro packages, embedded targets, corporate freeze windows).
- Do not promise an older compiler than you can support: a lower MSRV narrows the set of MSRV-compatible dependency releases and adds maintenance work. Evaluate required security and correctness fixes when choosing that floor.
- Announce MSRV changes and follow the library's published compatibility policy. Microsoft's guideline permits minor-version bumps; a stricter existing support contract takes precedence. Do not raise MSRV merely to use the newest syntax.
- Run `cargo msrv` (the `cargo-msrv` tool) to find the actual floor automatically.

## See Also

Sources: [Microsoft M-MSRV](https://microsoft.github.io/rust-guidelines/guidelines/project/index.html#M-MSRV), [Cargo Rust-version resolution](https://doc.rust-lang.org/cargo/reference/resolver.html#rust-version).

- [proj-workspace-deps](proj-workspace-deps.md) - use workspace dependency inheritance
- [lint-cargo-metadata](lint-cargo-metadata.md) - warn on missing Cargo.toml metadata
- [doc-cargo-metadata](doc-cargo-metadata.md) - fill Cargo.toml metadata fields
