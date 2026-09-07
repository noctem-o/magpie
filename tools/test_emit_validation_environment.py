#!/usr/bin/env python3
"""Focused tests for validation-environment provenance emission."""

from __future__ import annotations

from contextlib import redirect_stderr, redirect_stdout
import hashlib
import io
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

from tools import emit_validation_environment as provenance


class ValidationEnvironmentTests(unittest.TestCase):
    @staticmethod
    def _github_environment(event_name: str) -> dict[str, str]:
        environment = {
            "GITHUB_ACTIONS": "true",
            "GITHUB_EVENT_NAME": event_name,
            "GITHUB_JOB": "verify",
            "GITHUB_REF": "refs/pull/148/merge",
            "GITHUB_RUN_ATTEMPT": "1",
            "GITHUB_RUN_ID": "run-id",
            "GITHUB_SHA": "merge-sha",
            "GITHUB_WORKFLOW": "CI",
            "GITHUB_WORKFLOW_REF": "owner/repo/.github/workflows/ci.yml@ref",
            "ImageOS": "ubuntu24",
            "ImageVersion": "image-version",
            "MAGPIE_CANDIDATE_SHA": "candidate-sha",
            "MAGPIE_RUNNER_LABEL": "ubuntu-24.04",
            "MAGPIE_WORKFLOW_SHA": "workflow-sha",
            "RUNNER_ARCH": "X64",
            "RUNNER_ENVIRONMENT": "github-hosted",
            "RUNNER_NAME": "GitHub Actions runner",
            "RUNNER_OS": "Linux",
        }
        if event_name == "pull_request":
            environment["GITHUB_BASE_REF"] = "main"
            environment["GITHUB_HEAD_REF"] = "ci/pin-prealpha-assurance"
        return environment

    @staticmethod
    def _command_output(command: list[str] | tuple[str, ...]) -> str:
        command_tuple = tuple(command)
        outputs = {
            ("git", "rev-parse", "HEAD"): "checkout-sha",
            ("go", "env", "GOARCH"): "amd64",
            ("go", "env", "GOOS"): "linux",
            ("go", "version"): "go version go1.27.0 linux/amd64",
            ("cargo", "--version"): "cargo 1.98.1",
            ("rustc", "-Vv"): "rustc 1.98.1",
            (sys.executable, "--version"): "Python 3.12.14",
            (sys.executable, "-m", "pip", "--version"): "pip test",
        }
        try:
            return outputs[command_tuple]
        except KeyError as exc:
            raise AssertionError(f"unexpected mocked command: {command_tuple!r}") from exc

    def test_canonical_record_has_stable_key_order_and_digest(self) -> None:
        first = provenance.canonical_record({"z": 1, "a": {"y": 2, "b": 3}})
        second = provenance.canonical_record({"a": {"b": 3, "y": 2}, "z": 1})
        self.assertEqual(first, second)
        self.assertEqual(first, b'{"a":{"b":3,"y":2},"z":1}')
        self.assertEqual(
            hashlib.sha256(first).hexdigest(),
            hashlib.sha256(second).hexdigest(),
        )

    def test_tree_identity_is_order_independent_and_content_sensitive(self) -> None:
        with tempfile.TemporaryDirectory(prefix="magpie-provenance-tree-") as directory:
            root = Path(directory)
            (root / "z.txt").write_bytes(b"z")
            (root / "nested").mkdir()
            (root / "nested" / "a.txt").write_bytes(b"a")

            first = provenance._tree_identity(root)
            second = provenance._tree_identity(root)
            self.assertEqual(first, second)
            self.assertEqual(first["entry_count"], 2)
            self.assertEqual(
                first["identity_profile"], provenance.TREE_IDENTITY_PROFILE
            )

            (root / "nested" / "a.txt").write_bytes(b"changed")
            self.assertNotEqual(first["sha256"], provenance._tree_identity(root)["sha256"])

    def test_tree_identity_changes_when_file_is_renamed_or_moved(self) -> None:
        with tempfile.TemporaryDirectory(prefix="magpie-provenance-tree-") as directory:
            root = Path(directory)
            (root / "a.txt").write_bytes(b"identical payload")
            baseline = provenance._tree_identity(root)
            self.assertEqual(baseline["entry_count"], 1)

            (root / "a.txt").rename(root / "b.txt")
            self.assertEqual((root / "b.txt").read_bytes(), b"identical payload")
            renamed = provenance._tree_identity(root)
            self.assertNotEqual(baseline["sha256"], renamed["sha256"])

            (root / "nested").mkdir()
            (root / "b.txt").rename(root / "nested" / "b.txt")
            self.assertEqual(
                (root / "nested" / "b.txt").read_bytes(), b"identical payload"
            )
            moved = provenance._tree_identity(root)
            self.assertNotEqual(baseline["sha256"], moved["sha256"])
            self.assertNotEqual(renamed["sha256"], moved["sha256"])

    def test_tree_identity_distinguishes_file_from_symlink_with_same_text(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="magpie-provenance-tree-"
        ) as directory, tempfile.TemporaryDirectory(
            prefix="magpie-provenance-external-"
        ) as external:
            root = Path(directory)
            target = Path(external) / "target.txt"
            target.write_bytes(str(target).encode("utf-8"))

            (root / "f.txt").write_bytes(str(target).encode("utf-8"))
            file_identity = provenance._tree_identity(root)

            (root / "f.txt").unlink()
            os.symlink(target, root / "f.txt")
            self.assertEqual(
                (root / "f.txt").read_bytes(), str(target).encode("utf-8")
            )
            link_identity = provenance._tree_identity(root)

            self.assertEqual(file_identity["entry_count"], 1)
            self.assertEqual(link_identity["entry_count"], 1)
            self.assertNotEqual(file_identity["sha256"], link_identity["sha256"])

    def test_tree_identity_changes_when_symlink_target_changes(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="magpie-provenance-tree-"
        ) as directory, tempfile.TemporaryDirectory(
            prefix="magpie-provenance-external-"
        ) as external:
            root = Path(directory)
            first_target = Path(external) / "a.txt"
            second_target = Path(external) / "b.txt"
            first_target.write_bytes(b"identical payload")
            second_target.write_bytes(b"identical payload")

            os.symlink(first_target, root / "link.txt")
            first = provenance._tree_identity(root)

            (root / "link.txt").unlink()
            os.symlink(second_target, root / "link.txt")
            self.assertEqual((root / "link.txt").read_bytes(), b"identical payload")
            second = provenance._tree_identity(root)

            self.assertEqual(first["entry_count"], 1)
            self.assertEqual(second["entry_count"], 1)
            self.assertNotEqual(first["sha256"], second["sha256"])

    def test_tree_identity_ignores_contents_resolved_through_symlink(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="magpie-provenance-tree-"
        ) as directory, tempfile.TemporaryDirectory(
            prefix="magpie-provenance-external-"
        ) as external:
            root = Path(directory)
            target = Path(external) / "target.txt"
            target.write_bytes(b"original contents")
            os.symlink(target, root / "link.txt")

            first = provenance._tree_identity(root)
            target.write_bytes(b"changed contents")
            second = provenance._tree_identity(root)

            self.assertEqual(first, second)

    def test_tree_identity_ignores_empty_directories(self) -> None:
        with tempfile.TemporaryDirectory(prefix="magpie-provenance-tree-") as directory:
            root = Path(directory)
            (root / "a.txt").write_bytes(b"data")
            baseline = provenance._tree_identity(root)
            self.assertEqual(baseline["entry_count"], 1)

            (root / "empty-dir").mkdir()
            self.assertEqual(provenance._tree_identity(root), baseline)

            (root / "empty-dir").rmdir()
            self.assertEqual(provenance._tree_identity(root), baseline)

    def test_record_keeps_candidate_github_and_checkout_shas_distinct(self) -> None:
        environment = self._github_environment("pull_request")
        record = provenance.collect_environment(environment, self._command_output)
        source = record["source_and_workflow"]
        self.assertEqual(source["candidate_head_sha"], "candidate-sha")
        self.assertEqual(source["github_sha"], "merge-sha")
        self.assertEqual(source["actual_checked_out_head_sha"], "checkout-sha")
        self.assertEqual(source["workflow_sha"], "workflow-sha")
        self.assertEqual(source["github_head_ref"], "ci/pin-prealpha-assurance")
        self.assertEqual(source["github_base_ref"], "main")
        self.assertEqual(record["runner"]["selected_label"], "ubuntu-24.04")
        self.assertEqual(
            record["toolchains"]["rust"]["cargo_version"],
            "cargo 1.98.1",
        )
        self.assertEqual(
            record["toolchains"]["rust"]["rustc_verbose_version"],
            "rustc 1.98.1",
        )
        self.assertEqual(
            record["toolchains"]["python"]["version_output"],
            "Python 3.12.14",
        )
        self.assertNotEqual(
            record["toolchains"]["rust"]["cargo_version"],
            record["toolchains"]["python"]["version_output"],
        )
        self.assertEqual(
            record["governed_inputs"]["frozen_portable_corpus_manifest"][
                "sha256"
            ],
            provenance.FROZEN_CORPUS_MANIFEST_SHA256,
        )
        json.loads(provenance.canonical_record(record))

    def test_command_output_dispatch_is_exact(self) -> None:
        self.assertEqual(
            self._command_output([sys.executable, "--version"]),
            "Python 3.12.14",
        )
        self.assertEqual(
            self._command_output([sys.executable, "-m", "pip", "--version"]),
            "pip test",
        )
        self.assertEqual(
            self._command_output(["cargo", "--version"]),
            "cargo 1.98.1",
        )
        with self.assertRaisesRegex(AssertionError, "unexpected mocked command"):
            self._command_output(["unknown-tool", "--version"])

    def test_missing_required_github_coordinate_fails_without_record_output(self) -> None:
        environment = self._github_environment("pull_request")
        del environment["ImageVersion"]
        stdout = io.StringIO()
        stderr = io.StringIO()
        with mock.patch.dict(os.environ, environment, clear=True):
            with redirect_stdout(stdout), redirect_stderr(stderr):
                result = provenance.main()

        self.assertEqual(result, 2)
        self.assertEqual(stdout.getvalue(), "")
        self.assertIn("ImageVersion", stderr.getvalue())
        self.assertNotIn("validation-environment-json=", stderr.getvalue())
        self.assertNotIn("validation-environment-sha256=", stderr.getvalue())

    def test_each_common_github_coordinate_is_required(self) -> None:
        expected = (
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
        self.assertEqual(provenance.GITHUB_ACTIONS_REQUIRED_COORDINATES, expected)
        for coordinate in expected:
            with self.subTest(coordinate=coordinate):
                environment = self._github_environment("pull_request")
                del environment[coordinate]
                with self.assertRaisesRegex(
                    provenance.ValidationEnvironmentError,
                    f"coordinates: {coordinate}$",
                ):
                    provenance.collect_environment(environment, self._command_output)

    def test_multiple_missing_coordinates_are_sorted_and_whitespace_is_missing(
        self,
    ) -> None:
        environment = self._github_environment("pull_request")
        environment["RUNNER_NAME"] = "\t"
        environment["ImageVersion"] = "  "
        with self.assertRaisesRegex(
            provenance.ValidationEnvironmentError,
            "coordinates: ImageVersion, RUNNER_NAME$",
        ):
            provenance.collect_environment(environment, self._command_output)

    def test_push_does_not_require_pull_request_refs(self) -> None:
        environment = self._github_environment("push")
        environment["GITHUB_HEAD_REF"] = ""
        record = provenance.collect_environment(environment, self._command_output)
        source = record["source_and_workflow"]
        self.assertEqual(source["event_name"], "push")
        self.assertEqual(source["github_head_ref"], "")
        self.assertIsNone(source["github_base_ref"])

    def test_pull_request_requires_head_and_base_refs(self) -> None:
        for coordinate in provenance.PULL_REQUEST_REQUIRED_COORDINATES:
            with self.subTest(coordinate=coordinate):
                environment = self._github_environment("pull_request")
                del environment[coordinate]
                with self.assertRaisesRegex(
                    provenance.ValidationEnvironmentError,
                    f"coordinates: {coordinate}$",
                ):
                    provenance.collect_environment(environment, self._command_output)

    def test_local_manual_environment_remains_usable(self) -> None:
        record = provenance.collect_environment({}, self._command_output)
        self.assertIsNone(record["runner"]["hosted_image_version"])
        self.assertIsNone(record["source_and_workflow"]["candidate_head_sha"])
        self.assertEqual(
            record["source_and_workflow"]["actual_checked_out_head_sha"],
            "checkout-sha",
        )

    def test_unexpected_github_event_fails_closed(self) -> None:
        environment = self._github_environment("workflow_dispatch")
        with self.assertRaisesRegex(
            provenance.ValidationEnvironmentError,
            "unsupported GitHub Actions event.*workflow_dispatch$",
        ):
            provenance.collect_environment(environment, self._command_output)


if __name__ == "__main__":
    unittest.main()
