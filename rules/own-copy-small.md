# own-copy-small

> Implement `Copy` for small, simple types

## Why It Matters

Types that implement `Copy` can be implicitly duplicated rather than moved. Use it when duplication matches the type's value semantics and retaining the original is useful. Size can guide performance decisions, but `Copy` does not make copying free and Rust imposes no size limit on the trait.

## Bad

```rust
// Small type without Copy - requires explicit clone
#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn distance(p1: Point, p2: Point) -> f64 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

let origin = Point { x: 0.0, y: 0.0 };
let target = Point { x: 3.0, y: 4.0 };

let d1 = distance(origin.clone(), target.clone()); // Tedious
let d2 = distance(origin.clone(), target.clone()); // Every use needs clone
// origin and target still usable but verbose
```

## Good

```rust
// Small type with Copy - implicit duplication
#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn distance(p1: Point, p2: Point) -> f64 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

let origin = Point { x: 0.0, y: 0.0 };
let target = Point { x: 3.0, y: 4.0 };

let d1 = distance(origin, target); // Implicitly copied
let d2 = distance(origin, target); // Still works!
// origin and target remain valid
```

## Copy Requirements

A type can implement `Copy` only if:
1. All fields implement `Copy`
2. No custom `Drop` implementation

Owning types such as `String`, `Vec<T>`, and `Box<T>` are not `Copy`; a shared reference to them can still be `Copy`.

```rust
// ✅ Can be Copy
#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

// ❌ Cannot be Copy - contains String
#[derive(Clone)]
struct Person {
    name: String,  // String is not Copy
    age: u32,
}

// ❌ Cannot be Copy - has Drop
struct FileHandle {
    fd: i32,
}
impl Drop for FileHandle {
    fn drop(&mut self) { /* close file */ }
}
```

## Size Guidelines

These are rough review heuristics, not Rust trait limits or portable performance thresholds. First check that implicit duplication fits the public contract; benchmark copies versus borrowing when the difference matters.

| Size | Consideration |
|------|---------------|
| Small scalar-like values | `Copy` is often ergonomic when value semantics fit |
| Larger aggregates | Consider call patterns, copy frequency, and borrowing costs |
| Any size | All fields must be `Copy` and the type must not implement `Drop` |

```rust
use std::mem::size_of;

#[derive(Clone, Copy)]
struct SmallId(u64); // 8 bytes ✅

#[derive(Clone, Copy)]
struct Rect { x: f32, y: f32, w: f32, h: f32 } // 16 bytes ✅

#[derive(Clone, Copy)] // Legal: all fields are Copy, regardless of size
struct Transform {
    matrix: [[f64; 3]; 3], // Whether to expose Copy is an API/design choice
}
```

## Common Copy Types

Standard library types that are `Copy`:
- All primitives: `i32`, `f64`, `bool`, `char`, etc.
- Shared references: `&T` (note: `&mut T` is NOT `Copy` — copying a mutable reference would alias it, so it is reborrowed instead)
- Raw pointers: `*const T`, `*mut T`
- Function pointers: `fn(T) -> U`
- Tuples of `Copy` types: `(i32, f64)`
- Arrays of `Copy` types at any length: `[u8; 32]`, `[u8; 1024]`
- `Option<T>` where `T: Copy`
- `PhantomData<T>`

## See Also

- [own-clone-explicit](./own-clone-explicit.md) - When Clone without Copy is appropriate
- [type-newtype-ids](./type-newtype-ids.md) - Newtype pattern often uses Copy
