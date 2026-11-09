# perf-io-buffering

> Wrap `Read`/`Write` in `BufReader`/`BufWriter` for many small operations

## Why It Matters

Every unbuffered read or write to a file or socket is a syscall. Calling `read()` byte-by-byte or line-by-line without buffering can issue millions of syscalls per second, each with kernel-transition overhead that dwarfs the actual data transfer. `BufReader` and `BufWriter` batch those operations into large internal buffer reads and writes, cutting syscall count by orders of magnitude. This is one of the highest-impact, lowest-effort performance fixes available for IO-heavy code.

## Bad

```rust
use std::fs::File;
use std::io::{Read, Write};

// Every read call goes to the OS — catastrophic for line-by-line processing
fn count_lines_slow(path: &str) -> std::io::Result<usize> {
    let mut file = File::open(path)?;
    let mut count = 0usize;
    let mut byte = [0u8; 1];
    loop {
        match file.read(&mut byte) { // one syscall per byte
            Ok(0) => break,
            Ok(_) => {
                if byte[0] == b'\n' {
                    count += 1;
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(count)
}

// Writing many small records without buffering — each write is a syscall
fn write_records_slow(path: &str, records: &[String]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for record in records {
        file.write_all(record.as_bytes())?; // one syscall per record
        file.write_all(b"\n")?;             // another syscall
    }
    Ok(())
}
```

## Good

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

// Preserve the slow version's contract: count newline bytes, including non-UTF-8 input.
fn count_lines_fast(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut count = 0usize;
    loop {
        let bytes = reader.fill_buf()?;
        if bytes.is_empty() {
            break;
        }
        count += bytes.iter().filter(|&&byte| byte == b'\n').count();
        let consumed = bytes.len();
        reader.consume(consumed);
    }
    Ok(count)
}

// BufWriter batches writes; explicit flush() surfaces errors that drop() would swallow
fn write_records_fast(path: &str, records: &[String]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for record in records {
        writer.write_all(record.as_bytes())?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?; // MUST flush explicitly — drop() swallows flush errors
    Ok(())
}

// Custom buffer size when the default 8 KiB isn't optimal
fn process_large_file(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(64 * 1024, file); // 64 KiB buffer
    for line in reader.lines() {
        let _line = line?;
        // process...
    }
    Ok(())
}
```

## Key Points

- `BufWriter::flush()?` must be called explicitly. When a `BufWriter` is dropped, it attempts to flush, but any resulting error is **silently discarded**. Always flush before the writer goes out of scope.
- Current defaults use an 8 KiB buffer; that size is not a stable API guarantee. Measure alternative capacities for the actual file or stream.
- `BufReader` implements `BufRead`, which provides `lines()`, `read_line()`, and `read_until()` — use these instead of reading bytes manually.
- Network streams can benefit from batching small operations, but buffer boundaries, latency, and flushing behavior need workload-specific measurement; a write is not necessarily a TCP packet.
- If you wrap a type that is already internally buffered (e.g., `tokio::io::BufWriter` in async code), adding another layer is redundant.

## Verified Project

<!-- verify: fixture=performance -->

See the [executable implementations, regression tests, and benchmarks](../perf-validation/README.md) for this rule and its recorded measurement limits.

## See Also

`lines()` returns owned UTF-8 strings and may allocate per line. It counts an unterminated final line and rejects invalid UTF-8, unlike counting newline bytes. Choose the input contract before comparing performance. The [executable I/O tests](../perf-validation/tests/buffered_io.rs) cover both counterexamples, short reads/writes, and flush failures; [file-backed benchmarks](../perf-validation/benches/cases/buffered_io.rs) compare equivalent byte-counting workloads.

- [mem-with-capacity](mem-with-capacity.md) - pre-size buffers when the final size is known
- [perf-profile-first](perf-profile-first.md) - confirm IO is the bottleneck before tuning
