//! A process-level probe built under each release profile.

use rust_skills_perf_validation::measurement::{checksum_rounds, fixture};
use std::hint::black_box;
use std::path::PathBuf;
use std::time::Instant;

struct PanicMarker(PathBuf);

impl Drop for PanicMarker {
    fn drop(&mut self) {
        // A successful write demonstrates unwinding; an abort skips Drop.
        let _ = std::fs::write(&self.0, b"unwound");
    }
}

fn number(args: &[String], flag: &str, default: usize) -> Result<usize, String> {
    let Some(index) = args.iter().position(|argument| argument == flag) else {
        return Ok(default);
    };
    args.get(index + 1)
        .ok_or_else(|| format!("missing value for {flag}"))?
        .parse::<usize>()
        .map_err(|error| format!("invalid {flag}: {error}"))
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if let Some(index) = args
        .iter()
        .position(|argument| argument == "--panic-marker")
    {
        let path = args.get(index + 1).ok_or("missing panic marker path")?;
        let _marker = PanicMarker(PathBuf::from(path));
        panic!("intentional process-level panic contract probe");
    }
    let size = number(&args, "--size", 32_768)?;
    let rounds = number(&args, "--rounds", 64)?;
    let samples = number(&args, "--samples", 7)?;
    if samples == 0 || rounds == 0 {
        return Err("samples and rounds must be nonzero".to_owned());
    }
    let started = Instant::now();
    let data = fixture(size);
    let setup_ns = started.elapsed().as_nanos();
    let expected = checksum_rounds(&data, rounds);
    // Warm the code and data before collecting samples.
    black_box(checksum_rounds(black_box(&data), black_box(rounds)));
    let mut times = Vec::with_capacity(samples);
    for _ in 0..samples {
        let started = Instant::now();
        let result = black_box(checksum_rounds(black_box(&data), black_box(rounds)));
        times.push(started.elapsed().as_nanos());
        if result != expected {
            return Err("profile probe produced an inconsistent checksum".to_owned());
        }
    }
    println!(
        "{{\"size\":{size},\"rounds\":{rounds},\"checksum\":{expected},\"setup_ns\":{setup_ns},\"samples_ns\":{times:?}}}"
    );
    Ok(())
}

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}
