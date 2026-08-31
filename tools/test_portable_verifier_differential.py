#!/usr/bin/env python3
"""Focused tests for the three-way portable-verifier harness."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
import unittest
from unittest import mock


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from tools import check_portable_verifier_differential as harness  # noqa: E402


class DifferentialHarnessTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        (
            cls.manifest,
            cls.manifest_digest,
            cls.cases,
        ) = harness._load_frozen_cases(harness.ROOT)
        cls.profile = harness._profile_identity(cls.manifest)
        # Build transport fixtures from actual Python conformer results.  The
        # harness under test still obtains Python results itself; these bytes
        # only stand in for the external Go and Rust processes in unit tests.
        cls.python_cases = harness._run_python_actuals(cls.cases, cls.profile)
        cls.go_stdout = b"".join(
            (
                json.dumps(
                    {"id": case.case_id, "result": dict(case.result)},
                    ensure_ascii=True,
                    separators=(",", ":"),
                )
                + "\n"
            ).encode("utf-8")
            for case in cls.python_cases
        )
        cls.rust_stdout = json.dumps(
            {
                "manifest_identity": harness.MANIFEST_ID,
                "manifest_sha256": cls.manifest_digest,
                "results": [
                    {"id": case.case_id, "result": dict(case.result)}
                    for case in cls.python_cases
                ],
            },
            ensure_ascii=True,
            separators=(",", ":"),
        ).encode("utf-8")

    def _fake_processes(self, *, mutate_go: str | None = None):
        calls: list[tuple[list[str], str, dict[str, str] | None]] = []

        go_stdout = self.go_stdout
        if mutate_go is not None:
            lines = go_stdout.splitlines()
            first = json.loads(lines[0].decode("utf-8"))
            first["result"][mutate_go] = (
                99 if mutate_go in {"line", "record_index", "event_count"} else "wrong"
            )
            lines[0] = json.dumps(first, separators=(",", ":")).encode("utf-8")
            go_stdout = b"\n".join(lines) + b"\n"

        def run(
            command: list[str],
            *,
            cwd: str,
            env: dict[str, str] | None,
            stdout: object,
            stderr: object,
            check: bool,
        ) -> subprocess.CompletedProcess[bytes]:
            del stdout, stderr, check
            calls.append((command, cwd, env))
            if command[1:3] == ["run", "."]:
                return subprocess.CompletedProcess(command, 0, go_stdout, b"")
            if command[1:4] == ["test", "-p", "magpie-log"]:
                assert env is not None
                output_path = Path(env[harness.RUST_OUTPUT_ENVIRONMENT])
                output_path.write_bytes(self.rust_stdout)
                return subprocess.CompletedProcess(command, 0, b"", b"")
            raise AssertionError(f"unexpected command: {command!r}")

        return run, calls

    def test_full_three_way_run_uses_real_command_shapes_and_passes(self) -> None:
        fake_run, calls = self._fake_processes()
        with mock.patch.object(harness.subprocess, "run", side_effect=fake_run):
            report = harness.run_differential(
                go_executable=".scratch/toolchains/go1.27.0/go/bin/go.exe",
                cargo_executable="cargo",
            )

        self.assertTrue(report.passed, report.mismatches)
        self.assertEqual(report.case_count, 432)
        self.assertEqual([source.name for source in report.sources], ["rust", "python", "go"])
        self.assertEqual(len(calls), 2)
        self.assertEqual(calls[0][0][1:], ["run", ".", "--manifest", str(harness.ROOT / harness.MANIFEST_RELATIVE_PATH)])
        self.assertEqual(Path(calls[0][1]), harness.ROOT / harness.GO_MODULE_RELATIVE_PATH)
        self.assertEqual(calls[1][0][1:], ["test", "-p", "magpie-log", harness.RUST_TEST_NAME, "--locked"])
        self.assertIsNotNone(calls[1][2])
        self.assertNotIn(harness.RUST_OUTPUT_ENVIRONMENT, (calls[0][2] or {}))
        self.assertEqual(
            report.sources[0].inventory,
            {key: value for key, value in zip(harness.INVENTORY_KEYS, (19, 21, 13, 18, 272, 4, 5, 3, 16, 46, 15))},
        )

    def test_report_format_is_stable_and_includes_all_inventories(self) -> None:
        fake_run, _ = self._fake_processes()
        with mock.patch.object(harness.subprocess, "run", side_effect=fake_run):
            report = harness.run_differential()
        first = harness.format_report(report)
        second = harness.format_report(report)
        self.assertEqual(first, second)
        self.assertIn('"expected":{"ACCEPT":19', first)
        self.assertIn('"rust":{"ACCEPT":19', first)
        self.assertIn('result: PASS', first)

    def test_repeat_mode_proves_stable_output(self) -> None:
        fake_run, calls = self._fake_processes()
        with mock.patch.object(harness.subprocess, "run", side_effect=fake_run):
            report, stable = harness.run_repeated(repetitions=2)
        self.assertTrue(stable)
        self.assertTrue(report.passed, report.mismatches)
        self.assertEqual(len(calls), 4)

    def test_go_field_mismatch_is_reported(self) -> None:
        fake_run, _ = self._fake_processes(mutate_go="line")
        with mock.patch.object(harness.subprocess, "run", side_effect=fake_run):
            report = harness.run_differential()
        self.assertFalse(report.passed)
        self.assertTrue(any("go case a21-altered-message vs frozen expected: line differs" in item for item in report.mismatches))
        self.assertTrue(any("python vs go" in item and "line differs" in item for item in report.mismatches))

    def test_json_number_and_boolean_do_not_alias_integer_result_fields(self) -> None:
        accepted = next(
            case
            for case in self.python_cases
            if case.result["verdict"] == "ACCEPT"
            and case.result["event_count"] == 1
        )
        for malformed in (True, 1.0):
            with self.subTest(malformed=malformed):
                result = dict(accepted.result)
                result["event_count"] = malformed
                with self.assertRaisesRegex(
                    harness.DifferentialHarnessError,
                    "event_count is not an unsigned 64-bit integer",
                ):
                    harness._validate_result(result, "malformed producer")

    def test_operational_process_failure_is_not_a_semantic_mismatch(self) -> None:
        def failed_run(*args: object, **kwargs: object) -> subprocess.CompletedProcess[bytes]:
            del args, kwargs
            return subprocess.CompletedProcess([], 2, b"", b"tool unavailable")

        with mock.patch.object(harness.subprocess, "run", side_effect=failed_run):
            with self.assertRaisesRegex(harness.DifferentialHarnessError, "failed with exit 2"):
                harness.run_differential()

    def test_invalid_repeat_count_is_operational_failure(self) -> None:
        with self.assertRaisesRegex(harness.DifferentialHarnessError, "at least 1"):
            harness.run_repeated(repetitions=0)

    def test_main_maps_operational_failure_to_exit_two(self) -> None:
        with mock.patch.object(
            harness,
            "run_repeated",
            side_effect=harness.DifferentialHarnessError("missing tool"),
        ):
            self.assertEqual(harness.main([]), 2)


if __name__ == "__main__":
    unittest.main()
