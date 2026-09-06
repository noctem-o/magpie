"""Unit tests for the persistent A-018 diagnostic gate."""

from __future__ import annotations

from dataclasses import replace
import json
from pathlib import Path
import unittest

from tools import check_a018_diagnostics as checker


def diagnostic_line(
    code: str | None,
    *,
    level: str = "error",
    file_name: str | None = None,
    line_start: int | None = None,
    column_start: int | None = None,
    children: list[dict[str, object]] | None = None,
) -> str:
    payload: dict[str, object] = {
        "$message_type": "diagnostic",
        "code": None if code is None else {"code": code},
        "level": level,
    }
    if file_name is not None:
        payload["spans"] = [
            {
                "file_name": file_name,
                "line_start": line_start,
                "column_start": column_start,
                "is_primary": True,
            }
        ]
    if children is not None:
        payload["children"] = children
    return json.dumps(payload)


def expected_occurrence(
    code: str, line: int | None = None, column: int | None = None
) -> checker.ExpectedDiagnosticOccurrence:
    return checker.ExpectedDiagnosticOccurrence(
        code=code,
        relative_line=line,
        relative_column=column,
    )


class DiagnosticResultTests(unittest.TestCase):
    def test_expected_code_is_accepted_and_uncoded_notes_are_ignored(self) -> None:
        observed = checker.validate_compilation_result(
            "synthetic",
            "E0308",
            1,
            "\n".join(
                (
                    diagnostic_line(
                        "E0308",
                        children=[
                            {
                                "code": {"code": "E0308"},
                                "level": "error",
                            }
                        ],
                    ),
                    diagnostic_line("dead_code"),
                    diagnostic_line("E0308", level="note"),
                )
            ),
        )
        self.assertEqual(
            tuple(observed_item.code for observed_item in observed),
            ("E0308",),
        )

    def test_wrong_code_is_rejected(self) -> None:
        with self.assertRaisesRegex(
            checker.A018CheckError,
            r"expected diagnostic occurrences \{'E0308': 1\}, observed \{'E0425': 1\}",
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
            checker.parse_diagnostic_occurrences("compiler text")

    def test_expected_multiplicity_and_relative_spans_are_accepted(self) -> None:
        source_path = Path("/tmp/a018-synthetic.rs")
        expected = tuple(
            expected_occurrence("E0308", line, 55) for line in (11, 12, 13)
        )
        output = "\n".join(
            diagnostic_line(
                "E0308",
                file_name=str(source_path),
                line_start=line + 1,
                column_start=55,
            )
            for line in (11, 12, 13)
        )
        observed = checker.validate_compilation_result(
            "multi-call",
            "E0308",
            1,
            output,
            expected,
            source_path,
        )
        self.assertEqual(len(observed), 3)

    def test_three_expected_occurrences_but_two_observed_are_rejected(self) -> None:
        expected = tuple(expected_occurrence("E0308") for _ in range(3))
        with self.assertRaisesRegex(checker.A018CheckError, "expected.*3"):
            checker.validate_compilation_result(
                "multi-call",
                "E0308",
                1,
                "\n".join(diagnostic_line("E0308") for _ in range(2)),
                expected,
            )

    def test_three_expected_occurrences_but_one_observed_are_rejected(self) -> None:
        expected = tuple(expected_occurrence("E0308") for _ in range(3))
        with self.assertRaises(checker.A018CheckError):
            checker.validate_compilation_result(
                "multi-call", "E0308", 1, diagnostic_line("E0308"), expected
            )

    def test_unrelated_code_instead_of_one_occurrence_is_rejected(self) -> None:
        expected = tuple(expected_occurrence("E0308") for _ in range(3))
        output = "\n".join(
            (
                diagnostic_line("E0308"),
                diagnostic_line("E0308"),
                diagnostic_line("E0425"),
            )
        )
        with self.assertRaises(checker.A018CheckError):
            checker.validate_compilation_result(
                "multi-call", "E0308", 1, output, expected
            )

    def test_child_and_note_diagnostics_do_not_inflate_occurrence_count(self) -> None:
        output = "\n".join(
            (
                diagnostic_line(
                    "E0308",
                    children=[
                        {"code": {"code": "E0308"}, "level": "error"},
                    ],
                ),
                diagnostic_line("E0308", level="note"),
            )
        )
        observed = checker.parse_diagnostic_occurrences(output)
        self.assertEqual(len(observed), 1)

    def test_cargo_checker_build_is_not_offline_only(self) -> None:
        command = checker._cargo_claims_command(Path("/tmp/a018-target"))
        self.assertIn("--locked", command)
        self.assertNotIn("--offline", command)
        self.assertIn("+1.98.1", command)


class InventoryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.document, cls.inventory = checker.load_inventory()

    def test_reviewed_inventory_contract_is_exact(self) -> None:
        checker.validate_inventory_contract(self.document, self.inventory)
        self.assertEqual(len(self.inventory), 43)
        multi_call = next(
            witness
            for witness in self.inventory
            if witness.witness_id == "origin-admission-audit-18"
        )
        self.assertEqual(len(multi_call.diagnostic_occurrences), 3)

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


class ExtractionTests(unittest.TestCase):
    def test_zero_through_three_space_fences_are_extracted(self) -> None:
        for indentation in range(4):
            with self.subTest(indentation=indentation):
                raw_indent = " " * (indentation + 1)
                source = (
                    f"//!{raw_indent}```compile_fail,E0308\n"
                    f"//!{raw_indent}let _ = 1;\n"
                    f"//!{raw_indent}```\n"
                )
                witnesses = checker.extract_witnesses("synthetic.rs", source)
                self.assertEqual(len(witnesses), 1)
                self.assertEqual(witnesses[0].snippet, "let _ = 1;\n")

    def test_long_backtick_fences_are_extracted_at_each_valid_indentation(self) -> None:
        for indentation in range(4):
            for delimiter_length in (3, 4, 5, 8):
                with self.subTest(
                    indentation=indentation, delimiter_length=delimiter_length
                ):
                    raw_indent = " " * (indentation + 1)
                    fence = "`" * delimiter_length
                    source = (
                        f"//!{raw_indent}{fence}compile_fail,E0308\n"
                        f"//!{raw_indent}let _ = 1;\n"
                        f"//!{raw_indent}{fence}\n"
                    )
                    witnesses = checker.extract_witnesses("synthetic.rs", source)
                    self.assertEqual(len(witnesses), 1)
                    self.assertEqual(witnesses[0].snippet, "let _ = 1;\n")

    def test_longer_closing_fence_is_compatible(self) -> None:
        source = (
            "//! ````compile_fail,E0308\n"
            "//! ```\n"
            "//! let _ = 1;\n"
            "//! `````\n"
        )
        witnesses = checker.extract_witnesses("synthetic.rs", source)
        self.assertEqual(len(witnesses), 1)
        self.assertEqual(witnesses[0].snippet, "```\nlet _ = 1;\n")

    def test_shorter_fences_do_not_close_longer_opening(self) -> None:
        source = (
            "//! `````compile_fail,E0308\n"
            "//! ```\n"
            "//! ````\n"
            "//! let _ = 1;\n"
            "//! ``````\n"
        )
        witnesses = checker.extract_witnesses("synthetic.rs", source)
        self.assertEqual(len(witnesses), 1)
        self.assertEqual(witnesses[0].snippet, "```\n````\nlet _ = 1;\n")

    def test_four_space_fence_is_not_a_rustdoc_fence(self) -> None:
        for delimiter_length in (3, 4, 8):
            with self.subTest(delimiter_length=delimiter_length):
                fence = "`" * delimiter_length
                source = (
                    f"//!     {fence}compile_fail,E0308\n"
                    f"//!     let _ = 1;\n"
                    f"//!     {fence}\n"
                )
                self.assertEqual(checker.extract_witnesses("synthetic.rs", source), [])

    def test_indented_opening_and_closing_fences_are_extracted(self) -> None:
        source = "//!   ````compile_fail,E0308\n//!   let _ = 1;\n//! `````\n"
        witnesses = checker.extract_witnesses("synthetic.rs", source)
        self.assertEqual(len(witnesses), 1)
        self.assertEqual(witnesses[0].diagnostic, "E0308")

    def test_indented_unclassified_compile_fail_fence_is_rejected(self) -> None:
        source = "//!   ```compile_fail\n//!   let _ = 1;\n//!   ```\n"
        with self.assertRaisesRegex(checker.A018CheckError, "must name"):
            checker.extract_witnesses("synthetic.rs", source)

    def test_indented_unclassified_long_fence_is_rejected(self) -> None:
        source = "//!   ````compile_fail\n//!   let _ = 1;\n//!   ````\n"
        with self.assertRaisesRegex(checker.A018CheckError, "must name"):
            checker.extract_witnesses("synthetic.rs", source)

    def test_new_indented_fence_causes_inventory_drift(self) -> None:
        source_name = checker.GOVERNED_SOURCES[0]
        source_text = (checker.ROOT / source_name).read_text(encoding="utf-8")
        source_text += (
            "\n//!   ```compile_fail,E0308\n"
            "//!   let _ = 1;\n"
            "//!   ```\n"
        )
        extracted = checker.extract_witnesses(source_name, source_text)
        source_inventory = [
            witness for witness in self._inventory() if witness.source == source_name
        ]
        with self.assertRaisesRegex(checker.A018CheckError, "unexpected"):
            checker.compare_source_inventory(extracted, source_inventory)

    def test_new_long_fence_causes_inventory_drift(self) -> None:
        source_name = checker.GOVERNED_SOURCES[0]
        source_text = (checker.ROOT / source_name).read_text(encoding="utf-8")
        source_text += (
            "\n//! ````compile_fail,E0308\n"
            "//! let _ = 1;\n"
            "//! ````\n"
        )
        extracted = checker.extract_witnesses(source_name, source_text)
        source_inventory = [
            witness for witness in self._inventory() if witness.source == source_name
        ]
        with self.assertRaisesRegex(checker.A018CheckError, "unexpected"):
            checker.compare_source_inventory(extracted, source_inventory)

    def test_missing_compatible_long_fence_is_rejected(self) -> None:
        source = (
            "//! ````compile_fail,E0308\n"
            "//! let _ = 1;\n"
            "//! ```\n"
        )
        with self.assertRaisesRegex(checker.A018CheckError, "no closing fence"):
            checker.extract_witnesses("synthetic.rs", source)

    def test_missing_governed_fence_causes_inventory_drift(self) -> None:
        source_name = checker.GOVERNED_SOURCES[0]
        source_text = (checker.ROOT / source_name).read_text(encoding="utf-8")
        lines = source_text.splitlines()
        opening = next(
            index
            for index, line in enumerate(lines)
            if line.startswith("//! ```compile_fail,")
        )
        closing = next(
            index
            for index in range(opening + 1, len(lines))
            if lines[index] == "//! ```"
        )
        source_without_first_witness = "\n".join(
            lines[:opening] + lines[closing + 1 :]
        ) + "\n"
        extracted = checker.extract_witnesses(
            source_name, source_without_first_witness
        )
        source_inventory = [
            witness for witness in self._inventory() if witness.source == source_name
        ]
        with self.assertRaisesRegex(checker.A018CheckError, "missing"):
            checker.compare_source_inventory(extracted, source_inventory)

    @staticmethod
    def _inventory() -> list[checker.InventoryWitness]:
        _, inventory = checker.load_inventory()
        return inventory

    def test_unclassified_compile_fail_fence_is_rejected(self) -> None:
        source = "//! ```compile_fail\n//! let _ = 1;\n//! ```\n"
        with self.assertRaisesRegex(checker.A018CheckError, "must name"):
            checker.extract_witnesses("synthetic.rs", source)


if __name__ == "__main__":
    unittest.main()
