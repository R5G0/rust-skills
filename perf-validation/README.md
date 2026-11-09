# Performance rule validation

A standalone Rust 2024 project that executes all **13 `perf-*` rules** from
[`../rules`](../rules). It contains concrete reference/candidate implementations,
correctness and property tests, Criterion benchmarks, and a process-level release
profile experiment. The existing `checks/` crate remains the Markdown compile
harness; this project runs the behavior and measures workloads.

## Run

From the repository root, run:

```sh
# Formatting, Clippy, debug/release tests, 13-group coverage, benchmark smoke run
python perf-validation/scripts/verify.py

# Also collect actual timings and compare four production build profiles
python perf-validation/scripts/verify.py --bench --profiles

# Include the actual unstable collect_into API on an installed nightly
python perf-validation/scripts/verify.py --bench --profiles --nightly
```

Python 3.10+ orchestrates standard Cargo commands on Windows, Linux, and macOS.
The project pins Rust **1.97.1**; install its Clippy and rustfmt components if
needed. Use `--offline` when the locked dependencies are cached. Cargo downloads
dependencies normally otherwise. The optional nightly example requires
`rustup toolchain install nightly --profile minimal`.

Individual Cargo commands also work from this directory:

```sh
cargo +1.97.1 test --locked --test iterators
cargo +1.97.1 test --locked --release
cargo +1.97.1 bench --locked --bench perf_rules -- perf-chain-avoid
cargo +1.97.1 bench --locked --bench perf_rules -- --test
cargo +nightly run --locked --example collect_into_nightly --features nightly-collect-into
```

The runner writes [`results/latest.json`](results/latest.json), including commands,
test counts, host/toolchain metadata, estimated benchmark means and confidence
intervals, binary sizes, checksums, and process-level panic observations. Logs are
under `results/artifacts/`; Criterion's raw samples and HTML are under
`target/criterion/`. Those generated directories are ignored by Git. CI runs the
default command, so it checks correctness and benchmark execution without treating
noisy shared-runner timings as a speed requirement.

See the [readable local results](results/summary.md). The source rules now include
corrections based on these experiments; explicitly named original/legacy variants
remain in the tests to preserve reproducible counterexamples.

## Rule coverage

Each benchmark group has the exact source rule ID. The runner discovers the rule
files and fails if any group is missing. Its measured run also checks that all
13 rules produced measurements.

| Rule | Executable checks | Measurement |
|---|---|---|
| [`perf-iter-over-index`](../rules/perf-iter-over-index.md) | [`tests/iterators.rs`](tests/iterators.rs): empty/single/boundary values, unequal-length dot products, mutations, properties | Indexed vs iterator sum, 32/4096 items |
| [`perf-iter-lazy`](../rules/perf-iter-lazy.md) | Same file: equal pure results, short-circuit call counts, a counterexample with late effects | Eager vs lazy, early/late/missing matches |
| [`perf-collect-once`](../rules/perf-collect-once.md) | Same file: filtering, order, duplicate names, generated users | Staged vs one collect, mixed/zero selectivity |
| [`perf-chain-avoid`](../rules/perf-chain-avoid.md) | Same file: empty/unequal inputs, separator positions, properties | Chain vs separate loops on equal inputs |
| [`perf-entry-api`](../rules/perf-entry-api.md) | [`tests/collections.rs`](tests/collections.rs): occupied/vacant keys, lazy initialization, returned references, properties | Repeated/unique keys, reference vs entry |
| [`perf-drain-reuse`](../rules/perf-drain-reuse.md) | Same file: partial/full drains, final short batch, retained capacity, properties | Full workload with fresh vs reused batch buffers |
| [`perf-extend-batch`](../rules/perf-extend-batch.md) | Same file: appends, duplicates, map replacements, properties | Push vs extend on exact-size iterators |
| [`perf-collect-into`](../rules/perf-collect-into.md) | Same file: stable clear+extend; [`examples/collect_into_nightly.rs`](examples/collect_into_nightly.rs): actual nightly API, empty/repeated batches and capacity | Fresh collection vs stable buffer reuse |
| [`perf-ahash`](../rules/perf-ahash.md) | [`tests/hashing.rs`](tests/hashing.rs): std/AHash/Fx content equality, duplicate replacement, Unicode/long keys, properties | Strings of length 8/128, unique/repeated distributions, integer keys |
| [`perf-io-buffering`](../rules/perf-io-buffering.md) | [`tests/buffered_io.rs`](tests/buffered_io.rs): short/error/zero writes, read failures, explicit flush, arbitrary bytes, semantic counterexamples | File-backed equivalent byte reads and record writes, 1024/8192-byte buffers |
| [`perf-black-box-bench`](../rules/perf-black-box-bench.md) | [`tests/measurement.rs`](tests/measurement.rs): known kernel results; correct output is tested independently of timing | Same input with discarded output vs opaque input/output |
| [`perf-profile-first`](../rules/perf-profile-first.md) | Same file: prepared kernel vs complete pipeline equivalence, actual probe process | Pipeline vs kernel scopes, plus setup/kernel stage timings |
| [`perf-release-profile`](../rules/perf-release-profile.md) | Same file plus runner: identical checksums across built artifacts; panic probes verify whether `Drop` executes | Default release, thin LTO, codegen-units=1, combined fat-LTO/abort candidate |

## Interpreting the experiments

Candidates are experiments, not expected winners. All correctness tests use
deterministic behavior assertions; no test asserts a maximum duration or minimum
speed ratio. Property tests supplement the explicit edge cases.

Benchmarks consume results and use identical logical fixtures for each comparison.
Consumed inputs are restored with Criterion batching. Assertions and fixture
generation stay outside the timed region. Collection workloads include the cost
of the collection operations; the I/O experiments exclude opening/creating files
but include the operation and explicit writer flush. Page caches and virtualized
storage affect I/O results; `flush` is not `fsync`, and these are not durability
measurements. String/hash construction and collection destruction are included in
the map workloads.

The default Criterion settings use 30 samples, 300 ms warmup, and a 700 ms target
measurement window per case. Inspect the confidence intervals and raw data before
drawing conclusions. The profile runner measures warm process-local kernels;
build durations and binary sizes are observations for the current host/toolchain,
not clean-build comparisons or cross-platform guarantees. The pipeline and
kernel-only groups have different scopes and must not be presented as an A/B
speedup. Sampling a real application's call stacks remains necessary before using
these microbenchmarks to guide an application optimization.

The combined release candidate intentionally uses `panic = "abort"`. Its process
probe checks that destructors do not run on panic; the other three build profiles
must unwind. This exercises shipped binaries, because the standard Rust test
harness does not reproduce the abort setting.

The stable `perf-collect-into` benchmark measures its documented `extend`
alternative. The nightly example validates `collect_into` behavior separately;
it does not claim to measure that unstable API's speed. See
[`results/README.md`](results/README.md) for concrete findings from the local run.
