# lint-expect-reason

> Prefer a narrow `#[expect]` with a reason for intentional lint exceptions

## Why It Matters

A permanent `#[allow]` can hide a warning after the original reason for suppressing it disappears. Since Rust 1.81, `#[expect]` suppresses the named lint while reporting an unfulfilled expectation if that lint no longer occurs. A local reason explains the tradeoff to the next maintainer.

## Bad

```rust
#[allow(clippy::unused_async)]
pub async fn flush() {
    // In-memory implementation; the public API remains asynchronous.
}
```

The suppression stays silently in place if this implementation starts performing asynchronous I/O.

## Good

```rust
#[expect(
    clippy::unused_async,
    reason = "the in-memory implementation preserves the asynchronous storage API"
)]
pub async fn flush() {}
```

Run Clippy to check a Clippy expectation; ordinary `cargo check` does not run that tool's lint passes.

## Key Points

- Put the attribute on the smallest relevant item or expression, with the specific lint rather than a broad group.
- Ensure `unfulfilled_lint_expectations` remains enabled so obsolete exceptions are visible.
- Use `#[allow(..., reason = "...")]` where the warning intentionally varies by target, generated expansion, or supported toolchain and an expectation would be unreliable.
- An expectation cannot override a `forbid` lint.
- Respect the project's MSRV rather than introducing unsupported attributes.

## See Also

- [lint-workspace-lints](lint-workspace-lints.md) - inherit the shared policy before applying a local exception
- [lint-pedantic-selective](lint-pedantic-selective.md) - choose useful lints before suppressing them

## Sources

- [Microsoft M-LINT-OVERRIDE-EXPECT](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-LINT-OVERRIDE-EXPECT)
- [Rust Reference: lint expectations](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-expect-attribute)
