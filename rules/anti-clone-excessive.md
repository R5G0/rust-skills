# anti-clone-excessive

> Don't clone when borrowing works

## Why It Matters

`.clone()` allocates memory and copies data. When you only need to read data, borrowing (`&T`) is free. Excessive cloning wastes memory, CPU cycles, and often indicates misunderstanding of ownership.

## Bad

```rust
// Cloning to pass to a function that only reads
fn print_name(name: String) {  // Takes ownership
    println!("{}", name);
}
let name = "Alice".to_string();
print_name(name.clone());  // Unnecessary clone
print_name(name);          // Could have just done this

// Cloning in a loop
for item in items.clone() {  // Clones entire Vec
    process(&item);
}

// Cloning for comparison
if input.clone() == expected {  // Pointless clone
    // ...
}

// Cloning struct fields
fn get_name(&self) -> String {
    self.name.clone()  // Caller might not need ownership
}
```

## Good

<!-- verify: compile -->
```rust
struct User {
    name: String,
}

impl User {
    fn name(&self) -> &str {
        &self.name
    }
}

fn total_name_bytes(users: &[User]) -> usize {
    users.iter().map(|user| user.name().len()).sum()
}

fn main() {
    let users = vec![User { name: "Alice".to_owned() }];
    assert_eq!(total_name_bytes(&users), 5);
    // The collection and its names are still owned by the caller.
    assert_eq!(users[0].name(), "Alice");
}
```

## When to Clone

```rust
// Need owned data for async move
let name = name.clone();
tokio::spawn(async move {
    process(name).await;
});

// Storing in a new struct
struct Cache {
    data: String,
}
impl Cache {
    fn store(&mut self, data: &str) {
        self.data = data.to_string();  // Must own
    }
}

// Multiple owners (use Arc instead if frequent)
let shared = data.clone();
thread::spawn(move || use_data(shared));
```

## Alternatives to Clone

| Instead of | Use |
|------------|-----|
| `s.clone()` for reading | `&s` |
| `vec.clone()` for iteration | `&vec` or `vec.iter()` |
| `Clone` for shared ownership | `Arc<T>` |
| Clone in hot loop | Move outside loop |
| `s.to_string()` from `&str` | Accept `&str` if possible |

## Pattern: Clone on Write

```rust
use std::borrow::Cow;

fn process(input: Cow<str>) -> Cow<str> {
    if needs_modification(&input) {
        Cow::Owned(modify(&input))  // Clone only if needed
    } else {
        input  // No clone
    }
}
```

## Detecting Excessive Clones

```toml
# Cargo.toml
[lints.clippy]
clone_on_copy = "warn"
clone_on_ref_ptr = "warn"
redundant_clone = "warn"
```

## See Also

- [own-borrow-over-clone](./own-borrow-over-clone.md) - Borrowing patterns
- [own-cow-conditional](./own-cow-conditional.md) - Clone on write
- [own-arc-shared](./own-arc-shared.md) - Shared ownership
