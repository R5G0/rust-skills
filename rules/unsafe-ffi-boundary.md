# unsafe-ffi-boundary

> Keep foreign ABI, allocation ownership, and runtime state behind an explicit FFI boundary

## Why It Matters

A stable C or independently versioned plugin boundary needs more than `extern "C"`. Rust collection layout, allocator ownership, thread-local state, and type identity are not a portable cross-library protocol. Translate at the boundary and keep the core operations in safe Rust.

## Bad

```rust
pub extern "C" fn make_bytes() -> Vec<u8> {
    vec![1, 2, 3]
}
```

A foreign caller cannot interpret this vector through a stable C layout or infer how it must be released.

## Good

Keep Rust data opaque and return it to its originating library for operations and destruction:

```rust
use std::ffi::c_void;

struct Blob {
    bytes: Vec<u8>,
}

pub extern "C" fn blob_new() -> *mut c_void {
    Box::into_raw(Box::new(Blob { bytes: vec![1, 2, 3] })).cast()
}

/// Returns the byte count.
///
/// # Safety
///
/// handle must be a live pointer returned by this library's blob_new.
/// It must not be freed or mutated concurrently during this call.
pub unsafe extern "C" fn blob_len(handle: *const c_void) -> usize {
    // SAFETY: the caller guarantees a live Blob from this library and
    // excludes concurrent mutation or destruction while it is borrowed.
    let blob = unsafe { &*handle.cast::<Blob>() };
    blob.bytes.len()
}

/// Releases a blob.
///
/// # Safety
///
/// handle must be a live pointer from this library's blob_new. No borrowed
/// views or concurrent operations may remain; it must be released exactly once.
pub unsafe extern "C" fn blob_free(handle: *mut c_void) {
    // SAFETY: this is the original allocation and its ownership is consumed
    // once, in the same library that created it.
    unsafe { drop(Box::from_raw(handle.cast::<Blob>())) };
}
```

These functions illustrate the ABI and ownership contract. An exported library must also choose its symbol/export mechanism and publish matching foreign declarations. The caller treats the handle as opaque and keeps the library loaded until all handles are released.

## Key Points

- Use defined ABI types and `#[repr(C)]` records where layout is part of the contract. `repr(C)` does not make nested `String`, `Vec`, or trait objects portable.
- For borrowed pointer/length pairs, document validity, allocation bounds, alignment, lifetime, aliasing, and empty-buffer conventions. A null check cannot prove a pointer is valid.
- Return allocations to the correct originating allocator or provide a documented compatible allocation protocol. Do not reconstruct an arbitrary foreign pointer as a Rust Box or Vec.
- Do not assume independently built libraries share singleton state, thread locals, runtime registrations, or stable `TypeId` identities. Route operations through the owning library instead.
- Specify threading, callback lifetimes, and panic behavior. `extern "C"` aborts on an escaping Rust panic; `"C-unwind"` permits unwinding only with a compatible end-to-end contract. `catch_unwind` cannot catch aborting panics.
- Keep validation, pointer conversion, and error translation in a narrow module or glue crate. Create a separate crate when it improves the real packaging or deployment boundary.
- For FFI packages, the conventional `-sys` suffix identifies raw imports and `-ffi` identifies exported glue; follow an established ecosystem naming scheme when one already exists.

## See Also

- [unsafe-extern-block](unsafe-extern-block.md) - declare imported functions accurately
- [unsafe-no-mangle-unsafe](unsafe-no-mangle-unsafe.md) - choose and justify exported symbol names
- [type-repr-transparent](type-repr-transparent.md) - preserve a wrapper's required ABI
- [unsafe-panic-safety](unsafe-panic-safety.md) - keep Rust state valid on unwinding paths

## Sources

- [Microsoft M-ISOLATE-DLL-STATE and M-FFI-TRANSLATES](https://microsoft.github.io/rust-guidelines/guidelines/ffi/index.html)
- [Rustonomicon: FFI](https://doc.rust-lang.org/nomicon/ffi.html)
- [Standard library: catch_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)
