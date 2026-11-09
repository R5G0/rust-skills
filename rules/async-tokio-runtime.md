# async-tokio-runtime

> Configure Tokio runtime appropriately for your workload

## Why It Matters

Tokio's default multi-threaded runtime isn't always optimal. CPU-bound work needs different configuration than IO-bound work. Incorrect configuration leads to poor performance, blocked workers, or resource exhaustion. Understanding runtime options lets you tune for your specific use case.

## Bad

```rust
// Default runtime for everything - not optimal
#[tokio::main]
async fn main() {
    // CPU-heavy work on async executor starves IO tasks
    for data in datasets {
        let result = heavy_computation(data).await;
    }
}

// Single-threaded when multi-threaded is needed
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Can't utilize multiple cores for concurrent tasks
    for _ in 0..1000 {
        tokio::spawn(async { /* IO work */ });
    }
}
```

## Good

A multithreaded runtime executes spawned tasks on its worker pool:

```rust
#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    let handles: Vec<_> = (1..=3).map(|value| {
        tokio::spawn(async move { value * 2 })
    }).collect();
    let mut total = 0;
    for handle in handles {
        total += handle.await?;
    }
    assert_eq!(total, 12);
    Ok(())
}
```

For a current-thread runtime, select that flavor explicitly:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::task::yield_now().await;
}
```

Configure a multithreaded runtime manually when worker names and counts matter:

```rust
fn main() -> std::io::Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .thread_name("example-worker")
        .build()?;
    runtime.block_on(async { tokio::task::yield_now().await; });
    Ok(())
}
```

## Runtime Types

| Runtime | Use Case | Configuration |
|---------|----------|---------------|
| Multi-thread | IO-bound, many connections | `#[tokio::main]` (default) |
| Current-thread | CLI tools, tests, single connection | `flavor = "current_thread"` |
| Custom | Fine-tuned performance | `Builder::new_*()` |

## Worker Thread Tuning

```rust
use tokio::runtime::Builder;

// std::thread::available_parallelism() is stable since Rust 1.59
// and respects cgroup CPU quotas (unlike the unmaintained num_cpus crate)
let parallelism = std::thread::available_parallelism()
    .map(|n| n.get())
    .unwrap_or(1);

// IO-bound: more threads than cores can help
let io_runtime = Builder::new_multi_thread()
    .worker_threads(parallelism * 2)  // IO can benefit from oversubscription
    .max_blocking_threads(32)         // For spawn_blocking calls
    .enable_io()
    .enable_time()
    .build()?;

// CPU-bound: match core count
let cpu_runtime = Builder::new_multi_thread()
    .worker_threads(parallelism)      // No benefit from more than cores
    .build()?;
```

## Multiple Runtimes

```rust
// Separate runtimes for different workloads
struct App {
    io_runtime: Runtime,
    cpu_runtime: Runtime,
}

impl App {
    fn new() -> Self {
        Self {
            io_runtime: Builder::new_multi_thread()
                .worker_threads(8)
                .thread_name("io-worker")
                .build()
                .unwrap(),
            cpu_runtime: Builder::new_multi_thread()
                .worker_threads(4)
                .thread_name("cpu-worker")
                .build()
                .unwrap(),
        }
    }
    
    fn spawn_io<F>(&self, future: F) 
    where F: Future + Send + 'static, F::Output: Send + 'static 
    {
        self.io_runtime.spawn(future);
    }
    
    fn spawn_cpu<F>(&self, task: F) 
    where F: FnOnce() + Send + 'static 
    {
        self.cpu_runtime.spawn_blocking(task);
    }
}
```

## Runtime in Tests

```rust
// Single test runtime
#[tokio::test]
async fn test_single() {
    assert!(true);
}

// Multi-threaded test
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_concurrent() {
    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move { tx.send(42).unwrap() });
    assert_eq!(rx.await.unwrap(), 42);
}

// Custom runtime in test
#[test]
fn test_with_custom_runtime() {
    let rt = Builder::new_current_thread().build().unwrap();
    rt.block_on(async {
        // test code
    });
}
```

## See Also

- [async-spawn-blocking](./async-spawn-blocking.md) - Handling blocking code
- [async-no-lock-await](./async-no-lock-await.md) - Avoiding lock issues
- [async-joinset-structured](./async-joinset-structured.md) - Managing spawned tasks
