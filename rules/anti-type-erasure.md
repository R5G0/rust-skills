# anti-type-erasure

> Don't use Box<dyn Trait> when impl Trait works

## Why It Matters

`Box<dyn Trait>` (type erasure) introduces heap allocation and dynamic dispatch overhead. When you have a single concrete type or can use generics, `impl Trait` provides the same flexibility with zero overhead through monomorphization.

## Bad

```rust
// Unnecessary type erasure
fn get_iterator() -> Box<dyn Iterator<Item = i32>> {
    Box::new((0..10).map(|x| x * 2))
}

// Boxing for no reason
fn make_handler() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// Vec of boxed trait objects when one type would do
fn get_validators() -> Vec<Box<dyn Validator>> {
    vec![
        Box::new(LengthValidator),
        Box::new(RegexValidator),
    ]
}
```

## Good

<!-- verify: compile -->
```rust
fn get_iterator() -> impl Iterator<Item = i32> {
    (0..10).map(|value| value * 2)
}

fn make_handler() -> impl Fn(i32) -> i32 {
    |value| value + 1
}

fn main() {
    assert_eq!(get_iterator().sum::<i32>(), 90);
    assert_eq!(make_handler()(41), 42);
}
```

## When to Use Box<dyn Trait>

Type erasure IS appropriate when:

```rust
// Heterogeneous collection of different types
let handlers: Vec<Box<dyn Handler>> = vec![
    Box::new(LogHandler),
    Box::new(MetricsHandler),
    Box::new(AuthHandler),
];

// Type cannot be known at compile time
fn create_from_config(config: &Config) -> Box<dyn Database> {
    match config.db_type {
        DbType::Postgres => Box::new(PostgresDb::new()),
        DbType::Sqlite => Box::new(SqliteDb::new()),
    }
}

// Recursive types
struct Node {
    value: i32,
    children: Vec<Box<dyn NodeTrait>>,
}

// Breaking cycles in complex ownership
struct EventLoop {
    handlers: Vec<Box<dyn EventHandler>>,
}
```

## Comparison

| Approach | Allocation | Dispatch | Binary Size |
|----------|------------|----------|-------------|
| `impl Trait` | Stack/inline | Static | Larger (monomorphization) |
| `Box<dyn Trait>` | Heap | Dynamic | Smaller |
| Generics `<T>` | Stack/inline | Static | Larger |

## impl Trait Positions

```rust
use std::fmt::Display;
use std::future::Future;

fn process() -> impl Future<Output = Result<u32, &'static str>> {
    async { Ok(42) }
}

trait Handler { fn handle(&self, value: u32) -> u32; }

fn handle(handler: impl Handler) -> u32 {
    handler.handle(41)
}

// Return-position impl Trait in traits is stable since Rust 1.75.
trait Processor {
    fn process(&self) -> impl Display;
    type Output: Display;
    fn process_named(&self) -> Self::Output;
}
```

## Pattern: Enum Instead of dyn

```rust
// Instead of Box<dyn Shape>
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
```

## See Also

- [anti-over-abstraction](./anti-over-abstraction.md) - Excessive generics
- [type-generic-bounds](./type-generic-bounds.md) - Generic constraints
- [mem-box-large-variant](./mem-box-large-variant.md) - Boxing enum variants
- [trait-dyn-vs-generic](./trait-dyn-vs-generic.md) - Choose dispatch deliberately
- [closure-static-vs-dyn](./closure-static-vs-dyn.md) - Same tradeoff for closures
