# test-snapshot-testing

> Use snapshot testing (insta) for complex or serialized output

## Why It Matters

Asserting large structured output — pretty-printed structs, rendered error messages, JSON responses, generated code, CLI output — with hand-written `assert_eq!` is verbose, brittle, and hard to update when output intentionally changes. The `insta` crate records an approved snapshot on first run and diffs against it on subsequent runs; when output changes legitimately, `cargo insta review` presents a diff and lets you accept it in one keystroke. Snapshots are committed to the repo and reviewed in PRs, making output changes visible and deliberate rather than silent.

## Bad

```rust
#[test]
fn test_render_error() {
    let err = AppError::NotFound { id: 42 };
    // Fragile: must manually maintain this string forever
    assert_eq!(
        format!("{err}"),
        "resource with id 42 was not found in the database and could not be retrieved"
    );
}

#[test]
fn test_config_serialization() {
    let config = Config::default();
    let json = serde_json::to_string_pretty(&config).unwrap();
    // Hard to read, hard to update, easy to get wrong
    assert_eq!(json, "{\n  \"timeout\": 30,\n  \"retries\": 3\n}");
}
```

## Good

```toml
[dev-dependencies]
insta = { version = "1", features = ["json", "yaml"] }
```

<!-- verify: compile -->
```rust
fn render_report(names: &[&str]) -> String {
    format!("users: {}\n{}", names.len(), names.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::render_report;

    #[test]
    fn user_report_keeps_its_multiline_layout() {
        insta::assert_snapshot!(render_report(&["Ada", "Grace"]), @"users: 2\nAda\nGrace");
    }
}
```

## Workflow

The example above uses an inline snapshot whose approved output is reviewed in
the source. For snapshots stored in separate files:

1. Run tests for the first time: `cargo test` — insta creates `.snap.new` files.
2. Review and accept: `cargo insta review` — interactive diff; press `a` to accept.
3. Commit the `.snap` files alongside your code changes.
4. In CI, use `INSTA_UPDATE=no cargo test` to reject new or changed snapshots. `INSTA_UPDATE=unseen` accepts previously unseen snapshots, so it is not a strict approval gate.

```bash
# CI: fail on any unapproved snapshots
INSTA_UPDATE=no cargo test
```

## When to Use Snapshots vs assert_eq!

| Situation | Prefer |
|-----------|--------|
| Short, simple values (`true`, `42`, `"ok"`) | `assert_eq!` |
| Multi-line or structured output | `assert_debug_snapshot!` |
| JSON/YAML serialization | `assert_json_snapshot!` / `assert_yaml_snapshot!` |
| Rendered error messages | `assert_snapshot!` |
| Compiler-error-style output | `assert_snapshot!` |

## See Also

- [test-arrange-act-assert](test-arrange-act-assert.md) - structure tests as arrange/act/assert
- [test-proptest-properties](test-proptest-properties.md) - use proptest for property-based testing
- [test-doctest-examples](test-doctest-examples.md) - keep doc examples as executable tests
