#!/usr/bin/env python3
"""Direct file, CLI, and hostile-boundary tests for ``tools/verify_chain.py``."""

from __future__ import annotations

from contextlib import redirect_stderr, redirect_stdout
from io import StringIO
import json
from pathlib import Path
import subprocess
import sys
import unittest
from unittest import mock


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from tools import check_portable_verifier_python as harness  # noqa: E402
from tools import verify_chain  # noqa: E402


class VerifyChainRequiredPathTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.manifest, _ = harness.load_verified_manifest()
        cls.cases = harness.load_verified_cases(cls.manifest)
        cls.by_id = {case.case_id: case for case in cls.cases}
        cls.profile = cls.manifest["producing_context"]["signature_profile_identity"]
        cls.golden_key = (
            "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c"
        )

    def result(self, case_id: str) -> dict[str, object]:
        case = self.by_id[case_id]
        return verify_chain.verify_complete_history(
            self.profile,
            case.external_key,
            case.input_bytes,
        ).as_dict()

    def test_required_module_is_not_a_dispatch_wrapper(self) -> None:
        source = (ROOT / "tools" / "verify_chain.py").read_text(encoding="utf-8")
        self.assertNotIn("portable_verifier", source)
        self.assertNotIn("subprocess", source)

    def test_external_key_gate_precedes_file_reader(self) -> None:
        def forbidden_reader(path: Path) -> bytes:
            self.fail(f"history reader was called for governed bad key: {path}")

        outcome = verify_chain.verify_file(
            self.profile,
            "0100000000000000000000000000000000000000000000000000000000000000",
            "path-that-must-not-be-opened.jsonl",
            read_bytes=forbidden_reader,
        )

        self.assertEqual(
            outcome.as_dict(),
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

    def test_file_reader_bytes_retain_blank_record_and_terminator_distinctions(self) -> None:
        observed_paths: list[Path] = []

        def selected_case_reader(path: Path) -> bytes:
            observed_paths.append(path)
            return self.by_id["n30-extra-final-terminator"].input_bytes

        outcome = verify_chain.verify_file(
            self.profile,
            self.golden_key,
            "selected-history.jsonl",
            read_bytes=selected_case_reader,
        )

        self.assertEqual(observed_paths, [Path("selected-history.jsonl")])
        self.assertEqual(
            (outcome.rejection_class, outcome.line, outcome.record_index),
            ("Framing", 2, None),
        )

    def test_hostile_parser_and_precedence_witnesses_return_governed_results(self) -> None:
        expected = {
            "n12-invalid-leading-ff": ("Framing", 1, 0),
            "n13-bom-later": ("Framing", 2, 1),
            "n8-escaped-duplicate-signed-event-core": ("Schema", 1, 0),
            "n10-unknown-payload-note": ("Schema", 1, 0),
            "n14-nan-known-member": ("JsonSyntax", 1, 0),
            "n16-seq-negative-zero": ("Schema", 1, 0),
            "n4-numeric-status-0": ("Schema", 1, 0),
            "n28-content-hash-before-payload": ("ContentHash", 1, 0),
            "n28-signature-before-payload": ("Signature", 1, 0),
            "a21-d1-order-two-forged-chain": ("ExternalKey", None, None),
        }
        for case_id, wanted in expected.items():
            with self.subTest(case_id=case_id):
                result = self.result(case_id)
                self.assertEqual(
                    (result["class"], result["line"], result["record_index"]),
                    wanted,
                )

        identity_r = self.result("a21-d2-identity-r-equation-true")
        self.assertEqual(identity_r["verdict"], "ACCEPT")
        self.assertEqual(identity_r["event_count"], 1)

    def test_host_recursion_limit_is_operational_not_governed_json_syntax(self) -> None:
        deeply_nested = b"[" * 1_500 + b"0" + b"]" * 1_500

        try:
            outcome = verify_chain.verify_complete_history(
                self.profile,
                self.golden_key,
                deeply_nested,
            )
        except RecursionError:
            pass
        else:
            self.assertEqual(
                (outcome.verdict, outcome.rejection_class),
                ("REJECT", "Schema"),
            )

        with mock.patch.object(
            verify_chain.json,
            "loads",
            side_effect=RecursionError("simulated host recursion limit"),
        ):
            with self.assertRaises(RecursionError):
                verify_chain.verify_complete_history(
                    self.profile,
                    self.golden_key,
                    b"{}",
                )

        stdout = StringIO()
        stderr = StringIO()
        with mock.patch.object(
            verify_chain,
            "verify_file",
            side_effect=RecursionError("simulated host recursion limit"),
        ):
            with redirect_stdout(stdout), redirect_stderr(stderr):
                exit_code = verify_chain.main(
                    ["verify_chain.py", "selected-history.jsonl", self.golden_key]
                )

        self.assertEqual(exit_code, 2)
        self.assertEqual(stdout.getvalue(), "")
        self.assertIn("operational failure", stderr.getvalue())
        self.assertNotIn("Traceback", stderr.getvalue())

    def test_accepted_event_count_exhaustion_is_operational(self) -> None:
        with self.assertRaisesRegex(RuntimeError, "governed u64 domain"):
            verify_chain._accepted_next_count((1 << 64) - 1)

    def test_direct_process_exit_statuses_and_machine_result(self) -> None:
        script = ROOT / "tools" / "verify_chain.py"
        golden = ROOT / "crates" / "magpie-log" / "testdata" / "golden-v1.jsonl"
        accepted = subprocess.run(
            [sys.executable, str(script), str(golden), self.golden_key],
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(accepted.returncode, 0, accepted.stderr.decode("utf-8", "replace"))
        self.assertEqual(json.loads(accepted.stdout)["event_count"], 9)

        bad_key_missing_file = subprocess.run(
            [
                sys.executable,
                str(script),
                str(ROOT / "path-that-must-not-be-opened.jsonl"),
                "01" + "00" * 31,
            ],
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(bad_key_missing_file.returncode, 1)
        self.assertEqual(json.loads(bad_key_missing_file.stdout)["class"], "ExternalKey")
        self.assertEqual(bad_key_missing_file.stderr, b"")

        valid_key_missing_file = subprocess.run(
            [
                sys.executable,
                str(script),
                str(ROOT / "path-that-is-operationally-missing.jsonl"),
                self.golden_key,
            ],
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        self.assertEqual(valid_key_missing_file.returncode, 2)
        self.assertEqual(valid_key_missing_file.stdout, b"")
        self.assertIn(b"operational failure", valid_key_missing_file.stderr)


if __name__ == "__main__":
    unittest.main()
