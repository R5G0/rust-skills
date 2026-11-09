# api-io-boundary

> Accept the I/O capability a parser needs and bound externally controlled input

## Why It Matters

A parser tied to File cannot directly consume an in-memory buffer or a caller-owned stream. Read and Write separate data processing from transport setup without inventing a new trait. The boundary still needs size, allocation, and progress limits; changing a concrete parameter to a trait does not make untrusted input safe or synchronous I/O nonblocking.

## Bad

```rust
use std::fs::File;
use std::io::{self, Read};

pub fn read_record(mut file: File) -> io::Result<Vec<u8>> {
    let mut header = [0_u8; 4];
    file.read_exact(&mut header)?;
    let mut payload = vec![0; u32::from_be_bytes(header) as usize];
    file.read_exact(&mut payload)?;
    Ok(payload)
}
```

This requires a file, trusts the advertised size, and narrows the length without checking the target's pointer width.

## Good

```rust
use std::io::{self, Read};

pub fn read_record(
    reader: &mut impl Read,
    max_bytes: usize,
) -> io::Result<Vec<u8>> {
    let mut header = [0_u8; 4];
    reader.read_exact(&mut header)?;
    let length = usize::try_from(u32::from_be_bytes(header))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "record length is too large"))?;
    if length > max_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "record exceeds the configured byte limit",
        ));
    }

    let mut payload = Vec::new();
    payload.try_reserve_exact(length).map_err(io::Error::other)?;
    payload.resize(length, 0);
    reader.read_exact(&mut payload)?;
    Ok(payload)
}
```

A caller can pass a File, network stream, Cursor, or byte slice. On success, the function reads one length-prefixed record and leaves trailing records for the next call. Errors can consume a header or payload prefix without rolling it back; the caller must close the stream or use a protocol-defined resynchronization procedure before continuing.

## Key Points

- Set the byte budget from a trusted resource policy, including the number of concurrent records. A per-record bound does not by itself bound total application memory.
- Check narrowing conversions and arithmetic used for offsets or combined lengths before allocation. Propagate allocation failures where the API promises a fallible allocation path.
- Choose Read, BufRead, Write, or a borrowed byte slice according to actual use. Keep a concrete transport when transport-specific operations are part of the contract.
- Synchronous I/O can block indefinitely. Configure transport deadlines or cancellation at the appropriate adapter; do not run blocking reads directly on an async worker.
- For async APIs, use traits compatible with the selected runtime or an explicit runtime adapter. A synchronous Read implementation is not an async abstraction.
- Generic I/O is useful for composability and fakes; it does not require turning the whole library into a generic type hierarchy.

## See Also

- [test-mock-traits](test-mock-traits.md) - inject the smallest useful external-effect seam
- [perf-io-buffering](perf-io-buffering.md) - buffer I/O when the access pattern benefits
- [num-cast-try-from](num-cast-try-from.md) - check narrowing conversions
- [async-spawn-blocking](async-spawn-blocking.md) - isolate unavoidable blocking work

## Sources

- [Microsoft M-IMPL-IO](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-IMPL-IO)
- [Microsoft M-MOCKABLE-SYSCALLS](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-MOCKABLE-SYSCALLS)
- [Read contract](https://doc.rust-lang.org/std/io/trait.Read.html)
