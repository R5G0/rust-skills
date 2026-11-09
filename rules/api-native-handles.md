# api-native-handles

> Distinguish borrowing, transferring, and adopting ownership of native handles

## Why It Matters

Interop sometimes requires access to an underlying OS handle. Returning its numeric value does not transfer ownership or extend its lifetime. Prefer the standard borrowed and owned handle types, and document the full ownership preconditions when a raw handle must be adopted.

## Bad

```rust
#[cfg(unix)]
mod native {
    use std::fs::File;
    use std::os::fd::{AsRawFd, FromRawFd, RawFd};

    pub struct NativeFile(File);

    impl NativeFile {
        pub fn adopt(raw: RawFd) -> Self {
            // The safe caller has not established exclusive ownership.
            Self(unsafe { File::from_raw_fd(raw) })
        }

        pub fn release(self) -> RawFd {
            // self is dropped on return, closing the descriptor.
            self.0.as_raw_fd()
        }
    }
}
```

## Good

```rust
#[cfg(unix)]
mod native {
    use std::fs::File;
    use std::os::fd::{AsFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};

    pub struct NativeFile(File);

    impl NativeFile {
        pub fn from_owned(handle: OwnedFd) -> Self {
            Self(File::from(handle))
        }

        pub fn borrow(&self) -> BorrowedFd<'_> {
            self.0.as_fd()
        }

        pub fn into_owned(self) -> OwnedFd {
            self.0.into()
        }

        /// Adopts an exclusively owned descriptor.
        ///
        /// # Safety
        ///
        /// raw must be a valid, open descriptor that the caller owns.
        /// No other owner may close it after this ownership transfer.
        pub unsafe fn from_raw(raw: RawFd) -> Self {
            // SAFETY: the caller transfers the open descriptor's ownership
            // exactly once, as required by File::from_raw_fd.
            Self(unsafe { File::from_raw_fd(raw) })
        }
    }
}
```

The borrowed view cannot outlive the wrapper; the owned conversion consumes it without closing the descriptor early. Use the corresponding owned/borrowed handle or socket APIs on other platforms.

## Key Points

- Expose native access when interoperability needs it, rather than adding raw access to every wrapper.
- Borrowing does not permit the receiver to close or retain the handle beyond its valid lifetime.
- A transfer must prevent the old owner from running the close operation. Reconstructing two owners for the same raw handle violates the ownership contract.
- An unsafe constructor must describe handle kind, validity, ownership, thread affinity, and runtime/library origin where applicable. Checking a numeric range cannot establish those properties.
- Prefer a safe constructor from an established owned-handle type when it expresses all requirements.
- For handles with callbacks or asynchronous operations, define when outstanding operations finish and when the foreign owner may release them.

## See Also

- [unsafe-ffi-boundary](unsafe-ffi-boundary.md) - specify foreign ownership and runtime contracts
- [unsafe-safety-comment](unsafe-safety-comment.md) - document each unsafe operation and caller obligation
- [api-public-dependencies](api-public-dependencies.md) - choose stable interop types deliberately

## Sources

- [Microsoft M-ESCAPE-HATCHES](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-ESCAPE-HATCHES)
- [Rust standard library I/O safety](https://doc.rust-lang.org/std/os/fd/index.html)
- [FromRawFd safety contract](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html)
