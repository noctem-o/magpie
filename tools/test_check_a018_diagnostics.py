"""Unit tests for the persistent A-018 diagnostic gate."""

from __future__ import annotations

from dataclasses import replace
from copy import deepcopy
import json
from pathlib import Path
import unittest
from unittest import mock

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
        extracted = checker.extract_governed_sources(self.document)
        checker.compare_source_inventory(extracted, self.inventory)
        self.assertEqual(len(extracted), 43)

    def test_snippet_drift_is_rejected(self) -> None:
        extracted = checker.extract_governed_sources(self.document)
        changed = replace(
            extracted[0],
            snippet_sha256="0" * 64,
        )
        with self.assertRaisesRegex(checker.A018CheckError, "snippet hash changed"):
            checker.compare_source_inventory([changed, *extracted[1:]], self.inventory)


class GovernedSourceIdentityTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.document, cls.inventory = checker.load_inventory()
        cls.original_bytes = {
            source: (checker.ROOT / source).read_bytes()
            for source in checker.GOVERNED_SOURCES
        }

    def test_exact_current_sources_and_known_witnesses_pass(self) -> None:
        snapshots = checker.verified_governed_source_bytes(self.document)
        self.assertEqual(snapshots, self.original_bytes)
        extracted = checker.extract_governed_sources(self.document)
        checker.compare_source_inventory(extracted, self.inventory)
        self.assertEqual(len(extracted), 43)

    def assert_source_change_stops_gate(self, source: str, changed: bytes) -> None:
        snapshots = dict(self.original_bytes)
        snapshots[source] = changed
        # Exercise the real gate entry point, not just the digest helper. No
        # extraction or Cargo/rustc call may occur, even for the LAST source.
        document = deepcopy(self.document)
        before = deepcopy(document)

        def read_bytes(path: Path) -> bytes:
            return snapshots[path.relative_to(checker.ROOT).as_posix()]

        with (
            mock.patch.object(checker, "load_inventory", return_value=(document, self.inventory)),
            mock.patch.object(Path, "read_bytes", autospec=True, side_effect=read_bytes),
            mock.patch.object(checker, "extract_witnesses") as extraction,
            mock.patch.object(checker.subprocess, "run") as subprocess_run,
        ):
            with self.assertRaisesRegex(
                checker.A018CheckError,
                "governed source digest changed; explicit A-018 inventory review required",
            ) as error:
                checker.run_check()
            self.assertIn(source, str(error.exception))
            extraction.assert_not_called()
            subprocess_run.assert_not_called()
        self.assertEqual(document, before, "checking must never regenerate identities")

    def test_one_byte_change_in_each_source_fails_before_any_extraction(self) -> None:
        for source, raw in self.original_bytes.items():
            with self.subTest(source=source):
                self.assert_source_change_stops_gate(source, raw + b" ")

    def test_future_rustdoc_forms_cannot_escape_source_identity(self) -> None:
        forms = {
            "outer": '/// ```compile_fail,E0308\n/// missing_name();\n/// ```\npub fn added() {}\n',
            "quote": '//! > ```compile_fail,E0308\n//! > missing_name();\n//! > ```\n',
            "list": '//! - item\n//!\n//!   ```compile_fail,E0308\n//!   missing_name();\n//!   ```\n',
            "tab": '//!\t```compile_fail,E0308\n//!\tmissing_name();\n//!\t```\n',
            "target_ignore": '//! ```ignore-windows,compile_fail,E0308\n//! missing_name();\n//! ```\n',
            "explicit_rust": '//! ```text,rust,compile_fail,E0308\n//! missing_name();\n//! ```\n',
            "invalid_backtick_info": '//! ```text`\n//! ```compile_fail,E0308\n//! missing_name();\n//! ```\n',
            "block_doc": '/**\n```compile_fail,E0308\nmissing_name();\n```\n*/\npub fn added() {}\n',
            "doc_attribute": '#[doc = "```compile_fail,E0308\\nmissing_name();\\n```"]\npub fn added() {}\n',
        }
        for source, raw in self.original_bytes.items():
            for name, form in forms.items():
                with self.subTest(source=source, form=name):
                    # Inner docs belong before source items; outer docs after.
                    addition = form.encode("utf-8")
                    changed = addition + raw if form.startswith("//!") else raw + addition
                    self.assert_source_change_stops_gate(source, changed)

    def test_non_doc_rust_change_deliberately_requires_review(self) -> None:
        source = checker.GOVERNED_SOURCES[0]
        self.assert_source_change_stops_gate(
            source, self.original_bytes[source] + b"\nconst ADDED: u8 = 1;\n"
        )

    def test_raw_line_endings_are_not_normalized_before_hashing(self) -> None:
        source = checker.GOVERNED_SOURCES[0]
        self.assert_source_change_stops_gate(
            source, self.original_bytes[source].replace(b"\n", b"\r\n")
        )

    def test_witness_expectation_update_cannot_bypass_source_identity(self) -> None:
        document = deepcopy(self.document)
        document["witnesses"][0]["snippet_sha256"] = "0" * 64
        source = checker.GOVERNED_SOURCES[0]
        with mock.patch.object(Path, "read_bytes", return_value=self.original_bytes[source] + b" "):
            with self.assertRaisesRegex(checker.A018CheckError, "source digest changed"):
                checker.extract_governed_sources(document)

    def test_source_identity_set_and_hash_shape_are_required(self) -> None:
        for identities in (None, {}, {"unexpected.rs": {"sha256": "0" * 64}}):
            with self.subTest(identities=identities):
                with self.assertRaisesRegex(checker.A018CheckError, "exactly the three"):
                    checker.verified_governed_source_bytes({"governed_sources": identities})
        for invalid in (None, {}, {"sha256": "bad"}, {"sha256": 42}):
            with self.subTest(invalid=invalid):
                document = deepcopy(self.document)
                document["governed_sources"][checker.GOVERNED_SOURCES[0]] = invalid
                with self.assertRaisesRegex(checker.A018CheckError, "invalid governed source"):
                    checker.verified_governed_source_bytes(document)

    def test_missing_source_fails_closed(self) -> None:
        with mock.patch.object(Path, "read_bytes", side_effect=FileNotFoundError("missing")):
            with self.assertRaisesRegex(checker.A018CheckError, "cannot read governed source"):
                checker.extract_governed_sources(self.document)

    def test_extraction_uses_verified_snapshot_without_rereading(self) -> None:
        original_read = Path.read_bytes
        with mock.patch.object(Path, "read_bytes", autospec=True, side_effect=original_read) as reads:
            extracted = checker.extract_governed_sources(self.document)
        self.assertEqual(reads.call_count, 3)
        checker.compare_source_inventory(extracted, self.inventory)


class ExtractionTests(unittest.TestCase):
    """Convenience syntax coverage, NOT a Rustdoc completeness specification."""
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

    def test_rustdoc_info_whitespace_is_extracted_at_each_valid_indentation(
        self,
    ) -> None:
        separators = ("", " ", "    ", "\t", "\t\t", " \t ", "\v", "\f")
        for indentation in range(4):
            for delimiter_length in (3, 4, 5, 8):
                for separator in separators:
                    with self.subTest(
                        indentation=indentation,
                        delimiter_length=delimiter_length,
                        separator=repr(separator),
                    ):
                        raw_indent = " " * (indentation + 1)
                        fence = "`" * delimiter_length
                        source = (
                            f"//!{raw_indent}{fence}{separator}compile_fail,E0308\n"
                            f"//!{raw_indent}let _ = 1;\n"
                            f"//!{raw_indent}{fence}\n"
                        )
                        witnesses = checker.extract_witnesses("synthetic.rs", source)
                        self.assertEqual(len(witnesses), 1)
                        self.assertEqual(witnesses[0].snippet, "let _ = 1;\n")

    def test_non_ascii_whitespace_is_not_an_info_separator(self) -> None:
        for separator in ("\u00a0", "\u2003"):
            with self.subTest(separator=repr(separator)):
                source = (
                    f"//! ````{separator}compile_fail,E0308\n"
                    "//! let _ = 1;\n"
                    "//! ````\n"
                )
                self.assertEqual(checker.extract_witnesses("synthetic.rs", source), [])

    def test_rustdoc_info_whitespace_after_code_is_extracted(self) -> None:
        for suffix in ("", " ", "\t", "\v", "\f"):
            with self.subTest(suffix=repr(suffix)):
                source = (
                    f"//! ```` compile_fail,E0308{suffix}\n"
                    "//! let _ = 1;\n"
                    "//! ````\n"
                )
                witnesses = checker.extract_witnesses("synthetic.rs", source)
                self.assertEqual(len(witnesses), 1)

    def test_fence_opening_preserves_structure_before_classification(self) -> None:
        opening = checker._parse_opening_fence("  ~~~~~ compile_fail E0308")
        self.assertIsNotNone(opening)
        assert opening is not None
        self.assertEqual(opening.indent, 2)
        self.assertEqual(opening.delimiter_char, "~")
        self.assertEqual(opening.delimiter_length, 5)
        self.assertEqual(opening.raw_info_string, " compile_fail E0308")

    def test_rustdoc_compile_fail_forms_fail_closed_when_noncanonical(self) -> None:
        forms = (
            "compile_fail",
            "compile_fail E0308",
            "compile_fail ,E0308",
            "compile_fail, E0308",
            "compile_fail , E0308",
            "rust,compile_fail,E0308",
            "rust compile_fail E0308",
            "compile_fail,rust,E0308",
            "compile_fail,E0308,foo",
            "compile_fail,,E0308",
            "compile_fail , , E0308",
            "no_run,compile_fail,E0308",
            "should_panic,compile_fail,E0308",
            "edition2021,compile_fail,E0308",
            "rust,no_run,compile_fail,E0308",
        )
        for info in forms:
            with self.subTest(info=repr(info)):
                source = (
                    f"//! ```{info}\n"
                    "//! let _: u8 = \"wrong\";\n"
                    "//! ```\n"
                )
                with self.assertRaisesRegex(
                    checker.A018CheckError, "canonical governed annotation"
                ):
                    checker.extract_witnesses("synthetic.rs", source)

    def test_unknown_info_language_with_compile_fail_token_is_not_rustdoc_compile_fail(
        self,
    ) -> None:
        source = (
            "//! ```text,compile_fail,E0308\n"
            "//! let _: u8 = \"wrong\";\n"
            "//! ```\n"
        )
        self.assertEqual(checker.extract_witnesses("synthetic.rs", source), [])

    def test_tilde_fences_are_extracted_at_each_valid_length_and_indentation(
        self,
    ) -> None:
        for indentation in range(4):
            for delimiter_length in (3, 4, 5, 8):
                with self.subTest(
                    indentation=indentation,
                    delimiter_length=delimiter_length,
                ):
                    raw_indent = " " * (indentation + 1)
                    fence = "~" * delimiter_length
                    source = (
                        f"//!{raw_indent}{fence}compile_fail,E0308\n"
                        f"//!{raw_indent}let _ = 1;\n"
                        f"//!{raw_indent}{fence}\n"
                    )
                    witnesses = checker.extract_witnesses("synthetic.rs", source)
                    self.assertEqual(len(witnesses), 1)
                    self.assertEqual(witnesses[0].diagnostic, "E0308")

    def test_fences_require_same_delimiter_character(self) -> None:
        tilde_source = (
            "//! ~~~~compile_fail,E0308\n"
            "//! ````\n"
            "//! let _: u8 = \"wrong\";\n"
            "//! ~~~~\n"
        )
        backtick_source = (
            "//! ````compile_fail,E0308\n"
            "//! ~~~~\n"
            "//! let _: u8 = \"wrong\";\n"
            "//! ````\n"
        )
        for source in (tilde_source, backtick_source):
            with self.subTest(source=source.splitlines()[0]):
                witnesses = checker.extract_witnesses("synthetic.rs", source)
                self.assertEqual(len(witnesses), 1)
                self.assertIn("wrong", witnesses[0].snippet)

    def test_shorter_tilde_fences_do_not_close_longer_opening(self) -> None:
        source = (
            "//! ~~~~~compile_fail,E0308\n"
            "//! ~~~\n"
            "//! ~~~~\n"
            "//! let _: u8 = \"wrong\";\n"
            "//! ~~~~~~\n"
        )
        witnesses = checker.extract_witnesses("synthetic.rs", source)
        self.assertEqual(len(witnesses), 1)
        self.assertEqual(witnesses[0].snippet, '~~~\n~~~~\nlet _: u8 = "wrong";\n')

    def test_missing_compatible_tilde_fence_is_rejected(self) -> None:
        source = (
            "//! ~~~~compile_fail,E0308\n"
            "//! let _: u8 = \"wrong\";\n"
            "//! ````\n"
        )
        with self.assertRaisesRegex(checker.A018CheckError, "no closing fence"):
            checker.extract_witnesses("synthetic.rs", source)

    def test_noncompile_fence_body_is_parsed_as_one_structure(self) -> None:
        source = (
            "//! ```text\n"
            "//! ```compile_fail,E0308\n"
            "//! let _: u8 = \"wrong\";\n"
            "//! ```\n"
        )
        self.assertEqual(checker.extract_witnesses("synthetic.rs", source), [])

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
        for delimiter_char in ("`", "~"):
            for delimiter_length in (3, 4, 8):
                with self.subTest(
                    delimiter_char=delimiter_char,
                    delimiter_length=delimiter_length,
                ):
                    fence = delimiter_char * delimiter_length
                    source = (
                        f"//!     {fence}compile_fail,E0308\n"
                        f"//!     let _ = 1;\n"
                        f"//!     {fence}\n"
                    )
                    self.assertEqual(
                        checker.extract_witnesses("synthetic.rs", source), []
                    )

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

    def test_whitespace_separated_unclassified_fences_are_rejected(self) -> None:
        for separator in (" ", "\t", "\v", "\f"):
            with self.subTest(separator=repr(separator)):
                source = (
                    f"//! ````{separator}compile_fail\n"
                    "//! let _ = 1;\n"
                    "//! ````\n"
                )
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

    def test_new_whitespace_separated_fences_cause_inventory_drift(self) -> None:
        source_name = checker.GOVERNED_SOURCES[0]
        source_text = (checker.ROOT / source_name).read_text(encoding="utf-8")
        source_inventory = [
            witness for witness in self._inventory() if witness.source == source_name
        ]
        for delimiter_length, separator, indentation in (
            (4, " ", 0),
            (8, "\t", 2),
        ):
            with self.subTest(
                delimiter_length=delimiter_length,
                separator=repr(separator),
                indentation=indentation,
            ):
                raw_indent = " " * (indentation + 1)
                fence = "`" * delimiter_length
                source_with_extra = source_text + (
                    f"\n//!{raw_indent}{fence}{separator}compile_fail,E0308\n"
                    f"//!{raw_indent}let _: u8 = \"wrong\";\n"
                    f"//!{raw_indent}{fence}\n"
                )
                extracted = checker.extract_witnesses(source_name, source_with_extra)
                with self.assertRaisesRegex(checker.A018CheckError, "unexpected"):
                    checker.compare_source_inventory(extracted, source_inventory)

    def test_new_tilde_fence_causes_inventory_drift(self) -> None:
        source_name = checker.GOVERNED_SOURCES[0]
        source_text = (checker.ROOT / source_name).read_text(encoding="utf-8")
        source_inventory = [
            witness for witness in self._inventory() if witness.source == source_name
        ]
        source_with_extra = source_text + (
            "\n//! ~~~~~ compile_fail,E0308\n"
            "//! let _: u8 = \"wrong\";\n"
            "//! ~~~~~\n"
        )
        extracted = checker.extract_witnesses(source_name, source_with_extra)
        with self.assertRaisesRegex(checker.A018CheckError, "unexpected"):
            checker.compare_source_inventory(extracted, source_inventory)

    def test_new_noncanonical_fences_fail_closed_before_inventory_comparison(
        self,
    ) -> None:
        source_name = checker.GOVERNED_SOURCES[0]
        source_text = (checker.ROOT / source_name).read_text(encoding="utf-8")
        source_with_extra = source_text + (
            "\n//! ````` compile_fail E0308\n"
            "//! let _: u8 = \"wrong\";\n"
            "//! `````\n"
            "//! ~~~~~ compile_fail E0308\n"
            "//! let _: u8 = \"wrong\";\n"
            "//! ~~~~~\n"
        )
        with self.assertRaisesRegex(
            checker.A018CheckError, "canonical governed annotation"
        ):
            checker.extract_witnesses(source_name, source_with_extra)

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
