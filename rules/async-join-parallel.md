# async-join-parallel

> Use `join!` or `try_join!` for concurrent independent futures

## Why It Matters

Awaiting futures sequentially takes the sum of their durations. `join!` runs futures concurrently, taking only as long as the slowest one. For independent operations like multiple API calls or parallel file reads, this can dramatically reduce latency.

## Bad

```rust
async fn fetch_data() -> (User, Posts, Comments) {
    // Sequential: 300ms total (100 + 100 + 100)
    let user = fetch_user().await;        // 100ms
    let posts = fetch_posts().await;      // 100ms  
    let comments = fetch_comments().await; // 100ms
    
    (user, posts, comments)
}

async fn read_configs() -> Result<(Config, Settings)> {
    // Sequential: 20ms + 20ms = 40ms
    let config = fs::read_to_string("config.toml").await?;
    let settings = fs::read_to_string("settings.json").await?;
    
    Ok((parse_config(&config)?, parse_settings(&settings)?))
}
```

## Good

<!-- verify: compile -->
```rust
use tokio::sync::oneshot;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (tx, rx) = oneshot::channel();
    let receive = async { rx.await.expect("sender branch sends a value") };
    let send = async {
        tx.send(41).expect("receiver branch stays alive");
        1
    };
    // Sequentially awaiting receive first would never reach send.
    let (value, increment) = tokio::join!(receive, send);
    assert_eq!(value + increment, 42);
}
```

## join! vs try_join!

```rust
// join! - all futures run to completion, returns tuple
let (a, b, c) = join!(future_a, future_b, future_c);

// try_join! - short-circuits on first error
let (a, b, c) = try_join!(fallible_a, fallible_b, fallible_c)?;
// If fallible_b fails, returns Err immediately
// Other futures may still be running (cancellation is async)
```

## futures::join_all for Dynamic Collections

`join_all` preserves input order and waits for every future. Use `try_join_all` to return an error as soon as a fallible branch fails.

```rust
use futures::future::{join_all, try_join_all};

async fn double(value: u32) -> u32 { value.saturating_mul(2) }

async fn double_all(values: &[u32]) -> Vec<u32> {
    join_all(values.iter().map(|&value| double(value))).await
}

async fn parse_all(inputs: &[&str]) -> Result<Vec<u32>, std::num::ParseIntError> {
    try_join_all(inputs.iter().map(|input| async move { input.parse::<u32>() })).await
}
```

## Limiting Concurrency

```rust
use anyhow::Result;
use std::sync::Arc;
use futures::future::join_all;
use futures::stream::{self, StreamExt};

async fn fetch_with_limit(ids: &[u64]) -> Vec<Result<User>> {
    stream::iter(ids)
        .map(|id| fetch_user(*id))
        .buffer_unordered(10)  // Max 10 concurrent requests
        .collect()
        .await
}

// Or with tokio::sync::Semaphore
use tokio::sync::Semaphore;

async fn fetch_with_semaphore(ids: &[u64]) -> Vec<Result<User>> {
    let semaphore = Arc::new(Semaphore::new(10));
    
    let futures: Vec<_> = ids.iter().map(|id| {
        let semaphore = semaphore.clone();
        async move {
            let _permit = semaphore.acquire().await.unwrap();
            fetch_user(*id).await
        }
    }).collect();
    
    join_all(futures).await
}
```

## When NOT to Use join!

```rust
use anyhow::Result;

// Dependent futures must be sequential.
async fn create_and_populate() -> Result<()> {
    let db = create_database().await?;   // Must complete first
    populate_tables(&db).await?;          // Depends on db
    Ok(())
}

// For racing or early-exit logic, use select! rather than join!.

// ❌ Shared mutable state
async fn bad_shared_state() {
    let counter = Arc::new(Mutex::new(0));
    // This might work but can cause contention
    join!(
        increment(counter.clone()),
        increment(counter.clone()),
    );
}
```

## See Also

- [async-try-join](./async-try-join.md) - Error handling in concurrent futures
- [async-select-racing](./async-select-racing.md) - Racing futures
- [async-joinset-structured](./async-joinset-structured.md) - Dynamic task sets
