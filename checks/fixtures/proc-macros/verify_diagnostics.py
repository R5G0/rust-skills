#!/usr/bin/env python3
"""Compile invalid macro callers and require diagnostics at the offending input."""

import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parent
CASES = (
    ("enum", "enum Invalid { One }", "MyTrait can only be derived on structs, not enums", 2),
    ("union", "union Invalid { value: u32 }", "MyTrait can only be derived on structs, not unions", 2),
    ("empty", "struct Invalid;", "MyTrait requires at least one field", 2),
    ("tuple", "struct Invalid(u8);", "MyTrait requires named fields", 2),
)


def main():
    for name, definition, expected, line in CASES:
        directory = ROOT / "target" / "diagnostic-cases" / name
        (directory / "src").mkdir(parents=True, exist_ok=True)
        facade = (ROOT / "facade").as_posix()
        manifest = (
            f'[package]\nname = "invalid-{name}"\nversion = "0.0.0"\nedition = "2024"\n'
            f'[workspace]\n[dependencies]\nchecked-macros = {{ path = "{facade}" }}\n'
        )
        (directory / "Cargo.toml").write_text(manifest, encoding="utf-8")
        (directory / "src" / "main.rs").write_text(
            "#[derive(checked_macros::MyTrait)]\n" + definition + "\nfn main() {}\n",
            encoding="utf-8",
        )
        command = ["cargo", "check", "--offline", "--manifest-path", str(directory / "Cargo.toml"),
                   "--target-dir", str(ROOT / "target" / "diagnostic-build"), "--message-format=json"]
        result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, encoding="utf-8")
        messages = []
        for output_line in result.stdout.splitlines():
            try:
                event = json.loads(output_line)
            except json.JSONDecodeError:
                continue
            if event.get("reason") == "compiler-message":
                messages.append(event["message"])
        errors = [message for message in messages if message.get("level") == "error"]
        matching = [message for message in errors if message.get("message") == expected]
        if result.returncode == 0 or len(matching) != 1:
            raise SystemExit(f"{name}: expected one {expected!r} diagnostic; got {errors!r}\n{result.stderr}")
        spans = matching[0].get("spans", [])
        if not any(span.get("is_primary") and span.get("line_start") == line
                   and span.get("file_name", "").replace("\\", "/").endswith("src/main.rs") for span in spans):
            raise SystemExit(f"{name}: diagnostic did not point at the offending declaration: {spans!r}")
        if any("panicked" in message.get("message", "") for message in errors):
            raise SystemExit(f"{name}: macro panicked")
        print(f"OK: {name} emits the expected error at the offending input")


if __name__ == "__main__":
    main()
