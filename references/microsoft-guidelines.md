# Microsoft guideline review and mapping

Source: [Microsoft Pragmatic Rust Guidelines, v2026.6](https://microsoft.github.io/rust-guidelines/guidelines/), pinned to
[commit 9585e288d21d2da994a0550c926a09b62d79f8bd](https://github.com/microsoft/rust-guidelines/tree/9585e288d21d2da994a0550c926a09b62d79f8bd/src/guidelines).

The pinned source contains **90 distinct M-guideline pages**, all included by its
section indexes. Each has one disposition below: **45 applied, 44 conditional,
and one literal policy excluded**. These are source-guideline dispositions, not
the number of skill rules or a claim that every source clause or example should
be copied. Existing skill rules are reused where they express the relevant
contract; this reconciliation adds the missing I/O, native-handle and shared-state
boundaries.

**Applied** means the useful engineering guidance is represented in the linked
rules. **Conditional** identifies the task, compatibility or architectural limits
on adopting the source prescription. **Excluded literal policy** records a
specific prescription that this skill does not impose; it does not discard the
underlying goal of clarity.

A source recommendation never overrides a project's architecture, public API,
runtime, test layout, execution authorization or MSRV. In particular, this skill
does not automatically split bounded contexts, install tooling, run tests, replace
an error framework, select an allocator, or rewrite established APIs.

## Universal (11)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-UPSTREAM-GUIDELINES](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-UPSTREAM-GUIDELINES) | Applied | [api-common-traits](../rules/api-common-traits.md), [name-as-free](../rules/name-as-free.md), [name-no-get-prefix](../rules/name-no-get-prefix.md) | Retain Rust API naming, conversions, trait contracts and constructor conventions. |
| [M-STATIC-VERIFICATION](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-STATIC-VERIFICATION) | Conditional | [lint-workspace-lints](../rules/lint-workspace-lints.md), [lint-expect-reason](../rules/lint-expect-reason.md), [unsafe-miri-ci](../rules/unsafe-miri-ci.md) | Use relevant compiler, lint, feature and dependency checks; optional tools are not automatically installed or run. |
| [M-LINT-OVERRIDE-EXPECT](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-LINT-OVERRIDE-EXPECT) | Applied | [lint-expect-reason](../rules/lint-expect-reason.md) | Track local exceptions and stale suppressions with reasons; account for targets and tool-specific lint passes. |
| [M-PUBLIC-DEBUG](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-PUBLIC-DEBUG) | Applied | [api-common-traits](../rules/api-common-traits.md), [type-display-vs-debug](../rules/type-display-vs-debug.md), [obs-no-sensitive-data](../rules/obs-no-sensitive-data.md) | Make diagnostics useful while redacting secrets; do not derive Debug over sensitive fields. |
| [M-PUBLIC-DISPLAY](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-PUBLIC-DISPLAY) | Applied | [type-display-vs-debug](../rules/type-display-vs-debug.md) | Provide a meaningful readable representation where appropriate, preserving privacy and error conventions. |
| [M-SMALLER-CRATES](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-SMALLER-CRATES) | Conditional | [proj-workspace-large](../rules/proj-workspace-large.md) | Split at real reuse, deployment or dependency boundaries; size alone and assumed compile-time improvements are insufficient. |
| [M-WEASEL-WORDS](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-WEASEL-WORDS) | Conditional | [name-types-camel](../rules/name-types-camel.md), [anti-over-abstraction](../rules/anti-over-abstraction.md) | Prefer precise domain roles; established Service, Factory or lifecycle terms remain valid when they convey actual meaning. |
| [M-SHORT-NAMES](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-SHORT-NAMES) | Excluded literal policy | [name-types-camel](../rules/name-types-camel.md), [name-acronym-word](../rules/name-acronym-word.md) | Keep readable names; do not enforce a two-word maximum or prefer abbreviations solely for brevity. |
| [M-REGULAR-FN](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-REGULAR-FN) | Conditional | [anti-over-abstraction](../rules/anti-over-abstraction.md), [macro-prefer-functions](../rules/macro-prefer-functions.md) | Use free functions for behavior unrelated to a type; coherent associated operations and constructors remain appropriate. |
| [M-DOCUMENTED-MAGIC](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-DOCUMENTED-MAGIC) | Applied | [const-vs-static](../rules/const-vs-static.md) | Name policy constants and explain their units, chosen limits and external contracts instead of restating literals. |
| [M-LOG-STRUCTURED](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html#M-LOG-STRUCTURED) | Conditional | [obs-structured-fields](../rules/obs-structured-fields.md), [obs-levels-filter](../rules/obs-levels-filter.md), [obs-no-sensitive-data](../rules/obs-no-sensitive-data.md) | Use actual backend fields and event contracts; literal template syntax and allocation claims are backend-dependent. |

## Libraries: interoperability (7)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-DONT-LEAK-TYPES](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-DONT-LEAK-TYPES) | Applied | [api-public-dependencies](../rules/api-public-dependencies.md) | Treat exposed dependency types as compatibility commitments; intentional ecosystem integration can justify them. |
| [M-FOREIGN-REEXPORTS](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-FOREIGN-REEXPORTS) | Conditional | [api-public-dependencies](../rules/api-public-dependencies.md), [proj-pub-use-reexport](../rules/proj-pub-use-reexport.md) | Avoid accidental aliases while preserving deliberate facades, macro paths and published compatibility promises. |
| [M-IMPL-ASREF](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-IMPL-ASREF) | Conditional | [api-impl-asref](../rules/api-impl-asref.md), [own-slice-over-vec](../rules/own-slice-over-vec.md) | Use flexible borrowed views when they help callers; plain references and owned inputs can be simpler or cheaper. |
| [M-IMPL-IO](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-IMPL-IO) | Applied | [api-io-boundary](../rules/api-io-boundary.md), [test-mock-traits](../rules/test-mock-traits.md) | Separate transport setup from bounded parsing with the required Read/Write capability and runtime-specific adapters. |
| [M-IMPL-RANGEBOUNDS](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-IMPL-RANGEBOUNDS) | Applied | [api-range-bounds](../rules/api-range-bounds.md) | Make endpoint semantics explicit and normalize inclusive or excluded bounds with checked arithmetic. |
| [M-ESCAPE-HATCHES](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-ESCAPE-HATCHES) | Applied | [api-native-handles](../rules/api-native-handles.md), [unsafe-ffi-boundary](../rules/unsafe-ffi-boundary.md) | Expose required interop through explicit borrow, transfer and unsafe adoption contracts; do not fabricate duplicate owners. |
| [M-TYPES-SEND](https://microsoft.github.io/rust-guidelines/guidelines/libs/interop/index.html#M-TYPES-SEND) | Conditional | [async-fn-in-trait](../rules/async-fn-in-trait.md), [async-clone-before-await](../rules/async-clone-before-await.md), [own-arc-shared](../rules/own-arc-shared.md) | Check Send when the executor or public API requires it; thread-affine and local-executor types remain valid. |

## Libraries: API use (14)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-SIMPLE-ABSTRACTIONS](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-SIMPLE-ABSTRACTIONS) | Conditional | [anti-over-abstraction](../rules/anti-over-abstraction.md), [api-common-traits](../rules/api-common-traits.md) | Keep caller-facing types comprehensible; containers and useful framework generics need not obey a fixed nesting depth. |
| [M-SERVICES-CLONE](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-SERVICES-CLONE) | Conditional | [api-common-traits](../rules/api-common-traits.md), [own-arc-shared](../rules/own-arc-shared.md) | Clone shared service handles only when semantics justify it; cloning need not create independent mutable resources. |
| [M-ERRORS-CANONICAL-STRUCTS](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-ERRORS-CANONICAL-STRUCTS) | Conditional | [err-custom-type](../rules/err-custom-type.md), [err-source-chain](../rules/err-source-chain.md), [err-thiserror-lib](../rules/err-thiserror-lib.md) | Preserve sources and handleable structure; do not mandate one struct, backtrace capture or flattened Display chain. |
| [M-ESSENTIAL-FN-INHERENT](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-ESSENTIAL-FN-INHERENT) | Conditional | [api-common-traits](../rules/api-common-traits.md), [api-extension-trait](../rules/api-extension-trait.md) | Keep core operations discoverable; standard trait methods need not acquire redundant inherent copies automatically. |
| [M-FROM-ERROR](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-FROM-ERROR) | Applied | [err-from-impl](../rules/err-from-impl.md), [err-source-chain](../rules/err-source-chain.md) | Define canonical source-preserving conversions once; use contextual mappings where required. |
| [M-INIT-BUILDER](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-INIT-BUILDER) | Conditional | [api-builder-pattern](../rules/api-builder-pattern.md), [api-builder-must-use](../rules/api-builder-must-use.md) | Use builders for actual construction complexity; retain established constructors and avoid fixed option-count or dependency-container mandates. |
| [M-INIT-CASCADED](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-INIT-CASCADED) | Applied | [api-builder-pattern](../rules/api-builder-pattern.md), [api-newtype-safety](../rules/api-newtype-safety.md) | Group parameters that form meaningful domain values rather than creating artificial groups to satisfy a count. |
| [M-ASYNC-FN](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-ASYNC-FN) | Conditional | [async-future-size](../rules/async-future-size.md), [async-fn-in-trait](../rules/async-fn-in-trait.md) | Prefer ordinary async functions when equivalent; explicit futures remain useful for setup timing and Send or dispatch contracts. |
| [M-DI-HIERARCHY](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-DI-HIERARCHY) | Conditional | [anti-over-abstraction](../rules/anti-over-abstraction.md), [trait-dyn-vs-generic](../rules/trait-dyn-vs-generic.md), [test-mock-traits](../rules/test-mock-traits.md) | Choose concrete values, traits or dynamic dispatch from the real boundary; do not impose an enum-only mocking ladder. |
| [M-AVOID-WRAPPERS](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-AVOID-WRAPPERS) | Conditional | [own-slice-over-vec](../rules/own-slice-over-vec.md), [api-common-traits](../rules/api-common-traits.md) | Hide incidental ownership wrappers; retain wrappers that express lifetime, sharing, container or performance contracts. |
| [M-NO-PRELUDE](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-NO-PRELUDE) | Conditional | [proj-prelude-module](../rules/proj-prelude-module.md) | Prefer deliberate imports; justified trait-heavy ecosystems may provide a documented prelude. |
| [M-PARAMETER-CONSISTENCY](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-PARAMETER-CONSISTENCY) | Applied | [api-builder-pattern](../rules/api-builder-pattern.md), [type-named-fields](../rules/type-named-fields.md) | Keep repeated conceptual parameters ordered consistently; established APIs and algorithms may legitimately require multiple callbacks. |
| [M-COLLECTION-TRAITS](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-COLLECTION-TRAITS) | Applied | [api-impl-fromiterator](../rules/api-impl-fromiterator.md), [name-iter-convention](../rules/name-iter-convention.md) | Provide suitable iterator and collection traits with truthful size hints; stronger iterator guarantees require real support. |
| [M-BALANCED-MODULES](https://microsoft.github.io/rust-guidelines/guidelines/libs/ux/index.html#M-BALANCED-MODULES) | Conditional | [proj-pub-use-reexport](../rules/proj-pub-use-reexport.md), [doc-module-inner](../rules/doc-module-inner.md), [proj-workspace-large](../rules/proj-workspace-large.md) | Group by capability and make primary APIs discoverable; preserve established bounded contexts and module composition. |

## Libraries: resilience (9)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-BUILD-RESULT](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-BUILD-RESULT) | Applied | [api-builder-pattern](../rules/api-builder-pattern.md), [type-newtype-validated](../rules/type-newtype-validated.md) | Validate cross-field construction at build; strong inputs and genuinely fallible setters can validate earlier. |
| [M-STRONG-TYPES](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-STRONG-TYPES) | Conditional | [type-no-stringly](../rules/type-no-stringly.md), [api-io-boundary](../rules/api-io-boundary.md), [api-newtype-safety](../rules/api-newtype-safety.md) | Choose domain and standard types such as paths; numeric newtypes are valid when their invariants are part of the public contract. |
| [M-STRONG-TYPES-GUARD](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-STRONG-TYPES-GUARD) | Applied | [type-newtype-validated](../rules/type-newtype-validated.md), [api-parse-dont-validate](../rules/api-parse-dont-validate.md), [serde-try-from-validate](../rules/serde-try-from-validate.md) | Guard every construction and deserialization path; infallible conversions remain valid from already validated domains. |
| [M-AVOID-STATICS](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-AVOID-STATICS) | Applied | [api-explicit-state](../rules/api-explicit-state.md), [const-vs-static](../rules/const-vs-static.md) | Give correctness-sensitive shared identity an owner; immutable tables, caches and deliberate application singletons remain valid. |
| [M-MOCKABLE-SYSCALLS](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-MOCKABLE-SYSCALLS) | Conditional | [test-mock-traits](../rules/test-mock-traits.md), [api-io-boundary](../rules/api-io-boundary.md) | Inject clocks, I/O and randomness through useful seams; no universal controller tuple, runtime enum or hidden allocation guarantee. |
| [M-INTEGRATION-TESTS](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-INTEGRATION-TESTS) | Conditional | [test-integration-dir](../rules/test-integration-dir.md), [test-mock-traits](../rules/test-mock-traits.md) | Exercise observable consumer contracts using the project's test layout and execution authorization. |
| [M-TEST-UTIL](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-TEST-UTIL) | Conditional | [test-cfg-test-module](../rules/test-cfg-test-module.md), [test-mock-traits](../rules/test-mock-traits.md), [proj-feature-additive](../rules/proj-feature-additive.md) | Keep test-only capabilities deliberately gated; an additive Cargo feature is not a production security boundary. |
| [M-LOG-NOT-PRINT](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-LOG-NOT-PRINT) | Applied | [obs-library-facade](../rules/obs-library-facade.md), [obs-levels-filter](../rules/obs-levels-filter.md) | Send diagnostics through the application's telemetry stack; stdout remains a legitimate CLI interface. |
| [M-NO-GLOB-REEXPORTS](https://microsoft.github.io/rust-guidelines/guidelines/libs/resilience/index.html#M-NO-GLOB-REEXPORTS) | Conditional | [proj-pub-use-reexport](../rules/proj-pub-use-reexport.md) | Keep public exports intentional; generated and platform facades may justify controlled globs. |

## Libraries: building (3)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-FEATURES-ADDITIVE](https://microsoft.github.io/rust-guidelines/guidelines/libs/building/index.html#M-FEATURES-ADDITIVE) | Applied | [proj-feature-additive](../rules/proj-feature-additive.md), [lint-cfg-check](../rules/lint-cfg-check.md) | Make reusable library features compose; distinguish platform support and application deployment configuration. |
| [M-OOBE](https://microsoft.github.io/rust-guidelines/guidelines/libs/building/index.html#M-OOBE) | Conditional | [proj-build-rs-minimal](../rules/proj-build-rs-minimal.md), [proj-msrv-declare](../rules/proj-msrv-declare.md) | Make supported builds predictable and document prerequisites; do not claim every crate supports all Tier 1 targets without validation. |
| [M-SYS-CRATES](https://microsoft.github.io/rust-guidelines/guidelines/libs/building/index.html#M-SYS-CRATES) | Conditional | [proj-build-rs-minimal](../rules/proj-build-rs-minimal.md), [unsafe-ffi-boundary](../rules/unsafe-ffi-boundary.md) | Govern native inputs and source provenance; system linking, hermetic packaging and native tools need explicit platform decisions. |

## Macros (8)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-MACRO-LAST-RESORT](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-MACRO-LAST-RESORT) | Applied | [macro-prefer-functions](../rules/macro-prefer-functions.md) | Use ordinary language constructs when sufficient; macros remain useful for real syntax and boilerplate needs. |
| [M-EXAMPLE-OVER-PROC](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-EXAMPLE-OVER-PROC) | Applied | [macro-prefer-functions](../rules/macro-prefer-functions.md), [macro-rules-hygiene](../rules/macro-rules-hygiene.md) | Prefer declarative matching when sufficient; use proc macros when derives or structured syntax transformation justify them. |
| [M-MACRO-MAIN-CRATE](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-MACRO-MAIN-CRATE) | Conditional | [macro-proc-two-crate](../rules/macro-proc-two-crate.md), [macro-export-crate-path](../rules/macro-export-crate-path.md) | Use the documented facade path; crate-renaming support and direct macro consumption follow the actual compatibility contract. |
| [M-MACRO-VERSION-PIN](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-MACRO-VERSION-PIN) | Applied | [macro-proc-two-crate](../rules/macro-proc-two-crate.md), [macro-private-helpers](../rules/macro-private-helpers.md) | Pin coordinated generated-code helper protocols; independently consumed macros do not need arbitrary lockstep versions. |
| [M-MACRO-HELPERS](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-MACRO-HELPERS) | Applied | [macro-private-helpers](../rules/macro-private-helpers.md) | Use controlled fully qualified helper paths; doc(hidden) does not exempt generated callers from compatibility. |
| [M-PROC-IMPL](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-PROC-IMPL) | Conditional | [macro-proc-two-crate](../rules/macro-proc-two-crate.md), [macro-proc-error-spans](../rules/macro-proc-error-spans.md) | Separate substantial transformation logic when useful; a third crate and a particular test framework are not universal requirements. |
| [M-MACROS-DONT-LIE](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-MACROS-DONT-LIE) | Applied | [macro-prefer-functions](../rules/macro-prefer-functions.md), [macro-proc-two-crate](../rules/macro-proc-two-crate.md) | Document generated items and signature transformations; explicit established adapters need not be banned. |
| [M-PROC-IMPLIED-ITEMS](https://microsoft.github.io/rust-guidelines/guidelines/macros/index.html#M-PROC-IMPLIED-ITEMS) | Applied | [macro-proc-two-crate](../rules/macro-proc-two-crate.md), [macro-rules-hygiene](../rules/macro-rules-hygiene.md) | Make generated names, visibility and collision behavior part of the discoverable macro contract. |

## Applications (3)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-APP-ERROR](https://microsoft.github.io/rust-guidelines/guidelines/apps/index.html#M-APP-ERROR) | Conditional | [err-anyhow-app](../rules/err-anyhow-app.md), [err-thiserror-lib](../rules/err-thiserror-lib.md) | Erase errors at reporting boundaries when useful; retain typed domain and protocol errors where callers need to distinguish them. |
| [M-MIMALLOC-APPS](https://microsoft.github.io/rust-guidelines/guidelines/apps/index.html#M-MIMALLOC-APPS) | Conditional | [perf-profile-first](../rules/perf-profile-first.md), [perf-release-profile](../rules/perf-release-profile.md) | Benchmark allocator behavior on actual targets; reusable libraries must not unilaterally choose the application's global allocator. |
| [M-TARGET-CPU](https://microsoft.github.io/rust-guidelines/guidelines/apps/index.html#M-TARGET-CPU) | Applied | [opt-target-cpu](../rules/opt-target-cpu.md) | Use the verified deployment baseline or dispatch variants; native build-host features are not a portable default. |

## Foreign function interfaces (3)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-ISOLATE-DLL-STATE](https://microsoft.github.io/rust-guidelines/guidelines/ffi/index.html#M-ISOLATE-DLL-STATE) | Applied | [unsafe-ffi-boundary](../rules/unsafe-ffi-boundary.md), [api-explicit-state](../rules/api-explicit-state.md) | Route opaque handles through their owning library; do not assume portable Rust layout, TypeId, allocation or runtime identity. |
| [M-FFI-TRANSLATES](https://microsoft.github.io/rust-guidelines/guidelines/ffi/index.html#M-FFI-TRANSLATES) | Applied | [unsafe-ffi-boundary](../rules/unsafe-ffi-boundary.md), [api-io-boundary](../rules/api-io-boundary.md) | Keep pointer and ABI translation at a narrow boundary with safe core operations; split crates when the boundary warrants it. |
| [M-FFI-NAMING](https://microsoft.github.io/rust-guidelines/guidelines/ffi/index.html#M-FFI-NAMING) | Conditional | [unsafe-ffi-boundary](../rules/unsafe-ffi-boundary.md) | Use conventional sys/import and ffi/export package roles while preserving ecosystem naming and existing package contracts. |

## Correctness (7)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-UNSAFE](https://microsoft.github.io/rust-guidelines/guidelines/correctness/index.html#M-UNSAFE) | Applied | [unsafe-minimize-scope](../rules/unsafe-minimize-scope.md), [unsafe-panic-safety](../rules/unsafe-panic-safety.md), [unsafe-miri-ci](../rules/unsafe-miri-ci.md) | Prefer established safe operations; justify unsafe invariants and measure performance claims without treating Miri as a proof. |
| [M-UNSOUND](https://microsoft.github.io/rust-guidelines/guidelines/correctness/index.html#M-UNSOUND) | Applied | [unsafe-panic-safety](../rules/unsafe-panic-safety.md), [unsafe-minimize-scope](../rules/unsafe-minimize-scope.md) | Safe callers, panics and misbehaving safe traits must not trigger undefined behavior through the abstraction. |
| [M-UNSAFE-IMPLIES-UB](https://microsoft.github.io/rust-guidelines/guidelines/correctness/index.html#M-UNSAFE-IMPLIES-UB) | Applied | [unsafe-panic-safety](../rules/unsafe-panic-safety.md), [unsafe-safety-comment](../rules/unsafe-safety-comment.md) | Unsafe marks a soundness precondition rather than a merely destructive business operation. |
| [M-PANIC-IS-STOP](https://microsoft.github.io/rust-guidelines/guidelines/correctness/index.html#M-PANIC-IS-STOP) | Conditional | [err-result-over-panic](../rules/err-result-over-panic.md), [unsafe-panic-safety](../rules/unsafe-panic-safety.md) | Expected failures use errors; panic behavior and supervision follow the application contract, with no recovery promise. |
| [M-PANIC-ON-BUG](https://microsoft.github.io/rust-guidelines/guidelines/correctness/index.html#M-PANIC-ON-BUG) | Conditional | [err-expect-bugs-only](../rules/err-expect-bugs-only.md), [api-parse-dont-validate](../rules/api-parse-dont-validate.md) | True invariants may panic under the chosen policy; validation failures need not become bugs or process termination. |
| [M-PANIC-CONTINUATION](https://microsoft.github.io/rust-guidelines/guidelines/correctness/index.html#M-PANIC-CONTINUATION) | Conditional | [unsafe-panic-safety](../rules/unsafe-panic-safety.md) | Recovery requires valid state and an explicit boundary; catch_unwind neither repairs invariants nor mandates a whole-process restart. |
| [M-PANIC-MESSAGE](https://microsoft.github.io/rust-guidelines/guidelines/correctness/index.html#M-PANIC-MESSAGE) | Applied | [err-expect-bugs-only](../rules/err-expect-bugs-only.md), [doc-panics-section](../rules/doc-panics-section.md) | Explain broken invariants and useful diagnostic values without leaking sensitive data. |

## Performance (11)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-THROUGHPUT](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-THROUGHPUT) | Conditional | [perf-extend-batch](../rules/perf-extend-batch.md), [async-yield-cpu](../rules/async-yield-cpu.md), [opt-cache-friendly](../rules/opt-cache-friendly.md) | Balance batch size, cache behavior and tail latency; do not categorically prohibit single-item processing or work stealing. |
| [M-HOTPATH](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-HOTPATH) | Applied | [perf-profile-first](../rules/perf-profile-first.md), [test-criterion-bench](../rules/test-criterion-bench.md) | Identify relevant CPU and allocation paths with representative measurements, without reusing anecdotal speed percentages. |
| [M-YIELD-POINTS](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-YIELD-POINTS) | Applied | [async-yield-cpu](../rules/async-yield-cpu.md), [async-spawn-blocking](../rules/async-spawn-blocking.md) | Bound executor CPU work and admission; an await may be ready and cooperative budget is not a fixed time quantum. |
| [M-MEM-REUSE](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-MEM-REUSE) | Applied | [mem-reuse-collections](../rules/mem-reuse-collections.md), [mem-clone-from](../rules/mem-clone-from.md) | Reuse resources on measured allocation paths; do not require output-buffer APIs for every operation. |
| [M-LOG-OVERHEAD](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-LOG-OVERHEAD) | Applied | [obs-levels-filter](../rules/obs-levels-filter.md), [obs-structured-fields](../rules/obs-structured-fields.md) | Measure enabled telemetry and event volume; aggregate or sample without discarding required diagnostic behavior. |
| [M-AVOID-INDIRECTION](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-AVOID-INDIRECTION) | Conditional | [opt-cache-friendly](../rules/opt-cache-friendly.md), [own-arc-shared](../rules/own-arc-shared.md) | Optimize measured access paths; do not cache duplicated mutable configuration without an update or snapshot contract. |
| [M-BOX-DST](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-BOX-DST) | Conditional | [mem-boxed-slice](../rules/mem-boxed-slice.md) | Use fixed-size owned storage when appropriate; Box is not inherently immutable and exact handle sizes are target-specific. |
| [M-SHRINK-TO-FIT](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-SHRINK-TO-FIT) | Conditional | [mem-with-capacity](../rules/mem-with-capacity.md), [mem-boxed-slice](../rules/mem-boxed-slice.md) | Trim costly retained slack when beneficial; shrinking can reallocate, need not reach exact capacity and can hurt reusable buffers. |
| [M-FAST-HASHER](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-FAST-HASHER) | Applied | [perf-ahash](../rules/perf-ahash.md) | Choose from measurements and attacker control of keys; seed configuration matters and generic speed/security promises are insufficient. |
| [M-INITIAL-CAPACITY](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-INITIAL-CAPACITY) | Applied | [mem-with-capacity](../rules/mem-with-capacity.md), [api-io-boundary](../rules/api-io-boundary.md) | Reserve sensible known capacity within resource limits; an attacker-provided length is not permission for unlimited allocation. |
| [M-ASYNC-STACK-SIZE](https://microsoft.github.io/rust-guidelines/guidelines/performance/index.html#M-ASYNC-STACK-SIZE) | Applied | [async-future-size](../rules/async-future-size.md) | Measure future storage and drop scopes; moving setup changes when validation and side effects execute. |

## Projects (5)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-CARGO-WORKSPACE](https://microsoft.github.io/rust-guidelines/guidelines/project/index.html#M-CARGO-WORKSPACE) | Conditional | [proj-workspace-deps](../rules/proj-workspace-deps.md), [lint-workspace-lints](../rules/lint-workspace-lints.md) | Share actual metadata and dependency policy; do not force every isolated dependency or default feature into one uniform template. |
| [M-CRATES-IN-WORKSPACE](https://microsoft.github.io/rust-guidelines/guidelines/project/index.html#M-CRATES-IN-WORKSPACE) | Conditional | [proj-workspace-large](../rules/proj-workspace-large.md) | Use workspaces for related packages; preserve repositories with intentionally independent version or deployment boundaries. |
| [M-CRATES-FLAT-FOLDER](https://microsoft.github.io/rust-guidelines/guidelines/project/index.html#M-CRATES-FLAT-FOLDER) | Conditional | [proj-workspace-large](../rules/proj-workspace-large.md), [proj-flat-small](../rules/proj-flat-small.md) | Use a discoverable project layout; do not move packages merely to match one directory convention. |
| [M-LATEST-EDITION](https://microsoft.github.io/rust-guidelines/guidelines/project/index.html#M-LATEST-EDITION) | Conditional | [proj-msrv-declare](../rules/proj-msrv-declare.md) | Prefer a current stable edition for new crates when MSRV permits; established projects need an intentional migration. |
| [M-MSRV](https://microsoft.github.io/rust-guidelines/guidelines/project/index.html#M-MSRV) | Applied | [proj-msrv-declare](../rules/proj-msrv-declare.md) | Declare and validate the real compiler floor, including its edition and dependency requirements. |

## Documentation (4)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-CANONICAL-DOCS](https://microsoft.github.io/rust-guidelines/guidelines/docs/index.html#M-CANONICAL-DOCS) | Applied | [doc-all-public](../rules/doc-all-public.md), [doc-errors-section](../rules/doc-errors-section.md), [doc-panics-section](../rules/doc-panics-section.md), [doc-safety-section](../rules/doc-safety-section.md) | Document relevant behavior, errors, panic, safety and abort contracts rather than forcing empty sections. |
| [M-DOC-INLINE](https://microsoft.github.io/rust-guidelines/guidelines/docs/index.html#M-DOC-INLINE) | Conditional | [proj-pub-use-reexport](../rules/proj-pub-use-reexport.md), [doc-intra-links](../rules/doc-intra-links.md) | Inspect facade rustdoc and inline useful re-exports; avoid duplicate or misleading canonical paths. |
| [M-FIRST-DOC-SENTENCE](https://microsoft.github.io/rust-guidelines/guidelines/docs/index.html#M-FIRST-DOC-SENTENCE) | Applied | [doc-all-public](../rules/doc-all-public.md) | Lead with behavior and concise summaries; fixed sentence or word counts are not required. |
| [M-MODULE-DOCS](https://microsoft.github.io/rust-guidelines/guidelines/docs/index.html#M-MODULE-DOCS) | Applied | [doc-module-inner](../rules/doc-module-inner.md) | Explain subsystem purpose, entry points and important contracts at the relevant module boundary. |

## AI-assisted engineering (5)

| Source guideline | Disposition | Skill rules | Adopted scope or reason |
|---|---|---|---|
| [M-DESIGN-FOR-AI](https://microsoft.github.io/rust-guidelines/guidelines/ai/index.html#M-DESIGN-FOR-AI) | Applied | [api-common-traits](../rules/api-common-traits.md), [doc-all-public](../rules/doc-all-public.md), [doc-examples-section](../rules/doc-examples-section.md), [api-newtype-safety](../rules/api-newtype-safety.md), [test-mock-traits](../rules/test-mock-traits.md) | Use conventional typed APIs, actionable examples and testable behavior; agent use does not override project permissions or architecture. |
| [M-RUST-SHAPED](https://microsoft.github.io/rust-guidelines/guidelines/ai/index.html#M-RUST-SHAPED) | Applied | [anti-transliterated-port](../rules/anti-transliterated-port.md) | Preserve domain semantics while adapting ownership, errors and resource lifetimes instead of copying another language's structure. |
| [M-NO-META-DESIGN-DOCUMENTATION](https://microsoft.github.io/rust-guidelines/guidelines/ai/index.html#M-NO-META-DESIGN-DOCUMENTATION) | Conditional | [doc-module-inner](../rules/doc-module-inner.md) | Keep API docs focused on current contracts; durable rationale and separate design records remain useful. |
| [M-SINGLE-ITEM-PATH](https://microsoft.github.io/rust-guidelines/guidelines/ai/index.html#M-SINGLE-ITEM-PATH) | Conditional | [proj-pub-use-reexport](../rules/proj-pub-use-reexport.md) | Choose clear canonical public paths while preserving compatibility aliases and deliberate facades. |
| [M-TAUTOLOGICAL-TESTS](https://microsoft.github.io/rust-guidelines/guidelines/ai/index.html#M-TAUTOLOGICAL-TESTS) | Applied | [test-arrange-act-assert](../rules/test-arrange-act-assert.md), [test-proptest-properties](../rules/test-proptest-properties.md) | Verify observable behavior or independent properties instead of restating implementation definitions. |

## Source interpretation limits

- Unsafe abstractions must remain sound on panic; they need not universally use
  poisoning, catch unwinding, or restart the entire application.
- A ready await need not yield. Future-size and scheduling budgets depend on the
  compiler, target, executor and workload.
- Boxed slices have fixed length and can still be mutated. Shrinking and removing
  indirection are measured tradeoffs, not unconditional allocation wins.
- Tracing uses its actual structured-field API. Literal double-brace placeholders
  do not automatically interpolate, and format_args is not equivalent to allocating
  a String with format.
- Feature-gated test helpers must remain intentional. Cargo features are additive
  and can be enabled downstream, so a test-util feature is not a security boundary.
- Macro facade paths, companion versions and generated public items are compatibility
  contracts. The source's blanket refusal of crate renaming is not generalized.
- Some internal source cross-references have no corresponding guideline page
  in this revision. They are not additional reviewed IDs or evidence for invented
  runtime abstractions.

For upstream issues, source provenance and Google guidance, see
[source-review.md](source-review.md).
