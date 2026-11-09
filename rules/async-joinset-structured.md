# async-joinset-structured

> Use `JoinSet` for managing dynamic collections of spawned tasks

## Why It Matters

When spawning a variable number of tasks, collecting `JoinHandle`s in a `Vec` and using `join_all` works but lacks flexibility. `JoinSet` provides a better abstraction: add/remove tasks dynamically, get results as they complete, and abort all on drop. It's the idiomatic way to manage task collections.

## Bad

```rust
// Manual handle management
let mut handles: Vec<JoinHandle<Result<Data>>> = Vec::new();

for url in urls {
    handles.push(tokio::spawn(fetch(url)));
}

// Wait for all, in order (not as they complete)
let results = futures::future::join_all(handles).await;

// No easy way to cancel all, handle errors progressively, or add more tasks
```

## Good

<!-- verify: compile -->
```rust
use tokio::task::JoinSet;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), tokio::task::JoinError> {
    let mut tasks = JoinSet::new();
    for value in 1..=3 {
        tasks.spawn(async move { value * 2 });
    }
    let mut results = Vec::new();
    while let Some(result) = tasks.join_next().await {
        results.push(result?);
    }
    results.sort_unstable();
    assert_eq!(results, [2, 4, 6]);
    assert!(tasks.is_empty());
    Ok(())
}
```

## Dynamic Task Addition

```rust
use tokio::task::JoinSet;

async fn worker_pool(mut rx: mpsc::Receiver<Task>) {
    let mut set = JoinSet::new();
    let max_concurrent = 10;
    
    loop {
        tokio::select! {
            // Accept new tasks if under limit
            Some(task) = rx.recv(), if set.len() < max_concurrent => {
                set.spawn(process_task(task));
            }
            
            // Process completed tasks
            Some(result) = set.join_next() => {
                handle_result(result);
            }
            
            // Exit when no tasks and channel closed
            else => break,
        }
    }
}
```

## Abort on Drop

Dropping a `JoinSet` aborts its remaining async tasks. `abort_all()` requests cancellation without removing tasks from the set; drain the set when you need to observe their completion. A blocking task that has started cannot be stopped by aborting its handle.

```rust
use tokio::task::JoinSet;

async fn abort_explicitly() {
    let mut tasks = JoinSet::new();
    tasks.spawn(std::future::pending::<()>());
    tasks.abort_all();
    while let Some(result) = tasks.join_next().await {
        assert!(result.expect_err("pending task was cancelled").is_cancelled());
    }
}

async fn abort_by_leaving_scope() {
    let mut tasks = JoinSet::new();
    tasks.spawn(std::future::pending::<()>());
    // Dropping tasks on return requests abortion of the pending task.
}
```

## Error Handling Pattern

```rust
use tokio::task::JoinSet;

async fn fetch_all(urls: &[String]) -> Vec<Result<Data, Error>> {
    let mut set = JoinSet::new();
    let mut results = Vec::new();
    
    for url in urls {
        set.spawn(fetch(url.clone()));
    }
    
    while let Some(join_result) = set.join_next().await {
        let result = match join_result {
            Ok(task_result) => task_result,
            Err(join_error) => {
                if join_error.is_panic() {
                    Err(Error::TaskPanicked)
                } else {
                    Err(Error::TaskCancelled)
                }
            }
        };
        results.push(result);
    }
    
    results
}
```

## With Cancellation

```rust
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

async fn run_workers(shutdown: CancellationToken) {
    let mut set = JoinSet::new();
    
    for i in 0..4 {
        let token = shutdown.child_token();
        set.spawn(async move {
            loop {
                tokio::select! {
                    _ = token.cancelled() => break,
                    _ = do_work(i) => {}
                }
            }
        });
    }
    
    // Wait for shutdown
    shutdown.cancelled().await;
    
    // Abort remaining tasks
    set.abort_all();
    
    // Wait for all to finish (drain aborted tasks)
    while set.join_next().await.is_some() {}
}
```

## Spawning with Context

```rust
use tokio::task::JoinSet;

let mut set: JoinSet<(usize, Result<Data, Error>)> = JoinSet::new();

for (index, url) in urls.iter().enumerate() {
    let url = url.clone();
    set.spawn(async move {
        (index, fetch(&url).await)
    });
}

// Results include their index
while let Some(result) = set.join_next().await {
    if let Ok((index, data)) = result {
        results[index] = Some(data);
    }
}
```

## JoinSet vs join_all

| Feature | JoinSet | join_all |
|---------|---------|----------|
| Add tasks dynamically | Yes | No |
| Results as-completed | Yes | No (all at once) |
| Abort all on drop | Yes | No |
| Cancel individual | Yes | No |
| Memory efficient | Yes | Pre-allocates |

## See Also

- [async-join-parallel](./async-join-parallel.md) - Static concurrent futures
- [async-cancellation-token](./async-cancellation-token.md) - Cancellation patterns
- [async-try-join](./async-try-join.md) - Error handling in joins
