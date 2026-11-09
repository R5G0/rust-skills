# async-cancellation-token

> Use `CancellationToken` for graceful shutdown and task cancellation

## Why It Matters

Dropping a `JoinHandle` doesn't cancel the task—it just detaches it. For graceful shutdown, you need explicit cancellation. `tokio_util::sync::CancellationToken` provides a cooperative cancellation mechanism that tasks can check and respond to, enabling clean resource cleanup.

## Bad

```rust
// Dropping handle doesn't stop the task
let handle = tokio::spawn(async {
    loop {
        do_work().await;
    }
});

drop(handle);  // Task continues running in background!

// Using bool flag - not async-aware
let running = Arc::new(AtomicBool::new(true));

tokio::spawn({
    let running = running.clone();
    async move {
        while running.load(Ordering::Relaxed) {
            do_work().await;  // Can't wake up if blocked here
        }
    }
});

running.store(false, Ordering::Relaxed);
// Task won't stop until current do_work() completes
```

## Good

```rust
use tokio_util::sync::CancellationToken;

let token = CancellationToken::new();

let handle = tokio::spawn({
    let token = token.clone();
    async move {
        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    println!("Shutting down gracefully");
                    cleanup().await;
                    break;
                }
                _ = do_work() => {
                    // Work completed
                }
            }
        }
    }
});

// Later: trigger cancellation
token.cancel();
handle.await?;  // Task completes cleanly
```

## CancellationToken API

```rust
use tokio_util::sync::CancellationToken;

async fn token_example() {
    let token = CancellationToken::new();
    let shared = token.clone();
    let child = token.child_token();
    assert!(!token.is_cancelled());
    shared.cancel();
    token.cancelled().await;
    assert!(child.is_cancelled());
}
```

## Hierarchical Cancellation

Dropping a token does not cancel it. Cancel explicitly or use a drop guard when child tasks must be notified on every return path, including errors.

```rust
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::sync::CancellationToken;

async fn run_server(shutdown: CancellationToken) -> std::io::Result<()> {
    let _cancel_on_exit = shutdown.clone().drop_guard();
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        tokio::select! {
            _ = shutdown.cancelled() => break,
            connection = listener.accept() => {
                let (socket, _) = connection?;
                let child = shutdown.child_token();
                tokio::spawn(async move {
                    if let Err(error) = handle_connection(socket, child).await {
                        tracing::warn!(%error, "connection failed");
                    }
                });
            }
        }
    }
    // The guard cancels all child tokens. Join connection handles as well
    // when the caller needs to wait for their cleanup before returning.
    Ok(())
}

async fn handle_connection(mut socket: TcpStream, token: CancellationToken) -> std::io::Result<()> {
    let mut buffer = [0_u8; 1024];
    loop {
        tokio::select! {
            _ = token.cancelled() => return Ok(()),
            read = socket.read(&mut buffer) => {
                if read? == 0 { return Ok(()); }
            }
        }
    }
}
```

## Graceful Shutdown Pattern

```rust
use anyhow::{Context, Result};
use std::time::Duration;
use tokio::{signal, task::JoinSet};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<()> {
    let shutdown = CancellationToken::new();
    let trigger = shutdown.clone();
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to register Ctrl+C listener");
        trigger.cancel();
    });
    run_app(shutdown).await
}

async fn worker(shutdown: CancellationToken) -> Result<()> {
    shutdown.cancelled().await;
    Ok(())
}

async fn run_app(shutdown: CancellationToken) -> Result<()> {
    let _cancel_on_exit = shutdown.clone().drop_guard();
    let mut tasks = JoinSet::new();
    tasks.spawn(worker(shutdown.child_token()));
    tasks.spawn(worker(shutdown.child_token()));
    tokio::select! {
        _ = shutdown.cancelled() => {},
        Some(result) = tasks.join_next() => { result??; },
    }
    // Cancel siblings even when a task completed before the signal arrived.
    shutdown.cancel();
    tokio::time::timeout(Duration::from_secs(30), async {
        while let Some(result) = tasks.join_next().await { result??; }
        Ok::<(), anyhow::Error>(())
    }).await.context("workers exceeded the shutdown grace period")??;
    Ok(())
}
```

## DropGuard Pattern

```rust
use tokio_util::sync::CancellationToken;

// Auto-cancel on drop
let token = CancellationToken::new();
let guard = token.clone().drop_guard();

tokio::spawn({
    let token = token.clone();
    async move {
        token.cancelled().await;
        println!("Cancelled!");
    }
});

drop(guard);  // Automatically calls token.cancel()
```

## See Also

- [async-joinset-structured](./async-joinset-structured.md) - Managing multiple tasks
- [async-select-racing](./async-select-racing.md) - select! for cancellation
- [async-tokio-runtime](./async-tokio-runtime.md) - Runtime shutdown
