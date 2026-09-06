"""Unit tests for the persistent A-018 diagnostic gate."""

from __future__ import annotations

from dataclasses import replace
import json
import unittest

from tools import check_a018_diagnostics as checker


def diagnostic_line(code: str | None) -> str:
    payload = {
        "$message_type": "diagnostic",
        "code": None if code is None else {"code": code},
        "level": "error",
    }
    return json.dumps(payload)


class DiagnosticResultTests(unittest.TestCase):
    def test_expected_code_is_accepted_and_uncoded_notes_are_ignored(self) -> None:
        observed = checker.validate_compilation_result(
            "synthetic",
            "E0308",
            1,
            "\n".join(
                (
                    diagnostic_line("E0308"),
                    diagnostic_line("dead_code"),
                    diagnostic_line(None),
                )
            ),
        )
        self.assertEqual(observed, frozenset({"E0308"}))

    def test_wrong_code_is_rejected(self) -> None:
        with self.assertRaisesRegex(
            checker.A018CheckError,
            r"expected coded diagnostics \['E0308'\], observed \['E0425'\]",
        ):
            checker.validate_compilation_result(
                "synthetic",
                "E0308",
                1,
                diagnostic_line("E0425"),
            )

    def test_old_false_green_codes_are_rejected(self) -> None:
        for incidental_code in ("E0425", "E0061", "E0432"):
            with self.subTest(incidental_code=incidental_code):
                with self.assertRaises(checker.A018CheckError):
                    checker.validate_compilation_result(
                        "synthetic",
                        "E0308",
                        1,
                        diagnostic_line(incidental_code),
                    )

    def test_unexpected_success_is_rejected(self) -> None:
        with self.assertRaisesRegex(
            checker.A018CheckError,
            "unexpectedly compiled successfully",
        ):
            checker.validate_compilation_result("synthetic", "E0451", 0, "")

    def test_extra_coded_error_is_rejected(self) -> None:
        output = "\n".join((diagnostic_line("E0308"), diagnostic_line("E0425")))
        with self.assertRaisesRegex(checker.A018CheckError, "E0425"):
            checker.validate_compilation_result("synthetic", "E0308", 1, output)

    def test_non_json_diagnostic_output_is_rejected(self) -> None:
        with self.assertRaisesRegex(checker.A018CheckError, "non-JSON"):
            checker.parse_diagnostic_codes("compiler text")


class InventoryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.document, cls.inventory = checker.load_inventory()

    def test_reviewed_inventory_contract_is_exact(self) -> None:
        checker.validate_inventory_contract(self.document, self.inventory)
        self.assertEqual(len(self.inventory), 43)

    def test_missing_witness_is_rejected(self) -> None:
        with self.assertRaisesRegex(checker.A018CheckError, "witness count changed"):
            checker.validate_inventory_contract(self.document, self.inventory[:-1])

    def test_source_extraction_matches_hash_backed_inventory(self) -> None:
        extracted = checker.extract_governed_sources()
        checker.compare_source_inventory(extracted, self.inventory)
        self.assertEqual(len(extracted), 43)

    def test_snippet_drift_is_rejected(self) -> None:
        extracted = checker.extract_governed_sources()
        changed = replace(
            extracted[0],
            snippet_sha256="0" * 64,
        )
        with self.assertRaisesRegex(checker.A018CheckError, "snippet hash changed"):
            checker.compare_source_inventory([changed, *extracted[1:]], self.inventory)

    def test_unclassified_compile_fail_fence_is_rejected(self) -> None:
        source = "//! ```compile_fail\n//! let _ = 1;\n//! ```\n"
        with self.assertRaisesRegex(checker.A018CheckError, "must name"):
            checker.extract_witnesses("synthetic.rs", source)


if __name__ == "__main__":
    unittest.main()
