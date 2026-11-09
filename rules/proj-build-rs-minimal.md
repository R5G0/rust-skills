# proj-build-rs-minimal

> Keep build scripts deterministic, track their inputs, and make packaged prerequisites explicit

## Why It Matters

Cargo caches build-script results and reruns scripts when their tracked inputs change. A script that depends on untracked files, the author's checkout, ambient tools, or unpinned downloads can fail for downstream users or produce stale output. Minimize build-time work and verify the published package's build contract on its supported targets.

## Bad

```rust
use std::process::Command;

fn main() {
    // Assumes an undocumented tool and an untracked checkout file exist.
    let output = Command::new("protoc")
        .arg("../private-schemas/message.proto")
        .output()
        .expect("failed to run generator");
    assert!(output.status.success());
}
```

## Good

```rust
// build.rs: deterministic output from an included package input.
use std::{env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=schema/version.txt");

    let version = fs::read_to_string("schema/version.txt")?;
    let output_dir = PathBuf::from(env::var_os("OUT_DIR").ok_or("missing cargo OUT_DIR")?);
    fs::write(output_dir.join("version.txt"), version.trim())?;
    Ok(())
}
```

The manifest must include `schema/version.txt` in the packaged crate. If this is
simply static input, prefer `include_str!` or an ordinary constant and omit the
build script altogether.

## Rerun and Output Contracts

- Without explicit rerun directives, Cargo scans the package's files for changes;
  it does **not** necessarily execute the script on every build.
- Track every file, directory, and environment input actually read. Missing
  inputs can leave stale output; broad inputs cause unnecessary rebuilds.
- Emit `cargo::rerun-if-env-changed=NAME` for relevant environment configuration.
  The `cargo::` spelling requires Rust/Cargo 1.77; use `cargo:` for an older MSRV.
- Write generated output under `OUT_DIR`. Do not modify the source checkout or
  depend on files omitted by `cargo package`.
- Distinguish host tools from target artifacts when cross-compiling. Use Cargo's
  target information rather than the build script's own host `cfg` values.
- Use documented compiler capabilities or a focused probe when necessary,
  rather than substring matching a compiler's version output.

## Published and Native Dependencies

Pregenerate bindings or protocol code during release preparation when practical,
so downstream builds do not need the generator. Inspect packaged files and build
the package in an environment that lacks the author's untracked files and tools.

Native libraries may legitimately need a C toolchain, system library, or
platform SDK. Document those prerequisites and supported targets, gate optional
platform integrations, and prefer build paths that work with the intended
toolchain. Do not promise every Tier 1 target for a deliberately platform-specific
crate.

Avoid build-time downloads in the default path. If an explicit integration
requires them, pin and verify content, support a local source override for
hermetic/offline builds, and report failures clearly. An HTTPS URL alone does not
make downloaded build inputs reproducible.

## Sources

- [Microsoft: M-OOBE and M-SYS-CRATES](https://microsoft.github.io/rust-guidelines/guidelines/libs/building/index.html#M-OOBE)
- [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
- [Cargo package](https://doc.rust-lang.org/cargo/commands/cargo-package.html)

## See Also

- [proj-feature-additive](proj-feature-additive.md) - keep optional integrations composable
- [lint-cfg-check](lint-cfg-check.md) - declare custom configuration names
- [proj-msrv-declare](proj-msrv-declare.md) - preserve the supported toolchain contract
