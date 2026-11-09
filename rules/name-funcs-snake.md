# name-funcs-snake

> Use `snake_case` for functions, methods, variables, and modules

## Why It Matters

Rust uses `snake_case` for "value-level" names—functions, methods, variables, modules. This convention is enforced by the compiler and distinguishes runtime entities from types. Consistent naming makes code scannable and predictable.

## Bad

```rust
// CamelCase functions - compiler warns
fn calculateTotal() -> f64 { ... }  // warning: function `calculateTotal` should have a snake case name
fn getUserName() -> String { ... }  // warning

// Inconsistent naming
fn get_user() -> User { ... }
fn fetchOrder() -> Order { ... }  // Mixed conventions
```

## Good

<!-- verify: compile -->
```rust
mod user_service {
    pub fn calculate_total(unit_price: u32, item_count: u32) -> u32 { unit_price * item_count }
}
struct User { first_name: String, last_name: String }
impl User {
    fn full_name(&self) -> String { format!("{} {}", self.first_name, self.last_name) }
}
fn main() {
    let user_count = user_service::calculate_total(3, 4);
    let user = User { first_name: String::from("Ada"), last_name: String::from("Lovelace") };
    assert_eq!(user_count, 12);
    assert_eq!(user.full_name(), "Ada Lovelace");
}
```

## Acronyms in snake_case

```rust
// Lowercase acronyms in snake_case
fn parse_json() -> Json { ... }   // Not parse_JSON
fn connect_tcp() -> TcpStream { ... }   // Not connect_TCP
fn generate_uuid() -> Uuid { ... }      // Not generate_UUID

let http_response = fetch();
let json_data = parse();
```

## Local Variables

```rust
fn process_data(input_data: &[u8]) -> Result<Output, Error> {
    let raw_bytes = input_data;
    let decoded_string = decode(raw_bytes)?;
    let parsed_value = parse(&decoded_string)?;
    let final_result = transform(parsed_value)?;
    
    Ok(final_result)
}
```

## See Also

- [name-types-camel](./name-types-camel.md) - Type naming
- [name-consts-screaming](./name-consts-screaming.md) - Constant naming
- [name-lifetime-short](./name-lifetime-short.md) - Lifetime naming
