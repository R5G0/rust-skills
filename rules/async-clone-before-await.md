# async-clone-before-await

> Move existing owned handles across await; clone only when another owner or a snapshot is needed

## Why It Matters

Borrowing across `.await` is valid when the referenced data lives long enough. A borrow does not automatically make a future non-`Send`: `&T` is `Send` when `T: Sync`. Deep-cloning a collection to satisfy an imagined restriction adds work without improving correctness. `Send` depends on captured values and values retained across suspension, not on whether `.clone()` was called.

## Bad

```rust
use std::sync::Arc;

async fn process(items: Arc<Vec<u64>>) -> usize {
    // Unnecessary deep copy: the Arc keeps immutable data alive.
    let snapshot = items.as_ref().clone();
    tokio::task::yield_now().await;
    snapshot.len()
}
```

## Good

```rust
use std::sync::Arc;

async fn process(items: Arc<Vec<u64>>) -> usize {
    tokio::task::yield_now().await;
    // Borrow after suspension when no earlier borrow is needed.
    items.len()
}

async fn borrow_across_await(items: Arc<Vec<u64>>) -> usize {
    let slice = items.as_slice();
    tokio::task::yield_now().await;
    // Also valid and Send: [u64] is Sync.
    slice.len()
}

fn require_send<T: Send>(_: T) {}

fn check_send() {
    require_send(process(Arc::new(vec![1, 2, 3])));
    require_send(borrow_across_await(Arc::new(vec![1, 2, 3])));
}
```

## Another Owner Versus Another Copy

Move an existing `Arc` into a task when the caller no longer needs that handle. Clone the handle when the caller and task both need ownership. `Arc::clone` increments a reference count; cloning `Vec<T>` or `String` copies their contents.

```rust
use std::sync::Arc;

async fn spawn_readers() {
    let shared = Arc::new(vec![1_u8, 2, 3]);
    let task_data = Arc::clone(&shared);
    let task = tokio::spawn(async move {
        tokio::task::yield_now().await;
        task_data.len()
    });
    assert_eq!(shared.len(), 3);
    assert_eq!(task.await.expect("reader task panicked"), 3);
}
```

If only a scalar or small independent field is needed later, extract it before awaiting. Copy data deliberately when an independent version is required. For mutable shared state, take the snapshot under its synchronization; cloning alone does not make a read-modify-write operation atomic. An immutable `Arc` snapshot can often avoid a deep copy.

## Send and Local Tasks

- `Rc<T>` is neither `Send` nor `Sync`; cloning it does not change that. A future retaining an `Rc` across suspension cannot be passed to `tokio::spawn`.
- `Arc<T>` is `Send + Sync` only when `T: Send + Sync`; `Arc<RefCell<T>>` does not satisfy this requirement.
- `tokio::spawn` requires `Send + 'static` on both multi-thread and current-thread runtimes. `spawn_local` in a `LocalSet` supports non-`Send` tasks; it still requires `'static`.
- Scope a non-`Send` local value so it is destroyed before an await when it need not survive suspension. Values captured by an `async move` block are part of its initial state too.

## Lock Scope and Consistency

Release synchronous lock guards before awaiting. Async lock guards may intentionally span awaits when serializing access is part of the protocol, but other callers then wait. Releasing and reacquiring a lock permits intervening mutations: validate a version before applying a computed result, or use a protocol that preserves required atomicity. Do not split a critical section merely to satisfy this rule.

## Sources

- [std::marker::Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html)
- [Arc thread safety](https://doc.rust-lang.org/std/sync/struct.Arc.html#thread-safety)
- [Tokio spawn](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html)
- [Google Rust Style Guide: prefer borrows over deep copies](https://google.github.io/styleguide/rustguide.html#prefer-borrows-over-deep-copies)

## See Also

- [async-no-lock-await](async-no-lock-await.md) - Lock scope and consistency
- [own-arc-shared](own-arc-shared.md) - Shared ownership bounds
- [own-borrow-over-clone](own-borrow-over-clone.md) - Avoid unnecessary data copies
