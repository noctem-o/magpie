"""Hostile self-tests for the C4.3 dependency/native qualification checker.

Each test mutates exactly one aspect of an isolated copy of the repository
and requires the checker to fail closed with
``DependencyNativeCheckError``; a few positive-control tests rewrite
supported alternative syntaxes (block and folded run forms, comment
lines, parenthesized require blocks) and require the checker to keep
passing, so the new parsers are exercised in both directions. The
unmutated copy and the real repository must both pass.

Tests that mutate a repository file rebind the inventory digests first so
the failure reaches the intended semantic check rather than the digest
stage; the digest stage has its own dedicated witness.
"""

import hashlib
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

    def rebind(self) -> None:
        document = self.inventory()

        def rebind_node(node) -> None:
            if isinstance(node, dict):
                path = node.get("path")
                if isinstance(path, str):
                    node["sha256"] = hashlib.sha256(
                        (self.root / path).read_bytes()
                    ).hexdigest()
                for value in node.values():
                    rebind_node(value)
            elif isinstance(node, list):
                for value in node:
                    rebind_node(value)

        rebind_node(document)
        self.write_inventory(document)

    def assert_fails(self, message: str | None = None) -> None:
        with self.assertRaises(checker.DependencyNativeCheckError) as ctx:
            checker.run_check(self.root)
        if message is not None:
            self.assertIn(message, str(ctx.exception))

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
        self.rebind()
        self.assert_fails()

    def test_ci_python_version_changed(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "python-version: '3.12.14'", "python-version: '3.12.13'"
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_go_sum_hash_line_changed(self) -> None:
        self.mutate_text(
            checker.GO_SUM_RELATIVE,
            lambda text: text.replace(
                "h1:crnVqOiS4jqYleHd9vaKZ+HKtHfllngJIiOpNpoJsjo=",
                "h1:" + "A" * 44 + "=",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_inline_pip_package_in_ci(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: python -m pip install -r tools/requirements-portable-verifier.txt requests",
            ),
        )
        self.rebind()
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

    # --- P1 A: digest-reference closure --------------------------------

    def test_inventory_missing_bound_reference(self) -> None:
        document = self.inventory()
        del document["rust"]["cargo_lock"]
        self.write_inventory(document)
        self.assert_fails()

    def test_inventory_missing_sha256(self) -> None:
        document = self.inventory()
        del document["rust"]["cargo_lock"]["sha256"]
        self.write_inventory(document)
        self.assert_fails()

    def test_inventory_unexpected_reference(self) -> None:
        document = self.inventory()
        document["rust"]["rogue_manifest"] = {
            "path": "README.md",
            "sha256": "0" * 64,
        }
        self.write_inventory(document)
        self.assert_fails()

    def test_inventory_duplicate_reference(self) -> None:
        document = self.inventory()
        document["rust"]["rogue_duplicate"] = {
            "path": "Cargo.lock",
            "sha256": document["rust"]["cargo_lock"]["sha256"],
        }
        self.write_inventory(document)
        self.assert_fails()

    # --- P1 B: pip install escapes -------------------------------------

    def test_block_form_pip_escape(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          python -m pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_chained_pip_install_escape(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: python -m pip install -r tools/requirements-portable-verifier.txt "
                "&& python -m pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_folded_form_pip_escape(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: >\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          python -m pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_block_form_pip_comment_is_not_execution(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          # python -m pip install requests\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt",
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    # --- P1 C: effective workflow values --------------------------------

    def test_workflow_runs_on_changed(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "    runs-on: ubuntu-24.04",
                "    runs-on: ubuntu-22.04  # old: runs-on: ubuntu-24.04",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_unrelated_job_does_not_satisfy_governed_values(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "    runs-on: ubuntu-24.04\n",
                "    runs-on: ubuntu-22.04\n",
            )
            + "\n  decoy:\n"
            "    runs-on: ubuntu-24.04\n"
            "    steps:\n"
            "      - name: noop\n"
            "        run: echo ok\n",
        )
        self.rebind()
        self.assert_fails()

    def test_workflow_runner_label_env_changed(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "MAGPIE_RUNNER_LABEL: ubuntu-24.04",
                "MAGPIE_RUNNER_LABEL: ubuntu-22.04  # old: MAGPIE_RUNNER_LABEL: ubuntu-24.04",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_workflow_go_version_changed(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "go-version: '1.27.0'",
                "go-version: '1.26.0'  # old: go-version: '1.27.0'",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_workflow_rustup_default_changed(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "rustup default 1.98.1",
                "rustup default 1.97.0  # rustup default 1.98.1",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_workflow_unapproved_rustup_command(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "rustup default 1.98.1",
                "rustup default 1.98.1 && rustup update stable",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_workflow_duplicate_runs_on_key(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "    runs-on: ubuntu-24.04\n",
                "    runs-on: ubuntu-24.04\n    runs-on: ubuntu-22.04\n",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_workflow_duplicate_step_run_key(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "        run: python -m pip install -r tools/requirements-portable-verifier.txt\n",
                "        run: python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "        run: python -m pip install requests\n",
            ),
        )
        self.rebind()
        self.assert_fails()

    # --- P1 D: effective go.mod declaration -----------------------------

    def test_go_module_path_changed(self) -> None:
        self.mutate_text(
            checker.GO_MOD_RELATIVE,
            lambda text: text.replace(
                "module example.com/magpie/go-verify-chain",
                "module example.com/magpie/go-verify-chain-evil",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_go_module_comment_shadow(self) -> None:
        self.mutate_text(
            checker.GO_MOD_RELATIVE,
            lambda text: text.replace(
                "module example.com/magpie/go-verify-chain",
                "module example.com/evil // module example.com/magpie/go-verify-chain",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_go_directive_changed(self) -> None:
        self.mutate_text(
            checker.GO_MOD_RELATIVE,
            lambda text: text.replace(
                "go 1.27\n",
                "go 1.26 // go 1.27\n",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_go_edwards_version_changed(self) -> None:
        self.mutate_text(
            checker.GO_MOD_RELATIVE,
            lambda text: text.replace(
                "require filippo.io/edwards25519 v1.2.0",
                "require filippo.io/edwards25519 v1.3.0 // require filippo.io/edwards25519 v1.2.0",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_go_require_block_form_is_accepted(self) -> None:
        self.mutate_text(
            checker.GO_MOD_RELATIVE,
            lambda text: text.replace(
                "require filippo.io/edwards25519 v1.2.0",
                "require (\n    filippo.io/edwards25519 v1.2.0\n)",
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    def test_go_require_marked_indirect(self) -> None:
        self.mutate_text(
            checker.GO_MOD_RELATIVE,
            lambda text: text.replace(
                "require filippo.io/edwards25519 v1.2.0",
                "require filippo.io/edwards25519 v1.2.0 // indirect",
            ),
        )
        self.rebind()
        self.assert_fails()

    def test_go_replace_directive(self) -> None:
        self.mutate_text(
            checker.GO_MOD_RELATIVE,
            lambda text: text.rstrip("\n")
            + "\n\nreplace filippo.io/edwards25519 => example.com/evil v1.2.0\n",
        )
        self.rebind()
        self.assert_fails()

    def test_go_duplicate_module_directive(self) -> None:
        self.mutate_text(
            checker.GO_MOD_RELATIVE,
            lambda text: text.rstrip("\n")
            + "\n\nmodule example.com/magpie/go-verify-chain\n",
        )
        self.rebind()
        self.assert_fails()

    # --- P1 E: duplicate Cargo.lock packages ----------------------------

    def test_duplicate_rusqlite_package(self) -> None:
        self.mutate_text(
            checker.CARGO_LOCK_RELATIVE,
            lambda text: text.rstrip("\n")
            + "\n\n[[package]]\n"
            + 'name = "rusqlite"\n'
            + 'version = "0.38.0"\n'
            + 'source = "registry+https://github.com/rust-lang/crates.io-index"\n',
        )
        self.rebind()
        self.assert_fails()

    def test_duplicate_libsqlite3_sys_package(self) -> None:
        self.mutate_text(
            checker.CARGO_LOCK_RELATIVE,
            lambda text: text.rstrip("\n")
            + "\n\n[[package]]\n"
            + 'name = "libsqlite3-sys"\n'
            + 'version = "0.34.0"\n'
            + 'source = "registry+https://github.com/rust-lang/crates.io-index"\n',
        )
        self.rebind()
        self.assert_fails()

    # --- pre-audit: same-class closure for held candidate 4dbf867 -------

    def test_pip_install_in_quoted_text_is_not_execution(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                '          echo "pip install requests"\n'
                "          python -m pip install -r tools/requirements-portable-verifier.txt",
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    def test_pip_install_in_inline_comment_is_not_execution(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          echo ok # pip install requests\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt",
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    def test_unauthorized_pip_still_detected_among_noise(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                '          echo "pip install requests"\n'
                "          python -m pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "workflow pip install must install only from the requirements file"
        )

    # --- pip parser: command-identity detection follow-up ---------------

    def test_unquoted_echo_pip_text_is_not_execution(self) -> None:
        # Unquoted "pip install" tokens that are mere arguments to echo are
        # not pip invocations. The substring detector over-counted these and
        # rejected a valid workflow.
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          echo pip install requests\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt",
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    def test_pip_install_via_shell_c_is_detected(self) -> None:
        # A pip install hidden behind `sh -c "..."` must still be discovered:
        # the old detector stripped quoted content, which made this invisible
        # (a false green).
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                '          sh -c "pip install requests"',
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_versioned_python_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: python3 -m pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "workflow pip install must install only from the requirements file"
        )

    def test_env_prefixed_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: PIP_NO_INPUT=1 python -m pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "workflow pip install must install only from the requirements file"
        )

    def test_combined_shell_flag_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          bash -ec \"pip install requests\"\n"
                "          sh -ec \"pip install requests\"\n"
                "          bash -lc \"pip install requests\"",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 3"
        )

    def test_command_and_exec_prefixed_pip_install_are_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          command pip install requests\n"
                "          exec pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_pip3_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: pip3 install requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "workflow pip install must install only from the requirements file"
        )

    def test_versioned_pip3_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: pip3.12 install requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "workflow pip install must install only from the requirements file"
        )

    def test_sudo_prefixed_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          sudo pip install requests\n"
                "          sudo python -m pip install requests\n"
                "          sudo pip3 install requests\n"
                "          sudo -u root pip install requests\n"
                "          sudo -H pip install requests\n"
                "          sudo /usr/bin/pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 6"
        )

    def test_combined_shell_flag_echo_pip_text_is_not_execution(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                '          bash -ec "echo pip install requests"',
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    def test_command_v_pip_is_not_execution(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          command -v pip",
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    def test_sudo_non_pip_command_is_not_execution(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          sudo -u root cargo --version",
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    def test_env_option_prefixed_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          env -i pip install requests\n"
                "          env -u bash pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_env_option_non_pip_command_is_not_execution(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          env -i rustc --version",
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    def test_unknown_wrapper_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          nohup pip install requests\n"
                "          timeout 10 pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_unknown_wrapper_quoted_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                '          nohup "pip" "install" requests\n'
                '          env -i "pip" "install" requests',
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_unknown_wrapper_non_pip_command_is_not_execution(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          timeout 10 rustc --version",
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    def test_echo_command_substitution_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          echo $(pip install requests)",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_duplicate_verify_job_rejected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.rstrip("\n")
            + "\n  verify:\n"
            "    runs-on: ubuntu-22.04\n"
            "    steps:\n"
            "      - name: shadow\n"
            "        run: echo ok\n",
        )
        self.rebind()
        self.assert_fails("duplicate 'verify' job keys")

    def test_duplicate_with_key_rejected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "python-version: '3.12.14'",
                "python-version: '3.12.13'\n"
                "          python-version: '3.12.14'",
            ),
        )
        self.rebind()
        self.assert_fails("duplicate workflow step with key 'python-version'")

    def test_duplicate_action_step_rejected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "        uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262 # v4\n",
                "        uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262 # v4\n"
                "      - name: checkout again\n"
                "        uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262\n",
            ),
        )
        self.rebind()
        self.assert_fails("workflow action pins drifted")

    # --- audit F1: piped commands were invisible to the segmenter ---

    def test_piped_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: echo ok | pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "workflow pip install must install only from the requirements file"
        )

    def test_piped_pip_install_in_shell_c_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                '          bash -c "echo ok | pip install requests"',
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    # --- audit F2: backgrounded commands were invisible to the segmenter ---

    def test_backgrounded_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: echo ok & pip install requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "workflow pip install must install only from the requirements file"
        )

    # --- audit F3: backslash-escaped command names were invisible ---

    def test_backslash_escaped_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          p\\ip install requests\n"
                "          pi\\p install requests\n"
                "          python -m pi\\p install requests\n"
                "          env X=1 p\\ip install requests\n"
                "          command p\\ip install requests\n"
                "          bash -c 'p\\ip install requests'",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 6"
        )

    def test_backslash_inside_double_quotes_is_not_execution(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                '          echo "p\\ip install requests"',
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    def test_line_continuation_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          pip install \\\n"
                "              requests",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_nested_double_escaped_pip_install_in_shell_c_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                r'          bash -c "bash -c \"p\\ip install requests\""',
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    # --- audit F4: rust toolchain environment was not bound ---

    def test_workflow_rust_toolchain_env_changed(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "MAGPIE_RUST_TOOLCHAIN: '1.98.1'",
                "MAGPIE_RUST_TOOLCHAIN: '1.97.0'  # was '1.98.1'",
            ),
        )
        self.rebind()
        self.assert_fails("MAGPIE_RUST_TOOLCHAIN")

    def test_workflow_rustup_toolchain_env_changed(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "RUSTUP_TOOLCHAIN: '1.98.1'",
                "RUSTUP_TOOLCHAIN: '1.97.0'  # was '1.98.1'",
            ),
        )
        self.rebind()
        self.assert_fails("RUSTUP_TOOLCHAIN")

    def test_workflow_rust_toolchain_env_missing(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace("      MAGPIE_RUST_TOOLCHAIN: '1.98.1'\n", ""),
        )
        self.rebind()
        self.assert_fails("MAGPIE_RUST_TOOLCHAIN")

    def test_workflow_rustup_toolchain_env_missing(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace("      RUSTUP_TOOLCHAIN: '1.98.1'\n", ""),
        )
        self.rebind()
        self.assert_fails("RUSTUP_TOOLCHAIN")

    def test_workflow_duplicate_rust_toolchain_env_rejected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "MAGPIE_RUST_TOOLCHAIN: '1.98.1'",
                "MAGPIE_RUST_TOOLCHAIN: '1.98.0'\n"
                "      MAGPIE_RUST_TOOLCHAIN: '1.98.1'",
            ),
        )
        self.rebind()
        self.assert_fails("duplicate env key")

    # --- audit F5: command substitutions were invisible to the pip detector ---

    def test_command_substitution_pip_install_is_detected(self) -> None:
        # A pip install hidden inside a $(...) command substitution must be
        # discovered: the old detector tokenized the substitution opener as
        # part of an inert token and missed the pip inside (a false green).
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          $(pip install requests)",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_backtick_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          `pip install requests`",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_assignment_wrapped_command_substitution_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          x=$(pip install requests)",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_nested_command_substitution_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          sh -c \"$(pip install requests)\"",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_double_nested_command_substitution_pip_install_is_detected(self) -> None:
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt\n"
                "          echo $(echo $(pip install requests))",
            ),
        )
        self.rebind()
        self.assert_fails(
            "expected exactly one pip install command in the workflow, found 2"
        )

    def test_command_substitution_without_pip_is_not_execution(self) -> None:
        # A command substitution that does not run pip must not be counted
        # as an install (positive control for the F5 fix).
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          echo $(ls tools)\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt",
            ),
        )
        self.rebind()
        checker.run_check(self.root)

    def test_single_quoted_command_substitution_is_not_execution(self) -> None:
        # Single quotes make a substitution inert: the text inside is data,
        # not a command, so it must not be counted as an install.
        self.mutate_text(
            checker.WORKFLOW_RELATIVE,
            lambda text: text.replace(
                "run: python -m pip install -r tools/requirements-portable-verifier.txt",
                "run: |\n"
                "          echo '$(pip install requests)'\n"
                "          python -m pip install -r tools/requirements-portable-verifier.txt",
            ),
        )
        self.rebind()
        checker.run_check(self.root)


if __name__ == "__main__":
    unittest.main()
