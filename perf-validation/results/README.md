# Validation findings

The machine-readable run record is [`latest.json`](latest.json). It distinguishes
correctness checks, Criterion observations, profile artifacts, and the optional
nightly example. A passing suite means the stated behaviors were exercised; it
does not prove every source recommendation is faster.

The related rule documents have been corrected using these executable experiments.
Tests retain the earlier variants so the original counterexamples remain reproducible:

1. **`perf-io-buffering`: the original implementations have different contracts.**
   For `b"first\nlast"`, the byte-counting implementation returns 1 and the
   `lines()` implementation returns 2. For `b"\xff\n"`, the byte counter returns 1
   while `lines()` reports `InvalidData`. Both counterexamples are explicit
   passing regression tests. The fair buffering benchmark therefore compares
   equivalent newline-byte counters. `lines()` also constructs a `String` for
   each yielded line, contrary to the source comment.
2. **`perf-iter-lazy`: equal final values do not imply equal effects.** The eager
   version executes mappings after the tenth match; the lazy version skips them.
   Tests observe the call stream and demonstrate a late panic that only the eager
   path executes. Equivalence property tests use pure bounded transformations.
3. **`perf-drain-reuse`: reusing the destination does not remove source
   compaction.** Every prefix drain still moves remaining elements. The benchmarks
   include this cost. The source's generic batch-size example can fail to make
   progress with a zero chunk size; the executable API accepts `NonZeroUsize`.
4. **`perf-black-box-bench`: optimization barriers do not establish correctness.**
   The deliberately discarded-output case and the valid cases share the same
   four-element input. Criterion already consumes returned results; extra output
   barriers are a measurement choice. Timings alone cannot prove which compiler
   transformation occurred or provide an inlining guarantee.
5. **`perf-release-profile`: the combined candidate changes panic behavior.**
   The runner checks both checksums and actual destructor behavior after panic in
   separately built executables. Runtime, size, and incremental build duration are
   reported independently; no fixed speedup or ranking is assumed.

Hasher measurements use locally generated keys. Content equivalence and a local
speed result do not establish collision-attack resistance. The two profile-first
benchmark scopes and the optional nightly validation are labeled separately to
avoid treating distinct experiments as equivalent implementations.
