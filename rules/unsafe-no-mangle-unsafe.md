# unsafe-no-mangle-unsafe

> In Rust 2024, write `#[unsafe(no_mangle)]`, `#[unsafe(export_name = "...")]`, and `#[unsafe(link_section = "...")]` — not the bare attribute forms.

## Why It Matters

`#[no_mangle]`, `#[export_name]`, and `#[link_section]` require unsafe attribute syntax in Rust 2024. Symbol collisions and invalid section placement can violate assumptions outside Rust's type system. Linkers may reject collisions or resolve them in target-specific ways; neither outcome is a portable safety guarantee. Audit symbol uniqueness, ABI, and linker contracts.

## Bad

```rust
// Rust 2021 — bare attributes accepted, no warning about linker UB
#[no_mangle]
pub extern "C" fn init() {
    // ...
}

#[export_name = "plugin_entry"]
pub fn plugin_main() {
    // ...
}

#[link_section = ".init_array"]
static INIT: extern "C" fn() = init;
```

## Good

```rust
// SAFETY: this symbol is reserved by this example's embedding application.
#[unsafe(no_mangle)]
pub extern "C" fn rust_skills_demo_init() {
    // ...
}

// SAFETY: this distinct symbol has the documented C entry-point ABI.
#[unsafe(export_name = "rust_skills_demo_entry")]
pub extern "C" fn plugin_main() {
    // ...
}

// SAFETY: these platform-specific data sections allow ordinary read-only bytes.
// No constructor or pre-main execution is registered.
#[cfg_attr(target_os = "linux", unsafe(link_section = ".rust_skills_demo"))]
#[cfg_attr(target_vendor = "apple", unsafe(link_section = "__DATA,__rs_demo"))]
static MARKER: [u8; 4] = *b"demo";
```

## Migration

| Rust 2021 | Rust 2024 |
|-----------|-----------|
| `#[no_mangle]` | `#[unsafe(no_mangle)]` |
| `#[export_name = "sym"]` | `#[unsafe(export_name = "sym")]` |
| `#[link_section = ".sec"]` | `#[unsafe(link_section = ".sec")]` |

Run `cargo fix --edition` when migrating to the 2024 edition — it rewrites bare attribute forms to `#[unsafe(...)]` automatically. Review each one afterward: confirm that the exported symbol name is unique across the binary.

## Key Points

- The `unsafe(...)` wrapper does **not** require an `unsafe {}` block at the call site; it marks the *attribute itself* as load-bearing for safety. The annotation documents that the programmer accepted responsibility for symbol uniqueness and ABI correctness.
- Symbol collisions are especially dangerous in plugin architectures, `cdylib` crates, embedded firmware with custom linker scripts, and any codebase that links multiple Rust crates into a single binary.
- These attributes interact with `unsafe extern` blocks (see `unsafe-extern-block`): external symbols you import and symbols you export follow the same 2024-edition safety rules.
- The bare forms are errors in Rust 2024. On earlier editions, enable `unsafe_attr_outside_unsafe` during migration.
- Rust 1.96 made the first duplicate `export_name`, `link_name`, or `link_section` attribute take precedence. Keep one source of truth rather than stacking duplicates.
- Rust 1.97 validates Mach-O section specifiers and rejects empty export names. Use target-specific section names; do not apply ELF names to every non-macOS target.

## See Also

- [unsafe-extern-block](unsafe-extern-block.md) - wrap `extern` blocks in `unsafe extern` in Rust 2024
- [type-repr-transparent](type-repr-transparent.md) - use `#[repr(transparent)]` for FFI newtypes
