# conc-atomic-ordering

> Use the weakest correct memory `Ordering` for every atomic operation

## Why It Matters

Memory ordering is part of a synchronization protocol. `SeqCst` is a reasonable conservative starting point, but it cannot repair an invalid ownership or publication protocol. Weaken an ordering only after proving that the required happens-before relationships remain valid; performance differences depend on the operation and target architecture.

## Bad

```rust
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);
static DATA: AtomicU64 = AtomicU64::new(0);

// The independent counter does not need ordering with other memory.
fn increment() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}

fn producer() {
    DATA.store(42, Ordering::SeqCst);
    READY.store(true, Ordering::SeqCst); // overkill for a single flag
}

fn consumer() -> Option<u64> {
    if READY.load(Ordering::SeqCst) {
        Some(DATA.load(Ordering::SeqCst))
    } else {
        None
    }
}
```

## Good

```rust
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

// Relaxed: no ordering relative to other memory — fine for independent counters
fn increment() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
}

fn total() -> u64 {
    COUNTER.load(Ordering::Relaxed)
}

// Acquire/Release: paired handoff — producer writes data THEN sets flag (Release);
// consumer loads flag (Acquire) and is guaranteed to see the preceding write.
static READY: AtomicBool = AtomicBool::new(false);
static VALUE: AtomicU64 = AtomicU64::new(0);

fn producer(value: u64) {
    VALUE.store(value, Ordering::Relaxed);   // write payload first
    READY.store(true, Ordering::Release);    // publish with Release
}

fn consumer() -> Option<u64> {
    if READY.load(Ordering::Acquire) {       // synchronize with Release store
        Some(VALUE.load(Ordering::Relaxed))  // payload visible after Acquire
    } else {
        None
    }
}

// SeqCst adds a total order over SeqCst operations.
// Example: Dekker-style mutual exclusion involving two independent flags.
```

## Ordering Quick Reference

The publication example assumes one publication and no later writes to the payload. Repeated producers need an additional protocol if the reader must identify a particular payload version; a permanently true readiness flag does not provide that guarantee. Atomic payloads avoid a data race, but do not make compound state transitions atomic.

| Ordering | Use when |
|----------|----------|
| `Relaxed` | Operation is atomic but needs no ordering relative to other memory (counters, stats) |
| `Acquire` | Load that must see all stores preceding a matching `Release` on another thread |
| `Release` | Store that must be visible before a matching `Acquire` load on another thread |
| `AcqRel` | Read-modify-write (e.g. `compare_exchange`) acting as both Acquire and Release |
| `SeqCst` | Need a single global order observed by all threads across multiple atomic variables |

## Verification with loom

Use the `loom` crate to exhaustively verify ordering choices for small concurrent units — it explores all interleavings the C11 memory model permits:

```rust
#[cfg(loom)]
use loom::sync::atomic::{AtomicBool, Ordering};

#[cfg(loom)]
#[test]
fn test_handoff() {
    loom::model(|| {
        // ... spawn threads, assert invariants
    });
}
```

## See Also

- [conc-atomic-update](conc-atomic-update.md) - replace hand-written compare-exchange retry loops
- [own-mutex-interior](own-mutex-interior.md) - prefer `Mutex<T>` when lock-free isn't required
- [test-loom-concurrency](test-loom-concurrency.md) - exhaustively test concurrent code with loom
- [conc-scoped-threads](conc-scoped-threads.md) - safely share stack data across threads
