# async-no-lock-await

> Release synchronous lock guards before await; retain async locks only when the protocol requires it

## Why It Matters

Holding a lock across an `.await` point can cause deadlocks and severely hurt performance. The task may be suspended while holding the lock, blocking all other tasks waiting for it - potentially indefinitely.

Tokio's async locks permit guards across await when exclusive access is part of an I/O protocol. Use that deliberately: other callers must wait. Extracting a snapshot, awaiting, and reacquiring changes atomicity; if the result depends on unchanged state, compare a version or revalidate before applying it. A synchronous mutex is appropriate for short, low-contention sections without await; a long or contended acquisition can still block an executor worker.

## Bad

```rust
use tokio::sync::Mutex;

async fn bad_update(state: &Mutex<State>) {
    let mut guard = state.lock().await;
    
    // BAD: Lock held across await!
    let data = fetch_from_network().await;
    
    guard.value = data;
}  // Lock finally released

// This can deadlock or starve other tasks
```

## Good

<!-- verify: compile -->
```rust
use std::sync::Mutex;

async fn update(state: &Mutex<u32>) {
    // Perform independent asynchronous work before taking the synchronous lock.
    tokio::task::yield_now().await;
    let mut guard = state.lock().expect("another task panicked while holding the lock");
    *guard += 1;
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let state = Mutex::new(0);
    tokio::join!(update(&state), update(&state));
    assert_eq!(*state.lock().expect("example does not panic under lock"), 2);
}
```

## The Problem Visualized

```rust
// Task A:
let guard = mutex.lock().await;    // Acquires lock
expensive_io().await;              // Suspended, still holding lock!
// ... many milliseconds pass ...
drop(guard);                       // Finally releases

// Task B, C, D:
let guard = mutex.lock().await;    // All blocked waiting for A!
```

## Patterns for Extraction

```rust
use tokio::sync::Mutex;

// Pattern 1: Clone out, process, update
async fn pattern_clone(state: &Mutex<State>) {
    let config = state.lock().await.config.clone();
    let result = process_with_io(&config).await;
    state.lock().await.result = result;
}

// Pattern 2: Compute closure, apply
async fn pattern_closure(state: &Mutex<State>) {
    let update = compute_update().await;
    
    state.lock().await.apply(update);
}

// Pattern 3: Message passing
async fn pattern_message(
    state: &Mutex<State>,
    tx: mpsc::Sender<Update>,
) {
    let update = compute_update().await;
    tx.send(update).await.unwrap();
}

// Separate task handles updates
async fn state_manager(
    state: Arc<Mutex<State>>,
    mut rx: mpsc::Receiver<Update>,
) {
    while let Some(update) = rx.recv().await {
        state.lock().await.apply(update);
    }
}
```

## Using RwLock

```rust
use tokio::sync::RwLock;

async fn read_heavy(state: &RwLock<State>) {
    // Multiple readers OK, but still don't hold across await
    let value = {
        let guard = state.read().await;
        guard.value.clone()
    };
    
    // Process without lock
    let result = process(value).await;
    
    // Write lock for update
    state.write().await.result = result;
}
```

## std::sync::Mutex vs tokio::sync::Mutex

```rust
// Prefer std::sync::Mutex for work that does NOT span an .await —
// it is simpler, faster, and avoids the async overhead.
// Only reach for tokio::sync::Mutex when you genuinely must hold
// the lock across an .await point (rare; usually a sign to redesign).

// std::sync::Mutex in async (quick, non-awaiting operation — preferred):
async fn quick_update(state: &std::sync::Mutex<State>) {
    state.lock().unwrap().counter += 1;  // No await inside lock scope, OK
}

// tokio::sync::Mutex (only when the lock scope must span an .await):
async fn must_await_inside(state: &tokio::sync::Mutex<State>) {
    let mut guard = state.lock().await;
    // Only justified if you truly need the lock held across an async op
    // (usually you don't — extract data first, then release the lock)
}
```

## See Also

- [async-spawn-blocking](async-spawn-blocking.md) - Use spawn_blocking for CPU work
- [async-clone-before-await](async-clone-before-await.md) - Move handles or take deliberate snapshots
- [anti-lock-across-await](anti-lock-across-await.md) - Anti-pattern reference
