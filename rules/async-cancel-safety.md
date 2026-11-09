# async-cancel-safety

> Preserve progress and deadlines when select branches are cancelled and restarted

## Why It Matters

When a `tokio::select!` branch wins, the other branch futures are dropped. State owned by those futures is lost; external effects already performed are not rolled back. Keeping a buffer outside the future is insufficient if the progress cursor is still inside it. Review whether dropping and recreating each pending operation preserves the protocol, including queue position and deadlines.

## Bad

```rust
use tokio::{io::AsyncReadExt, net::TcpStream, sync::mpsc};

async fn receive(stream: &mut TcpStream, rx: &mut mpsc::Receiver<u8>) {
    let mut packet = [0_u8; 64];
    loop {
        tokio::select! {
            result = stream.read_exact(&mut packet) => {
                if result.is_err() { break; }
            }
            message = rx.recv() => {
                if message.is_none() { break; }
                // read_exact may have consumed bytes into packet.
                // Recreating it starts its internal cursor at zero.
            }
        }
    }
}
```

## Good

```rust
use tokio::{io::AsyncReadExt, net::TcpStream, sync::mpsc};

async fn receive(stream: &mut TcpStream, rx: &mut mpsc::Receiver<u8>) -> std::io::Result<()> {
    let mut packet = [0_u8; 64];
    let mut filled = 0;
    loop {
        tokio::select! {
            result = stream.read(&mut packet[filled..]) => {
                let count = result?;
                if count == 0 {
                    return if filled == 0 { Ok(()) } else {
                        Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "incomplete packet"))
                    };
                }
                filled += count;
                if filled == packet.len() {
                    println!("packet: {packet:?}");
                    filled = 0;
                }
            }
            message = rx.recv() => {
                if message.is_none() { return Ok(()); }
            }
        }
    }
}
```

Here `read` reports each completed chunk and the surrounding scope retains the cursor. Channel closure and EOF terminate the loop instead of repeatedly polling an immediately ready branch.

## Keep Deadlines Outside the Loop

Creating `sleep(duration)` on every iteration postpones the timeout whenever another branch wins. For a deadline covering the whole operation, create and pin one timer. Reset it only when an inactivity timeout is intended.

```rust
use std::time::Duration;
use tokio::sync::mpsc;

async fn receive_until_deadline(mut rx: mpsc::Receiver<u8>, duration: Duration) {
    let deadline = tokio::time::sleep(duration);
    tokio::pin!(deadline);
    loop {
        tokio::select! {
            _ = &mut deadline => break,
            message = rx.recv() => match message {
                Some(value) => println!("message: {value}"),
                None => break,
            }
        }
    }
}
```

Pinning preserves a future's location so a pending future can be polled again through a mutable reference. Moving a `Pin<Box<F>>` handle does not move the pinned `F`. Pinning does not make dropping that future cancellation-safe, and a completed future must not be polled again unless its API explicitly permits it.

## Operation Contracts

| Operation | Effect of cancelling and recreating |
|---|---|
| `mpsc::Receiver::recv`, `broadcast::Receiver::recv`, `watch::Receiver::changed` | Documented cancellation-safe |
| `AsyncReadExt::read`, `read_buf` | Documented cancellation-safe; keep completed data externally |
| `read_exact`, `read_to_end`, `read_to_string`, `write_all` | Partial I/O may have occurred; restarting can lose protocol progress |
| `Mutex::lock`, `RwLock::read`/`write`, `Semaphore::acquire` | Cancellation loses queue position; repeated retries may starve |
| `sleep(duration)` | Dropping cancels this timer; recreating starts a new deadline |
| `&mut JoinHandle<T>` | Losing the select does not cancel the task or consume its result |

To preserve an operation, create and pin it once outside the loop and select on `&mut future`. When it completes, stop polling it. If using a spawned task, retain its handle and define its shutdown behavior: dropping an owned `JoinHandle` detaches the task rather than aborting it. Cancellation can be intentional during shutdown, but must not silently corrupt an ongoing protocol.

## Sources

- [Tokio select cancellation safety](https://docs.rs/tokio/latest/tokio/macro.select.html#cancellation-safety)
- [Google Comprehensive Rust: cancellation](https://google.github.io/comprehensive-rust/concurrency/async-pitfalls/cancellation.html)
- [Google Comprehensive Rust: pinning](https://google.github.io/comprehensive-rust/concurrency/async-pitfalls/pin.html)

## See Also

- [async-select-racing](async-select-racing.md) - Racing futures and timeouts
- [async-joinset-structured](async-joinset-structured.md) - Task lifecycle
- [async-cancellation-token](async-cancellation-token.md) - Cooperative shutdown
