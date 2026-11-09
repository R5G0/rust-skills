# async-spawn-blocking

> Use `spawn_blocking` for CPU-intensive work

## Why It Matters

Async runtimes like Tokio use a small number of threads to handle many tasks. CPU-intensive or blocking operations on these threads starve other tasks. `spawn_blocking` moves such work to a dedicated thread pool.

## Bad

```rust
// BAD: Blocks the async runtime thread
async fn process_image(data: &[u8]) -> ProcessedImage {
    // CPU-intensive work on async thread!
    let resized = resize_image(data);      // Blocks!
    let compressed = compress(resized);     // Blocks!
    compressed
}

// BAD: Synchronous file I/O in async context
async fn read_large_file(path: &Path) -> Vec<u8> {
    std::fs::read(path).unwrap()  // Blocks the runtime!
}
```

## Good

<!-- verify: compile -->
```rust
async fn sum_on_blocking_pool(data: Vec<u64>) -> Result<u64, tokio::task::JoinError> {
    tokio::task::spawn_blocking(move || data.into_iter().sum()).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), tokio::task::JoinError> {
    assert_eq!(sum_on_blocking_pool(vec![1, 2, 3]).await?, 6);
    Ok(())
}
```

## What Counts as Blocking

CPU-intensive operations include cryptographic computation, image/video processing, compression, expensive parsing, and mathematical kernels. Blocking I/O includes `std::fs`, synchronous database and HTTP clients, and `std::thread::sleep`.

Measure worst-case poll duration and executor latency under realistic load. There is no universal microsecond threshold for moving work off a worker.

## Practical Examples

```rust
// Password hashing (CPU-intensive)
async fn hash_password(password: String) -> String {
    task::spawn_blocking(move || {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
    })
    .await
    .unwrap()
}

// JSON parsing of large documents
async fn parse_large_json(data: String) -> serde_json::Value {
    task::spawn_blocking(move || {
        serde_json::from_str(&data).unwrap()
    })
    .await
    .unwrap()
}

// Compression
async fn compress_data(data: Vec<u8>) -> Vec<u8> {
    task::spawn_blocking(move || {
        let mut encoder = flate2::write::GzEncoder::new(
            Vec::new(),
            flate2::Compression::default(),
        );
        encoder.write_all(&data).unwrap();
        encoder.finish().unwrap()
    })
    .await
    .unwrap()
}
```

## spawn_blocking vs spawn

```rust
// spawn: Runs async code on runtime threads
tokio::spawn(async {
    // Async code here
    some_async_operation().await;
});

// spawn_blocking: Runs sync code on blocking thread pool
tokio::task::spawn_blocking(|| {
    // Synchronous, possibly CPU-intensive code
    heavy_computation();
});

// spawn_blocking returns JoinHandle that can be awaited
let result = tokio::task::spawn_blocking(|| {
    expensive_sync_operation()
}).await?;
```

## Rayon for Parallel CPU Work

```rust
// For parallel CPU work, consider Rayon inside spawn_blocking
async fn parallel_process(items: Vec<Item>) -> Vec<Output> {
    task::spawn_blocking(move || {
        use rayon::prelude::*;
        items.par_iter()
            .map(|item| cpu_intensive_transform(item))
            .collect()
    })
    .await
    .unwrap()
}
```

## See Also

Bound admission for CPU-heavy jobs before spawning them, using a semaphore or a dedicated compute pool. The blocking pool's large default thread limit and queued closures do not bound application memory or CPU contention. Once a blocking closure has started, aborting its JoinHandle does not stop it; design cooperative shutdown when needed.

Source: [Tokio spawn_blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html).

- [async-tokio-fs](async-tokio-fs.md) - Use tokio::fs for async file I/O
- [async-no-lock-await](async-no-lock-await.md) - Don't hold locks across await
