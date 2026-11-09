use std::fs::File;
use std::hint::black_box;
use std::io::{Seek, Write};

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};
use rust_skills_perf_validation::buffered_io::*;

pub fn register(c: &mut Criterion) {
    let mut group = c.benchmark_group("perf-io-buffering");
    // File-backed fixtures exercise OS calls, but warm page caches and virtualized
    // storage mean these results are not a physical-disk durability measurement.
    for records_count in [32usize, 512] {
        let records: Vec<_> = (0..records_count)
            .map(|index| format!("record-{index:08}"))
            .collect();
        let bytes: Vec<_> = records
            .iter()
            .flat_map(|record| record.bytes().chain(std::iter::once(b'\n')))
            .collect();
        let mut fixture = tempfile::NamedTempFile::new().expect("create read fixture");
        fixture.write_all(&bytes).expect("populate read fixture");
        fixture.flush().expect("flush read fixture");
        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.bench_function(BenchmarkId::new("read/unbuffered", records_count), |b| {
            b.iter_batched(
                || File::open(fixture.path()).expect("open read fixture"),
                |file| black_box(count_newlines_unbuffered(file).expect("read fixture")),
                BatchSize::PerIteration,
            );
        });
        for capacity in [1024usize, 8192] {
            group.bench_function(
                BenchmarkId::new(format!("read/buffered-{capacity}"), records_count),
                |b| {
                    b.iter_batched(
                        || File::open(fixture.path()).expect("open read fixture"),
                        |file| {
                            black_box(
                                count_newlines_buffered(file, capacity).expect("read fixture"),
                            )
                        },
                        BatchSize::PerIteration,
                    );
                },
            );
        }

        group.bench_function(BenchmarkId::new("write/unbuffered", records_count), |b| {
            b.iter_batched_ref(
                || tempfile::tempfile().expect("create output fixture"),
                |file| write_records_unbuffered(file, black_box(&records)).expect("write fixture"),
                BatchSize::PerIteration,
            );
        });
        for capacity in [1024usize, 8192] {
            group.bench_function(
                BenchmarkId::new(format!("write/buffered-{capacity}"), records_count),
                |b| {
                    b.iter_batched_ref(
                        || tempfile::tempfile().expect("create output fixture"),
                        |file| {
                            write_records_buffered(file, black_box(&records), capacity)
                                .expect("write fixture")
                        },
                        BatchSize::PerIteration,
                    );
                },
            );
        }
        // Validate the exact fixture contract outside the timed loops.
        assert_eq!(
            count_newlines_buffered(File::open(fixture.path()).unwrap(), 8192).unwrap(),
            records_count
        );
        let mut output = tempfile::tempfile().unwrap();
        write_records_buffered(&mut output, &records, 8192).unwrap();
        assert_eq!(output.stream_position().unwrap(), bytes.len() as u64);
    }
    group.finish();
}
