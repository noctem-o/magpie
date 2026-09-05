#!/usr/bin/env python3
"""Emit the exact CI validation environment as canonical JSON plus SHA-256.

This is validation provenance, not a hermetic-build, artifact-signing, release,
or authority mechanism. In pull-request workflows the candidate head can differ
from both ``GITHUB_SHA`` and the commit actually checked out for validation, so
the record keeps those identities separate.
"""

from __future__ import annotations

import hashlib
import importlib.metadata
import json
import os
from pathlib import Path
import platform
import subprocess
import sys
from typing import Callable, Mapping, Sequence


ROOT = Path(__file__).resolve().parents[1]
FROZEN_CORPUS_MANIFEST_SHA256 = (
    "7d758d3f2dac1161fe15dd064b130ccbfcdaf0437ea8ab8d7772492194801a81"
)
TREE_IDENTITY_PROFILE = "magpie-validation-tree-sha256-v1"
GITHUB_ACTIONS_REQUIRED_COORDINATES = (
    "GITHUB_EVENT_NAME",
    "GITHUB_JOB",
    "GITHUB_REF",
    "GITHUB_RUN_ATTEMPT",
    "GITHUB_RUN_ID",
    "GITHUB_SHA",
    "GITHUB_WORKFLOW",
    "GITHUB_WORKFLOW_REF",
    "ImageOS",
    "ImageVersion",
    "MAGPIE_CANDIDATE_SHA",
    "MAGPIE_RUNNER_LABEL",
    "MAGPIE_WORKFLOW_SHA",
    "RUNNER_ARCH",
    "RUNNER_ENVIRONMENT",
    "RUNNER_NAME",
    "RUNNER_OS",
)
PULL_REQUEST_REQUIRED_COORDINATES = ("GITHUB_BASE_REF", "GITHUB_HEAD_REF")


class ValidationEnvironmentError(RuntimeError):
    """The hosted assurance environment cannot produce a complete record."""


def _sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def _sha256_file(path: Path) -> str:
    return _sha256_bytes(path.read_bytes())


def _tree_identity(path: Path) -> dict[str, object]:
    """Return a deterministic SHA-256 identity for tracked-style tree content.

    Empty directories are ignored, as Git ignores them. Each file entry binds
    its type, UTF-8/POSIX relative path, byte length, and content digest using
    explicit length prefixes. Symlinks, when present, bind their link target
    bytes rather than the target file contents.
    """

    entries: list[tuple[str, str, bytes]] = []
    for entry in path.rglob("*"):
        relative = entry.relative_to(path).as_posix()
        if entry.is_symlink():
            entries.append((relative, "symlink", os.readlink(entry).encode("utf-8")))
        elif entry.is_file():
            entries.append((relative, "file", entry.read_bytes()))

    digest = hashlib.sha256()
    digest.update(TREE_IDENTITY_PROFILE.encode("ascii") + b"\0")
    for relative, entry_type, content in sorted(entries):
        relative_bytes = relative.encode("utf-8")
        type_bytes = entry_type.encode("ascii")
        content_digest = hashlib.sha256(content).digest()
        digest.update(len(type_bytes).to_bytes(8, "big"))
        digest.update(type_bytes)
        digest.update(len(relative_bytes).to_bytes(8, "big"))
        digest.update(relative_bytes)
        digest.update(len(content).to_bytes(8, "big"))
        digest.update(content_digest)

    return {
        "entry_count": len(entries),
        "identity_profile": TREE_IDENTITY_PROFILE,
        "sha256": digest.hexdigest(),
    }


def _command_output(command: Sequence[str]) -> str:
    return subprocess.check_output(
        list(command),
        cwd=ROOT,
        stderr=subprocess.STDOUT,
        text=True,
    ).strip()


def _distribution_version(name: str) -> str:
    return importlib.metadata.version(name)


def _libsodium_version() -> dict[str, str | None]:
    """Query a public PyNaCl version hook when the installed build exposes one."""

    import nacl.bindings

    version_function = getattr(nacl.bindings, "sodium_version_string", None)
    if callable(version_function):
        value = version_function()
        if isinstance(value, bytes):
            value = value.decode("ascii")
        return {"source": "nacl.bindings.sodium_version_string", "version": str(value)}
    return {
        "source": "unavailable: PyNaCl runtime API exposes no libsodium version hook",
        "version": None,
    }


def _input_identity(relative_path: str) -> dict[str, str]:
    path = ROOT / relative_path
    return {"path": relative_path, "sha256": _sha256_file(path)}


def _validate_github_actions_environment(environment: Mapping[str, str]) -> str | None:
    if environment.get("GITHUB_ACTIONS") != "true":
        return None

    event_name = environment.get("GITHUB_EVENT_NAME")
    required = list(GITHUB_ACTIONS_REQUIRED_COORDINATES)
    if event_name is not None and event_name.strip() == "pull_request":
        required.extend(PULL_REQUEST_REQUIRED_COORDINATES)

    missing = sorted(
        name
        for name in required
        if name not in environment or not environment[name].strip()
    )
    if missing:
        raise ValidationEnvironmentError(
            "missing required GitHub Actions validation provenance coordinates: "
            + ", ".join(missing)
        )

    event_name = environment["GITHUB_EVENT_NAME"].strip()
    if event_name not in {"pull_request", "push"}:
        raise ValidationEnvironmentError(
            "unsupported GitHub Actions event for validation provenance: "
            + event_name
        )
    return event_name


def collect_environment(
    environ: Mapping[str, str] | None = None,
    command_output: Callable[[Sequence[str]], str] = _command_output,
) -> dict[str, object]:
    environment = os.environ if environ is None else environ
    _validate_github_actions_environment(environment)
    manifest = _input_identity("fixtures/verifier-language-v1/manifest.json")
    if manifest["sha256"] != FROZEN_CORPUS_MANIFEST_SHA256:
        raise RuntimeError(
            "frozen portable corpus manifest digest changed: "
            f"expected {FROZEN_CORPUS_MANIFEST_SHA256}, got {manifest['sha256']}"
        )

    vendor_identity = _tree_identity(ROOT / "tools/go-verify-chain/vendor")
    vendor_identity["path"] = "tools/go-verify-chain/vendor"

    return {
        "governed_inputs": {
            "cargo_lock": _input_identity("Cargo.lock"),
            "ci_workflow": _input_identity(".github/workflows/ci.yml"),
            "frozen_portable_corpus_manifest": manifest,
            "go_mod": _input_identity("tools/go-verify-chain/go.mod"),
            "go_sum": _input_identity("tools/go-verify-chain/go.sum"),
            "go_vendor_tree": vendor_identity,
            "python_requirements": _input_identity(
                "tools/requirements-portable-verifier.txt"
            ),
        },
        "runner": {
            "architecture": environment.get("RUNNER_ARCH"),
            "environment": environment.get("RUNNER_ENVIRONMENT"),
            "hosted_image_os": environment.get("ImageOS"),
            "hosted_image_version": environment.get("ImageVersion"),
            "name": environment.get("RUNNER_NAME"),
            "os": environment.get("RUNNER_OS"),
            "selected_label": environment.get("MAGPIE_RUNNER_LABEL"),
        },
        "schema": "magpie-validation-environment-v1",
        "source_and_workflow": {
            "actual_checked_out_head_sha": command_output(
                ["git", "rev-parse", "HEAD"]
            ),
            "candidate_head_sha": environment.get("MAGPIE_CANDIDATE_SHA"),
            "event_name": environment.get("GITHUB_EVENT_NAME"),
            "github_base_ref": environment.get("GITHUB_BASE_REF"),
            "github_head_ref": environment.get("GITHUB_HEAD_REF"),
            "github_ref": environment.get("GITHUB_REF"),
            "github_sha": environment.get("GITHUB_SHA"),
            "job": environment.get("GITHUB_JOB"),
            "run_attempt": environment.get("GITHUB_RUN_ATTEMPT"),
            "run_id": environment.get("GITHUB_RUN_ID"),
            "workflow": environment.get("GITHUB_WORKFLOW"),
            "workflow_ref": environment.get("GITHUB_WORKFLOW_REF"),
            "workflow_sha": environment.get("MAGPIE_WORKFLOW_SHA"),
        },
        "toolchains": {
            "go": {
                "goarch": command_output(["go", "env", "GOARCH"]),
                "goos": command_output(["go", "env", "GOOS"]),
                "version": command_output(["go", "version"]),
            },
            "python": {
                "cffi_version": _distribution_version("cffi"),
                "cryptography_version": _distribution_version("cryptography"),
                "implementation": platform.python_implementation(),
                "libsodium": _libsodium_version(),
                "pip_version": _distribution_version("pip"),
                "pip_version_output": command_output(
                    [sys.executable, "-m", "pip", "--version"]
                ),
                "pynacl_version": _distribution_version("PyNaCl"),
                "pycparser_version": _distribution_version("pycparser"),
                "version": platform.python_version(),
                "version_output": command_output([sys.executable, "--version"]),
            },
            "rust": {
                "cargo_version": command_output(["cargo", "--version"]),
                "rustc_verbose_version": command_output(["rustc", "-Vv"]),
            },
        },
    }


def canonical_record(record: Mapping[str, object]) -> bytes:
    return json.dumps(
        record,
        ensure_ascii=True,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")


def main() -> int:
    try:
        serialized = canonical_record(collect_environment())
    except ValidationEnvironmentError as error:
        print(f"validation environment error: {error}", file=sys.stderr)
        return 2
    print("validation-environment-json=" + serialized.decode("ascii"))
    print("validation-environment-sha256=" + _sha256_bytes(serialized))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
