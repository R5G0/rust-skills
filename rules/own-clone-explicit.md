# own-clone-explicit

> Use explicit `Clone` for types where copying has meaningful cost

## Why It Matters

`Clone` makes duplication explicit, while `Copy` permits implicit duplication; neither guarantees that copying is free. For owned buffers such as `String` and `Vec<T>`, cloning duplicates contents and can allocate. Implement `Clone` when duplication has a valid meaning, and prefer moving a value when the caller no longer needs it.

## Bad

```rust
// Cloning inside the callee when ownership is already available.
fn process_data(data: Vec<u32>) -> Vec<u32> {
    let mut result = data.clone(); // Unnecessary copy: data is never reused.
    result.reverse();
    result
}
```

## Good

```rust
fn process_data(mut data: Vec<u32>) -> Vec<u32> {
    data.reverse();
    data // Reuse the allocation supplied by the caller.
}

let disposable = vec![1, 2, 3];
let result = process_data(disposable); // Move when the original is no longer needed.

let retained = vec![1, 2, 3];
let result = process_data(retained.clone()); // Clone only to retain the original.
assert_eq!(retained, [1, 2, 3]);

// A read-only operation can instead borrow, without any owned input copy.
fn maximum(data: &[u32]) -> Option<u32> {
    data.iter().copied().max()
}
let largest = maximum(&retained);
```

## Custom Clone Implementation

Derive `Clone` when field-by-field cloning is correct. Implement it manually when the type needs different semantics or an allocation-reusing `clone_from` implementation:

```rust
#[derive(Debug)]
struct Document {
    id: u64,              // Cheap to copy
    content: String,      // Expensive to clone
    metadata: Metadata,   // Moderate cost
}

impl Clone for Document {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            content: self.content.clone(),
            metadata: self.metadata.clone(),
        }
    }
    
    // Optimization: reuse existing allocations
    fn clone_from(&mut self, source: &Self) {
        self.id = source.id;
        self.content.clone_from(&source.content); // Reuses capacity
        self.metadata.clone_from(&source.metadata);
    }
}
```

## clone_from Optimization

`clone_from` can reuse existing allocations:

```rust
let mut buffer = String::with_capacity(1000);

// Bad: drops old allocation, creates new one
buffer = source.clone();

// Good: reuses existing capacity if sufficient
buffer.clone_from(&source);
```

## Derive vs Manual Clone

```rust
// Derive when all fields need cloning
#[derive(Clone)]
struct Simple {
    data: Vec<u8>,
    name: String,
}

// Manual when you need special behavior
struct CachedValue {
    value: i32,
    cache: RefCell<Option<ExpensiveComputation>>,
}

impl Clone for CachedValue {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            cache: RefCell::new(None), // Don't clone cache, let it rebuild
        }
    }
}
```

## When to Avoid Clone

- Borrow when the callee only needs temporary access.
- Move an owned value when the caller is done with it.
- Consider `Cow` when a result can remain borrowed on a common path.
- Use `Arc` when shared ownership is actually needed. `Arc::clone` increments a reference count without cloning the pointee; it does not provide an independent deep copy.

## See Also

- [own-copy-small](./own-copy-small.md) - When implicit Copy is appropriate
- [own-cow-conditional](./own-cow-conditional.md) - Avoiding clones with Cow
- [mem-clone-from](./mem-clone-from.md) - Optimizing repeated clones
