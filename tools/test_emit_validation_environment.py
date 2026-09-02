#!/usr/bin/env python3
"""Focused tests for validation-environment provenance emission."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from tools import emit_validation_environment as provenance


class ValidationEnvironmentTests(unittest.TestCase):
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

    def test_record_keeps_candidate_github_and_checkout_shas_distinct(self) -> None:
        environment = {
            "GITHUB_SHA": "merge-sha",
            "MAGPIE_CANDIDATE_SHA": "candidate-sha",
            "MAGPIE_RUNNER_LABEL": "ubuntu-24.04",
            "MAGPIE_WORKFLOW_SHA": "workflow-sha",
        }

        def command_output(command: list[str] | tuple[str, ...]) -> str:
            command_tuple = tuple(command)
            outputs = {
                ("git", "rev-parse", "HEAD"): "checkout-sha",
                ("go", "env", "GOARCH"): "amd64",
                ("go", "env", "GOOS"): "linux",
                ("go", "version"): "go version go1.27.0 linux/amd64",
                ("cargo", "--version"): "cargo 1.98.0",
                ("rustc", "-Vv"): "rustc 1.98.0",
            }
            if command_tuple[-3:] == ("-m", "pip", "--version"):
                return "pip test"
            if command_tuple[-1:] == ("--version",):
                return "Python 3.12.14"
            return outputs[command_tuple]

        record = provenance.collect_environment(environment, command_output)
        source = record["source_and_workflow"]
        self.assertEqual(source["candidate_head_sha"], "candidate-sha")
        self.assertEqual(source["github_sha"], "merge-sha")
        self.assertEqual(source["actual_checked_out_head_sha"], "checkout-sha")
        self.assertEqual(source["workflow_sha"], "workflow-sha")
        self.assertEqual(record["runner"]["selected_label"], "ubuntu-24.04")
        self.assertEqual(
            record["governed_inputs"]["frozen_portable_corpus_manifest"][
                "sha256"
            ],
            provenance.FROZEN_CORPUS_MANIFEST_SHA256,
        )
        json.loads(provenance.canonical_record(record))


if __name__ == "__main__":
    unittest.main()
