# own-borrow-over-clone

> Prefer `&T` borrowing over `.clone()`

## Why It Matters

Borrow when a function only needs temporary access to data. Cloning owned buffers such as `String` or `Vec<T>` can allocate and duplicate their contents, while `Arc::clone` and `Rc::clone` share the same allocation by incrementing a reference count. Avoid unnecessary expensive clones; use ownership or cloning when the lifetime or value semantics require it.

## Bad

```rust
fn process(data: &String) {
    let local = data.clone();  // Unnecessary allocation!
    println!("{}", local);
}

fn count_words(text: &String) -> usize {
    let owned = text.clone();  // Why clone just to read?
    owned.split_whitespace().count()
}

// Clone in a loop - multiplied cost
fn process_all(items: &[String]) {
    for item in items {
        let copy = item.clone();  // N allocations!
        handle(&copy);
    }
}
```

## Good

```rust
fn process(data: &str) {  // Accept &str, more flexible
    println!("{}", data);  // No allocation needed
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()  // Just borrow
}

// Borrow in a loop - zero allocations
fn process_all(items: &[String]) {
    for item in items {
        println!("{item}");  // Read through a reference
    }
}
```

## When Clone Is Acceptable

```rust
use std::collections::HashMap;

// 1. Need owned data for storage
struct Cache {
    data: HashMap<String, String>,
}

impl Cache {
    fn insert(&mut self, key: &str, value: &str) {
        // Clone needed - we're storing owned data
        self.data.insert(key.to_string(), value.to_string());
    }
}

// 2. Copy non-static input for thread::spawn; scoped threads can borrow it
fn spawn_worker(data: &str) -> std::thread::JoinHandle<()> {
    let owned = data.to_owned();
    std::thread::spawn(move || println!("{owned}"))
}

// 3. Copy types (no heap allocation)
let x: i32 = 42;
let y = x;  // Copy, not clone - this is fine
```

## Evidence

The standard library documents `String::clone` as duplicating its buffer and `Arc::clone` as sharing the existing allocation. The Google Rust Style Guide likewise distinguishes unnecessary deep copies from expected reference-counted pointer clones. Choose based on the operation and ownership contract, rather than treating every `.clone()` as an allocation.

## See Also

- [own-slice-over-vec](own-slice-over-vec.md) - Accept slices instead of references to collections
- [own-cow-conditional](own-cow-conditional.md) - Use Cow for conditional ownership
- [mem-clone-from](mem-clone-from.md) - Reuse allocations when cloning
- [mem-take-replace](mem-take-replace.md) - Move out of &mut without cloning
