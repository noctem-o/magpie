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
PIP_INSTALL_RE = re.compile(r"^\s*run:.*pip install", re.MULTILINE)

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
    references: list[tuple[str, object]] = []

    def collect(node: object) -> None:
        if isinstance(node, dict):
            if "path" in node and "sha256" in node:
                references.append((node["path"], node["sha256"]))
            for value in node.values():
                collect(value)
        elif isinstance(node, list):
            for value in node:
                collect(value)

    collect(document)
    for relative, expected in references:
        if not isinstance(relative, str) or not isinstance(expected, str):
            raise DependencyNativeCheckError(
                "inventory file reference must carry string path and sha256"
            )
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
    _require(
        f"runs-on: {RUNNER_LABEL}" in workflow,
        f"workflow runner is not {RUNNER_LABEL}",
    )
    _require(
        f"MAGPIE_RUNNER_LABEL: {RUNNER_LABEL}" in workflow,
        "workflow runner label environment is missing or changed",
    )
    _require(
        f"python-version: '{PYTHON_VERSION}'" in workflow,
        f"workflow Python is not {PYTHON_VERSION}",
    )
    _require(
        f"go-version: '{GO_VERSION}'" in workflow,
        f"workflow Go is not {GO_VERSION}",
    )
    _require(
        f"rustup toolchain install {RUST_TOOLCHAIN}" in workflow,
        f"workflow Rust toolchain is not {RUST_TOOLCHAIN}",
    )
    _require(
        f"rustup default {RUST_TOOLCHAIN}" in workflow,
        f"workflow default Rust toolchain is not {RUST_TOOLCHAIN}",
    )


def _lock_package_block(lock_text: str, name: str) -> str | None:
    for block in re.split(r"^\[\[package\]\]\s*$", lock_text, flags=re.MULTILINE):
        if re.search(rf'^name = "{re.escape(name)}"$', block, flags=re.MULTILINE):
            return block
    return None


def _block_field(block: str, field: str) -> str | None:
    match = re.search(rf'^{field} = "([^"]+)"$', block, flags=re.MULTILINE)
    return None if match is None else match.group(1)


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
    rusqlite_block = _lock_package_block(lock_text, "rusqlite")
    _require(rusqlite_block is not None, "Cargo.lock no longer pins a rusqlite package")
    _require(
        _block_field(rusqlite_block, "version") == RUSQLITE_VERSION,
        f"locked rusqlite version changed: expected {RUSQLITE_VERSION}",
    )
    _require(
        _block_field(rusqlite_block, "source") == RUSQLITE_SOURCE,
        "locked rusqlite source changed",
    )
    _require(
        _block_field(rusqlite_block, "checksum") == RUSQLITE_CHECKSUM,
        "locked rusqlite checksum changed",
    )
    _require(
        '"libsqlite3-sys"' in rusqlite_block,
        "locked rusqlite no longer depends on libsqlite3-sys",
    )

    sqlite_block = _lock_package_block(lock_text, "libsqlite3-sys")
    _require(
        sqlite_block is not None,
        "Cargo.lock no longer pins a libsqlite3-sys package",
    )
    _require(
        _block_field(sqlite_block, "version") == LIBSQLITE3_SYS_VERSION,
        f"locked libsqlite3-sys version changed: expected {LIBSQLITE3_SYS_VERSION}",
    )
    _require(
        _block_field(sqlite_block, "source") == LIBSQLITE3_SYS_SOURCE,
        "locked libsqlite3-sys source changed",
    )
    _require(
        _block_field(sqlite_block, "checksum") == LIBSQLITE3_SYS_CHECKSUM,
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
    pip_lines = [
        line.strip()
        for line in workflow.splitlines()
        if line.strip().startswith("run:") and "pip install" in line
    ]
    _require(
        len(pip_lines) == 1,
        f"expected exactly one pip install step in the workflow, found {len(pip_lines)}",
    )
    _require(
        pip_lines[0] == "run: python -m pip install -r tools/requirements-portable-verifier.txt",
        f"workflow pip install must install only from the requirements file: {pip_lines[0]!r}",
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
    _require(f"module {GO_MODULE}" in go_mod, "go module path changed")
    _require(f"go {GO_DIRECTIVE}" in go_mod, "go directive changed")
    _require(
        f"require {EDWARDS_MODULE} {EDWARDS_VERSION}" in go_mod,
        "go module no longer requires the owner-approved dependency",
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
    observed = re.findall(r"^\s*uses:\s*(\S+)", workflow, flags=re.MULTILINE)
    for pin in observed:
        _require(
            ACTION_PIN_RE.match(pin) is not None,
            f"workflow action reference is not a full-SHA pin: {pin!r}",
        )
    _require(
        set(observed) == set(EXPECTED_ACTION_PINS),
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
