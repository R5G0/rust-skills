//! Behavioral checks for the benchmark and process-profile workloads.

use proptest::prelude::*;
use rust_skills_perf_validation::measurement::{checksum, checksum_rounds, fixture, pipeline};
use std::process::Command;

#[test]
fn checksum_has_known_results_and_defined_overflow() {
    assert_eq!(checksum(&[]), 0);
    assert_eq!(checksum(&[1]), 31);
    assert_eq!(checksum(&[1, 2]), 123_070);
    assert_eq!(checksum(&[u64::MAX]), u64::MAX - 30);
    assert_eq!(checksum_rounds(&[1], 0), 0);
    assert_eq!(checksum_rounds(&[1], 1), 31);
}

proptest! {
    #[test]
    fn pipeline_matches_separately_prepared_kernel(size in 0_usize..256, rounds in 0_usize..16) {
        prop_assert_eq!(pipeline(size, rounds), checksum_rounds(&fixture(size), rounds));
    }
}

#[test]
fn process_probe_reports_the_expected_work_and_rejects_empty_sample_sets() {
    let binary = env!("CARGO_BIN_EXE_profile_probe");
    let output = Command::new(binary)
        .args(["--size", "8", "--rounds", "3", "--samples", "2"])
        .output()
        .expect("profile probe must start");
    assert!(output.status.success(), "{:?}", output);
    let stdout = String::from_utf8(output.stdout).expect("probe emits UTF-8 JSON");
    assert!(stdout.contains(&format!("\"checksum\":{}", pipeline(8, 3))));
    assert!(stdout.contains("\"samples_ns\":["));
    let invalid = Command::new(binary)
        .args(["--samples", "0"])
        .output()
        .expect("profile probe must start");
    assert!(!invalid.status.success());
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("must be nonzero"));
}
