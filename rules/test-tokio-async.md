# test-tokio-async

> Use `#[tokio::test]` for async tests

## Why It Matters

Calling an async function creates a future; its body runs when that future is polled. Tokio-powered operations need a compatible runtime. `#[tokio::test]` constructs one and drives the test future automatically, avoiding repetitive runtime setup.

## Bad

```rust
// Won't compile - async fn can't be called without runtime
#[test]
async fn test_async_function() {  // Error!
    let result = fetch_data().await;
    assert!(result.is_ok());
}

// Manual runtime - verbose and error-prone
#[test]
fn test_async_function() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let result = fetch_data().await;
        assert!(result.is_ok());
    });
}
```

## Good

<!-- verify: compile -->
```rust
#[cfg(test)]
mod tests {
    async fn fetch_data(source: &[u8]) -> Result<Vec<u8>, &'static str> {
        tokio::task::yield_now().await;
        if source.is_empty() {
            Err("empty source")
        } else {
            Ok(source.to_vec())
        }
    }

    async fn fetch_user(users: &[(u64, &str)], id: u64) -> Option<String> {
        tokio::task::yield_now().await;
        users.iter().find(|(user_id, _)| *user_id == id)
            .map(|(_, name)| (*name).to_owned())
    }

    #[tokio::test]
    async fn fetch_data_returns_contents_and_rejects_empty_input() {
        assert_eq!(fetch_data(b"payload").await, Ok(b"payload".to_vec()));
        assert_eq!(fetch_data(b"").await, Err("empty source"));
    }

    #[tokio::test]
    async fn concurrent_user_queries_preserve_each_result() {
        let users = [(1, "Alice"), (2, "Bob")];
        let (first, second, missing) = tokio::join!(
            fetch_user(&users, 1),
            fetch_user(&users, 2),
            fetch_user(&users, 99),
        );
        assert_eq!(first.as_deref(), Some("Alice"));
        assert_eq!(second.as_deref(), Some("Bob"));
        assert_eq!(missing, None);
    }
}
```

## Runtime Configuration

The default flavor is `current_thread`, not `multi_thread`. The test macro needs
Tokio's `macros` and runtime features. Selecting `multi_thread` additionally needs
`rt-multi-thread`; `start_paused` and `advance` need `test-util` and `time`.
`full` does not include `test-util`, so enable it explicitly for time-controlled
tests, for example `tokio = { version = "1", features = ["full", "test-util"] }`.

<!-- verify: compile -->
```rust
#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::runtime::{Handle, RuntimeFlavor};
    use tokio::time::{Instant, advance};

    // #[tokio::test] defaults to a current-thread runtime.
    #[tokio::test]
    async fn default_runtime_uses_the_current_thread() {
        assert_eq!(Handle::current().runtime_flavor(), RuntimeFlavor::CurrentThread);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn explicit_current_thread_runtime_drives_spawned_tasks() {
        let result = tokio::spawn(async { 21 * 2 }).await.expect("task completed");
        assert_eq!(result, 42);
        assert_eq!(Handle::current().runtime_flavor(), RuntimeFlavor::CurrentThread);
    }

    // Requires Tokio's rt-multi-thread feature.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn explicitly_selected_runtime_is_multithreaded() {
        assert_eq!(Handle::current().runtime_flavor(), RuntimeFlavor::MultiThread);
    }

    // Requires Tokio's test-util and time features; supported on current_thread.
    #[tokio::test(start_paused = true)]
    async fn paused_time_advances_by_the_requested_duration() {
        let start = Instant::now();
        advance(Duration::from_secs(60)).await;
        assert_eq!(start.elapsed(), Duration::from_secs(60));
    }
}
```

## Testing Timeouts

```rust
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_operation_completes_in_time() {
    let result = timeout(
        Duration::from_secs(5),
        slow_operation()
    ).await;
    
    assert!(result.is_ok(), "Operation timed out");
}

#[tokio::test]
async fn test_timeout_triggers() {
    let result = timeout(
        Duration::from_millis(100),
        never_completes()
    ).await;
    
    assert!(result.is_err(), "Expected timeout");
}
```

## Testing Channels

```rust
use tokio::sync::mpsc;

#[tokio::test]
async fn test_channel_communication() {
    let (tx, mut rx) = mpsc::channel(10);
    
    tokio::spawn(async move {
        tx.send("hello").await.unwrap();
        tx.send("world").await.unwrap();
    });
    
    assert_eq!(rx.recv().await, Some("hello"));
    assert_eq!(rx.recv().await, Some("world"));
    assert_eq!(rx.recv().await, None);
}
```

## Testing with Mocks

```rust
use mockall::*;

#[automock]
#[async_trait::async_trait]
trait Database {
    async fn get_user(&self, id: u64) -> Option<User>;
}

#[tokio::test]
async fn test_with_mock_database() {
    let mut mock = MockDatabase::new();
    mock.expect_get_user()
        .with(eq(42))
        .returning(|_| Some(User { id: 42, name: "Alice".into() }));
    
    let service = UserService::new(mock);
    let user = service.find_user(42).await;
    
    assert_eq!(user.unwrap().name, "Alice");
}
```

## See Also

- [async-tokio-runtime](./async-tokio-runtime.md) - Runtime configuration
- [test-mock-traits](./test-mock-traits.md) - Mocking async traits
- [test-fixture-raii](./test-fixture-raii.md) - Async test cleanup
