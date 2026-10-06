"""Self-tests for the SQLite L0 qualification checker."""

from __future__ import annotations

import json
import unittest
from copy import deepcopy
from dataclasses import replace
from pathlib import Path
from unittest import mock

from tools import check_sqlite_l0_qualification as checker


class SqliteL0QualificationCheckerTests(unittest.TestCase):
    def setUp(self) -> None:
        self.document, self.witnesses = checker.load_inventory(checker.INVENTORY_PATH)

    def test_real_inventory_contract_is_satisfied(self) -> None:
        checker.validate_inventory_contract(self.document, self.witnesses)
        self.assertEqual(len(self.witnesses), checker.EXPECTED_TOTAL)

    def test_real_governed_sources_match_inventory_identity(self) -> None:
        checker.verified_governed_source_bytes(self.document)

    def test_real_source_extraction_matches_inventory(self) -> None:
        extracted = checker.extract_governed_sources(self.document)
        checker.compare_source_inventory(extracted, self.witnesses)

    def test_governed_source_digest_change_fails_closed(self) -> None:
        for source in checker.GOVERNED_SOURCES:
            with self.subTest(source=source):
                path = checker.ROOT / source
                original = path.read_bytes()
                changed = original + b"\n// drift\n"
                real_read_bytes = Path.read_bytes

                def read_bytes(path: Path) -> bytes:
                    if path == (checker.ROOT / source):
                        return changed
                    return real_read_bytes(path)

                with mock.patch.object(
                    Path, "read_bytes", autospec=True, side_effect=read_bytes
                ):
                    with self.assertRaises(checker.SqliteL0CheckError) as ctx:
                        checker.run_check()
                self.assertIn(source, str(ctx.exception))

    def test_line_ending_change_fails_closed(self) -> None:
        for source in checker.GOVERNED_SOURCES:
            with self.subTest(source=source):
                path = checker.ROOT / source
                original = path.read_bytes()
                changed = original.replace(b"\n", b"\r\n")
                real_read_bytes = Path.read_bytes

                def read_bytes(path: Path) -> bytes:
                    if path == (checker.ROOT / source):
                        return changed
                    return real_read_bytes(path)

                with mock.patch.object(
                    Path, "read_bytes", autospec=True, side_effect=read_bytes
                ):
                    with self.assertRaises(checker.SqliteL0CheckError) as ctx:
                        checker.run_check()
                self.assertIn(source, str(ctx.exception))

    def test_missing_governed_source_fails_closed(self) -> None:
        def read_bytes(path: Path) -> bytes:
            raise FileNotFoundError(path)

        with mock.patch.object(
            Path, "read_bytes", autospec=True, side_effect=read_bytes
        ):
            with self.assertRaises(checker.SqliteL0CheckError):
                checker.verified_governed_source_bytes(self.document)

    def test_inventory_contract_rejects_wrong_profile(self) -> None:
        document = deepcopy(self.document)
        document["profile"] = "wrong-profile"
        with self.assertRaises(checker.SqliteL0CheckError) as ctx:
            checker.validate_inventory_contract(document, self.witnesses)
        self.assertIn("profile", str(ctx.exception))

    def test_inventory_contract_rejects_wrong_lane(self) -> None:
        document = deepcopy(self.document)
        document["lane"] = "wrong-lane"
        with self.assertRaises(checker.SqliteL0CheckError) as ctx:
            checker.validate_inventory_contract(document, self.witnesses)
        self.assertIn("lane", str(ctx.exception))

    def test_inventory_contract_rejects_wrong_schema_version(self) -> None:
        document = deepcopy(self.document)
        document["schema_version"] = 2
        with self.assertRaises(checker.SqliteL0CheckError) as ctx:
            checker.validate_inventory_contract(document, self.witnesses)
        self.assertIn("schema_version", str(ctx.exception))

    def test_inventory_contract_rejects_changed_source_counts(self) -> None:
        document = deepcopy(self.document)
        document["source_file_counts"] = {
            checker.INTEGRATION_SOURCE: 30,
            checker.LIB_SOURCE: 8,
        }
        with self.assertRaises(checker.SqliteL0CheckError) as ctx:
            checker.validate_inventory_contract(document, self.witnesses)
        self.assertIn("source_file_counts", str(ctx.exception))

    def test_inventory_contract_rejects_changed_classification_counts(self) -> None:
        document = deepcopy(self.document)
        document["classification_counts"] = {"EXACT": 34, "PARTIAL": 5}
        with self.assertRaises(checker.SqliteL0CheckError) as ctx:
            checker.validate_inventory_contract(document, self.witnesses)
        self.assertIn("classification_counts", str(ctx.exception))

    def test_inventory_contract_rejects_missing_source_binding(self) -> None:
        document = deepcopy(self.document)
        del document["governed_sources"][checker.LIB_SOURCE]
        with self.assertRaises(checker.SqliteL0CheckError) as ctx:
            checker.validate_inventory_contract(document, self.witnesses)
        self.assertIn("governed SQLite L0 sources", str(ctx.exception))

    def test_inventory_contract_rejects_malformed_witness_fields(self) -> None:
        witness = self.witnesses[0]
        replacements = {
            "classification": "GAP",
            "overclaim_risk": "",
            "claim_fence": "WITNESSES: alpha.",
            "cargo_test_name": "wrong-cargo-name",
        }
        for field, value in replacements.items():
            with self.subTest(field=field):
                bad = replace(witness, **{field: value})
                others = [w for w in self.witnesses if w is not witness]
                with self.assertRaises(checker.SqliteL0CheckError):
                    checker.validate_inventory_contract(self.document, others + [bad])

        if witness.test_kind == "integration":
            bad_kind = "lib"
        else:
            bad_kind = "integration"
        bad = replace(witness, test_kind=bad_kind)
        others = [w for w in self.witnesses if w is not witness]
        with self.assertRaises(checker.SqliteL0CheckError):
            checker.validate_inventory_contract(self.document, others + [bad])

    def test_load_inventory_rejects_invalid_json(self) -> None:
        with mock.patch.object(
            Path, "read_bytes", autospec=True, return_value=b"{not json"
        ), mock.patch.object(checker, "verify_inventory_digest", return_value=None):
            with self.assertRaises(checker.SqliteL0CheckError):
                checker.load_inventory()

    def _inventory_text(self, mutate_document) -> str:
        document = deepcopy(self.document)
        mutate_document(document)
        return json.dumps(document, indent=2)

    def test_inventory_digest_catches_classification_swap(self) -> None:
        exact_index = next(
            i for i, witness in enumerate(self.witnesses) if witness.classification == "EXACT"
        )
        partial_index = next(
            i for i, witness in enumerate(self.witnesses) if witness.classification == "PARTIAL"
        )
        swapped = list(self.witnesses)
        swapped[exact_index] = replace(self.witnesses[exact_index], classification="PARTIAL")
        swapped[partial_index] = replace(self.witnesses[partial_index], classification="EXACT")
        # The structural contract tolerates the swap because aggregate counts are unchanged.
        checker.validate_inventory_contract(self.document, swapped)

        def mutate(document: dict) -> None:
            entries = document["witnesses"]
            entries[exact_index]["classification"] = "PARTIAL"
            entries[partial_index]["classification"] = "EXACT"

        with mock.patch.object(
            Path, "read_bytes",
            autospec=True,
            return_value=self._inventory_text(mutate).encode("utf-8"),
        ):
            with self.assertRaises(checker.SqliteL0CheckError) as ctx:
                checker.load_inventory()
        self.assertIn("digest", str(ctx.exception))

    def test_inventory_digest_catches_fence_prose_rewrite(self) -> None:
        witness = self.witnesses[0]
        rewritten_fence = (
            "WITNESSES: rewritten prose preserves both boundary markers. "
            "NOT: rewritten prose still denies the same out-of-scope claim."
        )
        mutated_list = list(self.witnesses)
        mutated_list[0] = replace(witness, claim_fence=rewritten_fence)
        # The structural contract tolerates the rewrite because both markers remain.
        checker.validate_inventory_contract(self.document, mutated_list)

        def mutate(document: dict) -> None:
            document["witnesses"][0]["claim_fence"] = rewritten_fence

        with mock.patch.object(
            Path, "read_bytes",
            autospec=True,
            return_value=self._inventory_text(mutate).encode("utf-8"),
        ):
            with self.assertRaises(checker.SqliteL0CheckError) as ctx:
                checker.load_inventory()
        self.assertIn("digest", str(ctx.exception))

    def test_inventory_digest_catches_family_swap(self) -> None:
        witness = self.witnesses[0]
        first_family = witness.families[0]
        replacement = (
            "ownership-gate" if first_family != "ownership-gate" else "creation-atomicity"
        )
        swapped_families = (replacement,)
        mutated_list = list(self.witnesses)
        mutated_list[0] = replace(witness, families=swapped_families)
        # The structural contract tolerates the swap because families stay non-empty.
        checker.validate_inventory_contract(self.document, mutated_list)

        def mutate(document: dict) -> None:
            document["witnesses"][0]["families"] = list(swapped_families)

        with mock.patch.object(
            Path, "read_bytes",
            autospec=True,
            return_value=self._inventory_text(mutate).encode("utf-8"),
        ):
            with self.assertRaises(checker.SqliteL0CheckError) as ctx:
                checker.load_inventory()
        self.assertIn("digest", str(ctx.exception))

    def test_inventory_digest_catches_line_ending_only_mutation(self) -> None:
        raw = checker.INVENTORY_PATH.read_bytes()
        crlf = raw.replace(b"\n", b"\r\n")
        self.assertNotEqual(crlf, raw)
        # The rewrite is semantically identical: text-mode readers normalize
        # CRLF to LF and would hash the governed digest, so the digest must
        # be computed over raw bytes to reject it.
        self.assertEqual(json.loads(crlf), json.loads(raw))
        with mock.patch.object(Path, "read_bytes", autospec=True, return_value=crlf):
            with self.assertRaises(checker.SqliteL0CheckError) as ctx:
                checker.load_inventory()
        self.assertIn("digest", str(ctx.exception))

    def test_attribute_followed_by_fn_is_extracted(self) -> None:
        witnesses = checker.extract_test_witnesses(
            checker.INTEGRATION_SOURCE, "#[test]\nfn alpha() {}\n"
        )
        self.assertEqual(
            witnesses,
            [
                checker.ExtractedWitness(
                    source=checker.INTEGRATION_SOURCE,
                    ordinal=1,
                    test_kind="integration",
                    test_name="alpha",
                    cargo_test_name="alpha",
                    source_line=2,
                )
            ],
        )

    def test_indented_module_test_is_extracted(self) -> None:
        source = "mod tests {\n    #[test]\n    fn beta() {}\n}\n"
        witnesses = checker.extract_test_witnesses(checker.LIB_SOURCE, source)
        self.assertEqual(
            witnesses,
            [
                checker.ExtractedWitness(
                    source=checker.LIB_SOURCE,
                    ordinal=1,
                    test_kind="lib",
                    test_name="beta",
                    cargo_test_name=f"{checker.LIB_TEST_PREFIX}beta",
                    source_line=3,
                )
            ],
        )

    def test_blank_line_between_attribute_and_fn_is_extracted(self) -> None:
        source = "#[test]\n\nfn gamma() {}\n"
        witnesses = checker.extract_test_witnesses(checker.INTEGRATION_SOURCE, source)
        self.assertEqual(witnesses[0].source_line, 3)

    def test_pub_crate_fn_is_extracted(self) -> None:
        source = "#[test]\npub(crate) fn delta() {}\n"
        witnesses = checker.extract_test_witnesses(checker.INTEGRATION_SOURCE, source)
        self.assertEqual(witnesses[0].test_name, "delta")

    def test_multiple_witnesses_use_sequential_ordinal(self) -> None:
        source = "#[test]\nfn alpha() {}\n#[test]\nfn beta() {}\n"
        witnesses = checker.extract_test_witnesses(checker.INTEGRATION_SOURCE, source)
        self.assertEqual([w.ordinal for w in witnesses], [1, 2])

    def test_non_test_attributes_are_not_extracted(self) -> None:
        source = "#[ignore]\nfn epsilon() {}\n#[cfg(test)]\nfn zeta() {}\n"
        self.assertEqual(
            checker.extract_test_witnesses(checker.INTEGRATION_SOURCE, source), []
        )

    def test_comment_between_attribute_and_fn_is_rejected(self) -> None:
        source = "#[test]\n// not a function\nfn alpha() {}\n"
        with self.assertRaises(checker.SqliteL0CheckError) as ctx:
            checker.extract_test_witnesses(checker.INTEGRATION_SOURCE, source)
        self.assertIn("not immediately followed", str(ctx.exception))

    def test_attribute_followed_by_non_function_is_rejected(self) -> None:
        source = "#[test]\nlet x = 1;\n"
        with self.assertRaises(checker.SqliteL0CheckError):
            checker.extract_test_witnesses(checker.INTEGRATION_SOURCE, source)

    def test_attribute_without_function_is_rejected(self) -> None:
        with self.assertRaises(checker.SqliteL0CheckError):
            checker.extract_test_witnesses(checker.INTEGRATION_SOURCE, "#[test]\n")

    def test_non_governed_source_is_rejected(self) -> None:
        with self.assertRaises(checker.SqliteL0CheckError):
            checker.extract_test_witnesses(
                "crates/magpie-log/src/lib.rs", "#[test]\nfn alpha() {}\n"
            )

    def test_compare_source_inventory_accepts_matching_witnesses(self) -> None:
        checker.compare_source_inventory(
            [self._extracted_witness()], [self._inventory_witness()]
        )

    def test_compare_source_inventory_rejects_missing_witness(self) -> None:
        with self.assertRaises(checker.SqliteL0CheckError) as ctx:
            checker.compare_source_inventory([], [self._inventory_witness()])
        self.assertIn("missing", str(ctx.exception))

    def test_compare_source_inventory_rejects_unexpected_witness(self) -> None:
        with self.assertRaises(checker.SqliteL0CheckError) as ctx:
            checker.compare_source_inventory([self._extracted_witness()], [])
        self.assertIn("unexpected", str(ctx.exception))

    def test_compare_source_inventory_rejects_source_line_drift(self) -> None:
        with self.assertRaises(checker.SqliteL0CheckError) as ctx:
            checker.compare_source_inventory(
                [self._extracted_witness(source_line=3)],
                [self._inventory_witness(source_line=2)],
            )
        self.assertIn("source_line", str(ctx.exception))

    def test_compare_source_inventory_rejects_duplicate_extracted_witness(self) -> None:
        witness = self._extracted_witness()
        with self.assertRaises(checker.SqliteL0CheckError) as ctx:
            checker.compare_source_inventory([witness, witness], [self._inventory_witness()])
        self.assertIn("duplicate", str(ctx.exception))

    def test_parse_test_listing_returns_only_matching_lines(self) -> None:
        output = "alpha: test\nnot-a-test\nbeta: test\n"
        self.assertEqual(
            checker.parse_test_listing(output), ("alpha", "beta")
        )

    def test_parse_test_listing_filters_lib_module_prefix(self) -> None:
        output = (
            "other::test: test\n"
            f"{checker.LIB_TEST_PREFIX}alpha: test\n"
        )
        self.assertEqual(
            checker.parse_test_listing(output, module_prefix=checker.LIB_TEST_PREFIX),
            (f"{checker.LIB_TEST_PREFIX}alpha",),
        )

    def test_parse_test_listing_rejects_duplicates(self) -> None:
        with self.assertRaises(checker.SqliteL0CheckError):
            checker.parse_test_listing("alpha: test\nalpha: test\n")

    def test_cargo_commands_use_exact_governed_toolchain_and_targets(self) -> None:
        self.assertEqual(
            checker._cargo_list_command("integration"),
            [
                "cargo",
                f"+{checker.RUST_TOOLCHAIN}",
                "test",
                "-p",
                "magpie-log",
                "--locked",
                "--test",
                "sqlite_l0",
                "--",
                "--list",
            ],
        )
        self.assertEqual(
            checker._cargo_list_command("lib"),
            [
                "cargo",
                f"+{checker.RUST_TOOLCHAIN}",
                "test",
                "-p",
                "magpie-log",
                "--locked",
                "--lib",
                "--",
                "--list",
            ],
        )

    def test_execution_command_uses_exact_witness_name(self) -> None:
        self.assertEqual(
            checker.execution_command(self._inventory_witness()),
            [
                "cargo",
                f"+{checker.RUST_TOOLCHAIN}",
                "test",
                "-p",
                "magpie-log",
                "--locked",
                "--test",
                "sqlite_l0",
                "alpha",
                "--",
                "--exact",
            ],
        )
        lib_witness = self._inventory_witness(
            witness_id="lib/beta",
            source=checker.LIB_SOURCE,
            ordinal=1,
            test_kind="lib",
            test_name="beta",
            cargo_test_name=f"{checker.LIB_TEST_PREFIX}beta",
        )
        self.assertEqual(
            checker.execution_command(lib_witness),
            [
                "cargo",
                f"+{checker.RUST_TOOLCHAIN}",
                "test",
                "-p",
                "magpie-log",
                "--locked",
                "--lib",
                f"{checker.LIB_TEST_PREFIX}beta",
                "--",
                "--exact",
            ],
        )

    def test_validate_execution_result_accepts_single_pass_output(self) -> None:
        output = self._successful_execution_output("alpha")
        checker.validate_execution_result(self._inventory_witness(), 0, output)

    def test_validate_execution_result_accepts_self_spawning_witness_output(self) -> None:
        cargo_test_name = (
            "p12_lost_process_acknowledgement_requires_reopen_instead_of_blind_retry"
        )
        output = "running 1 test\n" + self._successful_execution_output(cargo_test_name)
        witness = self._inventory_witness(
            witness_id=checker.SELF_SPAWNING_WITNESS_IDS[0],
            test_name=cargo_test_name,
            cargo_test_name=cargo_test_name,
        )
        checker.validate_execution_result(witness, 0, output)

    def test_validate_execution_result_accepts_each_self_spawning_witness(self) -> None:
        for witness_id in checker.SELF_SPAWNING_WITNESS_IDS:
            cargo_test_name = witness_id.split("/", 1)[1]
            output = (
                "running 1 test\n"
                + self._successful_execution_output(cargo_test_name)
            )
            witness = self._inventory_witness(
                witness_id=witness_id,
                test_name=cargo_test_name,
                cargo_test_name=cargo_test_name,
            )
            checker.validate_execution_result(witness, 0, output)

    def test_validate_execution_result_rejects_unexpected_self_spawning_output(self) -> None:
        cargo_test_name = (
            "p12_lost_process_acknowledgement_requires_reopen_instead_of_blind_retry"
        )
        base = self._successful_execution_output(cargo_test_name)
        witness = self._inventory_witness(
            witness_id=checker.SELF_SPAWNING_WITNESS_IDS[0],
            test_name=cargo_test_name,
            cargo_test_name=cargo_test_name,
        )
        for output in (base, "running 1 test\n" * 2 + base):
            with self.assertRaises(checker.SqliteL0CheckError):
                checker.validate_execution_result(witness, 0, output)

    def test_validate_execution_result_rejects_nonzero_exit(self) -> None:
        output = self._successful_execution_output("alpha")
        with self.assertRaises(checker.SqliteL0CheckError):
            checker.validate_execution_result(self._inventory_witness(), 1, output)

    def test_validate_execution_result_rejects_malformed_single_pass_output(self) -> None:
        output = self._successful_execution_output("alpha")
        cases = {
            "missing_running": output.replace("running 1 test\n", ""),
            "extra_running": "running 1 test\n" + output,
            "missing_test_line": output.replace("test alpha ... ok\n", ""),
            "failing_test_line": output.replace("test alpha ... ok", "test alpha ... FAILED"),
            "failed_result": output.replace(
                "test result: ok. 1 passed; 0 failed;",
                "test result: FAILED. 0 passed; 1 failed;",
            ),
            "zero_passed_result": output.replace(
                "test result: ok. 1 passed; 0 failed;",
                "test result: ok. 0 passed; 1 failed;",
            ),
            "ignored_result": output.replace(
                "0 ignored;", "1 ignored;"
            ),
            "measured_result": output.replace(
                "0 measured;", "1 measured;"
            ),
            "missing_result": output.replace(
                "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.01s\n",
                "",
            ),
            "extra_result": output + (
                "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; "
                "30 filtered out; finished in 0.01s\n"
            ),
        }
        for name, bad_output in cases.items():
            with self.subTest(name=name):
                with self.assertRaises(checker.SqliteL0CheckError):
                    checker.validate_execution_result(
                        self._inventory_witness(), 0, bad_output
                    )

    def test_successful_check_executes_every_inventory_witness(self) -> None:
        extracted = [
            checker.ExtractedWitness(
                source=witness.source,
                ordinal=witness.ordinal,
                test_kind=witness.test_kind,
                test_name=witness.test_name,
                cargo_test_name=witness.cargo_test_name,
                source_line=witness.source_line,
            )
            for witness in self.witnesses
        ]

        def discover(test_kind: str) -> tuple[str, ...]:
            field = "test_name" if test_kind == "integration" else "cargo_test_name"
            return tuple(
                sorted(
                    getattr(witness, field)
                    for witness in self.witnesses
                    if witness.test_kind == test_kind
                )
            )

        with mock.patch.object(
            checker, "extract_governed_sources", return_value=extracted
        ), mock.patch.object(
            checker, "discover_cargo_tests", side_effect=discover
        ), mock.patch.object(
            checker, "execute_witness"
        ) as execute:
            checker.run_check()
        self.assertEqual(execute.call_count, checker.EXPECTED_TOTAL)

    def test_discovery_drift_fails_before_execution(self) -> None:
        extracted = [
            checker.ExtractedWitness(
                source=witness.source,
                ordinal=witness.ordinal,
                test_kind=witness.test_kind,
                test_name=witness.test_name,
                cargo_test_name=witness.cargo_test_name,
                source_line=witness.source_line,
            )
            for witness in self.witnesses
        ]
        with mock.patch.object(
            checker, "extract_governed_sources", return_value=extracted
        ), mock.patch.object(
            checker, "discover_cargo_tests", return_value=()
        ), mock.patch.object(
            checker, "execute_witness"
        ) as execute:
            with self.assertRaises(checker.SqliteL0CheckError):
                checker.run_check()
        execute.assert_not_called()

    def test_execution_failure_propagates(self) -> None:
        extracted = [
            checker.ExtractedWitness(
                source=witness.source,
                ordinal=witness.ordinal,
                test_kind=witness.test_kind,
                test_name=witness.test_name,
                cargo_test_name=witness.cargo_test_name,
                source_line=witness.source_line,
            )
            for witness in self.witnesses
        ]

        def discover(test_kind: str) -> tuple[str, ...]:
            field = "test_name" if test_kind == "integration" else "cargo_test_name"
            return tuple(
                sorted(
                    getattr(witness, field)
                    for witness in self.witnesses
                    if witness.test_kind == test_kind
                )
            )

        with mock.patch.object(
            checker, "extract_governed_sources", return_value=extracted
        ), mock.patch.object(
            checker, "discover_cargo_tests", side_effect=discover
        ), mock.patch.object(
            checker, "execute_witness",
            side_effect=checker.SqliteL0CheckError("boom"),
        ):
            with self.assertRaisesRegex(checker.SqliteL0CheckError, "boom"):
                checker.run_check()

    def test_source_change_stops_before_discovery_or_execution(self) -> None:
        path = checker.ROOT / checker.INTEGRATION_SOURCE
        original = path.read_bytes()
        changed = original + b"\n// drift\n"
        lib_original = (checker.ROOT / checker.LIB_SOURCE).read_bytes()
        inventory_original = checker.INVENTORY_PATH.read_bytes()

        def read_bytes(path: Path) -> bytes:
            if path == (checker.ROOT / checker.INTEGRATION_SOURCE):
                return changed
            if path == (checker.ROOT / checker.LIB_SOURCE):
                return lib_original
            if path == checker.INVENTORY_PATH:
                return inventory_original
            raise AssertionError(f"unexpected source read: {path}")

        with mock.patch.object(
            Path, "read_bytes", autospec=True, side_effect=read_bytes
        ), mock.patch.object(checker, "discover_cargo_tests") as discovery, mock.patch.object(
            checker, "execute_witness"
        ) as execute:
            with self.assertRaises(checker.SqliteL0CheckError):
                checker.run_check()
        discovery.assert_not_called()
        execute.assert_not_called()

    def test_main_returns_one_on_check_error(self) -> None:
        with mock.patch.object(
            checker, "run_check", side_effect=checker.SqliteL0CheckError("boom")
        ):
            self.assertEqual(checker.main([]), 1)

    def test_main_returns_zero_on_success(self) -> None:
        with mock.patch.object(checker, "run_check") as run_check:
            self.assertEqual(checker.main([]), 0)
        run_check.assert_called_once_with()

    def _successful_execution_output(self, cargo_test_name: str) -> str:
        return "\n".join(
            (
                "running 1 test",
                f"test {cargo_test_name} ... ok",
                "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; "
                "30 filtered out; finished in 0.01s",
            )
        ) + "\n"

    def _inventory_witness(self, **overrides: object) -> checker.InventoryWitness:
        values: dict[str, object] = {
            "witness_id": "integration/alpha",
            "source": checker.INTEGRATION_SOURCE,
            "ordinal": 1,
            "test_kind": "integration",
            "test_name": "alpha",
            "cargo_test_name": "alpha",
            "source_line": 2,
            "classification": "EXACT",
            "families": ("alpha-family",),
            "overclaim_risk": "none",
            "claim_fence": "WITNESSES: alpha. NOT: beta.",
        }
        values.update(overrides)
        return checker.InventoryWitness(**values)  # type: ignore[arg-type]

    def _extracted_witness(self, **overrides: object) -> checker.ExtractedWitness:
        values: dict[str, object] = {
            "source": checker.INTEGRATION_SOURCE,
            "ordinal": 1,
            "test_kind": "integration",
            "test_name": "alpha",
            "cargo_test_name": "alpha",
            "source_line": 2,
        }
        values.update(overrides)
        return checker.ExtractedWitness(**values)  # type: ignore[arg-type]


if __name__ == "__main__":
    unittest.main()
