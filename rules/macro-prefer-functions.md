# macro-prefer-functions

> Reach for a macro only when a function or generic cannot express it

## Why It Matters

Macros expand token streams before their generated code is type-checked; they do not bypass Rust's type system. Expansion can add diagnostic, navigation, and build complexity. Prefer an ordinary function when it expresses the same operation clearly, and use a macro for a concrete syntactic or code-generation need.

Reach for a macro only when you genuinely need one of: variadic argument counts, a DSL with non-Rust syntax, blanket trait impls across an open-ended set of types, compile-time format/string checks, or eliminating mechanically repetitive boilerplate that a function truly cannot handle.

## Bad

```rust
// Nothing here requires a macro — no variadic args, no DSL, no trait impl.
macro_rules! double {
    ($x:expr) => {
        $x * 2
    };
}

fn main() {
    let n = double!(21);
    println!("{n}");
}
```

## Good

```rust
// A generic function is clearer, debuggable, and just as efficient.
#[inline]
fn double<T>(x: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    x + x
}

fn main() {
    let n = double(21_i32);
    println!("{n}");
}
```

## When to Reach for a Macro

| Situation | Use a macro? |
|-----------|-------------|
| Fixed argument count, any types | No — use generics |
| Truly variadic argument list (`vec![]`, `println!`) | Yes |
| Implementing a trait for many unrelated types | Yes — `macro_rules!` impl block |
| DSL / embedded syntax (SQL, HTML, regex literals) | Yes — proc-macro |
| Compile-time format string validation | Yes — `format_args!` style |
| Boilerplate a derive could generate | Yes — `#[derive(...)]` proc-macro |
| Simple computation or type conversion | No — use a function or trait |

## Choosing a Macro Form

When a macro is justified, prefer macro_rules when its matching and hygiene are sufficient; use a procedural macro when parsing, derives, or attribute transformation require it. Both forms can be inspected with expansion tooling, and their output is type-checked.

Make generated items, signature changes, and execution behavior discoverable in the macro's contract. Established signature adapters remain valid when their transformation is explicit; avoid hidden extra parameters or surprising changes to async behavior.

Sources: [Microsoft M-MACRO-LAST-RESORT](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-MACRO-LAST-RESORT), [M-EXAMPLE-OVER-PROC](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-EXAMPLE-OVER-PROC), [M-MACROS-DONT-LIE](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-MACROS-DONT-LIE).

## See Also

- [anti-over-abstraction](anti-over-abstraction.md) - avoid unnecessary abstraction layers
- [type-generic-bounds](type-generic-bounds.md) - add trait bounds only where needed
- [macro-rules-hygiene](macro-rules-hygiene.md) - hygiene and `$crate` for declarative macros
