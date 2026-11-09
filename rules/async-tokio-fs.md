# async-tokio-fs

> Use `tokio::fs` instead of `std::fs` in async code

## Why It Matters

`std::fs` operations are blocking—they stop the current thread until the syscall completes. In async code, this blocks the executor thread, preventing it from running other tasks. `tokio::fs` wraps filesystem operations in `spawn_blocking`, keeping the executor responsive.

## Bad

```rust
async fn process_files(paths: &[PathBuf]) -> Result<Vec<String>> {
    let mut contents = Vec::new();
    
    for path in paths {
        // BLOCKS the entire executor thread!
        let data = std::fs::read_to_string(path)?;
        contents.push(data);
    }
    
    Ok(contents)
}

// While reading a file, NO other tasks can run on this thread
```

## Good

```rust
use tokio::fs;
use std::io::Result;
use std::path::PathBuf;

async fn process_files(paths: &[PathBuf]) -> Result<Vec<String>> {
    let mut contents = Vec::new();
    
    for path in paths {
        // Non-blocking: allows other tasks to run
        let data = fs::read_to_string(path).await?;
        contents.push(data);
    }
    
    Ok(contents)
}

// Even better: concurrent reads
async fn process_files_concurrent(paths: &[PathBuf]) -> Result<Vec<String>> {
    let futures: Vec<_> = paths.iter()
        .map(|path| fs::read_to_string(path))
        .collect();
    
    futures::future::try_join_all(futures).await
}
```

## tokio::fs API

```rust
use tokio::fs;

// Reading
let contents = fs::read_to_string("file.txt").await?;
let bytes = fs::read("file.bin").await?;

// Writing
fs::write("output.txt", "contents").await?;

// File operations
let file = fs::File::open("file.txt").await?;
let file = fs::File::create("new.txt").await?;

// Directory operations
fs::create_dir("new_dir").await?;
fs::create_dir_all("nested/dir/path").await?;
fs::remove_dir("empty_dir").await?;
fs::remove_dir_all("dir_with_contents").await?;

// Metadata
let metadata = fs::metadata("file.txt").await?;
let canonical = fs::canonicalize("./relative").await?;

// Rename/remove
fs::rename("old.txt", "new.txt").await?;
fs::remove_file("file.txt").await?;

// Read directory
let mut entries = fs::read_dir("some_dir").await?;
while let Some(entry) = entries.next_entry().await? {
    println!("{}", entry.path().display());
}
```

## Async File I/O

```rust
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader};

// Read with buffer
let mut file = File::open("large.bin").await?;
let mut buffer = vec![0u8; 4096];
let bytes_read = file.read(&mut buffer).await?;

// Read all
let mut contents = Vec::new();
file.read_to_end(&mut contents).await?;

// Write
let mut file = File::create("output.bin").await?;
file.write_all(b"data").await?;
file.flush().await?;

// Buffered line reading
let file = File::open("lines.txt").await?;
let reader = BufReader::new(file);
let mut lines = reader.lines();

while let Some(line) = lines.next_line().await? {
    println!("{}", line);
}
```

## When std::fs is Acceptable

Synchronous reads during startup, before the async runtime starts, do not block executor workers. Once inside any runtime, even a current-thread runtime, a blocking call prevents that worker from polling other tasks; prefer `tokio::fs` or a blocking pool for operations with meaningful latency.

```rust
fn main() -> std::io::Result<()> {
    let config = std::fs::read_to_string("config.toml")?;
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async { assert!(!config.is_empty()); });
    Ok(())
}
```

## Performance Considerations

```rust
// tokio::fs uses spawn_blocking internally
// For many small files, the overhead adds up

// Batch operations when possible
let paths: Vec<_> = entries.iter()
    .map(|e| e.path())
    .collect();

let contents = futures::future::try_join_all(
    paths.iter().map(|p| fs::read_to_string(p))
).await?;

// For heavy I/O, consider memory-mapped files
// (requires unsafe or mmap crate)
```

## See Also

- [async-spawn-blocking](./async-spawn-blocking.md) - How tokio::fs works internally
- [async-tokio-runtime](./async-tokio-runtime.md) - Runtime configuration
- [err-context-chain](./err-context-chain.md) - Adding path context to IO errors
