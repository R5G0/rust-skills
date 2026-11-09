# own-mutex-interior

> Use `Mutex<T>` for interior mutability across threads

## Why It Matters

When shared mutable state must cross thread boundaries, `Mutex<T>` serializes access through a lock guard. `Mutex<T>` is `Send` and `Sync` when `T: Send`; unlike `RwLock<T>`, it does not require `T: Sync` to share the mutex. `RefCell<T>` alone cannot synchronize access between threads.

## Bad

```rust
use std::cell::RefCell;
use std::sync::Arc;

// RefCell is !Sync - this won't compile
let shared = Arc::new(RefCell::new(vec![]));

// ERROR: RefCell cannot be shared between threads safely
std::thread::spawn({
    let shared = shared.clone();
    move || shared.borrow_mut().push(1)
});
```

## Good

```rust
use std::sync::{Arc, Mutex};

let shared = Arc::new(Mutex::new(vec![]));

let handles: Vec<_> = (0..10).map(|i| {
    let shared = shared.clone();
    std::thread::spawn(move || {
        let mut data = shared.lock().unwrap();
        data.push(i);
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}

println!("{:?}", shared.lock().unwrap()); // All values present
```

## Mutex Poisoning

If a thread panics while holding a lock, the mutex becomes "poisoned":

```rust
use std::sync::{Arc, Mutex};

let mutex = Arc::new(Mutex::new(0));

// Handle poisoning gracefully
match mutex.lock() {
    Ok(guard) => println!("Value: {}", *guard),
    Err(poisoned) => {
        // Recover only after deciding the protected invariants still hold
        let guard = poisoned.into_inner();
        println!("Recovered value: {}", *guard);
    }
}

// Or ignore poisoning (use with caution)
let guard = mutex.lock().unwrap_or_else(|e| e.into_inner());
```

## Consider parking_lot::Mutex

`parking_lot::Mutex` offers different poisoning, locking, and footprint behavior. Compare it against `std::sync::Mutex` for the target workload rather than assuming a universal speed advantage:

```rust
use parking_lot::Mutex;
use std::sync::Arc;

let shared = Arc::new(Mutex::new(vec![]));

// No poisoning, no Result to unwrap
let mut data = shared.lock();
data.push(42);
// Lock automatically released when guard drops
```

Relevant differences:

- No poisoning: `lock()` returns a guard directly instead of a `LockResult`.
- Its raw mutex uses one byte of lock state; the complete `Mutex<T>` also contains `T` and any padding. Standard-library mutex layout varies with platform and toolchain, so compare `size_of::<Mutex<T>>()` for the actual types and target.
- Contended and uncontended performance depend on workload and platform; benchmark before switching for speed.
- Explicit fair unlocking is available. Default fairness behavior depends on the `parking_lot` version; consult its documentation.

Changing from the standard mutex requires reviewing poisoning recovery, return types, and scheduling assumptions; it is not a drop-in semantic replacement.

## When to Use What

| Type | Threading | Overhead | Use Case |
|------|-----------|----------|----------|
| `RefCell<T>` | Single | Minimal | Interior mutability, same thread |
| `Mutex<T>` | Multi | Locking | Shared mutable state across threads |
| `RwLock<T>` | Multi | Locking | Many readers, few writers |
| `parking_lot::Mutex` | Multi | Workload-dependent | Alternative with no poisoning and explicit fair unlocking |

## See Also

- [own-rwlock-readers](./own-rwlock-readers.md) - When reads dominate writes
- [own-refcell-interior](./own-refcell-interior.md) - Single-threaded alternative
- [async-no-lock-await](./async-no-lock-await.md) - Avoiding locks across await points
- [conc-atomic-ordering](./conc-atomic-ordering.md) - Lock-free alternative for simple state
