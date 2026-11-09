# async-yield-cpu

> Bound uninterrupted CPU work on async workers and offload sustained computation

## Why It Matters

An async function can perform a long computation without giving its executor another opportunity to poll ready tasks. An `.await` on an immediately ready future need not suspend execution. Process a measured amount of work between cooperative scheduling points, and use a bounded compute pool for sustained CPU workloads.

## Bad

```rust
pub async fn checksum(data: &[u8]) -> u64 {
    let mut sum = 0_u64;
    for &byte in data {
        sum = sum.wrapping_add(u64::from(byte));
    }
    sum
}
```

Polling this future processes the entire input before returning.

## Good

```rust
pub async fn checksum(data: &[u8]) -> u64 {
    let mut sum = 0_u64;
    for chunk in data.chunks(1024) {
        for &byte in chunk {
            sum = sum.wrapping_add(u64::from(byte));
        }
        tokio::task::consume_budget().await;
    }
    sum
}
```

Here each chunk consumes one unit of Tokio's cooperative budget; suspension happens when that budget is exhausted. The chunk size and budget together determine how much computation can occur before a yield. Choose a chunk from measured execution time and tail latency; `1024` is an example, not a scheduling guarantee.

## Key Points

- For occasional short work, `yield_now().await` is another scheduling point; repeatedly yielding after every trivial operation can waste throughput.
- Neither a scheduler yield nor an I/O `.await` promises that another particular task will run next. A single expensive operation cannot be preempted by a yield after it.
- `has_budget_remaining()` only observes a budget; pure CPU loops must consume budget or explicitly yield.
- Put runtime-specific cooperation behind an adapter when a library supports multiple executors.
- Bound CPU-job admission before spawning work. `spawn_blocking` avoids blocking an async worker but does not itself bound concurrency or queue memory.
- Release unnecessary guards before yielding, and account for cancellation if intermediate effects are externally visible.

## See Also

- [async-spawn-blocking](async-spawn-blocking.md) - isolate blocking and sustained CPU work
- [async-bounded-channel](async-bounded-channel.md) - keep overload bounded
- [async-cancel-safety](async-cancel-safety.md) - consider partial work when cancellation drops the future
- [perf-profile-first](perf-profile-first.md) - measure throughput and tail latency

## Sources

- [Microsoft M-YIELD-POINTS](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-YIELD-POINTS)
- [Tokio cooperative budget](https://docs.rs/tokio/latest/tokio/task/coop/fn.consume_budget.html)
