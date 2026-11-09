# own-rwlock-readers

> Use `RwLock<T>` when reads significantly outnumber writes

## Why It Matters

`Mutex<T>` serializes readers as well as writers; `RwLock<T>` permits multiple readers or one exclusive writer. Concurrent reads can improve throughput, but reader bookkeeping, contention, and the duration of critical sections affect the result. Benchmark against a mutex for the actual workload rather than choosing solely by the read/write ratio.

## Bad

```rust
use std::sync::{Arc, Mutex};

// Configuration rarely changes but is read constantly
let config = Arc::new(Mutex::new(Config::load()));

// Every read blocks other reads unnecessarily
fn get_setting(config: &Mutex<Config>, key: &str) -> String {
    let guard = config.lock().unwrap();
    guard.get(key).to_string()
}

// 100 threads reading = serialized, one at a time
```

## Good

```rust
use std::sync::{Arc, RwLock};

// Multiple readers can proceed concurrently
let config = Arc::new(RwLock::new(Config::load()));

fn get_setting(config: &RwLock<Config>, key: &str) -> String {
    let guard = config.read().unwrap(); // Multiple threads can hold read lock
    guard.get(key).to_string()
}

fn update_setting(config: &RwLock<Config>, key: &str, value: &str) {
    let mut guard = config.write().unwrap(); // Exclusive access for writes
    guard.set(key, value);
}

// 100 threads reading = parallel execution
```

## parking_lot::RwLock

Consider `parking_lot::RwLock` when its upgradable reads, poisoning behavior, or measured performance fit the application:

```rust
use parking_lot::RwLock;
use std::collections::HashMap;

let data = RwLock::new(HashMap::<String, String>::new());

// Read - no unwrap needed
let value = data.read().get("key").cloned();

// Write
data.write().insert("key".to_string(), "value".to_string());

// An upgradable read keeps the check and insertion in one lock operation
let upgradeable = data.upgradable_read();
if upgradeable.get("key").is_none() {
    let mut write = parking_lot::RwLockUpgradableReadGuard::upgrade(upgradeable);
    write.insert("key".to_string(), "default".to_string());
}
```

## When RwLock Hurts

RwLock has overhead for tracking readers. It can be slower than Mutex when:

| Scenario | Candidate to evaluate |
|----------|-----------------------|
| Writes contend with reads or writes | `Mutex` may be simpler or faster |
| Very short critical sections | Compare mutex overhead against reader bookkeeping |
| Single-threaded shared mutation | `RefCell` may suffice |
| Substantial read sections that can overlap | `RwLock` may benefit from concurrent readers |

## Scheduling and Downgrading

The standard `RwLock` leaves reader/writer priority to the operating system; do not assume a fairness or starvation guarantee. `parking_lot` documents its own locking policy and explicit `unlock_fair` operations, but its default unlocking behavior differs across versions. Check the version in use before relying on a scheduling guarantee, and avoid recursive read locking when writers may be waiting.

`std::sync::RwLockWriteGuard::downgrade` has been stable since Rust 1.92. It atomically converts an exclusive write guard into a shared read guard, preventing another writer from modifying the value between the write and subsequent reads. This controls lock mode, not scheduling fairness.

```rust
use std::sync::{RwLock, RwLockWriteGuard};

let value = RwLock::new(0);
let mut write = value.write().expect("lock was not poisoned");
*write = 42;
let read = RwLockWriteGuard::downgrade(write);
assert_eq!(*read, 42);
```

## Real-World Pattern: Cached Computation

Computing outside the write lock reduces lock hold time, but concurrent misses can duplicate work. This pattern assumes that computation is repeatable and replacing one result with another is valid. Use an initialization primitive or recheck under a write lock when those assumptions do not hold.

```rust
use parking_lot::RwLock;
use std::sync::Arc;

struct CachedData {
    cache: RwLock<Option<ExpensiveResult>>,
}

impl CachedData {
    fn get(&self) -> ExpensiveResult {
        // Fast path: read lock
        if let Some(cached) = self.cache.read().as_ref() {
            return cached.clone();
        }
        
        // Slow path: compute and cache
        let result = compute_expensive();
        *self.cache.write() = Some(result.clone());
        result
    }
}
```

## See Also

- [own-mutex-interior](./own-mutex-interior.md) - When writes are frequent
- [async-no-lock-await](./async-no-lock-await.md) - RwLock in async contexts
