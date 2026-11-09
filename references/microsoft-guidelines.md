# Microsoft guidance adopted by this skill

Source: [Microsoft Pragmatic Rust Guidelines, v2026.6](https://microsoft.github.io/rust-guidelines/guidelines/).
The reviewed source is pinned to
[`9585e288d21d2da994a0550c926a09b62d79f8bd`](https://github.com/microsoft/rust-guidelines/tree/9585e288d21d2da994a0550c926a09b62d79f8bd/src/guidelines).
The public [checklist](https://microsoft.github.io/rust-guidelines/guidelines/checklist/index.html)
provides the guideline identifiers used below.

This skill adopts portable behavior and compatibility contracts, with explicit
tradeoffs. A Microsoft recommendation does not override a project's architecture,
runtime, test layout, published compatibility policy, or MSRV. This document is a
selection and mapping, not a claim to reproduce every normative clause.

## Applied and reinforced guidance

| Microsoft guidance | Skill rule(s) | Application |
|---|---|---|
| M-LINT-OVERRIDE-EXPECT | New `lint-expect-reason` | Track stale lint exceptions with scoped expectations and reasons; allow remains useful when targets or toolchains vary |
| M-DONT-LEAK-TYPES, M-FOREIGN-REEXPORTS | New `api-public-dependencies`; `proj-pub-use-reexport` | Treat exposed dependency types as compatibility commitments; intentional ecosystem facades remain valid |
| M-IMPL-RANGEBOUNDS | New `api-range-bounds` | Make interval semantics explicit and normalize endpoints with checked arithmetic |
| M-SIMPLE-ABSTRACTIONS, M-DI-HIERARCHY, M-ESSENTIAL-FN-INHERENT | `anti-over-abstraction`, `api-common-traits` | Keep concrete application APIs simple; use traits for real boundaries and inherent methods for core discoverability |
| M-MOCKABLE-SYSCALLS | `test-mock-traits` | Inject clocks, randomness and I/O through the smallest useful seam, including values, closures and standard I/O traits |
| M-OOBE, M-SYS-CRATES | `proj-build-rs-minimal` | Publish predictable build inputs; document native prerequisites and support controlled source locations |
| M-FEATURES-ADDITIVE | `proj-feature-additive` | Reusable library features compose; distinguish this from binary deployment configuration |
| M-UNSAFE, M-UNSAFE-IMPLIES-UB, M-UNSOUND | New `unsafe-panic-safety`; `unsafe-minimize-scope` | Preserve memory safety even if safe callbacks panic or misbehave; unsafe is not a marker for merely destructive operations |
| M-ISOLATE-DLL-STATE, M-FFI-TRANSLATES, M-FFI-NAMING | New `unsafe-ffi-boundary` | Specify ABI, allocation ownership, runtime state and panic contracts at the foreign boundary |
| M-YIELD-POINTS | New `async-yield-cpu` | Bound uninterrupted work; an immediately ready await need not yield |
| M-ASYNC-STACK-SIZE | New `async-future-size` | Measure future storage and preserve timing semantics when moving setup outside an async function |
| M-FAST-HASHER | `perf-ahash` | Choose hashers from threat model and measurements; avoid universal speed/security promises |
| M-TARGET-CPU | `opt-target-cpu` | Compile for a verified deployment minimum, rather than blindly using the build host's native features |
| M-CARGO-WORKSPACE, M-CRATES-IN-WORKSPACE, M-MSRV | `proj-workspace-deps`, `lint-workspace-lints`, `proj-msrv-declare` | Share actual workspace configuration and test the declared compiler floor; edition 2024 needs Rust 1.85 or newer |
| M-MACRO-VERSION-PIN, M-MACRO-HELPERS | `macro-proc-two-crate`, `macro-private-helpers` | Keep a private generated-code protocol compatible using coordinated versions; doc(hidden) alone grants no compatibility exemption |
| M-PROC-IMPLIED-ITEMS | `macro-proc-two-crate` | Make generated public items and signature changes part of the documented macro contract |
| M-FIRST-DOC-SENTENCE, M-CANONICAL-DOCS, M-MODULE-DOCS | `doc-all-public`, `doc-module-inner` | Summarize behavior and document relevant failure/safety/abort contracts without rigid word counts |
| M-DOC-INLINE, M-SINGLE-ITEM-PATH, M-NO-GLOB-REEXPORTS | `proj-pub-use-reexport` | Choose intentional public paths and explicit exports; inspect rustdoc rendering and preserve required compatibility aliases |
| M-NO-META-DESIGN-DOCUMENTATION | `doc-module-inner` | Keep API docs focused on the current contract; useful enduring rationale and dedicated design records remain appropriate |
| M-TAUTOLOGICAL-TESTS | `test-arrange-act-assert` | Assert observable behavior or independent properties, not copied definitions |
| M-RUST-SHAPED | New `anti-transliterated-port` | Preserve domain semantics while adapting ownership, errors and resource lifetimes to Rust |
| M-APP-ERROR | `err-anyhow-app` | Permit reporting-oriented error erasure where structured domain errors are no longer required |

Existing ownership, newtype, builder, From/AsRef, FromIterator, structured logging,
documentation and macro-hygiene rules already cover substantial parts of the
guidance. They are retained instead of duplicating a rule for each source ID.

## Recommendations that remain conditional

- **M-MIMALLOC-APPS:** compare allocator behavior on the actual workload and
  target. Installing a particular global allocator is not a cost-free universal
  optimization, and reusable libraries do not choose their application's allocator.
- **M-TYPES-SEND:** require and check Send when the runtime/public contract needs
  it. Local executors, deliberately thread-affine objects and no_std targets
  remain valid; cloning Rc never changes its thread-safety properties.
- **M-SERVICES-CLONE:** cheap shared service handles can improve ergonomics, but
  cloning must not imply independent mutable state or be added to every service.
- **M-ERRORS-CANONICAL-STRUCTS:** error structs can preserve implementation freedom;
  typed enums and the project's established error framework remain valid.
  Preserve error sources rather than flattening the full chain into every Display.
- **M-SMALLER-CRATES, M-CRATES-FLAT-FOLDER, M-BALANCED-MODULES:** follow actual
  packaging, deployment and dependency boundaries. Crate or module size alone
  does not justify splitting a bounded context.
- **M-INTEGRATION-TESTS, M-TEST-UTIL:** external-consumer tests and separately gated
  fixtures are useful where appropriate; respect existing test structure and
  execution authorization instead of moving or running tests automatically.
- **M-NO-PRELUDE:** explicit imports are a good default; established trait-heavy
  ecosystems may have a justified prelude.
- **M-PANIC-IS-STOP, M-PANIC-ON-BUG, M-PANIC-CONTINUATION:** distinguish bugs from
  expected failures and define panic policy at a real supervision boundary.
  Do not introduce universal catch_unwind/restart wrappers.
- **M-SHRINK-TO-FIT, M-BOX-DST, M-AVOID-INDIRECTION:** measure retained capacity,
  future growth and allocation costs. Boxed owned data is not inherently immutable,
  and shrinking a reusable collection can increase future allocation work.
- **M-LOG-STRUCTURED, M-LOG-OVERHEAD:** use the logging backend's actual structured
  field API and measure hot-path overhead. Literal placeholder conventions from
  another logging stack do not automatically interpolate in tracing.
- **M-PROC-IMPL:** a separate proc-macro implementation crate can aid testing and
  reuse, but is not mandatory for a small macro.

For the Google mappings and upstream issue/PR revisions, see
[source-review.md](source-review.md).
