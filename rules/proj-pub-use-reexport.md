# proj-pub-use-reexport

> Choose deliberate public paths and explicit reexports without accidentally expanding the API

## Why It Matters

Reexports can separate a library's public organization from private implementation
modules. They also create public compatibility commitments. A flat root is not
always clearer than domain modules; choose documented entry points and avoid
unnecessary aliases. Preserve already-supported paths when a project's migration
policy requires compatibility.

## Bad

```rust
pub mod storage {
    pub struct Client;
}

// Two public paths for the same item, added without an API design reason.
pub use storage::Client;
```

## Good

```rust
mod storage {
    /// A connection to the backing store.
    pub struct Client;
}

// One deliberate public entry point. Rustdoc may inline private-module
// reexports automatically; this annotation makes the intent explicit.
#[doc(inline)]
pub use storage::Client;
```

A public domain module with no root alias is equally valid. Internal reexports
used to assemble a facade and hidden macro-support paths do not create the same
user-facing ambiguity.

## Dependency Types

A third-party type in a public signature couples callers to that dependency's
API and type identity even without a reexport. Adding `pub use` does not remove
this coupling. Expose standard types where practical, or own the boundary with
a justified wrapper. Deliberate ecosystem facades can reexport dependency types
when they accept and maintain that commitment.

```rust
// A deliberate ecosystem facade; keep its external origin visible in docs.
#[doc(no_inline)]
pub use bytes::Bytes;
```

For your own items, use `#[doc(inline)]` when it improves discovery at the
documented entry point. For external types, retaining links to their original
crate helps users understand provenance. Review the rendered rustdoc output
rather than applying attributes mechanically.

## Explicit Export Lists

Prefer named reexports over `pub use module::*`: a glob can silently grow the
API when a new item is added to the module or its dependency. Small intentional
forwarding modules may justify a glob; review their resulting public surface.
A prelude is optional and should solve repeated import friction, rather than
being created for every library.

## Sources

- [Microsoft M-SINGLE-ITEM-PATH](https://microsoft.github.io/rust-guidelines/guidelines/ai/index.html#M-SINGLE-ITEM-PATH)
- [Microsoft M-FOREIGN-REEXPORTS](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-FOREIGN-REEXPORTS)
- [Microsoft M-DOC-INLINE](https://microsoft.github.io/rust-guidelines/guidelines/docs/index.html#M-DOC-INLINE)
- [Rustdoc doc attribute](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html)

## See Also

- [api-public-dependencies](api-public-dependencies.md) - Public dependency contracts
- [proj-pub-crate-internal](proj-pub-crate-internal.md) - Internal visibility
- [proj-prelude-module](proj-prelude-module.md) - Conditional prelude design
