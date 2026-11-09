# name-no-get-prefix

> Omit get_ prefix for simple getters

## Why It Matters

Rust convention omits the `get_` prefix for simple field access. Methods like `len()`, `name()`, `value()` are cleaner than `get_len()`, `get_name()`, `get_value()`. This follows the principle of making the common case concise.

The `get` prefix is reserved for methods that DO something beyond simple field access.

## Bad

```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    fn get_name(&self) -> &str {      // Verbose
        &self.name
    }
    
    fn get_age(&self) -> u32 {         // Verbose
        self.age
    }
    
    fn get_is_adult(&self) -> bool {   // Doubly verbose
        self.age >= 18
    }
}

let name = user.get_name();
let age = user.get_age();
```

## Good

```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    fn name(&self) -> &str {           // Clean
        &self.name
    }
    
    fn age(&self) -> u32 {             // Clean
        self.age
    }
    
    fn is_adult(&self) -> bool {       // Boolean uses is_ prefix
        self.age >= 18
    }
}

let name = user.name();
let age = user.age();
```

## When get_ IS Appropriate

Use `get` when the method does more than simple access:

```rust
use std::collections::HashMap;

let mut map = HashMap::from([("timeout", 30)]);
assert_eq!(map.get("timeout"), Some(&30));
if let Some(value) = map.get_mut("timeout") { *value = 60; }
let values = [10, 20];
assert_eq!(values.get(1), Some(&20));
assert_eq!(values.get(2), None);

#[derive(Clone, Default)]
struct Config { timeout: u64 }
struct Context { configs: HashMap<String, Config>, current_env: String }
impl Context {
    fn get_config(&self) -> Config {
        self.configs.get(&self.current_env).cloned().unwrap_or_default()
    }
}
```

## Standard Library Examples

API-name catalogue; concrete receivers and required arguments are omitted.

```text
// No get_ prefix
String::len()
Vec::len()
Vec::capacity()
Vec::is_empty()
Path::file_name()
Option::is_some()
Result::is_ok()

// With get - returns Option or does lookup
Vec::get(index)
HashMap::get(key)
BTreeMap::get(key)
```

## Pattern: Getter/Setter Pairs

```rust
impl Config {
    // Getter: no prefix
    fn timeout(&self) -> Duration {
        self.timeout
    }
    
    // Setter: use set_ prefix
    fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}
```

## Pattern: Builder Methods

```rust
impl ConfigBuilder {
    // Builder methods: no get_, no set_
    fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }
}
```

## Decision Guide

| Pattern | Naming |
|---------|--------|
| Simple field access | `name()`, `value()`, `len()` |
| Boolean property | `is_valid()`, `has_items()` |
| Fallible access | `get()`, `get_mut()` |
| Setter | `set_name()`, `set_value()` |
| Builder | `name()`, `value()` (consuming self) |

## Verified Example

<!-- verify: compile -->
```rust
struct User { name: String, age: u32 }
impl User {
    fn name(&self) -> &str { &self.name }
    fn age(&self) -> u32 { self.age }
    fn is_adult(&self) -> bool { self.age >= 18 }
}
fn main() {
    let user = User { name: String::from("Ada"), age: 36 };
    assert_eq!(user.name(), "Ada");
    assert_eq!(user.age(), 36);
    assert!(user.is_adult());
}
```

## See Also

- [name-is-has-bool](./name-is-has-bool.md) - Boolean naming
- [api-builder-pattern](./api-builder-pattern.md) - Builder pattern
