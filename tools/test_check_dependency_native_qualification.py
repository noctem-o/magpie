"""Hostile self-tests for the C4.3 dependency/native qualification checker.

Each test mutates exactly one aspect of an isolated copy of the repository
and requires the checker to fail closed with
``DependencyNativeCheckError``. The unmutated copy and the real repository
must both pass.
"""

import json
import shutil
import tempfile
import unittest
from pathlib import Path

from tools import check_dependency_native_qualification as checker

BOUND_FILES = (
    checker.PROFILE_RELATIVE,
    checker.WORKSPACE_MANIFEST_RELATIVE,
    checker.CARGO_LOCK_RELATIVE,
    "crates/magpie-claims/Cargo.toml",
    "crates/magpie-episodic/Cargo.toml",
    "crates/magpie-log/Cargo.toml",
    checker.REQUIREMENTS_RELATIVE,
    checker.GO_MOD_RELATIVE,
    checker.GO_SUM_RELATIVE,
    checker.GO_VENDOR_RELATIVE,
    checker.WORKFLOW_RELATIVE,
    "tools/dependency_native_inventory.json",
)


class CheckerTestCase(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory(prefix="dep-native-check-")
        self.addCleanup(self._tmp.cleanup)
        self.root = Path(self._tmp.name)
        for relative in BOUND_FILES:
            source = checker.ROOT / relative
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, destination)

    def inventory(self) -> dict:
        path = self.root / "tools/dependency_native_inventory.json"
        return json.loads(path.read_text(encoding="utf-8"))

    def write_inventory(self, document) -> None:
        path = self.root / "tools/dependency_native_inventory.json"
        path.write_text(json.dumps(document, indent=2) + "\n", encoding="utf-8")

    def artifact(self, distribution: str) -> dict:
        for candidate in self.inventory()["python"]["artifacts"]:
            if candidate["distribution"] == distribution:
                return candidate
        raise AssertionError(f"artifact {distribution} not found")

    def mutate_text(self, relative: str, transform) -> None:
        path = self.root / relative
        path.write_text(transform(path.read_text(encoding="utf-8")), encoding="utf-8")

    def assert_fails(self) -> None:
        with self.assertRaises(checker.DependencyNativeCheckError):
            checker.run_check(self.root)

    def test_unmutated_tree_passes(self) -> None:
        checker.run_check(self.root)

    def test_real_repository_passes(self) -> None:
        checker.run_check()

    def test_repo_file_drifts_from_inventory_hash(self) -> None:
        path = self.root / checker.CARGO_LOCK_RELATIVE
        path.write_bytes(path.read_bytes() + b" ")
        self.assert_fails()

    def test_inventory_cargo_lock_sha_changed(self) -> None:
        document = self.inventory()
        document["rust"]["cargo_lock"]["sha256"] = "0" * 64
        self.write_inventory(document)
        self.assert_fails()

    def test_inventory_pynacl_wheel_sha_changed(self) -> None:
        document = self.inventory()
        for artifact in document["python"]["artifacts"]:
            if artifact["distribution"] == "PyNaCl":
                artifact["wheel"]["sha256"] = "f" * 64
        self.write_inventory(document)
        self.assert_fails()

    def test_inventory_pynacl_version_mismatch(self) -> None:
        document = self.inventory()
        for artifact in document["python"]["artifacts"]:
            if artifact["distribution"] == "PyNaCl":
                artifact["version"] = "1.6.3"
        self.write_inventory(document)
        self.assert_fails()

    def test_extra_artifact_added(self) -> None:
        document = self.inventory()
        document["python"]["artifacts"].append(
            {
                "distribution": "six",
                "version": "1.17.0",
                "wheel": {
                    "filename": "six-1.17.0-py2.py3-none-any.whl",
                    "sha256": "0" * 64,
                    "size_bytes": 11000,
                    "platform_tag": "any",
                },
                "acquisition": "OBSERVED",
                "url": "https://files.pythonhosted.org/packages/example/six-1.17.0.whl",
                "state": "SELECTED",
            }
        )
        self.write_inventory(document)
        self.assert_fails()

    def test_artifact_removed(self) -> None:
        document = self.inventory()
        document["python"]["artifacts"] = [
            artifact
            for artifact in document["python"]["artifacts"]
            if artifact["distribution"] != "cryptography"
        ]
        self.write_inventory(document)
        self.assert_fails()

    def test_sqlite_version_drift(self) -> None:
        document = self.inventory()
        document["native_states"]["bundled_sqlite"]["version"] = "3.50.1"
        self.write_inventory(document)
        self.assert_fails()

    def test_libsodium_fabricated_observed(self) -> None:
        document = self.inventory()
        document["native_states"]["libsodium"]["state"] = "OBSERVED"
        document["native_states"]["libsodium"]["version"] = "1.0.20"
        self.write_inventory(document)
        self.assert_fails()

    def test_action_pin_shortened(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "actions/checkout@11d5960a326750d5838078e36cf38b85af677262",
                "actions/checkout@11d5960",
            ),
        )
        self.assert_fails()

    def test_ci_python_version_changed(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "python-version: '3.12.14'", "python-version: '3.12.13'"
            ),
        )
        self.assert_fails()

    def test_go_sum_hash_line_changed(self) -> None:
        self.mutate_text(
            checker.GO_SUM_RELATIVE,
            lambda text: text.replace(
                "h1:crnVqOiS4jqYleHd9vaKZ+HKtHfllngJIiOpNpoJsjo=",
                "h1:crnVqOiS4jqYleHd9vaKZ+HKtHfllngJIiOpNpoJsjoX",
            ),
        )
        self.assert_fails()

    def test_inline_pip_package_in_ci(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: python -m pip install "
                "-r tools/requirements-portable-verifier.txt requests",
            ),
        )
        self.assert_fails()

    def test_inventory_schema_changed(self) -> None:
        document = self.inventory()
        document["schema"] = "magpie-dependency-native-inventory-v2"
        self.write_inventory(document)
        self.assert_fails()

    def test_missing_top_level_key(self) -> None:
        document = self.inventory()
        del document["non_claims"]
        self.write_inventory(document)
        self.assert_fails()

    def test_duplicate_key_json_rejected(self) -> None:
        path = self.root / "tools/dependency_native_inventory.json"
        text = path.read_text(encoding="utf-8")
        duplicated = text.replace(
            '"schema": "magpie-dependency-native-inventory-v1"',
            '"schema": "magpie-dependency-native-inventory-v1", '
            '"schema": "magpie-dependency-native-inventory-v1"',
            1,
        )
        self.assertNotEqual(text, duplicated)
        path.write_text(duplicated, encoding="utf-8")
        self.assert_fails()

    def test_missing_inventory_fails_closed(self) -> None:
        (self.root / "tools/dependency_native_inventory.json").unlink()
        self.assert_fails()


if __name__ == "__main__":
    unittest.main()
