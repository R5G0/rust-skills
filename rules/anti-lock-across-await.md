# anti-lock-across-await

> Avoid synchronous guards and unnecessary async critical sections across await points

## Why It Matters

Holding a lock guard across an `.await` keeps other users waiting while the task is suspended. A synchronous lock can block an executor worker and deadlock when another task needs that worker to release the guard. Async locks can intentionally serialize an I/O protocol across await; choose that contract explicitly rather than treating all guards alike.

The snapshot examples below allow intervening mutations. If a result must match the current state, revalidate under the lock or use a version check before committing it. Releasing and reacquiring a lock is not equivalent to one atomic critical section.

## Bad

```rust
use std::sync::Mutex;
use tokio::sync::Mutex as AsyncMutex;

// DEADLOCK RISK: std::sync::Mutex held across await
async fn bad_std_mutex(data: &Mutex<Vec<i32>>) {
    let mut guard = data.lock().unwrap();
    do_async_work().await;  // Lock held during await!
    guard.push(42);
}

// BLOCKS OTHER TASKS: tokio Mutex held across await
async fn bad_async_mutex(data: &AsyncMutex<Vec<i32>>) {
    let mut guard = data.lock().await;
    slow_network_call().await;  // Lock held for entire call!
    guard.push(42);
}
```

## Good

<!-- verify: compile -->
```rust
use std::sync::Mutex;

async fn append_from_snapshot(data: &Mutex<Vec<i32>>) {
    let previous = {
        let guard = data.lock().expect("another task panicked while holding the lock");
        guard.last().copied().unwrap_or(0)
    }; // Synchronous guard is dropped before suspension.
    tokio::task::yield_now().await;
    data.lock().expect("another task panicked while holding the lock").push(previous + 1);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let data = Mutex::new(vec![1]);
    append_from_snapshot(&data).await;
    assert_eq!(*data.lock().expect("example does not panic under lock"), [1, 2]);
}
```

## Pattern: Clone Before Await

```rust
use anyhow::Result;
use tokio::sync::Mutex as AsyncMutex;

async fn process(data: &AsyncMutex<Config>) -> Result<()> {
    // Clone inside lock scope
    let config = data.lock().await.clone();
    
    // Now use config freely across awaits
    let result = fetch_data(&config.url).await?;
    process_result(&config, result).await?;
    
    Ok(())
}
```

## Pattern: Restructure to Avoid Lock

```rust
// Instead of locking a shared map
struct Service {
    data: AsyncMutex<HashMap<String, Data>>,
}

// Use channels or owned data
struct BetterService {
    // Each task owns its data via channels
    sender: mpsc::Sender<Request>,
}

impl BetterService {
    async fn request(&self, key: String) -> Data {
        let (tx, rx) = oneshot::channel();
        self.sender.send(Request { key, respond: tx }).await?;
        rx.await?
    }
}
```

## What Can Cross Await

| Type | Safe Across Await? |
|------|--------------------|
| `std::sync::Mutex` guard | **NO** - can deadlock |
| `std::sync::RwLock` guard | **NO** - can deadlock |
| `tokio::sync::Mutex` guard | Allowed but blocks tasks |
| `tokio::sync::RwLock` guard | Allowed but blocks tasks |
| Owned values | Yes |
| `Arc<T>` | Lifetime-safe; Send requires T: Send + Sync |
| References | Depends on lifetime and thread-safety bounds |

## Detection

```toml
# Cargo.toml
[lints.clippy]
await_holding_lock = "deny"
await_holding_refcell_ref = "deny"
```

## See Also

- [async-no-lock-await](./async-no-lock-await.md) - Async lock patterns
- [async-clone-before-await](./async-clone-before-await.md) - Ownership and deliberate snapshots
- [own-mutex-interior](./own-mutex-interior.md) - Mutex usage
