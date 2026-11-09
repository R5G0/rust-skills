# api-common-traits

> Implement standard traits when their semantics fit the public type

## Why It Matters

Standard traits make types interoperable with collections, diagnostics, and generic algorithms. Their implementations also promise semantics: `Clone` duplicates a value or handle, `Eq` requires reflexivity, and `Hash` must agree with equality. Choose those contracts deliberately rather than deriving every trait just because the fields permit it.

## Bad

```rust
// Bare struct - severely limited usability
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Can't debug
println!("{:?}", point);  // Error: Debug not implemented

// Can't compare
if point1 == point2 { }  // Error: PartialEq not implemented

// Can't use in HashMap
let mut map: HashMap<Point, Value> = HashMap::new();
map.insert(point, value); // Insertion needs Eq + Hash; constructing the map alone does not.

// Can't clone
let copy = point.clone();  // Error: Clone not implemented
```

## Good

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Now everything works
println!("{:?}", point);
assert_eq!(point1, point2);
let copy = point;  // Copy, not just Clone

// For hashable types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(u64);

let mut map: HashMap<UserId, User> = HashMap::new();
```

## Trait Derivation Guide

| Trait | Derive When | Requirements |
|-------|-------------|--------------|
| `Debug` | Public diagnostic output is useful | Derive needs Debug fields; redact secrets manually |
| `Clone` | Duplicating the value or shared handle is meaningful | Derive needs Clone fields; honor resource semantics |
| `Copy` | Small, simple types | All fields implement Copy, no Drop |
| `PartialEq` | Comparison makes sense | All fields implement PartialEq |
| `Eq` | Equality is reflexive as well as symmetric and transitive | PartialEq; plain floating-point equality does not satisfy this |
| `Hash` | Hashing is useful, such as for map keys | Equal values must hash equally; HashMap keys additionally need Eq |
| `Default` | Sensible default exists | Derive needs Default fields; a manual implementation can choose values |
| `PartialOrd` | Ordering makes sense | PartialEq, all fields implement PartialOrd |
| `Ord` | A total ordering is defined | Eq + PartialOrd with consistent ordering; floats need an explicit policy |

## Common Trait Bundles

```rust
// ID types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(u64);

// Value types
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector2 { x: f32, y: f32 }

// Configuration
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Config {
    name: String,
    options: HashMap<String, String>,
}

// Error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidSyntax(String),
    UnexpectedToken(Token),
}
```

## Manual Implementations

```rust
// When derive doesn't do what you want
struct CaseInsensitiveString(String);

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for CaseInsensitiveString {}

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Must be consistent with PartialEq
        self.0.to_lowercase().hash(state);
    }
}

// Custom Debug for sensitive data
struct Password(String);

impl Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Password([REDACTED])")
    }
}
```

## Serde Traits

```rust
use serde::{Serialize, Deserialize};

// For serializable types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub data: Vec<Item>,
}

// With custom serialization
#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub verbose: bool,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}
```

## Choose Contracts by Type Role

Public value types often benefit from Debug and meaningful comparison. A unique
resource may intentionally have no Clone implementation; a client handle may
clone by sharing one underlying connection pool. Document that distinction and
do not silently recreate the resource. Implement shared ownership only when the
service actually needs it, rather than putting every type behind an Arc.

Debug output must not expose credentials or other sensitive fields. Do not derive
PartialEq for a service merely to make tests convenient when equality has no
useful contract. Likewise, provide Default only when callers can use the result
meaningfully; it need not perform hidden I/O or manufacture invalid state.

## Sources

- [Microsoft: M-SERVICES-CLONE](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-SERVICES-CLONE) - shared service handles when required
- [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html) - standard traits in context
- [Rust API Guidelines: interoperability](https://rust-lang.github.io/api-guidelines/interoperability.html)

## See Also

- [own-copy-small](./own-copy-small.md) - When to implement Copy
- [api-default-impl](./api-default-impl.md) - Implementing Default
- [doc-examples-section](./doc-examples-section.md) - Documenting trait implementations
- [type-display-vs-debug](./type-display-vs-debug.md) - Display vs Debug responsibilities
