# Changelog

All notable changes to this skill are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/), and the project aims to follow
semantic versioning for the rule set.

## [Unreleased] — R5G0 fork

### Issue completion
- Account for all six upstream issues (#1, #3, #6, #8, #9, #10) in one
  requirement-to-resolution record, with an independent recheck of #6/#8/#10.
- Resolve the Node 18 installer report with exact CLI prerequisites,
  missing-styleText diagnostics, and a verified complete Git-only path.
- Clarify inherited authorship/source provenance and reconcile every current
  Microsoft guideline against its pinned published inventory.
- Add focused library guidance for I/O boundaries, native-handle ownership and
  explicit instance state; correct related constants, builders, macros and telemetry.

### Fixed
- Apply upstream issues #6, #8 and #10 and the dyn-dispatch correction in PR #7.
- Correct path coercions, clone/Copy costs, Cow operations, Send/Sync bounds,
  mutex footprints, downgrade stability and scheduling claims.
- Correct cancellation progress, deadline reuse, channel closure and task
  lifecycle guidance; repair unsound safe raw-pointer wrappers and UTF-8 slicing.
- Correct const API compatibility, must_use defaults, lint inheritance,
  atomic ordering and conditional performance guidance.

### Added
- Eleven focused Microsoft-inspired rules: lint expectations, public dependencies,
  checked range APIs, FFI boundaries, panic safety, cooperative CPU work, future
  size, idiomatic ports, I/O boundaries, native handles and explicit state;
  documented conditional policies and source revision.
- Six reviewed Rust 1.95–1.97 rules from PR #5, with additional corrections.
- Two portable Google-inspired rules: named fields and UTF-8 boundaries.
- A source review mapping adopted guidance and excluded project-specific policies.

### Changed
- 284 rules across 26 categories; metadata version 1.5.1+fork.3.
- README category counts are generated from rule files with the skill index.
- Pin the compile harness to Rust 1.97.1, add async-trait, require complete
  Cargo diagnostics and exact suspect-baseline parity.

## [1.5.1]

### Changed
- Depth pass: expanded `own-rc-single-thread` (breaking cycles with `Weak`, the
  `Rc::clone` idiom, `strong_count`/`weak_count`, `!Send`/`!Sync`) and
  `own-refcell-interior` (`Cell` for `Copy` types).
- Added cross-references from ~18 foundational rules to the newer categories
  (`conc-`, `conv-`, `num-`, `serde-`, `trait-`, `closure-`, `coll-`, `pat-`)
  for better navigation between related rules.

## [1.5.0]

### Added
- **Const & Compile-Time** category (`const-`, 4 rules): `const fn`, `const` vs
  `static`, const generics, inline `const { }` blocks.
- **Trait & Generics Design** category (`trait-`, 6 rules): static vs dynamic
  dispatch, associated types vs generic params, default methods, blanket impls,
  object safety, the orphan rule + newtype.
- **Collections** category (`coll-`, 4 rules): map choice (HashMap/BTreeMap/
  IndexMap), sequence choice (Vec/VecDeque), set membership, `BinaryHeap`.
- `checks/gen_index.py` — generates `SKILL.md`'s priority table and Quick
  Reference (and the rule counts) from `rules/` so the index can't drift; CI
  runs it in `--check` mode.
- `CONTRIBUTING.md` and this `CHANGELOG.md`.

Now 265 rules across 26 categories.

## [1.4.0]

### Added
- **Closures** category (`closure-`, 5 rules): Fn/FnMut/FnOnce bounds, returning
  `impl Fn`, `move` capture, static vs dynamic dispatch, disjoint capture.

Now 251 rules across 23 categories.

## [1.3.0]

### Added
- **Serde** category (`serde-`, 8 rules): rename_all, default, skip, flatten,
  enum representation, deny_unknown_fields, custom (de)serialize, validate-on-
  deserialize.
- **Numeric & Arithmetic Safety** category (`num-`, 5 rules): explicit overflow
  handling, `as` vs `TryFrom`, float comparison, clamping, `NonZero`.

Now 246 rules across 22 categories.

## [1.2.0]

### Added
- **Macros** category (`macro-`, 8 rules): declarative-macro hygiene and
  fragment specifiers, and procedural-macro design with `syn`/`quote`.
- **Observability** category (`obs-`, 7 rules): `tracing`, spans, structured
  fields, error chains, and keeping secrets out of logs.

Now 233 rules across 20 categories.

## [1.1.x]

### Added
- **Unsafe Code** (`unsafe-`), **Concurrency** (`conc-`), **Conversions**
  (`conv-`), and **Pattern Matching** (`pat-`) categories, plus new rules across
  existing categories — 39 rules in total.
- A compile-check harness (`checks/`) and a GitHub Actions CI workflow that
  validates rule structure, links, the index, and that examples compile.

### Changed
- Updated throughout for the Rust 2024 edition and current stable (Rust 1.96):
  fixed `&mut T` is not `Copy`, `impl Trait` in traits, `collect_into` status,
  `resolver = "3"`, `env::set_var` now `unsafe`, and more.

Now 218 rules across 18 categories.

## [1.0.0]

### Added
- Initial release: 179 rules across 14 categories.

[1.5.0]: https://github.com/leonardomso/rust-skills
[1.4.0]: https://github.com/leonardomso/rust-skills
[1.3.0]: https://github.com/leonardomso/rust-skills
[1.2.0]: https://github.com/leonardomso/rust-skills
[1.1.x]: https://github.com/leonardomso/rust-skills
[1.0.0]: https://github.com/leonardomso/rust-skills
