#!/usr/bin/env python3
"""Focused tests for the required Python reference-verifier path."""

from __future__ import annotations

import copy
import sys
from pathlib import Path
import unittest
from unittest import mock


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from tools import check_portable_verifier_python as harness  # noqa: E402
from tools import portable_verifier as prior_conformer  # noqa: E402
from tools import verify_chain  # noqa: E402


class PortableVerifierHarnessTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.manifest, cls.manifest_digest = harness.load_verified_manifest()
        cls.cases = harness.load_verified_cases(cls.manifest)
        cls.by_id = {case.case_id: case for case in cls.cases}
        cls.profile = cls.manifest["producing_context"]["signature_profile_identity"]

    def verify_case(self, case_id: str) -> dict[str, object]:
        case = self.by_id[case_id]
        return verify_chain.verify_complete_history(
            self.profile, case.external_key, case.input_bytes
        ).as_dict()

    def test_full_frozen_corpus_matches_every_governed_field(self) -> None:
        report = harness.run_corpus()

        self.assertTrue(report.passed, report.mismatches)
        self.assertEqual(report.case_count, 432)
        self.assertEqual(
            report.inventory_vector,
            (19, 21, 13, 18, 272, 4, 5, 3, 16, 46, 15),
        )
        self.assertEqual(
            report.manifest_digest,
            "7d758d3f2dac1161fe15dd064b130ccbfcdaf0437ea8ab8d7772492194801a81",
        )
        self.assertTrue(
            all(
                set(comparison.actual) == set(harness.GOVERNED_FIELDS)
                and comparison.actual == comparison.expected
                for comparison in report.comparisons
            )
        )

    def test_prior_independent_conformer_artifact_remains_fixed(self) -> None:
        for case in self.cases:
            with self.subTest(case_id=case.case_id):
                actual = prior_conformer.verify_complete_history(
                    self.profile,
                    case.external_key,
                    case.input_bytes,
                ).as_dict()
                self.assertEqual(actual, case.expected)

    def test_external_key_gate_precedes_empty_snapshot_acceptance(self) -> None:
        result = self.verify_case("n3-key-empty")

        self.assertEqual(
            result,
            {
                "verdict": "REJECT",
                "class": "ExternalKey",
                "line": None,
                "record_index": None,
                "event_count": None,
                "tip": None,
                "ordered_recomputed_hashes": [],
            },
        )
        self.assertEqual(self.by_id["n3-key-empty"].input_bytes, b"")

    def test_empty_snapshot_acceptance_and_full_width_timestamp_boundaries(self) -> None:
        empty = self.verify_case("k15-empty-sign-bit-one-prime-key")
        self.assertEqual(empty["verdict"], "ACCEPT")
        self.assertEqual(empty["event_count"], 0)
        self.assertEqual(empty["tip"], "0" * 64)
        self.assertEqual(empty["ordered_recomputed_hashes"], [])

        for case_id in (
            "u64-p1-timestamp-9007199254740993",
            "u64-p2-timestamp-9223372036854775808",
            "u64-p3-timestamp-18446744073709551615",
        ):
            with self.subTest(case_id=case_id):
                result = self.verify_case(case_id)
                self.assertEqual(result["verdict"], "ACCEPT")
                self.assertEqual(result["event_count"], 1)
                self.assertEqual(len(result["ordered_recomputed_hashes"]), 1)
                self.assertEqual(result["tip"], result["ordered_recomputed_hashes"][0])

    def test_u64_maximum_sequence_is_schema_valid_then_sequence_failure(self) -> None:
        for case_id in (
            "u64-p4-sequence-9223372036854775808",
            "u64-p5-sequence-18446744073709551615",
        ):
            with self.subTest(case_id=case_id):
                result = self.verify_case(case_id)
                self.assertEqual(result["verdict"], "REJECT")
                self.assertEqual(result["class"], "Sequence")
                self.assertEqual(result["line"], 1)
                self.assertEqual(result["record_index"], 0)

    def test_arbitrarily_long_json_integer_is_governed_schema(self) -> None:
        case = self.by_id["p1-golden-v1"]
        hostile = case.input_bytes.replace(b'"seq":0', b'"seq":' + (b"1" * 5000), 1)

        result = verify_chain.verify_complete_history(
            self.profile, case.external_key, hostile
        ).as_dict()

        self.assertEqual(
            (result["verdict"], result["class"], result["line"], result["record_index"]),
            ("REJECT", "Schema", 1, 0),
        )

    def test_framing_coordinates_preserve_zero_length_and_candidate_indices(self) -> None:
        zero_length = self.verify_case("n7-lf-only-empty-record")
        self.assertEqual(
            (zero_length["class"], zero_length["line"], zero_length["record_index"]),
            ("Framing", 1, None),
        )

        later_bom = self.verify_case("n13-bom-later")
        self.assertEqual(
            (later_bom["class"], later_bom["line"], later_bom["record_index"]),
            ("Framing", 2, 1),
        )

        terminal_lone_cr = self.verify_case("n30-terminal-lone-cr")
        self.assertEqual(
            (
                terminal_lone_cr["class"],
                terminal_lone_cr["line"],
                terminal_lone_cr["record_index"],
            ),
            ("Framing", 1, 0),
        )

    def test_input_hash_preflight_rejects_tampered_manifest_before_verification(self) -> None:
        tampered = copy.deepcopy(self.manifest)
        tampered["cases"][0]["input_sha256"] = "0" * 64

        with self.assertRaisesRegex(harness.HarnessError, "input SHA-256 mismatch"):
            harness.load_verified_cases(tampered)

    def test_manifest_digest_preflight_is_pinned(self) -> None:
        with mock.patch.object(harness, "EXPECTED_MANIFEST_SHA256", "0" * 64):
            with self.assertRaisesRegex(harness.HarnessError, "manifest bytes"):
                harness.load_verified_manifest()


if __name__ == "__main__":
    unittest.main()
