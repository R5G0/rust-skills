# lint-unsafe-doc

> Require documentation for unsafe blocks

## Why It Matters

The `undocumented_unsafe_blocks` lint ensures every unsafe block has a `// SAFETY:` comment explaining why the operation is sound. Unsafe code is the source of most memory safety bugs—documenting invariants catches mistakes and helps reviewers.

## Configuration

```rust
#![warn(clippy::undocumented_unsafe_blocks)]
```

Or in `Cargo.toml`:

```toml
[lints.clippy]
undocumented_unsafe_blocks = "warn"
```

For strict enforcement:

```toml
[lints.clippy]
undocumented_unsafe_blocks = "deny"
```

## Bad

```rust
pub fn read_data(ptr: *const u8, len: usize) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(ptr, len)  // WARN: undocumented
    }
}

impl Buffer {
    pub fn get_unchecked(&self, index: usize) -> &u8 {
        unsafe { self.data.get_unchecked(index) }  // WARN
    }
}
```

## Good

<!-- verify: compile -->
```rust
/// Borrow initialized bytes from a raw pointer.
///
/// # Safety
///
/// `ptr` must be non-null, properly aligned, and valid for `len` initialized
/// bytes within one allocation. The region must remain live and immutable for
/// the chosen lifetime `'a`, and its total size must not exceed `isize::MAX`.
pub unsafe fn read_data<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    // SAFETY: The caller upholds from_raw_parts' allocation, validity,
    // aliasing, and lifetime requirements documented above.
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    /// Return a byte without a bounds check.
    ///
    /// # Safety
    ///
    /// `index` must be less than the length of the backing vector.
    pub unsafe fn get_unchecked(&self, index: usize) -> &u8 {
        // SAFETY: The caller guarantees index < self.data.len().
        // A debug assertion alone would not establish that release invariant.
        unsafe { self.data.get_unchecked(index) }
    }
}
```

## SAFETY Comment Format

```rust
// SAFETY: <explanation of why this is sound>
unsafe {
    // ...
}
```

The comment should explain:
1. **What invariants are upheld** - preconditions that make this safe
2. **Why the invariants hold** - how you know they're satisfied
3. **What could go wrong** - if invariants are violated

## Examples by Category

### Pointer Operations

```rust
// SAFETY: ptr was obtained from Box::into_raw, so it's valid
// and properly aligned. We're taking back ownership.
let boxed = unsafe { Box::from_raw(ptr) };
```

### Unchecked Operations

```rust
fn checked_first(data: &[u8]) -> Option<&u8> {
    if data.is_empty() {
        return None;
    }
    // SAFETY: The preceding emptiness check establishes that index 0 is valid.
    Some(unsafe { data.get_unchecked(0) })
}
// Ordinary application code should prefer data.first() for this operation.
```

### FFI Calls

```rust
// SAFETY: libc::getenv is safe to call with a null-terminated
// string. We ensure null termination with CString::new.
// Do not retain or dereference the returned pointer across environment
// mutation; further validity and synchronization obligations still apply.
let value = unsafe { libc::getenv(key.as_ptr()) };
```

### Trait Implementations

```rust
// SAFETY: MyType contains no pointers or interior mutability,
// and all bit patterns are valid MyType values.
unsafe impl Send for MyType {}
unsafe impl Sync for MyType {}
```

## Related Lints

```toml
[lints.clippy]
undocumented_unsafe_blocks = "warn"
# Also consider:
multiple_unsafe_ops_per_block = "warn"  # One operation per block
```

## See Also

- [doc-safety-section](./doc-safety-section.md) - `# Safety` in docs
- [lint-deny-correctness](./lint-deny-correctness.md) - Correctness lints
- [type-repr-transparent](./type-repr-transparent.md) - FFI safety
