//! Criterion experiments grouped by the original rule identifier.

#![expect(
    missing_docs,
    reason = "Criterion generates a public harness entry point"
)]

#[path = "cases/buffered_io.rs"]
mod buffered_io;
#[path = "cases/collections.rs"]
mod collections;
#[path = "cases/hashing.rs"]
mod hashing;
#[path = "cases/iterators.rs"]
mod iterators;
#[path = "cases/measurement.rs"]
mod measurement;

use criterion::{Criterion, criterion_group, criterion_main};
use std::time::Duration;

fn register(c: &mut Criterion) {
    iterators::register(c);
    collections::register(c);
    hashing::register(c);
    buffered_io::register(c);
    measurement::register(c);
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(300))
        .measurement_time(Duration::from_millis(700))
        .sample_size(30);
    targets = register
}
criterion_main!(benches);
