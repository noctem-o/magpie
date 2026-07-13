#!/usr/bin/env python3
"""Check Magpie's non-publishable release metadata and package inventories."""

from __future__ import annotations

import json
import os
from pathlib import Path
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[1]
PACKAGES = ("magpie-log", "magpie-claims", "magpie-episodic")
EXPECTED_VERSION = "0.1.0"
EXPECTED_REPOSITORY = "https://github.com/noctem-o/magpie"
REQUIRED_COMMON = {
    "Cargo.lock",
    "Cargo.toml",
    "Cargo.toml.orig",
    "LICENSE-APACHE",
    "LICENSE-MIT",
    "README.md",
    "src/lib.rs",
}
REQUIRED_PACKAGE_FILES = {
    "magpie-log": {
        "examples/regen_golden.rs",
        "testdata/golden-v1.jsonl",
        "tests/chain.rs",
        "tests/deadbolt_anchor_fixture.rs",
        "tests/golden.rs",
    },
    "magpie-claims": {
        "examples/tour.rs",
        "src/deterministic_verifier_context.rs",
        "tests/deadbolt_anchor_fixture.rs",
        "tests/deadbolt_occurrence_context.rs",
        "tests/deadbolt_occurrence_standing_v1.rs",
        "tests/deterministic_verifier_context.rs",
        "tests/regenerable.rs",
        "tests/standing_replay_snapshot.rs",
        "tests/standing_resolution.rs",
    },
    "magpie-episodic": {"tests/episodic.rs"},
}
PROHIBITED_PREFIXES = (".git/", ".github/", ".agent-runs/", "target/", "tickets/")


def fail(message: str) -> None:
    raise RuntimeError(message)


def run_cargo(*args: str) -> str:
    command = [os.environ.get("CARGO", "cargo"), *args]
    result = subprocess.run(
        command,
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if result.returncode != 0:
        fail(f"{' '.join(command)} failed:\n{result.stderr.strip()}")
    return result.stdout


def check_metadata() -> None:
    metadata = json.loads(
        run_cargo("metadata", "--locked", "--no-deps", "--format-version", "1")
    )
    packages = {package["name"]: package for package in metadata["packages"]}
    if set(packages) != set(PACKAGES):
        fail(f"unexpected workspace packages: {sorted(packages)}")

    for name in PACKAGES:
        package = packages[name]
        expected = {
            "version": EXPECTED_VERSION,
            "publish": [],
            "repository": EXPECTED_REPOSITORY,
            "license": "MIT OR Apache-2.0",
            "edition": "2021",
        }
        for field, value in expected.items():
            if package[field] != value:
                fail(f"{name}: expected {field}={value!r}, got {package[field]!r}")
        readme = Path(package["readme"])
        if not readme.is_absolute():
            readme = Path(package["manifest_path"]).parent / readme
        if readme.resolve() != (ROOT / "README.md").resolve():
            fail(f"{name}: README does not resolve to workspace README.md")

    for name in ("magpie-claims", "magpie-episodic"):
        dependency = next(
            dep for dep in packages[name]["dependencies"] if dep["name"] == "magpie-log"
        )
        if dependency["source"] is not None or dependency["req"] != "*":
            fail(f"{name}: magpie-log must remain an unversioned path-only dependency")
        expected_path = (ROOT / "crates" / "magpie-log").resolve()
        if Path(dependency["path"]).resolve() != expected_path:
            fail(f"{name}: magpie-log path does not resolve to {expected_path}")


def check_licenses() -> None:
    for license_name in ("LICENSE-APACHE", "LICENSE-MIT"):
        root_bytes = (ROOT / license_name).read_bytes()
        for package in PACKAGES:
            local = ROOT / "crates" / package / license_name
            if local.read_bytes() != root_bytes:
                fail(f"{local.relative_to(ROOT)} differs from root {license_name}")


def check_inventories() -> None:
    for package in PACKAGES:
        output = run_cargo("package", "--list", "-p", package, "--locked")
        files = {line.strip().replace("\\", "/") for line in output.splitlines() if line.strip()}
        required = REQUIRED_COMMON | REQUIRED_PACKAGE_FILES[package]
        missing = sorted(required - files)
        if missing:
            fail(f"{package}: missing inventory files: {', '.join(missing)}")
        prohibited = sorted(
            path
            for path in files
            if path.endswith(".crate")
            or path.startswith(PROHIBITED_PREFIXES)
            or Path(path).name.startswith(".env")
        )
        if prohibited:
            fail(f"{package}: prohibited inventory files: {', '.join(prohibited)}")
        print(f"{package}: inventory checked ({len(files)} files)")


def main() -> int:
    try:
        check_metadata()
        check_licenses()
        check_inventories()
    except (KeyError, OSError, StopIteration, ValueError, RuntimeError) as error:
        print(f"release metadata check failed: {error}", file=sys.stderr)
        return 1
    print("release metadata and package inventories are consistent")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
