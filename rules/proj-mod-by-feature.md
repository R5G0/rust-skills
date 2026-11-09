# proj-mod-by-feature

> Organize modules by feature, not type

## Why It Matters

Feature-based organization keeps related code together, making navigation intuitive and changes localized. Type-based organization (all handlers in one folder, all models in another) scatters related code across the codebase, making features harder to understand and modify.

## Bad

```
src/
├── controllers/
│   ├── user_controller.rs
│   ├── order_controller.rs
│   └── product_controller.rs
├── models/
│   ├── user.rs
│   ├── order.rs
│   └── product.rs
├── services/
│   ├── user_service.rs
│   ├── order_service.rs
│   └── product_service.rs
└── repositories/
    ├── user_repository.rs
    ├── order_repository.rs
    └── product_repository.rs
```

## Good

```
src/
├── user/
│   ├── mod.rs           # Re-exports public items
│   ├── model.rs         # User struct, types
│   ├── repository.rs    # Database operations
│   ├── service.rs       # Business logic
│   └── handler.rs       # HTTP handlers
├── order/
│   ├── mod.rs
│   ├── model.rs
│   ├── repository.rs
│   ├── service.rs
│   └── handler.rs
├── product/
│   ├── mod.rs
│   ├── model.rs
│   ├── repository.rs
│   └── handler.rs
└── lib.rs
```

## Benefits

| Aspect | Type-Based | Feature-Based |
|--------|------------|---------------|
| Finding code | Search across folders | One folder per feature |
| Adding feature | Touch 4+ folders | Create one folder |
| Understanding feature | Jump between folders | Everything in one place |
| Deleting feature | Hunt through codebase | Delete one folder |
| Code ownership | Unclear | Clear feature owners |

## Module Structure

```rust
// src/user/mod.rs
mod model;
mod repository;
mod service;
mod handler;

// Re-export public API
pub use model::{User, UserId, CreateUserRequest};
pub use handler::router;
pub(crate) use service::UserService;
```

## Shared Code

```
src/
├── user/
├── order/
├── shared/              # Cross-cutting concerns
│   ├── mod.rs
│   ├── database.rs      # Connection pool
│   ├── error.rs         # Common error types
│   └── middleware.rs    # Auth, logging
└── lib.rs
```

## When to Flatten

Small modules don't need deep nesting:

```
src/
├── user/
│   ├── mod.rs           # Contains User struct + simple functions
│   └── repository.rs    # Only if complex enough
├── config.rs            # Simple enough for single file
└── lib.rs
```

## Hybrid Approach

For larger features, nest further by concern:

```
src/
├── billing/
│   ├── mod.rs
│   ├── invoice/
│   │   ├── mod.rs
│   │   ├── model.rs
│   │   └── service.rs
│   ├── payment/
│   │   ├── mod.rs
│   │   ├── model.rs
│   │   └── processor.rs
│   └── shared.rs
```

## Verified Project

<!-- verify: fixture=layouts -->

See the real [validation fixture source](../checks/fixtures/layouts/feature-tool/src/user/mod.rs). The user feature owns model.rs and repository.rs and exposes a small public API. The feature-tool crate tests known and missing users.

Run from `checks/fixtures/layouts` with Rust 1.97.1: `cargo test --workspace --all-targets`, `cargo test --workspace --doc`, `cargo doc --workspace --no-deps`, `cargo fmt --all --check`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## See Also

- [proj-flat-small](./proj-flat-small.md) - Keep small projects flat
- [proj-pub-use-reexport](./proj-pub-use-reexport.md) - Clean public API
- [proj-lib-main-split](./proj-lib-main-split.md) - Lib/main separation
