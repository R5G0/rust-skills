# async-bounded-channel

> Use bounded channels to apply backpressure and prevent unbounded memory growth

## Why It Matters

Unbounded channels grow without limit when producers outpace consumers. In production, this leads to memory exhaustion. Bounded channels apply backpressure—producers wait when the channel is full, naturally throttling the system. This prevents OOM and makes resource usage predictable.

## Bad

```rust
use tokio::sync::mpsc;

// Unbounded channel - can grow forever
let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

// Fast producer, slow consumer = unbounded memory growth
tokio::spawn(async move {
    loop {
        let msg = generate_message();
        tx.send(msg).unwrap();  // Never blocks, never fails (until OOM)
    }
});

tokio::spawn(async move {
    while let Some(msg) = rx.recv().await {
        slow_process(msg).await;  // Can't keep up
    }
});
// Memory grows unboundedly until crash
```

## Good

<!-- verify: compile -->
```rust
use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(2);
    tx.send(1).await?;
    tx.send(2).await?;
    // Full capacity rejects try_send; send().await would wait for space.
    assert!(matches!(tx.try_send(3), Err(mpsc::error::TrySendError::Full(3))));
    assert_eq!(rx.recv().await, Some(1));
    tx.send(3).await?;
    drop(tx);
    assert_eq!(rx.recv().await, Some(2));
    assert_eq!(rx.recv().await, Some(3));
    assert_eq!(rx.recv().await, None);
    Ok(())
}
```

## Choosing Buffer Size

```rust
// Too small: frequent blocking, reduced throughput
let (tx, rx) = mpsc::channel::<Item>(1);

// Too large: delayed backpressure, memory waste
let (tx, rx) = mpsc::channel::<Item>(1_000_000);

// Guidelines:
// - Start with expected burst size
// - Measure actual usage in production
// - Err on the smaller side initially

// Small items, high throughput
let (tx, rx) = mpsc::channel::<u64>(1000);

// Large items, moderate throughput  
let (tx, rx) = mpsc::channel::<LargeStruct>(100);

// Low latency requirement
let (tx, rx) = mpsc::channel::<Command>(10);
```

## Handling Full Channel

```rust
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

let (tx, mut rx) = mpsc::channel::<Message>(100);

// Option 1: Wait indefinitely (default)
tx.send(msg).await?;

// Option 2: Try send, fail if full
match tx.try_send(msg) {
    Ok(()) => println!("Sent"),
    Err(TrySendError::Full(msg)) => {
        println!("Channel full, dropping message");
    }
    Err(TrySendError::Closed(msg)) => {
        println!("Receiver dropped");
    }
}

// Option 3: Timeout
match timeout(Duration::from_secs(1), tx.send(msg)).await {
    Ok(Ok(())) => println!("Sent"),
    Ok(Err(_)) => println!("Channel closed"),
    Err(_) => println!("Timeout - channel full for too long"),
}

// Option 4: send with permit reservation
let permit = tx.reserve().await?;
permit.send(msg);  // Guaranteed to succeed
```

## Channel Types

```rust
// mpsc: many producers, single consumer
let (tx, rx) = mpsc::channel::<Message>(100);
let tx2 = tx.clone();  // Can clone sender

// oneshot: single value, one producer, one consumer
let (tx, rx) = oneshot::channel::<Response>();
tx.send(response);  // Can only send once

// broadcast: multiple consumers, each gets all messages
let (tx, _) = broadcast::channel::<Event>(100);
let mut rx1 = tx.subscribe();
let mut rx2 = tx.subscribe();

// watch: single latest value, multiple consumers
let (tx, rx) = watch::channel::<State>(initial);
// Receivers see latest value, not all values
```

## Worker Pool Pattern

The receiver mutex serializes dequeuing, while processing happens after releasing it. This example sorts the final values for deterministic output; worker execution order is unspecified.

```rust
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

async fn process_with_workers(items: Vec<u32>) -> Result<Vec<u32>, tokio::task::JoinError> {
    let (tx, rx) = mpsc::channel::<u32>(100);
    let rx = Arc::new(Mutex::new(rx));
    let workers: Vec<_> = (0..4).map(|_| {
        let rx = Arc::clone(&rx);
        tokio::spawn(async move {
            let mut output = Vec::new();
            loop {
                let item = rx.lock().await.recv().await;
                match item {
                    Some(item) => output.push(item.saturating_mul(2)),
                    None => break,
                }
            }
            output
        })
    }).collect();
    for item in items {
        tx.send(item).await.expect("workers stay alive while the producer sends");
    }
    drop(tx);
    let mut output = Vec::new();
    for worker in workers {
        output.extend(worker.await?);
    }
    output.sort_unstable();
    Ok(output)
}
```

## See Also

- [async-mpsc-queue](./async-mpsc-queue.md) - Multi-producer patterns
- [async-oneshot-response](./async-oneshot-response.md) - Request-response pattern
- [async-watch-latest](./async-watch-latest.md) - Latest-value broadcasting
