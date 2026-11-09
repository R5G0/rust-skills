# async-future-size

> Measure hot future sizes and avoid capturing large setup state unnecessarily

## Why It Matters

An async function captures its arguments before it is first polled. Locals needed across a suspension point can also occupy its state machine. Frequently instantiated large futures increase task memory and movement before pinning; optimize these costs when profiling identifies a relevant hot path.

## Bad

```rust
pub struct Payload([u8; 4096]);

pub async fn first_byte(payload: Payload) -> u8 {
    tokio::task::yield_now().await;
    payload.0[0]
}
```

The payload must remain available after the suspension point even though only one byte is needed.

## Good

```rust
use std::future::Future;

pub struct Payload([u8; 4096]);

pub fn first_byte(payload: Payload) -> impl Future<Output = u8> {
    let first = payload.0[0];
    async move {
        tokio::task::yield_now().await;
        first
    }
}

pub fn future_bytes() -> usize {
    let future = first_byte(Payload([7; 4096]));
    std::mem::size_of_val(&future)
}
```

The returned future captures one byte. The setup now runs when the function is called, before polling: preserve the original timing of validation, side effects, and resource ownership if they are part of the API contract.

## Key Points

- Inspect drop scopes as well as explicit reads: a value with a destructor may remain live until the end of its scope.
- Use `size_of_val` on representative future values. A size threshold is a target/compiler regression budget, not a stable ABI.
- Scope or consume large temporary state before suspension when its contents are no longer needed.
- Boxing a rare large branch can shrink inline storage, but adds allocation and does not eliminate the stored data. Measure the complete tradeoff.
- Keep ordinary `async fn` when its state is already small. Returning `impl Future` is useful here because setup happens outside the captured state, not because the spelling alone saves memory.
- Shrinking a future does not make sustained CPU work cooperative.

## See Also

- [mem-assert-type-size](mem-assert-type-size.md) - apply size budgets to measured hot types
- [async-clone-before-await](async-clone-before-await.md) - keep required owned handles without deep-copying data
- [async-yield-cpu](async-yield-cpu.md) - separate future storage from execution cost

## Sources

- [Microsoft M-ASYNC-STACK-SIZE](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-ASYNC-STACK-SIZE)
- [Rust Reference: async function arguments and future creation](https://doc.rust-lang.org/reference/items/functions.html#async-functions)
