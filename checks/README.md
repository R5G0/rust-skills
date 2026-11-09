# Rule validation

The pinned Rust **1.97.1**, edition 2024 validation has two separate obligations:

1. Extract illustrative Rust blocks and reject real type/syntax errors.
   `baseline.txt` has **no accepted SUSPECT diagnostics**.
2. Require at least one compiled positive example or a successfully executed
   real Cargo fixture for **every rule**. Comments, missing-symbol fragments,
   and disabled test bodies cannot satisfy this obligation.

## Run

```sh
rustup component add rustfmt clippy llvm-tools-preview --toolchain 1.97.1
bash checks/check.sh
```

This is also the CI entry point. On Windows without Bash, run these commands
in order from `checks/`:

```powershell
python validate.py
python gen_index.py --check
python gen.py
cargo +1.97.1 check --locked --examples --target x86_64-unknown-linux-gnu --keep-going --message-format=json > check.json
python analyze.py check.json --check-baseline baseline.txt
python verify_rules.py check.json
```

Cargo's extraction check can return nonzero for context-dependent snippets; the
following gates decide the result and reject incomplete output. Install the
cross-check target with `rustup target add x86_64-unknown-linux-gnu --toolchain
1.97.1` when necessary. Native fixtures and test harnesses run on the host.

## Evidence and scope

`gen.py` writes exact candidate blocks to Cargo examples. The manifest binds
them to source document hashes and lines. `verify_rules.py` rejects stale source
or compiler output, and writes [`verification.json`](verification.json) with
per-rule evidence and every non-compiling extracted block. Missing coverage,
failing marked positives, and failing fixtures/tests fail the gate. Selected
test examples run with the actual Rust test harness, including namespaced and
parameterized test attributes. An expected test-bearing example running zero
tests fails.

Rules can mark a complete primary block with `<!-- verify: compile -->` or name
a curated project with `<!-- verify: fixture=layouts -->`. Unmarked rules still
need an actual independently compiling example. Fixture markers execute only
projects explicitly listed in the verifier, not arbitrary Markdown commands.

| Fixture | Checks |
|---|---|
| [`layouts`](fixtures/layouts/Cargo.toml) | Workspaces, modules, bins, integration tests, doctests, documentation links, formatting, Clippy |
| [`proc-macros`](fixtures/proc-macros/Cargo.toml) | Real derive/facade/consumer crates, generics and `$crate` paths, invalid-input diagnostics and primary source spans |
| [`optimization`](fixtures/optimization/Cargo.toml) | Cold paths, actual LTO/codegen builds, instrument/train/merge/profile-use PGO workflow and checked output |
| [`cfg-check`](fixtures/cfg-check/Cargo.toml) | Features/custom cfg, enabled paths and rejection of a misspelled cfg |
| [`performance`](../perf-validation/Cargo.toml) | Tests and benchmark execution for all 13 performance rules; measurements reported separately |

This minimum gate does not claim that every supplementary snippet is standalone
or that every algorithm is formally proved correct. Secondary snippets can
depend on application types/helpers; those errors remain visible in the report.
Intentional invalid examples use `compile_fail`; signature catalogs, directory
layouts, and pseudocode are labeled as such. Nightly, platform-specific, FFI,
Miri, and optional-feature claims need their corresponding environment. A stable
compile result does not establish those stronger properties.

The classifier separates context/name-resolution fragments, extraction artifacts,
and inference-only cases. Unstable-feature errors are not name-resolution errors.
No real-error allowlist is used to make rule coverage pass.

Generated examples, logs and build directories are ignored. Source programs,
lockfiles, the verifier and latest per-rule evidence are tracked. Regenerate
evidence with the pinned toolchain after changing rules or fixtures.
