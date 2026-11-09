//! Workloads used to investigate benchmark elimination and release profiles.

/// Mix every input value with explicitly wrapping arithmetic.
///
/// This deterministic, CPU-bound kernel is a synthetic workload, not a hash
/// function for security or a prediction of application-wide speedups.
pub fn checksum(data: &[u64]) -> u64 {
    checksum_seeded(data, 0)
}

fn checksum_seeded(data: &[u64], seed: u64) -> u64 {
    data.iter().fold(seed, |state, &value| {
        state.rotate_left(7).wrapping_add(value).wrapping_mul(31)
    })
}

/// Repeat a kernel with a changing seed so all rounds contribute to the result.
pub fn checksum_rounds(data: &[u64], rounds: usize) -> u64 {
    let mut result = 0_u64;
    for round in 0..rounds {
        result = checksum_seeded(data, result ^ round as u64);
    }
    result
}

/// Produce reproducible input outside a timed kernel-only benchmark.
pub fn fixture(size: usize) -> Vec<u64> {
    (0..size)
        .map(|index| (index as u64).wrapping_mul(17).wrapping_add(3))
        .collect()
}

/// Include input preparation in a whole-pipeline measurement.
pub fn pipeline(size: usize, rounds: usize) -> u64 {
    checksum_rounds(&fixture(size), rounds)
}
