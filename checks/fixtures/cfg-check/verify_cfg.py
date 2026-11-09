#!/usr/bin/env python3
"""Execute enabled cfg branches and require precise diagnostics for cfg typos."""

from __future__ import annotations

import json
import os
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parent
TARGET = ROOT / "target"
CARGO = ["cargo", "+1.97.1"]
BASE_TEST = "base_transform_is_available"
FEATURE_TEST = "checked_feature_body_executes"
CUSTOM_TEST = "custom_cfg_body_executes"
CONFIGURATIONS = (
    ("default", [], False, (BASE_TEST,)),
    ("all-features", ["--all-features"], False, (BASE_TEST, FEATURE_TEST)),
    ("custom-cfg", [], True, (BASE_TEST, CUSTOM_TEST)),
    ("all-flags", ["--all-features"], True, (BASE_TEST, FEATURE_TEST, CUSTOM_TEST)),
)
NEGATIVE_CASES = (
    ("feature-typo", 'feature = "cheked"', "cheked"),
    ("custom-cfg-typo", "validaton_mode", "validaton_mode"),
)


def environment(directory: Path, *, custom_cfg: bool = False) -> dict[str, str]:
    result = dict(os.environ)
    # Make each configuration explicit without changing the user's environment.
    result.pop("RUSTFLAGS", None)
    result.pop("CARGO_ENCODED_RUSTFLAGS", None)
    result["CARGO_TARGET_DIR"] = str(directory)
    if custom_cfg:
        result["CARGO_ENCODED_RUSTFLAGS"] = "--cfg\x1fvalidation_mode"
    return result


def execute(label: str, command: list[str], env: dict[str, str]) -> subprocess.CompletedProcess:
    print(f"==> {label}", flush=True)
    completed = subprocess.run(command, cwd=ROOT, env=env, capture_output=True,
                               text=True, encoding="utf-8", errors="replace", check=False)
    (TARGET / f"{label}.log").write_text(completed.stdout + completed.stderr,
                                        encoding="utf-8")
    return completed


def require_success(label: str, completed: subprocess.CompletedProcess) -> None:
    if completed.returncode:
        raise RuntimeError(f"{label} failed:\n{completed.stdout}\n{completed.stderr}")


def main() -> int:
    TARGET.mkdir(parents=True, exist_ok=True)
    rustc = subprocess.check_output(["rustc", "+1.97.1", "-Vv"], text=True).strip()
    host = re.search(r"^host: (.+)$", rustc, re.M).group(1)
    report = {"rustc": rustc, "host": host, "configurations": [], "negative_cases": []}

    for label, flags, custom, expected in CONFIGURATIONS:
        command = CARGO + ["test", "--offline", "--locked", "--workspace", "--target", host] + flags
        completed = execute(label, command, environment(TARGET / label, custom_cfg=custom))
        require_success(label, completed)
        log = completed.stdout + completed.stderr
        for name in expected:
            if not re.search(r"\b" + re.escape(name) + r"\s+\.\.\.\s+ok\b", log):
                raise RuntimeError(f"{label}: expected enabled test {name!r} did not execute")
        disabled = {BASE_TEST, FEATURE_TEST, CUSTOM_TEST} - set(expected)
        for name in disabled:
            if re.search(r"\b" + re.escape(name) + r"\s+\.\.\.", log):
                raise RuntimeError(f"{label}: unexpected disabled test {name!r} executed")
        report["configurations"].append({"name": label, "command": command,
                                          "custom_cfg_enabled": custom,
                                          "executed_tests": list(expected), "passed": True})

    for name, condition, token in NEGATIVE_CASES:
        directory = TARGET / "negative-cases" / name
        (directory / "src").mkdir(parents=True, exist_ok=True)
        (directory / "Cargo.toml").write_text(
            '[package]\nname = "invalid-' + name + '"\nversion = "0.0.0"\nedition = "2024"\n'
            '[workspace]\n[features]\nchecked = []\n[lints.rust]\n'
            'unexpected_cfgs = { level = "deny", check-cfg = [\'cfg(validation_mode)\'] }\n',
            encoding="utf-8",
        )
        (directory / "src" / "lib.rs").write_text(
            f"#[cfg({condition})]\npub fn hidden() -> u32 {{ 42 }}\n", encoding="utf-8")
        env = environment(TARGET / "negative-build")
        lock_command = CARGO + ["generate-lockfile", "--offline", "--manifest-path", str(directory / "Cargo.toml")]
        require_success(name + "-lock", execute(name + "-lock", lock_command, env))
        command = CARGO + ["check", "--offline", "--locked", "--manifest-path", str(directory / "Cargo.toml"),
                           "--target", host, "--message-format=json"]
        completed = execute(name, command, env)
        errors = []
        for line in completed.stdout.splitlines():
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue
            if event.get("reason") == "compiler-message" and event.get("message", {}).get("level") == "error":
                errors.append(event["message"])
        matching = [error for error in errors
                    if (error.get("code") or {}).get("code") == "unexpected_cfgs"
                    and token in error.get("message", "")]
        if completed.returncode == 0 or len(matching) != 1 or len(errors) != 1:
            raise RuntimeError(f"{name}: expected exactly one unexpected_cfgs error for {token!r}; got {errors!r}")
        if not any(span.get("is_primary") and span.get("line_start") == 1
                   and span.get("file_name", "").replace("\\", "/").endswith("src/lib.rs")
                   for span in matching[0].get("spans", [])):
            raise RuntimeError(f"{name}: diagnostic does not point at the misspelled cfg")
        report["negative_cases"].append({"name": name, "command": command,
                                         "diagnostic_code": "unexpected_cfgs", "token": token,
                                         "primary_line": 1, "passed": True})

    (TARGET / "verification.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print("PASS: four real cfg configurations and two precise unexpected_cfgs diagnostics")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (RuntimeError, OSError, subprocess.CalledProcessError) as error:
        raise SystemExit(f"cfg verification failed: {error}")
