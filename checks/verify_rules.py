#!/usr/bin/env python3
"""Require fresh compile evidence for every rule, including real project fixtures.

This is a per-rule minimum gate, not a claim that contextual fragments compile
standalone. The JSON report preserves every failed extracted block as well.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
CARGO = ["cargo", "+1.97.1"]


def command(args: list[str], cwd: Path, *, env: dict | None = None) -> dict:
    print("==>", cwd.relative_to(ROOT), " ".join(args), flush=True)
    completed = subprocess.run(args, cwd=cwd, env=env, capture_output=True,
                               text=True, encoding="utf-8", errors="replace")
    if completed.returncode:
        raise RuntimeError(completed.stdout[-3000:] + completed.stderr[-9000:])
    return {"command": args, "cwd": str(cwd.relative_to(ROOT)), "passed": True,
            "tests_passed": sum(map(int, re.findall(r"test result: ok\. (\d+) passed", completed.stdout)))}


def optimization_pgo(cwd: Path) -> list[dict]:
    evidence = []
    target = cwd / "target"
    target.mkdir(exist_ok=True)
    if not target.resolve().is_relative_to(ROOT):
        raise RuntimeError("PGO artifacts must stay inside the repository")
    sysroot = subprocess.check_output(["rustc", "+1.97.1", "--print", "sysroot"], text=True).strip()
    details = subprocess.check_output(["rustc", "+1.97.1", "-Vv"], text=True)
    host = re.search(r"^host: (.+)$", details, re.M).group(1)
    suffix = ".exe" if os.name == "nt" else ""
    llvm = Path(sysroot) / "lib" / "rustlib" / host / "bin" / f"llvm-profdata{suffix}"
    if not llvm.is_file():
        raise RuntimeError("PGO validation needs rustup component add llvm-tools-preview --toolchain 1.97.1")
    # A fresh owned directory keeps previous training data out of the experiment.
    with tempfile.TemporaryDirectory(prefix="pgo-", dir=target) as temporary:
        directory = Path(temporary).resolve()
        data = directory / "raw"
        data.mkdir()
        env = dict(os.environ)
        env["CARGO_ENCODED_RUSTFLAGS"] = f"-Cprofile-generate={data}"
        env["CARGO_TARGET_DIR"] = str(directory / "instrumented")
        evidence.append(command(CARGO + ["build", "--locked", "--release", "--target", host], cwd, env=env))
        binary = Path(env["CARGO_TARGET_DIR"]) / host / "release" / f"optimization-validation{suffix}"
        for size in (1, 100, 1000):
            evidence.append(command([str(binary), str(size)], cwd))
        raw = sorted(data.glob("*.profraw"))
        if not raw:
            raise RuntimeError("instrumented program did not produce a profile")
        merged = directory / "merged.profdata"
        evidence.append(command([str(llvm), "merge", "-o", str(merged)] + [str(path) for path in raw], cwd))
        env["CARGO_ENCODED_RUSTFLAGS"] = f"-Cprofile-use={merged}"
        env["CARGO_TARGET_DIR"] = str(directory / "optimized")
        evidence.append(command(CARGO + ["build", "--locked", "--release", "--target", host], cwd, env=env))
        optimized = Path(env["CARGO_TARGET_DIR"]) / host / "release" / f"optimization-validation{suffix}"
        result = subprocess.check_output([str(optimized), "100"], cwd=cwd, text=True).strip()
        if result != "4950":
            raise RuntimeError("PGO optimized output changed")
        evidence.append({"check": "PGO optimized executable output", "expected": "4950", "passed": True})
    return evidence


def fixtures(names: set[str]) -> dict:
    result = {}
    for name in sorted(names):
        if name == "performance":
            cwd = ROOT / "perf-validation"
            commands = [["test", "--locked"], ["bench", "--locked", "--bench", "perf_rules", "--", "--test"]]
        elif name in {"layouts", "proc-macros", "optimization", "cfg-check"}:
            cwd = HERE / "fixtures" / name
            commands = [["test", "--locked", "--workspace"]]
            if name not in {"optimization", "cfg-check"}:
                commands.append(["test", "--locked", "--workspace", "--doc"])
        else:
            raise RuntimeError(f"unknown fixture marker: {name}")
        result[name] = [command(CARGO + args, cwd) for args in commands]
        if name == "optimization":
            for profile in ("release-lto", "release-codegen"):
                result[name].append(command(CARGO + ["build", "--locked", "--profile", profile], cwd))
            result[name] += optimization_pgo(cwd)
        if name == "layouts":
            result[name].append(command(CARGO + ["doc", "--locked", "--workspace", "--no-deps"], cwd))
            result[name].append(command(CARGO + ["fmt", "--all", "--", "--check"], cwd))
            result[name].append(command(CARGO + ["clippy", "--locked", "--workspace", "--all-targets", "--", "-D", "warnings"], cwd))
        diagnostic_check = cwd / "verify_diagnostics.py"
        if name == "proc-macros" and diagnostic_check.exists():
            result[name].append(command([sys.executable, str(diagnostic_check)], cwd))
        if name == "cfg-check":
            result[name].append(command([sys.executable, str(cwd / "verify_cfg.py")], cwd))
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", nargs="?", default="check.json", help="fresh Cargo JSON after gen.py")
    args = parser.parse_args()
    (HERE / "verification.json").write_text('{"status":"running"}\n', encoding="utf-8")
    spec = importlib.util.spec_from_file_location("analysis", HERE / "analyze.py")
    analysis = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(analysis)
    manifest = analysis.manifest
    output = HERE / args.output
    if output.stat().st_mtime_ns < (HERE / "manifest.json").stat().st_mtime_ns:
        raise RuntimeError("Cargo output predates the generated manifest; rerun the compile check")
    for info in manifest.values():
        text = (ROOT / "rules" / info["file"]).read_text(encoding="utf-8")
        if info.get("source_sha256") != hashlib.sha256(text.encode()).hexdigest():
            raise RuntimeError(f"source changed since extraction: {info['file']}; rerun gen.py and Cargo")
    errors = analysis.parse(output)
    def has_rust_content(info: dict) -> bool:
        source_lines = (ROOT / "rules" / info["file"]).read_text(encoding="utf-8").splitlines()
        block = []
        for line in source_lines[info["line"] - 1:]:
            if line.strip() == "```":
                break
            block.append(line)
        return any(line.strip() and not line.lstrip().startswith(("//", "use ")) for line in block)

    clean = {name: info for name, info in manifest.items() if name not in errors and has_rust_content(info)}
    rule_files = sorted((ROOT / "rules").glob("*.md"))
    marker_pattern = re.compile(r"<!-- verify: fixture=([a-z-]+) -->")
    texts = {path.name: path.read_text(encoding="utf-8") for path in rule_files}
    fixture_names = {name for text in texts.values() for name in marker_pattern.findall(text)}
    fixture_results = fixtures(fixture_names)
    coverage = {}
    missing = []
    harness_examples = {}
    for path in rule_files:
        text = texts[path.name]
        lines = text.splitlines()
        selected_lines = {index + 2 for index, line in enumerate(lines)
                          if line.strip() == "```rust" and index > 0 and
                          any("<!-- verify: compile -->" in previous for previous in lines[max(0, index - 2):index])}
        candidates = [(name, info) for name, info in clean.items() if info["file"] == path.name]
        selected = [(name, info) for name, info in candidates if info["line"] in selected_lines]
        failing_selected = selected_lines - {info["line"] for _, info in selected}
        if failing_selected:
            raise RuntimeError(f"marked positive examples do not compile: {path.name}:{sorted(failing_selected)}")
        project_names = marker_pattern.findall(text)
        if not project_names and not candidates:
            missing.append(path.name)
            continue
        preferred = selected or sorted(candidates, key=lambda pair: ("good" not in pair[1]["section"].lower(), pair[1]["line"]))[:1]
        for name, info in preferred:
            source = (HERE / "examples" / f"{name}.rs").read_text(encoding="utf-8")
            active_code = "\n".join(line for line in source.splitlines()
                                    if not line.lstrip().startswith("//"))
            has_tests = re.search(r'#\s*\[\s*(?:[a-z_]+::)*test(?:\s*\([^]]*\))?\s*\]', active_code) is not None
            if has_tests or re.search(r'#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]', active_code):
                harness_examples[name] = has_tests
        coverage[path.stem] = {"sha256": hashlib.sha256(text.encode()).hexdigest(),
                               "examples": [info for _, info in preferred], "fixtures": project_names}
    if missing:
        raise RuntimeError("rules without compile evidence:\n" + "\n".join(missing))
    harness_results = []
    for name in sorted(harness_examples):
        evidence = command(CARGO + ["test", "--locked", "--example", name], HERE)
        if harness_examples[name] and evidence["tests_passed"] == 0:
            raise RuntimeError(f"test-bearing example ran zero tests: {name}")
        harness_results.append(evidence)
    report = {"status": "passed", "toolchain": "1.97.1", "compile_target": "x86_64-unknown-linux-gnu",
              "rules_covered": len(coverage), "blocks_checked": len(manifest),
              "blocks_compiled_cleanly": len(manifest) - len(errors),
              "eligible_positive_blocks": len(clean), "rules": coverage,
              "fixtures": fixture_results, "test_harness_checks": harness_results,
              "contextual_or_invalid_blocks": [{**manifest[name], "codes": sorted({analysis.code_of(diag) or diag['message'] for diag in diags})}
                                               for name, diags in sorted(errors.items())]}
    (HERE / "verification.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(f"PASS: {len(coverage)}/{len(rule_files)} rules have fresh compile evidence; "
          f"{report['blocks_compiled_cleanly']}/{len(manifest)} extracted blocks pass in the checked configuration "
          f"({len(clean)} contain nontrivial Rust content).")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (RuntimeError, OSError, subprocess.CalledProcessError) as error:
        (HERE / "verification.json").write_text(json.dumps({"status": "failed", "error": str(error)}) + "\n", encoding="utf-8")
        print(f"RULE VERIFICATION FAILED: {error}", file=sys.stderr)
        raise SystemExit(1)
