#!/usr/bin/env python3
"""Run the native Rust validation project and retain auditable results.

No shell commands are evaluated. Timings are observations, never pass/fail gates.
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import platform
import re
import statistics
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
ARTIFACTS = ROOT / "results" / "artifacts"
RULES = sorted(path.stem for path in (ROOT.parent / "rules").glob("perf-*.md"))


def run(label: str, command: list[str], records: list[dict]) -> str:
    print(f"==> {label}", flush=True)
    started = time.perf_counter()
    completed = subprocess.run(command, cwd=ROOT, capture_output=True, text=True,
                               encoding="utf-8", errors="replace", check=False)
    elapsed = time.perf_counter() - started
    log = completed.stdout + completed.stderr
    (ARTIFACTS / f"{label}.log").write_text(log, encoding="utf-8")
    records.append({"check": label, "command": command, "exit_code": completed.returncode,
                    "elapsed_seconds": round(elapsed, 3),
                    "tests_passed": sum(map(int, re.findall(r"test result: ok\. (\d+) passed", log)))})
    if completed.returncode:
        print(log[-16_000:], file=sys.stderr)
        raise RuntimeError(f"{label} failed with exit code {completed.returncode}")
    print(f"    passed ({elapsed:.2f}s)", flush=True)
    return completed.stdout


def criterion_results(target: Path, expected_ids: set[str], started_ns: int) -> list[dict]:
    results = []
    for path in sorted((target / "criterion").glob("**/new/estimates.json")):
        benchmark_path = path.parent / "benchmark.json"
        if not benchmark_path.exists() or path.stat().st_mtime_ns < started_ns:
            continue
        identity = json.loads(benchmark_path.read_text(encoding="utf-8"))
        if identity["full_id"] not in expected_ids:
            continue
        estimates = json.loads(path.read_text(encoding="utf-8"))
        mean = estimates["mean"]
        results.append({"id": identity["full_id"], "mean_ns": mean["point_estimate"],
                        "confidence_interval_ns": mean["confidence_interval"]})
    return results


def profile_matrix(cargo: list[str], target: Path, records: list[dict]) -> list[dict]:
    results = []
    for profile in ("release", "release-lto", "release-codegen", "release-tuned"):
        run(f"build-{profile}", cargo + ["build", "--locked", "--profile", profile,
                                        "--bin", "profile_probe"], records)
        executable = target / profile / ("profile_probe.exe" if os.name == "nt" else "profile_probe")
        stdout = run(f"probe-{profile}", [str(executable)], records)
        result = json.loads(stdout)
        result.update({"profile": profile, "binary_bytes": executable.stat().st_size,
                       "median_ns": statistics.median(result["samples_ns"])})
        marker = ARTIFACTS / f"{profile}-panic-drop.txt"
        marker.unlink(missing_ok=True)
        completed = subprocess.run([str(executable), "--panic-marker", str(marker)], cwd=ROOT,
                                   capture_output=True, text=True, check=False)
        (ARTIFACTS / f"panic-{profile}.log").write_text(completed.stdout + completed.stderr,
                                                      encoding="utf-8")
        should_unwind = profile != "release-tuned"
        if completed.returncode == 0 or marker.exists() != should_unwind:
            raise RuntimeError(f"unexpected panic contract for {profile}")
        result["panic_probe"] = {"exit_code": completed.returncode, "drop_ran": marker.exists()}
        results.append(result)
    if len({result["checksum"] for result in results}) != 1:
        raise RuntimeError("release profiles changed the workload result")
    return results


def write_summary(report: dict) -> None:
    lines = ["# Local verification results", "", f"Status: **{report['status']}**.", "",
             f"Host: `{report['platform']}`. CPU: `{report['cpu']}`.", "",
             "Timings describe this host and these synthetic fixtures. They do not establish universal speedups.", "",
             "## Executed checks", "", "| Check | Passed tests | Result |", "|---|---:|---|"]
    for check in report["checks"]:
        lines.append(f"| {check['check']} | {check['tests_passed']} | {'passed' if check['exit_code'] == 0 else 'failed'} |")
    lines += ["", f"Nightly collect_into: {report['nightly_collect_into']}.", "",
              "## Criterion measurements", "",
              "Means and their 95% confidence intervals are in nanoseconds per iteration. Different sizes or scopes are not A/B pairs.", ""]
    for rule in report["rules"]:
        measurements = [item for item in report["benchmarks"] if item["id"].split("/", 1)[0] == rule]
        lines += [f"### {rule}", "", f"Measured cases: {len(measurements)}.", ""]
        if measurements:
            lines += ["| Case | Mean (ns) | 95% interval (ns) |", "|---|---:|---|"]
            for item in measurements:
                interval = item["confidence_interval_ns"]
                name = item["id"].split("/", 1)[1]
                lines.append(f"| {name} | {item['mean_ns']:.3f} | {interval['lower_bound']:.3f}–{interval['upper_bound']:.3f} |")
            lines.append("")
    lines += ["## Production profile artifacts", "",
              "Kernel medians are warm in-process measurements; build caches differ, so build durations are not compile-cost comparisons.", "",
              "| Profile | Binary bytes | Kernel median (ns) | Drop on panic |", "|---|---:|---:|---|"]
    for item in report["profiles"]:
        lines.append(f"| {item['profile']} | {item['binary_bytes']} | {item['median_ns']:.0f} | {item['panic_probe']['drop_ran']} |")
    lines += ["", "Source mappings and counterexamples: [findings](README.md). Raw run record: [latest.json](latest.json).", ""]
    (ROOT / "results" / "summary.md").write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bench", action="store_true", help="collect Criterion measurements, not just smoke tests")
    parser.add_argument("--profiles", action="store_true", help="build and execute the release profile matrix")
    parser.add_argument("--nightly", action="store_true", help="also run collect_into on an installed nightly toolchain")
    parser.add_argument("--offline", action="store_true", help="use only cached Cargo dependencies")
    args = parser.parse_args()
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    records: list[dict] = []
    cargo = ["cargo", "+1.97.1"]
    if args.offline:
        cargo.append("--offline")
    report = {"created_utc": datetime.now(timezone.utc).isoformat(),
              "platform": platform.platform(), "cpu": os.environ.get("PROCESSOR_IDENTIFIER", platform.processor()),
              "logical_cpus": os.cpu_count(), "rules": RULES,
              "checks": records, "benchmarks": [], "profiles": [],
              "nightly_collect_into": "not run; requires an installed nightly toolchain"}
    try:
        report["rustc"] = run("toolchain", ["rustc", "+1.97.1", "-Vv"], records).strip()
        report["base_revision"] = subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
        metadata = json.loads(subprocess.check_output(
            cargo + ["metadata", "--locked", "--no-deps", "--format-version", "1"], cwd=ROOT, text=True))
        target = Path(metadata["target_directory"])
        run("format", cargo + ["fmt", "--all", "--", "--check"], records)
        run("clippy", cargo + ["clippy", "--locked", "--all-targets", "--", "-D", "warnings"], records)
        run("tests-debug", cargo + ["test", "--locked"], records)
        run("tests-release", cargo + ["test", "--locked", "--release"], records)
        listing = run("benchmark-list", cargo + ["bench", "--locked", "--bench", "perf_rules", "--", "--list"], records)
        benchmark_ids = {line.removesuffix(": benchmark") for line in listing.splitlines()
                         if line.startswith("perf-") and line.endswith(": benchmark")}
        registered = {identity.split("/", 1)[0] for identity in benchmark_ids}
        missing = sorted(set(RULES) - registered)
        if missing:
            raise RuntimeError(f"rules missing executable benchmark coverage: {missing}")
        report["registered_rule_groups"] = sorted(registered)
        report["registered_benchmarks"] = sorted(benchmark_ids)
        run("benchmark-smoke", cargo + ["bench", "--locked", "--bench", "perf_rules", "--", "--test"], records)
        if args.bench:
            measurement_started_ns = time.time_ns()
            run("benchmark-measurements", cargo + ["bench", "--locked", "--bench", "perf_rules"], records)
            report["measurement_started_ns"] = measurement_started_ns
            report["benchmarks"] = criterion_results(target, benchmark_ids, measurement_started_ns)
            measured = {result["id"] for result in report["benchmarks"]}
            if benchmark_ids != measured:
                raise RuntimeError(f"missing fresh Criterion measurements: {sorted(benchmark_ids - measured)}")
        if args.profiles:
            report["profiles"] = profile_matrix(cargo, target, records)
        if args.nightly:
            report["nightly_rustc"] = run("nightly-toolchain", ["rustc", "+nightly", "-Vv"], records).strip()
            run("nightly-collect-into", ["cargo", "+nightly"] + (["--offline"] if args.offline else []) +
                ["run", "--locked", "--example", "collect_into_nightly", "--features", "nightly-collect-into"], records)
            report["nightly_collect_into"] = "passed"
        report["status"] = "passed"
    except (RuntimeError, subprocess.CalledProcessError, OSError, ValueError) as error:
        report["status"] = "failed"
        report["error"] = str(error)
        print(str(error), file=sys.stderr)
    destination = ROOT / "results" / "latest.json"
    destination.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    write_summary(report)
    print(f"Report: {destination}", flush=True)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
