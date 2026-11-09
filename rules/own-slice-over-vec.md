# own-slice-over-vec

> Accept `&[T]` not `&Vec<T>`, `&str` not `&String`

## Why It Matters

Accepting `&[T]` instead of `&Vec<T>` makes your function more flexible - it can accept slices from arrays, vectors, or other sources. Similarly, `&str` accepts string slices from `String`, `&'static str`, or substrings.

## Bad

```rust
// Overly restrictive - only accepts &Vec
fn sum(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()
}

// Overly restrictive - only accepts &String
fn greet(name: &String) {
    println!("Hello, {}", name);
}

// Can't call with arrays or slices
let arr = [1, 2, 3];
// sum(&arr);  // ERROR: expected &Vec<i32>

let literal = "world";
// greet(&literal);  // ERROR: expected &String
```

## Good

```rust
// Flexible - accepts any slice-like thing
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

// Flexible - accepts any string-like thing
fn greet(name: &str) {
    println!("Hello, {}", name);
}

// Now all of these work:
let vec = vec![1, 2, 3];
let arr = [4, 5, 6];
let slice = &vec[0..2];

sum(&vec);    // Vec coerces to slice
sum(&arr);    // Array coerces to slice
sum(slice);   // Slice works directly

let string = String::from("Alice");
let literal = "Bob";

greet(&string);  // String coerces to &str
greet(literal);  // &str works directly
```

## The Deref Coercion Chain

```rust
// These coercions happen automatically:
// Vec<T>  -> &[T]   (via Deref)
// String  -> &str   (via Deref)
// Box<T>  -> &T     (via Deref)
// Arc<T>  -> &T     (via Deref)

use std::sync::Arc;

fn process(data: &[u8]) {
    println!("{}", data.len());
}

let vec: Vec<u8> = vec![1, 2, 3];
let boxed: Box<[u8]> = vec![1, 2, 3].into_boxed_slice();
let arc: Arc<[u8]> = Arc::from(&[1, 2, 3][..]);

process(&vec);    // Works
process(&boxed);  // Works
process(&arc);    // Works
```

## Borrow the Data Inside Wrappers

When only the contained data is needed, accept `&T` rather than `&Box<T>` and `Option<&T>` rather than `&Option<T>`. This follows the Google Rust Style Guide's flexible-reference guidance. Keep wrapper-specific parameters when their identity or operations are actually part of the contract.

```rust
fn length(value: &str) -> usize {
    value.len()
}

fn optional_length(value: Option<&str>) -> usize {
    value.map_or(0, str::len)
}

let boxed = Box::new(String::from("hello"));
assert_eq!(length(&boxed), 5);
assert_eq!(length("world"), 5);

let owned = Some(String::from("hello"));
assert_eq!(optional_length(owned.as_deref()), 5);
assert_eq!(optional_length(Some("world")), 5);
assert_eq!(optional_length(None), 0);
```

Use `Option::as_ref()` to convert `&Option<T>` into `Option<&T>`; use `as_deref()` when the contained value should also be dereferenced, such as `Option<String>` to `Option<&str>`.

## Path Types Too

`&Path` accepts `&Path` and, through deref coercion, `&PathBuf`. String inputs require an explicit conversion such as `Path::new`, or a generic parameter accepting `AsRef<Path>`.

```rust
use std::path::{Path, PathBuf};

fn read_config(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

fn read_config_flexible(path: impl AsRef<Path>) -> std::io::Result<String> {
    read_config(path.as_ref())
}

fn callers(path: &Path, buffer: &PathBuf, text: &String) {
    let _ = read_config(path);
    let _ = read_config(buffer);
    let _ = read_config(Path::new(text));
    let _ = read_config_flexible("config.toml");
    let _ = read_config_flexible(text);
}
```

Choose the generic form when its extra accepted inputs help callers; it is not inherently better than a concrete `&Path` API.

## When Iteration Is Enough

A reusable library function that only traverses its input can accept `impl IntoIterator<Item = T>` (or borrowed items) to support noncontiguous collections and iterator adapters. A slice remains a clear choice for application code, indexed access, or callers that already have contiguous data; generic iteration is not a requirement for every function.

## When to Accept Owned Types

```rust
// Accept owned when you need to store it
struct Logger {
    prefix: String,  // Needs to own the string
}

impl Logger {
    // Take ownership - caller decides to clone or move
    fn new(prefix: String) -> Self {
        Self { prefix }
    }
    
    // Or use Into for flexibility
    fn with_prefix(prefix: impl Into<String>) -> Self {
        Self { prefix: prefix.into() }
    }
}
```

## See Also

- [api-impl-asref](api-impl-asref.md) - Accept `impl AsRef<T>` for maximum flexibility
- [own-borrow-over-clone](own-borrow-over-clone.md) - Prefer borrowing over cloning
