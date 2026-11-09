# unsafe-panic-safety

> Preserve memory safety if user code panics during an unsafe state transition

## Why It Matters

Closures, comparisons, cloning, allocation, and destructors may panic. An unsafe abstraction must remain memory-safe if unwinding observes intermediate state: initialized elements must be tracked correctly, and owned values must not be dropped twice. Logical rollback is a separate requirement from this minimum soundness guarantee.

## Bad

```rust
pub fn append_cloned<T: Clone>(destination: &mut Vec<T>, source: &[T]) {
    destination.reserve(source.len());
    let start = destination.len();

    // Unsound: the length exposes uninitialized elements if clone panics.
    unsafe {
        destination.set_len(start + source.len());
        for (offset, value) in source.iter().enumerate() {
            destination.as_mut_ptr().add(start + offset).write(value.clone());
        }
    }
}
```

A panic in `Clone` leaves the vector claiming ownership of uninitialized entries.

## Good

Prefer a safe implementation that already maintains these invariants:

```rust
pub fn append_cloned<T: Clone>(destination: &mut Vec<T>, source: &[T]) {
    destination.extend_from_slice(source);
}
```

If a custom low-level implementation is justified, publish each initialized entry only after construction succeeds:

```rust
pub fn append_cloned_raw<T: Clone>(destination: &mut Vec<T>, source: &[T]) {
    destination.reserve(source.len());
    for value in source {
        let cloned = value.clone();
        let initialized = destination.len();
        // SAFETY: reserve made room for every source element. The next slot
        // is uninitialized, and write transfers ownership of cloned into it.
        // No user code runs between write and the matching length update.
        unsafe {
            destination.as_mut_ptr().add(initialized).write(cloned);
            destination.set_len(initialized + 1);
        }
    }
}
```

If cloning fails, the length still describes exactly the initialized elements. This example demonstrates the invariant; it does not establish a performance advantage over the safe operation.

## Key Points

- Do fallible or user-controlled work before entering a temporarily invalid state, or install an RAII guard that restores a memory-safe state on unwind.
- Safe traits can have buggy implementations. Their unexpected results or panics may produce a logical error, but must not cause your unsafe implementation to invoke undefined behavior.
- Consider partial initialization, aliasing, reentrancy, and destructor behavior. Do not depend on a destructor always running: safe code may leak values.
- Poisoning is one possible policy; an abstraction that remains valid after a panic need not become poisoned.
- `catch_unwind` does not repair broken invariants and cannot catch aborting panics. Recovery and restart behavior belong to the application's explicit contract.
- Reserve `unsafe` for contracts whose violation can cause undefined behavior, rather than operations that are merely destructive or error-prone.

## See Also

- [unsafe-minimize-scope](unsafe-minimize-scope.md) - expose a sound safe API around small unsafe operations
- [unsafe-maybeuninit](unsafe-maybeuninit.md) - track initialization before creating values
- [unsafe-safety-comment](unsafe-safety-comment.md) - explain the required invariants
- [err-result-over-panic](err-result-over-panic.md) - report expected failures with Result

## Sources

- [Microsoft M-UNSAFE and M-UNSOUND](https://microsoft.github.io/rust-guidelines/guidelines/correctness/index.html#M-UNSAFE)
- [Rustonomicon: exception safety](https://doc.rust-lang.org/nomicon/exception-safety.html)
