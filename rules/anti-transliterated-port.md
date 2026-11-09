# anti-transliterated-port

> Preserve domain behavior when porting code; redesign language-specific mechanisms for Rust

## Why It Matters

An algorithm or business contract should survive a language migration, but its
original ownership, exception, inheritance and task-management machinery may
solve problems Rust handles differently. Translate the observable behavior and
choose Rust types and lifetimes for its implementation. Similar names or control
flow alone are not evidence that a port needs rewriting.

## Bad

When the caller needs a temporary view of a lookup result, this status-plus-output
API introduces mutation and a copy that the contract does not require:

```rust
use std::collections::HashMap;

fn find_name(names: &HashMap<u64, String>, id: u64, output: &mut Option<String>) -> bool {
    match names.get(&id) {
        Some(name) => {
            *output = Some(name.clone());
            true
        }
        None => {
            *output = None;
            false
        }
    }
}
```

## Good

```rust
use std::collections::HashMap;

fn find_name(names: &HashMap<u64, String>, id: u64) -> Option<&str> {
    names.get(&id).map(String::as_str)
}
```

If the caller must retain the result beyond the map's lifetime or mutation,
an owned return can be appropriate. The choice follows the caller's contract,
not a universal prohibition on copying.

## Review Boundaries

- Use Result and typed failures where the source language uses recoverable exceptions.
- Use ownership and Drop for resource lifetimes instead of manually coordinated cleanup.
- Choose traits for actual generic/dynamic contracts rather than adding an interface per class.
- Supervise spawned tasks according to their runtime lifecycle and cancellation contracts.
- Preserve protocol formats, domain invariants and observable ordering; changing syntax
  is not justification for changing those behaviors.

## Sources

- [Microsoft M-RUST-SHAPED](https://microsoft.github.io/rust-guidelines/guidelines/ai/index.html#M-RUST-SHAPED)

## See Also

- [anti-over-abstraction](anti-over-abstraction.md) - Avoid abstractions without an actual boundary
- [own-borrow-over-clone](own-borrow-over-clone.md) - Choose ownership for the caller's needs
- [err-result-over-panic](err-result-over-panic.md) - Recoverable failure contracts
