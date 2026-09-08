"""Check C4.3 dependency and native qualification against the repository.

This checker binds the committed qualification inventory
(``tools/dependency_native_inventory.json``) to the repository files it
covers and to governed constants. It is the C4.3 dependency/native
evidence gate for the ``magpie-public-prealpha-assurance-v1`` profile:

- Rust: ``rusqlite 0.37.0`` with the ``bundled`` feature is declared in the
  workspace, locked to ``libsqlite3-sys 0.35.0``, and used by the crates
  that store log and episodic data.
- Python: the portable-verifier lane requirements pin exactly four
  distributions, each bound to one lane wheel (CPython 3.12, linux,
  x86_64), and the workflow installs only from that requirements file.
- Go: the vendored verifier module retains its single owner-approved
  dependency.
- GitHub Actions: every action reference uses a full-SHA pin.
- Native states: the bundled SQLite version is recorded from the
  hash-verified ``libsqlite3-sys`` crate, and libsodium is recorded as
  UNAVAILABLE with no version inference.

States used: SELECTED, BOUND, OBSERVED, INFERRED, UNAVAILABLE. This checker
performs no vulnerability review and never reports a skipped or unavailable
check as PASS.
"""

from __future__ import annotations

import hashlib
import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

INVENTORY_RELATIVE = "tools/dependency_native_inventory.json"

INVENTORY_SCHEMA = "magpie-dependency-native-inventory-v1"

PROFILE_ID = "magpie-public-prealpha-assurance-v1"
PROFILE_RELATIVE = "docs/design/public-prealpha-assurance-profile-v1.md"

RUST_TOOLCHAIN = "1.98.1"
PYTHON_VERSION = "3.12.14"
GO_VERSION = "1.27.0"
RUNNER_LABEL = "ubuntu-24.04"

RUSQLITE_VERSION = "0.37.0"
RUSQLITE_SOURCE = "registry+https://github.com/rust-lang/crates.io-index"
RUSQLITE_CHECKSUM = (
    "165ca6e57b20e1351573e3729b958bc62f0e48025386970b6e4d29e7a7e71f3f"
)
RUSQLITE_FEATURES = ("bundled",)
RUSQLITE_WORKSPACE_VERSION = "0.37"
LIBSQLITE3_SYS_VERSION = "0.35.0"
LIBSQLITE3_SYS_SOURCE = "registry+https://github.com/rust-lang/crates.io-index"
LIBSQLITE3_SYS_CHECKSUM = (
    "133c182a6a2c87864fe97778797e46c7e999672690dc9fa3ee8e241aa4a9c13f"
)

CARGO_LOCK_RELATIVE = "Cargo.lock"
WORKSPACE_MANIFEST_RELATIVE = "Cargo.toml"
CRATE_MANIFESTS = (
    "crates/magpie-claims/Cargo.toml",
    "crates/magpie-episodic/Cargo.toml",
    "crates/magpie-log/Cargo.toml",
)
RUSQLITE_CONSUMERS = (
    "crates/magpie-episodic/Cargo.toml",
    "crates/magpie-log/Cargo.toml",
)

REQUIREMENTS_RELATIVE = "tools/requirements-portable-verifier.txt"
EXPECTED_PINS = (
    "PyNaCl==1.6.2",
    "cffi==2.0.0",
    "cryptography==50.0.0",
    "pycparser==3.0",
)

# distribution -> (version, wheel filename, wheel sha256, size bytes)
EXPECTED_WHEELS: dict[str, tuple[str, str, str, int]] = {
    "PyNaCl": (
        "1.6.2",
        "pynacl-1.6.2-cp38-abi3-manylinux_2_34_x86_64.whl",
        "c8a231e36ec2cab018c4ad4358c386e36eede0319a0c41fed24f840b1dac59f6",
        1402859,
    ),
    "cffi": (
        "2.0.0",
        "cffi-2.0.0-cp312-cp312-manylinux2014_x86_64.manylinux_2_17_x86_64.whl",
        "3e17ed538242334bf70832644a32a7aae3d83b57567f9fd60a26257e992b79ba",
        219572,
    ),
    "cryptography": (
        "50.0.0",
        "cryptography-50.0.0-cp311-abi3-manylinux_2_34_x86_64.whl",
        "82148ec5bddac30b51a5b3c1945075f896fa022cb93f8e4a01e9f6ee95292c5f",
        4734462,
    ),
    "pycparser": (
        "3.0",
        "pycparser-3.0-py3-none-any.whl",
        "b727414169a36b7d524c1c3e31839a521725078d7b2ff038656844266160a992",
        48172,
    ),
}
PYNAWL_WHEEL_SHA256 = EXPECTED_WHEELS["PyNaCl"][2]

GO_MODULE = "example.com/magpie/go-verify-chain"
GO_DIRECTIVE = "1.27"
GO_MOD_RELATIVE = "tools/go-verify-chain/go.mod"
GO_SUM_RELATIVE = "tools/go-verify-chain/go.sum"
GO_VENDOR_RELATIVE = "tools/go-verify-chain/vendor/modules.txt"
EDWARDS_MODULE = "filippo.io/edwards25519"
EDWARDS_VERSION = "v1.2.0"
EDWARDS_GO_SUM_LINES = (
    "filippo.io/edwards25519 v1.2.0 h1:crnVqOiS4jqYleHd9vaKZ+HKtHfllngJIiOpNpoJsjo=",
    "filippo.io/edwards25519 v1.2.0/go.mod "
    "h1:xzAOLCNug/yB62zG1bQ8uziwrIqIuxhctzJT18Q77mc=",
)
EDWARDS_VENDOR_LINES = (
    "# filippo.io/edwards25519 v1.2.0",
    "## explicit; go 1.24.0",
    "filippo.io/edwards25519",
    "filippo.io/edwards25519/field",
)

WORKFLOW_RELATIVE = ".github/workflows/ci.yml"
EXPECTED_ACTION_PINS = (
    "actions/cache@0057852bfaa89a56745cba8c7296529d2fc39830",
    "actions/checkout@11d5960a326750d5838078e36cf38b85af677262",
    "actions/setup-go@924ae3a1cded613372ab5595356fb5720e22ba16",
    "actions/setup-python@5fda3b95a4ea91299a34e894583c3862153e4b97",
)
ACTION_PIN_RE = re.compile(r"^actions/[a-z0-9-]+@[0-9a-f]{40}$")

SQLITE_VERSION = "3.50.2"
SQLITE_NUMBER = "3050002"
SQLITE_SOURCE_ID = (
    "2025-06-28 14:00:48 "
    "2af157d77fb1304a74176eaee7fbc7c7e932d946bf25325e9c26c91db19e3079"
)

EXPECTED_NON_CLAIMS = (
    "No dependency vulnerability review and no vulnerability-freedom claim.",
    "No secure-acquisition, SLSA, or build-provenance attestation.",
    "No hermetic-build or reproducibility claim.",
    "No license determination.",
    "No C4.4 or C4.5 assurance establishment.",
    "No release-readiness or C5 selection.",
)

BOUND_FILES = (
    PROFILE_RELATIVE,
    WORKSPACE_MANIFEST_RELATIVE,
    CARGO_LOCK_RELATIVE,
    *CRATE_MANIFESTS,
    REQUIREMENTS_RELATIVE,
    GO_MOD_RELATIVE,
    GO_SUM_RELATIVE,
    GO_VENDOR_RELATIVE,
    WORKFLOW_RELATIVE,
)


class DependencyNativeCheckError(RuntimeError):
    """Raised when dependency or native qualification evidence fails."""


def _require(condition: bool, message: str) -> None:
    if not condition:
        raise DependencyNativeCheckError(message)


def _sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _strip_yaml_comment(text: str) -> str:
    in_single = False
    in_double = False
    for i, ch in enumerate(text):
        if ch == "'" and not in_double:
            in_single = not in_single
        elif ch == '"' and not in_single:
            in_double = not in_double
        elif ch == "#" and not in_single and not in_double:
            if i == 0 or text[i - 1] in " \t":
                return text[:i].rstrip()
    return text.rstrip()


def _yaml_scalar(text: str) -> str:
    text = text.strip()
    if len(text) >= 2 and text[0] == text[-1] and text[0] in ("'", '"'):
        return text[1:-1]
    return text


def _split_effective_segments(command: str) -> list[str]:
    segments: list[str] = []
    for line in command.splitlines():
        for part in re.split(r"\s*(?:&&|\|\||;)\s*", line):
            part = part.strip()
            if part:
                segments.append(part)
    return segments


def _strip_shell_noise(segment: str) -> str:
    out: list[str] = []
    in_single = False
    in_double = False
    for i, ch in enumerate(segment):
        if in_single:
            if ch == "'":
                in_single = False
            continue
        if in_double:
            if ch == '"':
                in_double = False
            continue
        if ch == "'":
            in_single = True
            continue
        if ch == '"':
            in_double = True
            continue
        if ch == "#" and (i == 0 or segment[i - 1] in " \t"):
            break
        out.append(ch)
    return "".join(out)


def _read_run_block(
    lines: list[str], i: int, n: int, style: str
) -> tuple[str, int]:
    block_lines: list[str] = []
    i += 1
    while i < n:
        bl = lines[i]
        if not bl.strip():
            block_lines.append("")
            i += 1
            continue
        if not bl.startswith("          "):
            break
        block_lines.append(bl.strip())
        i += 1
    while block_lines and not block_lines[-1]:
        block_lines.pop()
    if style.startswith(">"):
        return " ".join(l for l in block_lines if l), i
    return "\n".join(block_lines), i


def _parse_step(lines: list[str], i: int, n: int) -> tuple[dict, int]:
    step: dict = {"name": None, "uses": None, "run": None, "with_": {}}
    seen_step: set[str] = set()

    def note_key(key: str) -> None:
        _require(
            key not in seen_step,
            f"duplicate workflow step key {key!r}",
        )
        seen_step.add(key)

    rest = lines[i][8:].strip()
    if rest:
        m = re.match(r"([A-Za-z_][A-Za-z0-9_-]*):\s*(.*)$", rest)
        if m is not None:
            key = m.group(1)
            value = _strip_yaml_comment(m.group(2).strip())
            if key in ("name", "uses", "run", "with"):
                note_key(key)
            if key == "name":
                step["name"] = _yaml_scalar(value)
            elif key == "uses":
                step["uses"] = _yaml_scalar(value)
            elif key == "run":
                if value in ("|", ">") or value.startswith("|") or value.startswith(">"):
                    step["run"], i = _read_run_block(lines, i, n, value)
                    return step, i
                step["run"] = _yaml_scalar(value)
    i += 1
    while i < n:
        pl = lines[i]
        if not pl.strip():
            i += 1
            continue
        if pl.startswith("      - "):
            break
        if pl.startswith("    ") and not pl.startswith("      "):
            break
        m = re.match(r"^        (\S+):\s*(.*)$", pl)
        if not m:
            i += 1
            continue
        key = m.group(1)
        value = _strip_yaml_comment(m.group(2).strip())
        if key in ("name", "uses", "run", "with"):
            note_key(key)
        if key == "name":
            step["name"] = _yaml_scalar(value)
            i += 1
        elif key == "uses":
            step["uses"] = _yaml_scalar(value)
            i += 1
        elif key == "run":
            if value in ("|", ">") or value.startswith("|") or value.startswith(">"):
                step["run"], i = _read_run_block(lines, i, n, value)
            else:
                step["run"] = _yaml_scalar(value)
                i += 1
        elif key == "with":
            i += 1
            while i < n:
                wl = lines[i]
                if not wl.strip():
                    i += 1
                    continue
                if not wl.startswith("          "):
                    break
                wm = re.match(r"^          ([A-Za-z_][A-Za-z0-9_-]*):\s*(.*)$", wl)
                if wm:
                    _require(
                        wm.group(1) not in step["with_"],
                        f"duplicate workflow step with key {wm.group(1)!r}",
                    )
                    step["with_"][wm.group(1)] = _yaml_scalar(
                        _strip_yaml_comment(wm.group(2))
                    )
                i += 1
        else:
            i += 1
    return step, i


def _parse_workflow_verify_job(text: str) -> dict:
    lines = text.splitlines()
    n = len(lines)
    result: dict = {"runs_on": None, "env": {}, "steps": []}
    verify_jobs = [i for i in range(n) if lines[i].rstrip() == "  verify:"]
    if not verify_jobs:
        raise DependencyNativeCheckError("workflow has no 'verify' job")
    _require(
        len(verify_jobs) == 1,
        "workflow contains duplicate 'verify' job keys",
    )
    i = verify_jobs[0] + 1
    seen_job: set[str] = set()
    while i < n:
        line = lines[i]
        if not line.strip():
            i += 1
            continue
        if line.startswith("  ") and not line.startswith("    "):
            break
        if line.startswith("    runs-on:"):
            _require(
                "runs-on" not in seen_job,
                "duplicate 'runs-on' key in verify job",
            )
            seen_job.add("runs-on")
            result["runs_on"] = _yaml_scalar(_strip_yaml_comment(line[12:]))
            i += 1
            continue
        if line.startswith("    env:"):
            _require("env" not in seen_job, "duplicate 'env' key in verify job")
            seen_job.add("env")
            i += 1
            while i < n:
                el = lines[i]
                if not el.strip():
                    i += 1
                    continue
                if not el.startswith("      "):
                    break
                em = re.match(r"^      ([A-Za-z_][A-Za-z0-9_]*):\s*(.*)$", el)
                if em:
                    _require(
                        em.group(1) not in result["env"],
                        f"duplicate env key {em.group(1)!r} in verify job",
                    )
                    result["env"][em.group(1)] = _yaml_scalar(
                        _strip_yaml_comment(em.group(2))
                    )
                i += 1
            continue
        if line.startswith("    steps:"):
            _require("steps" not in seen_job, "duplicate 'steps' key in verify job")
            seen_job.add("steps")
            i += 1
            while i < n:
                sl = lines[i]
                if not sl.strip():
                    i += 1
                    continue
                if sl.startswith("    ") and not sl.startswith("      "):
                    break
                if sl.startswith("      - "):
                    step, i = _parse_step(lines, i, n)
                    result["steps"].append(step)
                    continue
                i += 1
            break
        i += 1
    return result


def _split_go_comment(line: str) -> tuple[str, str]:
    match = re.search(r"\s//", line)
    if match is None:
        return line.strip(), ""
    return line[: match.start()].strip(), line[match.start() :].strip()


def _parse_go_mod(text: str) -> dict:
    result: dict = {"module": None, "go": None, "requires": []}
    lines = text.splitlines()
    i = 0
    n = len(lines)
    while i < n:
        line, _comment = _split_go_comment(lines[i])
        if not line or line.startswith("//"):
            i += 1
            continue
        if line.startswith("module "):
            rest = line[7:].strip()
            _require(rest, "malformed go module directive")
            _require(result["module"] is None, "duplicate go module directive")
            result["module"] = rest
            i += 1
        elif line.startswith("go "):
            rest = line[3:].strip()
            _require(rest, "malformed go directive")
            _require(result["go"] is None, "duplicate go directive")
            result["go"] = rest
            i += 1
        elif line.startswith("require "):
            raw_line = lines[i]
            rest = raw_line[raw_line.index("require ") + 8 :].strip()
            if rest.startswith("("):
                i += 1
                while i < n:
                    raw = lines[i]
                    stripped, comment = _split_go_comment(raw)
                    if stripped == ")":
                        i += 1
                        break
                    if stripped and not stripped.startswith("//"):
                        parts = stripped.split()
                        if len(parts) >= 2:
                            indirect = bool(re.search(r"\bindirect\b", comment))
                            result["requires"].append((parts[0], parts[1], indirect))
                        else:
                            _require(False, "malformed go require directive")
                    i += 1
                _require(i <= n and lines[i - 1].strip() == ")", "unterminated go require block")
            else:
                parts, comment = _split_go_comment(rest)
                parts = parts.split()
                if len(parts) >= 2:
                    indirect = bool(re.search(r"\bindirect\b", comment))
                    result["requires"].append((parts[0], parts[1], indirect))
                else:
                    _require(False, "malformed go require directive")
            i += 1
        else:
            tokens = line.split()
            if tokens and tokens[0] in ("replace", "exclude", "retract"):
                _require(False, f"go {tokens[0]} directive is not permitted")
            i += 1
    return result


def _cargo_lock_packages(lock_text: str) -> dict[str, list[dict]]:
    lock = tomllib.loads(lock_text)
    packages = lock.get("package", [])
    by_name: dict[str, list[dict]] = {}
    for pkg in packages:
        name = pkg.get("name")
        if isinstance(name, str):
            by_name.setdefault(name, []).append(pkg)
    return by_name


def _reject_duplicate_keys(pairs: list[tuple[str, object]]) -> dict[str, object]:
    document: dict[str, object] = {}
    for key, value in pairs:
        _require(
            key not in document,
            f"inventory JSON contains duplicate key {key!r}",
        )
        document[key] = value
    return document


def _load_inventory(root: Path) -> dict[str, object]:
    path = root / INVENTORY_RELATIVE
    _require(path.is_file(), f"inventory is missing: {INVENTORY_RELATIVE}")
    try:
        document = json.loads(path.read_bytes(), object_pairs_hook=_reject_duplicate_keys)
    except ValueError as exc:
        raise DependencyNativeCheckError(f"inventory is not valid JSON: {exc}") from exc
    _require(isinstance(document, dict), "inventory root must be a JSON object")
    return document


def verify_inventory_contract(document: dict[str, object]) -> None:
    expected_top = {
        "schema",
        "profile",
        "toolchains",
        "rust",
        "python",
        "go",
        "github_actions",
        "native_states",
        "non_claims",
    }
    keys = set(document)
    missing = expected_top - keys
    _require(not missing, f"inventory is missing top-level keys: {sorted(missing)}")
    extra = keys - expected_top
    _require(not extra, f"inventory has unexpected top-level keys: {sorted(extra)}")
    _require(
        document.get("schema") == INVENTORY_SCHEMA,
        f"inventory schema changed: expected {INVENTORY_SCHEMA!r}, "
        f"got {document.get('schema')!r}",
    )


def verify_file_hashes(root: Path, document: dict[str, object]) -> None:
    references: list[tuple[object, object]] = []

    def collect(node: object) -> None:
        if isinstance(node, dict):
            if "path" in node:
                references.append((node.get("path"), node.get("sha256")))
            for value in node.values():
                collect(value)
        elif isinstance(node, list):
            for value in node:
                collect(value)

    collect(document)
    seen: set[str] = set()
    normalized: list[tuple[str, str]] = []
    for relative, expected in references:
        if not isinstance(relative, str):
            raise DependencyNativeCheckError("inventory file reference path must be a string")
        _require(
            relative not in seen,
            f"inventory contains duplicate file reference: {relative}",
        )
        seen.add(relative)
        _require(
            isinstance(expected, str),
            f"inventory file reference has no sha256: {relative}",
        )
        normalized.append((relative, expected))
    missing = set(BOUND_FILES) - seen
    _require(
        not missing,
        f"inventory is missing bound file references: {sorted(missing)}",
    )
    unexpected = seen - set(BOUND_FILES)
    _require(
        not unexpected,
        f"inventory references unexpected files: {sorted(unexpected)}",
    )
    for relative, expected in normalized:
        path = root / relative
        _require(path.is_file(), f"bound file is missing: {relative}")
        observed = _sha256(path)
        _require(
            observed == expected,
            f"bound file digest drifted: {relative}: "
            f"inventory {expected}, repo {observed}",
        )


def verify_profile(document: dict[str, object]) -> None:
    profile = document.get("profile")
    _require(isinstance(profile, dict), "inventory 'profile' must be an object")
    _require(
        profile.get("id") == PROFILE_ID,
        f"profile id changed: expected {PROFILE_ID!r}, got {profile.get('id')!r}",
    )
    doc = profile.get("document")
    _require(isinstance(doc, dict), "inventory 'profile.document' must be an object")
    _require(
        doc.get("path") == PROFILE_RELATIVE,
        f"profile document path changed: expected {PROFILE_RELATIVE!r}",
    )


def verify_toolchains(root: Path, document: dict[str, object]) -> None:
    toolchains = document.get("toolchains")
    _require(isinstance(toolchains, dict), "inventory 'toolchains' must be an object")
    _require(
        toolchains
        == {
            "go": GO_VERSION,
            "python": PYTHON_VERSION,
            "rust": RUST_TOOLCHAIN,
            "runner": RUNNER_LABEL,
        },
        f"toolchain lane changed: {toolchains!r}",
    )

    workflow = (root / WORKFLOW_RELATIVE).read_text(encoding="utf-8")
    job = _parse_workflow_verify_job(workflow)
    _require(
        job["runs_on"] == RUNNER_LABEL,
        f"workflow runner is not {RUNNER_LABEL}",
    )
    _require(
        job["env"].get("MAGPIE_RUNNER_LABEL") == RUNNER_LABEL,
        "workflow runner label environment is missing or changed",
    )

    python_steps = [
        step
        for step in job["steps"]
        if step.get("uses") == f"actions/setup-python@5fda3b95a4ea91299a34e894583c3862153e4b97"
    ]
    _require(len(python_steps) == 1, "workflow has no effective setup-python step")
    _require(
        python_steps[0].get("with_", {}).get("python-version") == PYTHON_VERSION,
        f"workflow Python is not {PYTHON_VERSION}",
    )

    go_steps = [
        step
        for step in job["steps"]
        if step.get("uses") == f"actions/setup-go@924ae3a1cded613372ab5595356fb5720e22ba16"
    ]
    _require(len(go_steps) == 1, "workflow has no effective setup-go step")
    _require(
        go_steps[0].get("with_", {}).get("go-version") == GO_VERSION,
        f"workflow Go is not {GO_VERSION}",
    )

    rust_segments: list[str] = []
    for step in job["steps"]:
        run = step.get("run")
        if isinstance(run, str):
            rust_segments.extend(_split_effective_segments(run))
    install_prefix = f"rustup toolchain install {RUST_TOOLCHAIN}"
    default_target = f"rustup default {RUST_TOOLCHAIN}"

    def _install_targets(segment: str) -> bool:
        return segment == install_prefix or segment.startswith(install_prefix + " ")

    rustup_segments = [s for s in rust_segments if s.startswith("rustup ")]
    _require(
        any(_install_targets(s) for s in rustup_segments),
        f"workflow Rust toolchain is not {RUST_TOOLCHAIN}",
    )
    defaults = [s for s in rustup_segments if s.startswith("rustup default ")]
    _require(
        bool(defaults)
        and all(s == default_target for s in defaults),
        f"workflow default Rust toolchain is not {RUST_TOOLCHAIN}: {defaults!r}",
    )
    unapproved = [
        s
        for s in rustup_segments
        if not _install_targets(s) and s != default_target
    ]
    _require(
        not unapproved,
        f"workflow contains unapproved rustup command: {unapproved!r}",
    )


def verify_cargo_identity(root: Path, document: dict[str, object]) -> None:
    rust = document.get("rust")
    _require(isinstance(rust, dict), "inventory 'rust' must be an object")

    workspace = tomllib.loads((root / WORKSPACE_MANIFEST_RELATIVE).read_text(encoding="utf-8"))
    spec = workspace.get("workspace", {}).get("dependencies", {}).get("rusqlite")
    _require(
        spec == {"version": RUSQLITE_WORKSPACE_VERSION, "features": list(RUSQLITE_FEATURES)},
        f"workspace rusqlite specification changed: {spec!r}",
    )

    lock_text = (root / CARGO_LOCK_RELATIVE).read_text(encoding="utf-8")
    try:
        packages = _cargo_lock_packages(lock_text)
    except tomllib.TOMLDecodeError as exc:
        raise DependencyNativeCheckError(f"Cargo.lock is not valid TOML: {exc}") from exc

    rusqlite_packages = packages.get("rusqlite", [])
    _require(
        len(rusqlite_packages) == 1,
        f"expected exactly one rusqlite package in Cargo.lock, "
        f"found {len(rusqlite_packages)}",
    )
    rusqlite_package = rusqlite_packages[0]
    _require(
        rusqlite_package.get("version") == RUSQLITE_VERSION,
        f"locked rusqlite version changed: expected {RUSQLITE_VERSION}",
    )
    _require(
        rusqlite_package.get("source") == RUSQLITE_SOURCE,
        "locked rusqlite source changed",
    )
    _require(
        rusqlite_package.get("checksum") == RUSQLITE_CHECKSUM,
        "locked rusqlite checksum changed",
    )
    rusqlite_dependencies = rusqlite_package.get("dependencies", [])
    _require(
        isinstance(rusqlite_dependencies, list)
        and "libsqlite3-sys" in rusqlite_dependencies,
        "locked rusqlite no longer depends on libsqlite3-sys",
    )

    sqlite_packages = packages.get("libsqlite3-sys", [])
    _require(
        len(sqlite_packages) == 1,
        f"expected exactly one libsqlite3-sys package in Cargo.lock, "
        f"found {len(sqlite_packages)}",
    )
    sqlite_package = sqlite_packages[0]
    _require(
        sqlite_package.get("version") == LIBSQLITE3_SYS_VERSION,
        f"locked libsqlite3-sys version changed: expected {LIBSQLITE3_SYS_VERSION}",
    )
    _require(
        sqlite_package.get("source") == LIBSQLITE3_SYS_SOURCE,
        "locked libsqlite3-sys source changed",
    )
    _require(
        sqlite_package.get("checksum") == LIBSQLITE3_SYS_CHECKSUM,
        "locked libsqlite3-sys checksum changed",
    )

    for relative in RUSQLITE_CONSUMERS:
        manifest = (root / relative).read_text(encoding="utf-8")
        _require(
            "rusqlite.workspace = true" in manifest,
            f"{relative} no longer consumes the workspace rusqlite dependency",
        )

    inv_rusqlite = rust.get("rusqlite")
    _require(isinstance(inv_rusqlite, dict), "inventory 'rust.rusqlite' must be an object")
    _require(
        inv_rusqlite.get("version") == RUSQLITE_VERSION
        and inv_rusqlite.get("source") == RUSQLITE_SOURCE
        and inv_rusqlite.get("checksum") == RUSQLITE_CHECKSUM
        and tuple(inv_rusqlite.get("features", ())) == RUSQLITE_FEATURES
        and inv_rusqlite.get("state") == "BOUND",
        f"inventory rusqlite binding drifted: {inv_rusqlite!r}",
    )
    inv_sqlite = rust.get("libsqlite3_sys")
    _require(
        isinstance(inv_sqlite, dict)
        and inv_sqlite.get("version") == LIBSQLITE3_SYS_VERSION
        and inv_sqlite.get("source") == LIBSQLITE3_SYS_SOURCE
        and inv_sqlite.get("checksum") == LIBSQLITE3_SYS_CHECKSUM
        and inv_sqlite.get("state") == "BOUND",
        f"inventory libsqlite3-sys binding drifted: {inv_sqlite!r}",
    )


def verify_python_governance(root: Path, document: dict[str, object]) -> None:
    python = document.get("python")
    _require(isinstance(python, dict), "inventory 'python' must be an object")

    requirements = (root / REQUIREMENTS_RELATIVE).read_text(encoding="utf-8")
    pins = [
        line.strip()
        for line in requirements.splitlines()
        if line.strip() and not line.strip().startswith("#")
    ]
    _require(
        sorted(pins) == sorted(EXPECTED_PINS),
        f"requirements pins drifted: {pins!r}",
    )

    workflow = (root / WORKFLOW_RELATIVE).read_text(encoding="utf-8")
    job = _parse_workflow_verify_job(workflow)
    pip_segments: list[str] = []
    for step in job["steps"]:
        run = step.get("run")
        if not isinstance(run, str):
            continue
        for segment in _split_effective_segments(run):
            effective = _strip_shell_noise(segment).strip()
            if "pip install" in effective:
                pip_segments.append(effective)
    _require(
        len(pip_segments) == 1,
        f"expected exactly one pip install command in the workflow, "
        f"found {len(pip_segments)}",
    )
    _require(
        pip_segments[0]
        == "python -m pip install -r tools/requirements-portable-verifier.txt",
        f"workflow pip install must install only from the requirements file: "
        f"{pip_segments[0]!r}",
    )

    lane = python.get("lane")
    _require(
        isinstance(lane, dict)
        and lane.get("python") == PYTHON_VERSION
        and lane.get("implementation") == "CPython"
        and lane.get("os") == "linux"
        and lane.get("arch") == "x86_64"
        and lane.get("selected_in") == WORKFLOW_RELATIVE,
        f"Python lane changed: {lane!r}",
    )

    artifacts = python.get("artifacts")
    _require(isinstance(artifacts, list), "inventory 'python.artifacts' must be a list")
    by_distribution: dict[str, dict[str, object]] = {}
    for artifact in artifacts:
        _require(isinstance(artifact, dict), "artifact entries must be objects")
        distribution = artifact.get("distribution")
        _require(isinstance(distribution, str), "artifact distribution must be a string")
        _require(
            distribution not in by_distribution,
            f"inventory lists duplicate artifact for {distribution}",
        )
        by_distribution[distribution] = artifact

    _require(
        set(by_distribution) == set(EXPECTED_WHEELS),
        f"artifact set drifted: {sorted(by_distribution)}",
    )
    for distribution, (version, filename, sha256, size) in EXPECTED_WHEELS.items():
        artifact = by_distribution[distribution]
        wheel = artifact.get("wheel")
        _require(isinstance(wheel, dict), f"{distribution} wheel entry must be an object")
        _require(
            artifact.get("version") == version,
            f"{distribution} version drifted: expected {version}, "
            f"got {artifact.get('version')!r}",
        )
        _require(
            wheel.get("filename") == filename
            and wheel.get("sha256") == sha256
            and wheel.get("size_bytes") == size,
            f"{distribution} wheel binding drifted: {wheel!r}",
        )


def verify_go_identity(root: Path, document: dict[str, object]) -> None:
    go = document.get("go")
    _require(isinstance(go, dict), "inventory 'go' must be an object")

    go_mod = (root / GO_MOD_RELATIVE).read_text(encoding="utf-8")
    parsed_go_mod = _parse_go_mod(go_mod)
    _require(
        parsed_go_mod["module"] == GO_MODULE,
        "go module path changed",
    )
    _require(
        parsed_go_mod["go"] == GO_DIRECTIVE,
        "go directive changed",
    )
    edwards_requires = [
        (version, indirect)
        for module, version, indirect in parsed_go_mod["requires"]
        if module == EDWARDS_MODULE
    ]
    _require(
        len(edwards_requires) == 1,
        f"expected exactly one go require for {EDWARDS_MODULE}, "
        f"found {len(edwards_requires)}",
    )
    _require(
        not edwards_requires[0][1],
        f"{EDWARDS_MODULE} must remain a direct go require",
    )
    _require(
        edwards_requires[0][0] == EDWARDS_VERSION,
        f"go module requires wrong {EDWARDS_MODULE} version: {edwards_requires[0][0]!r}",
    )

    go_sum_lines = (root / GO_SUM_RELATIVE).read_text(encoding="utf-8").splitlines()
    _require(
        go_sum_lines == list(EDWARDS_GO_SUM_LINES),
        f"go.sum contents drifted: {go_sum_lines!r}",
    )

    vendor_lines = (root / GO_VENDOR_RELATIVE).read_text(encoding="utf-8").splitlines()
    _require(
        vendor_lines == list(EDWARDS_VENDOR_LINES),
        f"vendor/modules.txt contents drifted: {vendor_lines!r}",
    )

    _require(go.get("module") == GO_MODULE, "inventory go module changed")
    _require(go.get("go_directive") == GO_DIRECTIVE, "inventory go directive changed")
    dependencies = go.get("dependencies")
    _require(
        isinstance(dependencies, list) and len(dependencies) == 1,
        "inventory go dependencies must list exactly one module",
    )
    dependency = dependencies[0]
    _require(
        isinstance(dependency, dict)
        and dependency.get("module") == EDWARDS_MODULE
        and dependency.get("version") == EDWARDS_VERSION
        and dependency.get("go_sum_hash")
        == EDWARDS_GO_SUM_LINES[0].split(" ", 2)[2]
        and dependency.get("go_sum_mod_hash")
        == EDWARDS_GO_SUM_LINES[1].split(" ", 2)[2]
        and dependency.get("state") == "BOUND",
        f"inventory go dependency binding drifted: {dependency!r}",
    )


def verify_github_pins(root: Path, document: dict[str, object]) -> None:
    actions = document.get("github_actions")
    _require(
        isinstance(actions, dict), "inventory 'github_actions' must be an object"
    )

    workflow = (root / WORKFLOW_RELATIVE).read_text(encoding="utf-8")
    job = _parse_workflow_verify_job(workflow)
    observed = [step.get("uses") for step in job["steps"] if step.get("uses")]
    for pin in observed:
        _require(
            ACTION_PIN_RE.match(pin) is not None,
            f"workflow action reference is not a full-SHA pin: {pin!r}",
        )
    _require(
        sorted(observed) == sorted(EXPECTED_ACTION_PINS),
        f"workflow action pins drifted: {sorted(observed)}",
    )
    _require(
        sorted(actions.get("pins", ())) == sorted(EXPECTED_ACTION_PINS),
        f"inventory action pins drifted: {actions.get('pins')!r}",
    )
    _require(
        actions.get("state") == "BOUND",
        "inventory github_actions state must be BOUND",
    )


def verify_native_states(document: dict[str, object]) -> None:
    native = document.get("native_states")
    _require(
        isinstance(native, dict)
        and set(native) == {"bundled_sqlite", "libsodium"},
        "inventory 'native_states' must contain exactly bundled_sqlite and libsodium",
    )

    sqlite = native.get("bundled_sqlite")
    _require(isinstance(sqlite, dict), "inventory bundled_sqlite must be an object")
    _require(
        sqlite.get("state") == "OBSERVED"
        and sqlite.get("version") == SQLITE_VERSION
        and sqlite.get("sql_number") == SQLITE_NUMBER
        and sqlite.get("source_id") == SQLITE_SOURCE_ID,
        f"bundled SQLite observation drifted: {sqlite!r}",
    )
    evidence = sqlite.get("evidence")
    _require(isinstance(evidence, dict), "bundled_sqlite evidence must be an object")
    crate = evidence.get("crate")
    _require(
        isinstance(crate, dict)
        and crate.get("sha256") == LIBSQLITE3_SYS_CHECKSUM,
        "bundled SQLite evidence crate must match the locked libsqlite3-sys checksum",
    )

    sodium = native.get("libsodium")
    _require(isinstance(sodium, dict), "inventory libsodium must be an object")
    _require(
        sodium.get("state") == "UNAVAILABLE" and sodium.get("version") is None,
        "libsodium must remain UNAVAILABLE with no version inference",
    )
    sodium_evidence = sodium.get("evidence")
    _require(
        isinstance(sodium_evidence, dict)
        and isinstance(sodium_evidence.get("wheel"), dict)
        and sodium_evidence["wheel"].get("sha256") == PYNAWL_WHEEL_SHA256,
        "libsodium evidence wheel must match the lane PyNaCl wheel digest",
    )


def verify_non_claims(document: dict[str, object]) -> None:
    non_claims = document.get("non_claims")
    _require(
        isinstance(non_claims, list) and set(non_claims) == set(EXPECTED_NON_CLAIMS),
        f"inventory non_claims drifted: {non_claims!r}",
    )


def run_check(root: Path | None = None) -> None:
    root = Path(root) if root is not None else ROOT
    document = _load_inventory(root)
    verify_inventory_contract(document)
    verify_file_hashes(root, document)
    verify_profile(document)
    verify_toolchains(root, document)
    verify_cargo_identity(root, document)
    verify_python_governance(root, document)
    verify_go_identity(root, document)
    verify_github_pins(root, document)
    verify_native_states(document)
    verify_non_claims(document)


def main() -> int:
    try:
        run_check()
    except DependencyNativeCheckError as exc:
        print(f"dependency-native qualification check FAILED: {exc}", file=sys.stderr)
        return 1
    print("dependency-native qualification check: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
