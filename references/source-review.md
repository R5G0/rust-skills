# Source review for this fork

Reviewed on 2026-11-10. This is a selective maintenance pass, not a claim that
every upstream example or every external guideline was independently audited.
Repository conventions and a project's MSRV take precedence over generic advice.

## Upstream issues and pull requests

Base: leonardomso/rust-skills at
`fd2a861ab0406a4ac536a55274d14ea6fd1ca9c9` (265 rules).

| Source | Status at review | Disposition |
|---|---|---|
| [Issue #6](https://github.com/leonardomso/rust-skills/issues/6), [PR #7](https://github.com/leonardomso/rust-skills/pull/7) | Issue and PR open | Separate Send bounds from dyn compatibility; use async-trait or explicit boxed futures for dyn dispatch; include the harness dependency |
| [Issue #8](https://github.com/leonardomso/rust-skills/issues/8) | Open | Correct all ten reviewed ownership rules: path conversions, cloning costs, Cow methods, Send/Sync bounds, ownership choices, overflow, mutex footprints, downgrade/fairness and Copy heuristics |
| [Issue #10](https://github.com/leonardomso/rust-skills/issues/10) | Open | Remove compulsory deep clones and false Rc-to-Send claims; also correct the issue's implication that ordinary borrows cannot cross await |
| [PR #5](https://github.com/leonardomso/rust-skills/pull/5) at `ce04df8988405e6b4ee25be3d32db1e31c1f6d83` | Closed, unmerged | Adopt six focused rules and relevant Rust 1.95–1.97 updates; pin example checking to 1.97.1 |
| [PR #4](https://github.com/leonardomso/rust-skills/pull/4) at `791a2478cbf00f28d943bd18a2dbcbb5b008574b` | Closed, unmerged | Use targeted correctness ideas such as exact compile-baseline parity and conditional optimization guidance; do not wholesale import its 343-rule corpus or source-audit claims |
| [Issue #3](https://github.com/leonardomso/rust-skills/issues/3) | Open | Adopt selected Microsoft guidance with explicit context and compatibility limits; the Zero To Production book/production corpus from PR #4 remains outside this pass |
| [Issue #9](https://github.com/leonardomso/rust-skills/issues/9) | Open | Node 18 installer failure belongs to the external installer; use a supported Node LTS for installation |
| [PR #2](https://github.com/leonardomso/rust-skills/pull/2), [Issue #1](https://github.com/leonardomso/rust-skills/issues/1) | Merged / source inquiry | Already represented by the base corpus and attribution |

The PR #5 material was corrected during review too: atomic update closures use
lazy `then` to avoid eager overflow, bit helpers distinguish unsigned APIs,
NonZero layout guarantees are scoped, workspace lint inheritance is valid,
and atomic examples avoid data races. Compiler flag/cache advice follows the
official Cargo and Clippy documentation rather than the PR's original wording.

## Google guidance applied

Primary style source: [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html),
reviewed at [commit cceb297](https://github.com/google/styleguide/blob/cceb297e93677c31053e5e75de6b701a9204fc03/rustguide.md).
The guide was written for Google codebases; portable engineering tradeoffs are
adopted with their exceptions rather than as organization-wide mandates.

| Guidance | Applied rule(s) | Boundary |
|---|---|---|
| Prefer unique ownership and borrowing; distinguish deep copies from handle clones | `own-borrow-over-clone`, `own-rc-single-thread`, `own-arc-shared`, `async-clone-before-await` | Shared ownership remains appropriate for real shared lifetime requirements |
| Accept flexible borrowed inputs, including `&T` and `Option<&T>` | `own-slice-over-vec` | Wrapper-specific semantics can justify wrapper references |
| Choose iterator inputs/outputs for reusable library APIs | `own-slice-over-vec`, `perf-iter-lazy` | Slices and Vec remain useful for application code, indexing and owned results |
| Balance generic Into convenience against API complexity and code size | `api-impl-into` | Do not add generic conversions mechanically to every application function |
| Define traits for actual polymorphism or extension needs | `anti-over-abstraction` | Avoid fixed implementation-count thresholds; a real boundary can justify one implementation |
| Prefer named fields for values whose positions are ambiguous | New `type-named-fields` | Local pairs and single-field newtypes remain appropriate |
| Treat public const as a compatibility promise | `const-fn` | Check MSRV; runtime calls are not guaranteed compile-time evaluation |
| Use must_use where ignoring a value is a likely bug | `api-must-use` | Result already warns; annotate meaningful status values, not ordinary side effects |
| Require sound safe wrappers, not merely small unsafe blocks | `unsafe-minimize-scope`, `unsafe-extern-block` | Validate every safe input or retain an explicit unsafe caller contract |

Additional teaching sources:

- [Comprehensive Rust: cancellation](https://google.github.io/comprehensive-rust/concurrency/async-pitfalls/cancellation.html)
  and [pinning](https://google.github.io/comprehensive-rust/concurrency/async-pitfalls/pin.html)
  inform `async-cancel-safety`: keep progress and deadlines outside recreated
  futures, distinguish pinning from cancellation safety, and handle EOF/closure.
- [Comprehensive Rust: String](https://google.github.io/comprehensive-rust/std-types/string.html)
  informs new `type-utf8-boundaries` and the repaired
  `api-extension-trait` truncation example. Byte offsets, scalar values and
  grapheme clusters are distinct contracts.

Already-covered advice (newtypes, enum state parameters, standard traits,
documentation and pattern matching) is retained without duplicate rules.

## Microsoft guidance applied

The requested Microsoft pass uses official v2026.6 at
`9585e288d21d2da994a0550c926a09b62d79f8bd`. The
[Microsoft mapping](microsoft-guidelines.md) records adopted rules, existing
coverage and recommendations that remain conditional. It covers lint exceptions,
public dependency contracts, interval APIs, testability, builds/features,
FFI/panic safety, async work/storage, documentation and macro compatibility.

This is a fresh selective review of the current official source, not an import
of PR #4's older coverage ledger or its whole-corpus validation claims.

## Google policies not generalized

- Chromium's ban on async/await, standard-library thread creation, and its
  GN/import and flat-crate layout conventions.
- Fuchsia's preferred mutex implementation and platform-specific test frameworks.
- Organization-specific import grouping and mandatory file/module naming.
- Blanket prohibitions on catch_unwind, unstable features or pre-main execution;
  those require project and runtime context, not automatic rewrites.
- Mandatory non_exhaustive on every public type; evolution contracts and local
  API policy determine whether that restriction is appropriate.

## Validation scope

The index and rule links are regenerated and checked. The compile harness uses
Rust 1.97.1, edition 2024, x86_64-unknown-linux-gnu with a locked dependency graph.
It classifies unresolved illustrative fragments separately and retains an
explicit inherited suspect baseline (113 signatures after this pass, eleven
removed compared with PR #5). Both new and stale baseline signatures,
and incomplete Cargo runs, fail validation. This is a regression gate, not proof
of runtime behavior or soundness of the entire inherited corpus.
