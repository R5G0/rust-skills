# unsafe-minimize-scope

> Keep unsafe operations narrowly scoped and expose safe APIs only when their invariants are enforced

## Why It Matters

A small `unsafe` block helps reviewers locate the operations requiring a safety proof, but its size does not establish soundness. A safe function must uphold that proof for every input safe callers can supply. Prefer an equivalent safe implementation; justify any performance-motivated unsafe alternative with measurements.

## Bad

```rust
// UNSOUND: an integer bounds check cannot establish pointer validity.
fn sum_at(ptr: *const i32, len: usize, index: usize) -> i32 {
    assert!(index < len, "index out of bounds");
    // A safe caller can pass a null, dangling, or misaligned pointer.
    let value = unsafe { *ptr.add(index) };
    value + 1
}
```

## Good

```rust
// A slice carries the validity and lifetime guarantees the operation needs.
fn increment_at(values: &[i32], index: usize) -> Option<i32> {
    values.get(index)?.checked_add(1)
}

fn main() {
    assert_eq!(increment_at(&[10, 20], 1), Some(21));
    assert_eq!(increment_at(&[], 0), None);
    assert_eq!(increment_at(&[i32::MAX], 0), None);
}
```

When raw pointers are genuinely required, retain an unsafe caller contract and
isolate the operation that relies on it:

```rust
/// Copies bytes from a foreign buffer.
///
/// # Safety
/// If `len` is nonzero, `ptr` must identify a single allocation containing
/// `len` initialized, readable bytes, remain live for this call, and have no
/// concurrent writes to those bytes. The range must fit within `isize::MAX`.
unsafe fn copy_foreign_bytes(ptr: *const u8, len: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(len);
    for index in 0..len {
        // SAFETY: the caller guarantees readable initialized bytes in one
        // live allocation without concurrent writes; index is within len.
        let byte = unsafe { *ptr.add(index) };
        result.push(byte);
    }
    result
}

fn main() {
    let bytes = [3, 7, 11];
    // SAFETY: bytes remains live and unchanged, and the entire array is valid.
    let copied = unsafe { copy_foreign_bytes(bytes.as_ptr(), bytes.len()) };
    assert_eq!(copied, bytes);
}
```

## Review the Entire Abstraction

- Bounds checks establish an index range, not allocation validity, initialization,
  alignment, lifetime, or aliasing guarantees. A null check alone is insufficient.
- Safe constructors and methods must preserve every invariant relied on by unsafe
  internals. Reducing a block's size does not reduce that responsibility.
- `unsafe_op_in_unsafe_fn` is **warn-by-default** in Rust 2024, not a hard error.
  Enable it at `deny` when the project wants explicit blocks enforced as errors.
- Consecutive operations sharing one safety argument may use one small block.

## Sources

- [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html) — unsafe abstractions and technical justification
- [Comprehensive Rust: Dereferencing Raw Pointers](https://google.github.io/comprehensive-rust/unsafe-rust/dereferencing.html)
- [Rust 2024 Edition Guide: unsafe operations in unsafe functions](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html)

## See Also

- [unsafe-safety-comment](unsafe-safety-comment.md) - document safety preconditions and their proof
- [unsafe-send-sync-manual](unsafe-send-sync-manual.md) - preserve cross-thread invariants
