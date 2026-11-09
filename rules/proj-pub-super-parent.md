# proj-pub-super-parent

> Use pub(super) for parent-only visibility

## Why It Matters

`pub(super)` exposes items within the immediate parent module and its descendants. This is useful for helper functions and types that submodules share but shouldn't be visible to the rest of the crate.

## Bad

```rust
// src/parser/mod.rs
pub mod lexer;
pub mod ast;

// src/parser/lexer.rs
pub fn internal_helper() {  // Visible to entire crate!
    // Helper only needed by lexer and ast
}

pub(crate) struct Token {  // Visible to entire crate
    // Only parser submodules need this
}
```

## Good

<!-- verify: compile -->
```rust
mod parser {
    pub fn word_count(input: &str) -> usize {
        lexer::lex(input).len()
    }

    mod lexer {
        // Both items are accessible to parser and its descendants, not crate root.
        pub(super) struct Token;

        pub(super) fn lex(input: &str) -> Vec<Token> {
            input.split_whitespace().map(|_| Token).collect()
        }
    }
}

fn main() {
    assert_eq!(parser::word_count("one two"), 2);
}
```

## Visibility Hierarchy

```
src/
├── lib.rs           # crate root
├── parser/
│   ├── mod.rs       # parent of lexer; sees lexer pub(super) items
│   ├── lexer.rs     # pub(super) items scoped to parser
│   └── ast.rs       # pub(super) items scoped to parser
└── codegen.rs       # cannot see lexer pub(super) items
```

## Pattern: Layered Visibility

```rust
// src/database/mod.rs
mod connection;
mod query;
mod pool;

// Visible to database's parent and that parent's descendants
pub(super) struct RawConnection { /* ... */ }

// Entire crate can see
pub(crate) struct Pool { /* ... */ }

// Everyone can see
pub struct Database { /* ... */ }
```

## Pattern: Test Helpers

```rust
// src/parser/mod.rs
mod lexer;
mod ast;

#[cfg(test)]
mod tests {
    use super::*;
    
    // Test helper visible only to parser module's tests
    pub(in crate::parser) fn make_test_token() -> Token {
        Token { kind: TokenKind::Test, span: Span::dummy() }
    }
}

// src/parser/lexer.rs
#[cfg(test)]
mod tests {
    use super::super::tests::make_test_token;
    // ...
}
```

## Comparison

| Visibility | Scope | Use Case |
|------------|-------|----------|
| `pub` | Everywhere | Public API |
| `pub(crate)` | Crate-wide | Internal shared utilities |
| `pub(super)` | Parent module | Submodule helpers |
| `pub(in path)` | Specific path | Precise control |
| (private) | Current module | Implementation details |

## When to Use pub(super)

- Helper functions shared between sibling modules
- Types used by submodules but not the rest of crate
- Implementation details of a module group
- Test utilities for a module tree

## See Also

- [proj-pub-crate-internal](./proj-pub-crate-internal.md) - Crate visibility
- [proj-pub-use-reexport](./proj-pub-use-reexport.md) - Re-export patterns
- [proj-mod-by-feature](./proj-mod-by-feature.md) - Feature organization
