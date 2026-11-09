# own-lifetime-elision

> Rely on lifetime elision rules; add explicit lifetimes only when required

## Why It Matters

Rust's lifetime elision rules handle most common borrowing patterns automatically. Adding explicit lifetimes where they're not needed clutters code without adding clarity. However, understanding when elision applies helps you know when explicit lifetimes are truly necessary.

## Bad

```rust
// Unnecessary explicit lifetimes - elision handles these
fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}

fn get_name<'a>(person: &'a Person) -> &'a str {
    &person.name
}

impl<'a> Display for Wrapper<'a> {
    fn fmt<'b>(&'b self, f: &'b mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

## Good

<!-- verify: compile -->
```rust
use std::fmt::{self, Display, Formatter};

struct Person { name: String }
struct Wrapper<'a>(&'a str);

fn first_word(input: &str) -> &str {
    input.split_whitespace().next().unwrap_or("")
}

fn get_name(person: &Person) -> &str {
    &person.name
}

impl Display for Wrapper<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

fn main() {
    let person = Person { name: "Alice".into() };
    assert_eq!(get_name(&person), "Alice");
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(Wrapper("borrowed").to_string(), "borrowed");
}
```

## The Three Elision Rules

1. Each input reference gets its own lifetime:

   ```rust
   fn lengths(left: &str, right: &str) -> (usize, usize) { (left.len(), right.len()) }
   fn lengths_explicit<'a, 'b>(left: &'a str, right: &'b str) -> (usize, usize) {
       (left.len(), right.len())
   }
   ```

2. With one input reference, output references receive its lifetime:

   ```rust
   fn identity(input: &str) -> &str { input }
   fn identity_explicit<'a>(input: &'a str) -> &'a str { input }
   ```

3. For a method with `&self` or `&mut self`, elided output references receive the receiver's lifetime:

   ```rust
   struct Named { name: String }
   impl Named {
       fn name(&self, _other: &str) -> &str { &self.name }
       fn name_explicit<'a, 'b>(&'a self, _other: &'b str) -> &'a str { &self.name }
   }
   ```

## When Explicit Lifetimes ARE Required

```rust
// Multiple input references, output could come from either
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct holding references
struct Parser<'input> {
    source: &'input str,
    position: usize,
}

// Multiple distinct lifetimes needed
struct Context<'s, 'c> {
    source: &'s str,
    cache: &'c mut Cache,
}

// Static lifetime for constants
fn get_default() -> &'static str {
    "default"
}
```

## Anonymous Lifetime `'_`

Use `'_` to let the compiler infer while being explicit about the presence of a lifetime:

```rust
// In struct definitions
impl Iterator for Parser<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> { ... }
}

// In function signatures where it adds clarity
fn parse(input: &str) -> Result<Ast<'_>, Error> { ... }

// Especially useful in trait bounds
fn process(data: &impl AsRef<str>) -> Cow<'_, str> { ... }
```

## Common Patterns

```rust
fn trim(input: &str) -> &str { input.trim() }
fn first(values: &[i32]) -> Option<&i32> { values.first() }

// Multiple input references need an explicit relationship for this output.
fn pick<'a>(left: &'a str, right: &'a str, first: bool) -> &'a str {
    if first { left } else { right }
}
```

Without that relationship the return lifetime is ambiguous:

```rust,compile_fail
fn pick(left: &str, right: &str, first: bool) -> &str {
    if first { left } else { right }
}
```

## See Also

- [own-borrow-over-clone](./own-borrow-over-clone.md) - Prefer borrowing to avoid ownership issues
- [api-impl-asref](./api-impl-asref.md) - Generic borrowing with AsRef
