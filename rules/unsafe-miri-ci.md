# unsafe-miri-ci

> Run `cargo miri test` in CI for every crate that contains `unsafe` code.

## Why It Matters

Miri interprets Rust programs and can detect undefined behavior on supported executed paths, including invalid memory accesses, uninitialized reads, pointer misuse, and data races. Its checks complement code review and ordinary tests.

A successful run is evidence about the inputs, execution, target, and toolchain tested; it does not prove soundness or explore every possible thread interleaving. Safe application code can also exercise unsafe implementations in dependencies, so the absence of local `unsafe` blocks alone does not establish that Miri has no value. See the [Miri project's documented limitations](https://github.com/rust-lang/miri#readme).

## Bad

```yaml
# CI that tests but never runs Miri — unsafe code ships unverified.
- name: Test
  run: cargo test --all-features
```

## Good

```yaml
# .github/workflows/miri.yml
name: Miri

on: [push, pull_request]

jobs:
  miri:
    name: Miri (nightly)
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install nightly toolchain with Miri
        run: |
          rustup toolchain install nightly --component miri
          rustup override set nightly
          cargo miri setup

      - name: Run Miri
        env:
          MIRIFLAGS: "-Zmiri-strict-provenance"
        run: cargo miri test --all-features
```

## Key Points

- **Nightly only**: Miri requires a nightly toolchain. Pin a specific nightly date in `rust-toolchain.toml` if you need reproducible CI.
- **Slower execution**: Miri interprets the program. Measure the cost of your suite and use targeted tests or a separate job when a full run is impractical.
- **`MIRIFLAGS`**: `-Zmiri-strict-provenance` enables stricter pointer provenance checks, catching casts that violate the provenance model. Add `-Zmiri-tree-borrows` to opt into the newer Tree Borrows model.
- **Aliasing models and flags**: available models, defaults, and flags can change with nightly. Record the pinned toolchain and `MIRIFLAGS`; consult that version's documentation rather than assuming results apply to future compilers.
- **Setup command**: `cargo miri setup` pre-builds the Miri sysroot so the first test run is not cold. Run it once per CI cache key.
- Select tests that exercise the relevant unsafe implementation, including unsafe code reached through safe dependency APIs.

## When It's Acceptable to Skip

- A project-specific coverage assessment shows the selected tests exercise no relevant unsafe implementation, including dependencies; zero local `unsafe` blocks alone is insufficient.
- A specific generated-code path cannot run under Miri; document the unsupported operation and complement testing with a generator audit rather than exempting all generated code.
- Very large integration tests that make Miri impractical — run a targeted unit-test subset instead.

## Verified Example

This small unsafe helper has tests suitable for `cargo miri test`. Compiling the example checks its types; detecting undefined behavior still requires running the tests under Miri.

<!-- verify: compile -->
```rust
fn first_raw(values: &[u32]) -> Option<u32> {
    if values.is_empty() {
        return None;
    }
    // SAFETY: the nonempty slice guarantees a live, aligned first u32;
    // u32 is Copy, so reading it does not move out an owned non-Copy value.
    Some(unsafe { values.as_ptr().read() })
}

#[cfg(test)]
mod tests {
    use super::first_raw;

    #[test]
    fn reads_a_live_first_element_or_reports_empty_input() {
        assert_eq!(first_raw(&[]), None);
        assert_eq!(first_raw(&[7, 9]), Some(7));
    }
}

fn main() {
    assert_eq!(first_raw(&[]), None);
    assert_eq!(first_raw(&[7, 9]), Some(7));
}
```

## See Also

- [unsafe-maybeuninit](unsafe-maybeuninit.md) - use `MaybeUninit<T>` for uninitialized memory
- [unsafe-safety-comment](unsafe-safety-comment.md) - document every unsafe block
- [test-criterion-bench](test-criterion-bench.md) - use criterion for benchmarking (separate from Miri)
