# macro-proc-two-crate

> Put procedural macros in a dedicated `proc-macro = true` crate and re-export from the facade

## Why It Matters

A crate marked `proc-macro = true` in `Cargo.toml` compiles for the host (the build machine) and can **only** export procedural macros — no regular types, traits, or functions. If your library needs both a derive/attribute macro and ordinary APIs, you must split into two crates: a `mycrate-derive` (or `mycrate-macros`) proc-macro crate and a `mycrate` facade crate that re-exports everything.

The facade approach ensures:
- Users add only `mycrate` as a dependency.
- Generated code refers to types through `::mycrate::__private::...`, so the impl crate version is invisible.
- Exact companion-version pins keep a private generated-code protocol compatible; workspace inheritance avoids repeating those requirements.

## Bad

```rust
// A single crate with `proc-macro = true` in Cargo.toml that also tries
// to export regular items:
#[proc_macro_derive(Greet)]
pub fn derive_greet(input: TokenStream) -> TokenStream { /* ... */ }

pub trait Greet { fn greet(&self) -> String; } // error: a proc-macro crate
pub struct Config;                              // can only export proc-macros
```

## Good

Split into a `proc-macro = true` crate plus a facade that re-exports it (full manifests and code below):

```rust
// users depend only on `mycrate`:
use mycrate::Greet;        // the trait
#[derive(mycrate::Greet)]  // the derive, re-exported from mycrate-derive
struct Robot;
```

## Crate Layout

```
my-workspace/
├── Cargo.toml          # workspace manifest
├── mycrate/
│   ├── Cargo.toml
│   └── src/lib.rs
└── mycrate-derive/
    ├── Cargo.toml
    └── src/lib.rs
```

## Cargo.toml Files

```toml
# my-workspace/Cargo.toml
[workspace]
members = ["mycrate", "mycrate-derive"]
resolver = "3"   # default for the 2024 edition; use "2" for 2021

[workspace.dependencies]
mycrate-derive = { path = "mycrate-derive", version = "=0.1.0" }
syn  = { version = "2", features = ["derive"] }
quote = "1"
proc-macro2 = "1"
```

```toml
# mycrate-derive/Cargo.toml
[package]
name = "mycrate-derive"
version = "0.1.0"
edition = "2024"

[lib]
proc-macro = true   # required — makes this a proc-macro crate

[dependencies]
syn.workspace   = true
quote.workspace = true
proc-macro2.workspace = true
```

```toml
# mycrate/Cargo.toml
[package]
name = "mycrate"
version = "0.1.0"
edition = "2024"

[dependencies]
mycrate-derive.workspace = true
```

## Re-export from the Facade

```rust
// mycrate/src/lib.rs

// Re-export the derive macro so users write `use mycrate::Greet;`
// or `#[derive(mycrate::Greet)]`.
pub use mycrate_derive::Greet;

/// The trait that `#[derive(Greet)]` implements.
pub trait Greet {
    fn greet(&self) -> String;
}

#[doc(hidden)]
pub mod __private {
    // Helpers referenced by generated `impl` blocks.
    pub fn format_greeting(name: &str) -> String {
        format!("hello, {name}")
    }
}
```

```rust
// mycrate-derive/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Greet)]
pub fn derive_greet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics ::mycrate::Greet for #name #ty_generics #where_clause {
            fn greet(&self) -> String {
                ::mycrate::__private::format_greeting(#name_str)
            }
        }
    }
    .into()
}
```

## Companion Compatibility

If generated code calls a facade's private helper protocol, publish compatible
companion versions together and use exact dependency pins to prevent a newer
macro generating calls an older runtime cannot handle. Shared workspace
inheritance alone does not lock published semver ranges. Independently usable
macro crates do not need lockstep releases merely because they are proc macros.

Keep generated public items and signature changes visible in the documented
macro contract. A separate implementation crate is useful when substantial
parsing/transformation logic benefits from ordinary library APIs; a third crate
is not required for every small proc macro.

Sources: [Microsoft M-MACRO-VERSION-PIN](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-MACRO-VERSION-PIN), [M-PROC-IMPLIED-ITEMS](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-PROC-IMPLIED-ITEMS).

## Verified Project

<!-- verify: fixture=proc-macros -->
The [proc-macro fixture workspace](../checks/fixtures/proc-macros/Cargo.toml) compiles a real derive crate, facade, and separate consumer. Run `cargo test --manifest-path checks/fixtures/proc-macros/Cargo.toml --workspace --locked` to verify their crate boundaries and generated implementations.

## See Also

- [macro-proc-syn-quote](macro-proc-syn-quote.md) - building proc-macros with syn and quote
- [macro-private-helpers](macro-private-helpers.md) - hiding helpers behind `__private`
- [proj-workspace-deps](proj-workspace-deps.md) - workspace dependency inheritance
- [err-thiserror-lib](err-thiserror-lib.md) - thiserror as a real-world two-crate example
