# async-try-join

> Use `try_join!` for concurrent fallible operations with early return on error

## Why It Matters

When running multiple fallible operations concurrently, `try_join!` returns `Err` as soon as any future fails, without waiting for the others. This provides fail-fast behavior while still running operations in parallel. For many operations, use `futures::future::try_join_all`.

## Bad

```rust
// Sequential - slow and no early return benefit
async fn fetch_all() -> Result<(A, B, C)> {
    let a = fetch_a().await?;  // If this fails, we wait for nothing
    let b = fetch_b().await?;  // But if this fails, we waited for A
    let c = fetch_c().await?;
    Ok((a, b, c))
}

// join! ignores errors
async fn fetch_all() -> (Result<A>, Result<B>, Result<C>) {
    let (a, b, c) = join!(fetch_a(), fetch_b(), fetch_c());
    // All complete even if first one failed
    (a, b, c)  // Now we have to handle three Results
}
```

## Good

<!-- verify: compile -->
```rust
async fn parse_number(input: &str) -> Result<u32, std::num::ParseIntError> {
    tokio::task::yield_now().await;
    input.parse()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), std::num::ParseIntError> {
    let pair = tokio::try_join!(parse_number("20"), parse_number("22"))?;
    assert_eq!(pair, (20, 22));
    assert!(tokio::try_join!(parse_number("invalid"), parse_number("22")).is_err());
    Ok(())
}
```

## Error Handling Patterns

```rust
use anyhow::Result;
use futures::TryFutureExt;
use futures::future::try_join_all;
use tokio::try_join;

// Different error types - need common error type
async fn mixed_operations() -> Result<(A, B), Error> {
    let (a, b) = try_join!(
        fetch_a().map_err(Error::from),  // Convert errors
        fetch_b().map_err(Error::from),
    )?;
    Ok((a, b))
}

// Return all values on success; drop the remaining futures on the first error
async fn all_or_nothing(ids: &[u64]) -> Result<Vec<User>> {
    try_join_all(ids.iter().map(|id| fetch_user(*id))).await
}

// Collect successes, log failures
async fn best_effort(ids: &[u64]) -> Vec<User> {
    let results = futures::future::join_all(
        ids.iter().map(|id| fetch_user(*id))
    ).await;
    
    results.into_iter()
        .filter_map(|r| match r {
            Ok(user) => Some(user),
            Err(e) => {
                log::warn!("Failed to fetch user: {}", e);
                None
            }
        })
        .collect()
}
```

## Cancellation Behavior

```rust
use anyhow::Result;
use tokio::try_join;

// try_join! drops its remaining branch futures on error.
async fn with_cancellation() -> Result<()> {
    // If fetch_a() fails, fetch_b() and fetch_c() are dropped
    // Dropped branch futures are never polled again. Already performed external
    // effects remain; dropping a separately spawned task handle does not abort it.
    
    try_join!(
        async {
            fetch_a().await?;
            cleanup_a().await;  // May not run if other future fails
            Ok::<_, anyhow::Error>(())
        },
        async {
            fetch_b().await?;
            cleanup_b().await;  // May not run if other future fails
            Ok::<_, anyhow::Error>(())
        },
    )?;
    
    Ok(())
}

// For guaranteed cleanup, use Drop guards or explicit handling
```

## With Timeout

The timeout argument must be a future. Wrap the combined operation in an async block; calling `try_join!` directly produces a result after awaiting its branches, which is too late for the outer timeout.

```rust
use anyhow::{Context, Result};
use tokio::time::{timeout, Duration};

async fn fetch_a() -> Result<u32> { Ok(20) }
async fn fetch_b() -> Result<u32> { Ok(22) }

async fn fetch_with_timeout() -> Result<(u32, u32)> {
    timeout(Duration::from_secs(10), async {
        tokio::try_join!(fetch_a(), fetch_b())
    }).await.context("combined operation timed out")?
}

async fn individual_timeouts() -> Result<(u32, u32)> {
    tokio::try_join!(
        async {
            timeout(Duration::from_secs(5), fetch_a()).await.context("operation a timed out")?
        },
        async {
            timeout(Duration::from_secs(5), fetch_b()).await.context("operation b timed out")?
        },
    )
}
```

## try_join! vs FuturesUnordered

```rust
use futures::stream::{FuturesUnordered, StreamExt};

// try_join!: wait for all, fail fast
let (a, b, c) = try_join!(fa, fb, fc)?;

// FuturesUnordered: process as they complete
let mut futures = FuturesUnordered::new();
futures.push(fetch_a());
futures.push(fetch_b());
futures.push(fetch_c());

while let Some(result) = futures.next().await {
    match result {
        Ok(data) => process(data),
        Err(e) => return Err(e),  // Can fail fast manually
    }
}
```

## See Also

- [async-join-parallel](./async-join-parallel.md) - Non-fallible concurrent futures
- [async-select-racing](./async-select-racing.md) - First-to-complete semantics
- [err-question-mark](./err-question-mark.md) - Error propagation
