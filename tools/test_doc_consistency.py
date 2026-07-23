"""Consistency tests for the post-v3 documentation reconciliation PR.

This PR (tickets 0055) touches only Markdown surfaces: `README.md`,
`docs/design/historical-review-remediation-ledger.md`,
`docs/design/standing-aggregation-independence-groups.md`,
`tickets/0054-standing-policy-v3-external-report-corroboration-implementation.md`,
and the new `tickets/0055-post-v3-architecture-ledger-reconciliation.md`.

There is no Rust, Python, or other executable surface changed by this PR, so
there is nothing to unit-test in the ordinary sense. What *is* checkable is
whether the documentation stays internally consistent: whether the exact test
function names, file paths, and commit hashes the prose cross-references
actually exist, and whether specific stale-language regressions that this PR
was explicitly designed to fix do not creep back in.

These tests are read-only: they open the five changed files (plus the Rust
source/test files the docs reference) and assert on their text content. They
do not exercise runtime behavior, and they intentionally avoid depending on
`git` so they remain fast and hermetic.

Run with:
    python -m unittest tools.test_doc_consistency -v
or:
    python tools/test_doc_consistency.py
"""

from __future__ import annotations

import re
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent

README = REPO_ROOT / "README.md"
LEDGER = REPO_ROOT / "docs" / "design" / "historical-review-remediation-ledger.md"
AGG_NOTE = REPO_ROOT / "docs" / "design" / "standing-aggregation-independence-groups.md"
TICKET_0054 = (
    REPO_ROOT
    / "tickets"
    / "0054-standing-policy-v3-external-report-corroboration-implementation.md"
)
TICKET_0055 = REPO_ROOT / "tickets" / "0055-post-v3-architecture-ledger-reconciliation.md"

CHANGED_DOC_FILES = [README, LEDGER, AGG_NOTE, TICKET_0054, TICKET_0055]

# Exact-hash constants asserted to be cross-consistent across the changed
# documentation surfaces.
PR68_MERGE_COMMIT = "7720a2749f3b85f95e38580d0af4fd26c6f02658"
TICKET_0054_HEAD = "f030be8ff329c93fd7138d6f831bb44cc42e9409"
PR56_MERGE_COMMIT = "351333ac17cb1d7b82d0ee144568d48a7406f80a"
PR57_MERGE_COMMIT = "c12ef0756a2984c99329679d8630ddec5bc4bc39"

FULL_SHA_RE = re.compile(r"^[0-9a-f]{40}$")

# Every exact Rust test function name the changed docs cite, mapped to the
# repo-relative source file the docs claim it lives in. Regression coverage
# here guards against the docs drifting from the code they describe (e.g. a
# later rename/removal of one of these tests without updating the prose).
DOC_REFERENCED_TESTS = {
    "legacy_raw_refuted_remains_audit_visible_and_does_not_veto_matched_v2_support": (
        "crates/magpie-claims/tests/deterministic_support_standing_v2.rs"
    ),
    "inherited_governed_refuted_overrides_matched_deterministic_support": (
        "crates/magpie-claims/src/standing_v2.rs"
    ),
    "same_origin_multiplicity_is_visible_but_counts_once": (
        "crates/magpie-claims/tests/standing_v3.rs"
    ),
    "many_same_origin_contributions_remain_visible_and_count_once": (
        "crates/magpie-claims/tests/standing_v3.rs"
    ),
    "exact_public_happy_path_supports_but_never_settles": (
        "crates/magpie-claims/tests/standing_v3.rs"
    ),
    "more_than_two_groups_are_all_retained_and_repeated_resolution_is_identical": (
        "crates/magpie-claims/tests/standing_v3.rs"
    ),
    "standing_precedence_and_never_settled_law_are_exact": (
        "crates/magpie-claims/src/standing_v3.rs"
    ),
    "grouping_ignored_order_and_distinct_group_count_are_structural": (
        "crates/magpie-claims/src/standing_v3.rs"
    ),
    "canonical_bytes_are_ordered_compact_and_repeatable": (
        "crates/magpie-claims/src/standing_v3.rs"
    ),
    "trusted_conflict_is_terminal_and_admits_no_group": (
        "crates/magpie-claims/tests/origin_admission_audit.rs"
    ),
    "bare_deadbolt_anchor_requires_verifier_context_and_does_not_settle": (
        "crates/magpie-claims/tests/standing_resolution.rs"
    ),
    "summary_only_verification_returns_count_and_tip_without_events": (
        "crates/magpie-log/src/logimpl.rs"
    ),
    "verify_chain_uses_one_record_snapshot": "crates/magpie-log/tests/chain.rs",
    "writer_recovery_verifies_once_and_recovers_exact_count_and_tip": (
        "crates/magpie-log/tests/chain.rs"
    ),
    "verification_and_replay_preserve_late_error_parity_and_apply_no_prefix": (
        "crates/magpie-log/tests/chain.rs"
    ),
}

# The exact five paths this PR is authorised (by ticket 0055 section 3) to
# touch, in the order the ticket lists them.
EXPECTED_ALLOWLIST = [
    "README.md",
    "docs/design/historical-review-remediation-ledger.md",
    "docs/design/standing-aggregation-independence-groups.md",
    "tickets/0054-standing-policy-v3-external-report-corroboration-implementation.md",
    "tickets/0055-post-v3-architecture-ledger-reconciliation.md",
]


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def extract_fenced_block_after(text: str, heading: str) -> str:
    """Return the contents of the first ``` fenced block that appears after
    the given heading (or literal marker) in ``text``.
    """
    marker_index = text.index(heading)
    remainder = text[marker_index:]
    match = re.search(r"```(?:text|rust)?\n(.*?)\n```", remainder, re.DOTALL)
    if not match:
        raise AssertionError(f"no fenced block found after {heading!r}")
    return match.group(1)


def extract_section(text: str, heading: str) -> str:
    """Return the text of a ``## N. Heading`` section, up to (but excluding)
    the next ``## `` heading.
    """
    marker_index = text.index(heading)
    remainder = text[marker_index + len(heading) :]
    next_heading = re.search(r"\n## ", remainder)
    return remainder[: next_heading.start()] if next_heading else remainder


class ChangedFilesExistTests(unittest.TestCase):
    """Sanity checks that every file this PR touches is present and non-empty."""

    def test_all_changed_doc_files_exist(self) -> None:
        for path in CHANGED_DOC_FILES:
            with self.subTest(path=path):
                self.assertTrue(path.is_file(), f"missing changed file: {path}")

    def test_all_changed_doc_files_are_non_empty(self) -> None:
        for path in CHANGED_DOC_FILES:
            with self.subTest(path=path):
                self.assertGreater(len(read(path).strip()), 0)

    def test_ticket_0055_is_new_and_uniquely_named(self) -> None:
        self.assertTrue(TICKET_0055.is_file())
        # Guard against an accidental duplicate/renamed ticket file for 0055.
        siblings = list((REPO_ROOT / "tickets").glob("0055-*.md"))
        self.assertEqual(len(siblings), 1)
        self.assertEqual(siblings[0], TICKET_0055)


class ReadmeContentTests(unittest.TestCase):
    def setUp(self) -> None:
        self.text = read(README)

    def test_mentions_v1_v2_and_narrow_v3_corroboration_rule(self) -> None:
        self.assertIn(
            "one narrow v3 external-report corroboration rule", self.text
        )
        self.assertIn("v1 direct occurrence settlement", self.text)
        self.assertIn("v2 deterministic direct support", self.text)

    def test_tour_paragraph_notes_v3_is_not_part_of_tour(self) -> None:
        self.assertIn(
            "The newer explicit policy v3 corroboration rule (ledger entry 25) is\n"
            "not part of this tour, and the tour's output is unchanged by it.",
            self.text,
        )

    def test_layout_section_describes_aggregation_note_as_first_rule_implemented(
        self,
    ) -> None:
        self.assertIn(
            "design/standing-aggregation-independence-groups.md   "
            "aggregation doctrine: first v3 rule implemented, broader constraints future",
            self.text,
        )

    def test_governed_claim_memory_section_records_ticket_0054_and_pr68(self) -> None:
        self.assertIn("ratified by Ticket 0053, implemented by Ticket 0054", self.text)
        self.assertIn("merged via PR #68", self.text)
        self.assertIn(
            "That corroboration rule is the only implemented aggregation rule.",
            self.text,
        )

    def test_entry_9_bounds_aggregation_phasing_to_policy_v3(self) -> None:
        # Regression: ticket 0055 section 7 records that an earlier pass over
        # this PR found and fixed an unbounded aggregation-phasing claim in
        # entry 9. Guard against that exact stale phrasing reappearing.
        self.assertIn(
            "Aggregation beyond the first narrow policy-v3 rule (entry 25),",
            self.text,
        )
        self.assertNotIn(
            "Aggregation, contradiction debt, invalidation, and supersession remain\n"
            "   separate later phases.",
            self.text,
        )

    def test_entries_16_through_23_reference_ticket_0054_entry_25(self) -> None:
        # Every "later" placeholder in entries 16-23 should now point at the
        # concrete implementing ticket/entry rather than an open-ended "standing
        # policy v3" placeholder.
        occurrences = re.findall(
            r"implemented later by Ticket 0054 \(entry\s*\n?\s*25\)", self.text
        )
        self.assertGreaterEqual(len(occurrences), 5)

    def test_entry_24_records_pr56_and_pr57_progress(self) -> None:
        self.assertIn("PR #56", self.text)
        self.assertIn("PR #57", self.text)
        self.assertIn(
            "landed the legacy raw versus inherited governed `Refuted` regression",
            self.text,
        )
        self.assertIn(
            "removed parsed-event retention for verification-only and writer-recovery",
            self.text,
        )
        self.assertIn(
            "`LogStore::read_records` raw-record materialisation\n"
            "    remains future interface work.",
            self.text,
        )

    def test_entry_25_records_exact_publication_commits(self) -> None:
        self.assertIn(TICKET_0054_HEAD, self.text)
        self.assertIn(PR68_MERGE_COMMIT, self.text)
        self.assertIn("merged via PR #68 at merge", self.text)
        self.assertIn("on 2026-07-23.", self.text)

    def test_entry_25_never_claims_settled_for_external_report(self) -> None:
        self.assertIn(
            "lane contributes\ngoverned `Supported`, never `Settled`, to the exact "
            "requested `ExternalReport`\nclaim.",
            self.text,
        )

    def test_does_not_overclaim_pr57_removed_read_records_materialisation(self) -> None:
        # Explicit stop condition from ticket 0055 section 11: never claim
        # PR #57 removed raw-record materialisation.
        self.assertNotIn("PR #57 removed `LogStore::read_records`", self.text)
        self.assertNotIn("read_records` raw-record materialisation is resolved", self.text)


class HistoricalLedgerContentTests(unittest.TestCase):
    def setUp(self) -> None:
        self.text = read(LEDGER)

    def test_section_5_is_renamed_to_dedicated_follow_up_status(self) -> None:
        self.assertIn("## 5. Dedicated follow-up status", self.text)
        self.assertNotIn("## 5. Live dedicated implementation follow-ups", self.text)

    def test_subsections_5_1_and_5_2_present(self) -> None:
        self.assertIn("### 5.1 Legacy raw-refutation regression tests", self.text)
        self.assertIn("### 5.2 Verification-only memory retention", self.text)

    def test_5_1_records_pr56_merge_commit_and_both_test_names(self) -> None:
        self.assertIn(PR56_MERGE_COMMIT, self.text)
        self.assertIn(
            "legacy_raw_refuted_remains_audit_visible_and_does_not_veto_matched_v2_support",
            self.text,
        )
        self.assertIn(
            "inherited_governed_refuted_overrides_matched_deterministic_support",
            self.text,
        )
        self.assertIn(
            "(`crates/magpie-claims/tests/deterministic_support_standing_v2.rs`)",
            self.text,
        )
        self.assertIn("(`crates/magpie-claims/src/standing_v2.rs` test module)", self.text)

    def test_5_2_records_pr57_merge_commit_and_landed_test_names(self) -> None:
        self.assertIn(PR57_MERGE_COMMIT, self.text)
        for name in (
            "summary_only_verification_returns_count_and_tip_without_events",
            "verify_chain_uses_one_record_snapshot",
            "writer_recovery_verifies_once_and_recovers_exact_count_and_tip",
            "verification_and_replay_preserve_late_error_parity_and_apply_no_prefix",
        ):
            with self.subTest(name=name):
                self.assertIn(name, self.text)

    def test_5_2_still_marks_read_records_as_unresolved(self) -> None:
        self.assertIn("Still unresolved:", self.text)
        self.assertIn(
            "`LogStore::read_records` materialises the complete raw-record snapshot\n"
            "  for every mode;",
            self.text,
        )
        self.assertIn(
            "The complete memory/scalability concern is therefore partially resolved, not\n"
            "done.",
            self.text,
        )

    def test_row_21_preserve_source_standing_mentions_policy_v3(self) -> None:
        self.assertIn(
            "the later exact policy-v3 corroboration rule consumes only the internally "
            "derived support-contribution audit",
            self.text,
        )

    def test_row_37_mentions_partial_pr57_resolution(self) -> None:
        self.assertIn("Partially resolved by PR #57", self.text)

    def test_row_45_keep_verifier_before_aggregation_says_landed_not_future(self) -> None:
        self.assertIn(
            "the landed sequence placed origin verification and audits before policy v3.",
            self.text,
        )

    def test_row_47_records_pr56_regression_tests_landed(self) -> None:
        self.assertIn(
            "The dedicated regression tests landed in PR #56 without changing production "
            "standing behavior",
            self.text,
        )


class AggregationNoteContentTests(unittest.TestCase):
    def setUp(self) -> None:
        self.text = read(AGG_NOTE)

    def test_status_section_describes_living_doctrine_not_proposed(self) -> None:
        self.assertIn("Living doctrine note, originally ratified as a proposed design note.", self.text)

    def test_status_block_records_ticket_0054_and_0053(self) -> None:
        self.assertIn(
            "external-report corroboration runtime implemented by Ticket 0054", self.text
        )
        self.assertIn(
            "first narrow rule implemented by Ticket 0054; broader rules future", self.text
        )

    def test_future_tests_section_renamed_to_implemented_and_future(self) -> None:
        self.assertIn("## Implemented and Future Tests", self.text)
        self.assertNotIn("## Future Tests", self.text)

    def test_implemented_tests_list_contains_all_landed_names(self) -> None:
        for name in DOC_REFERENCED_TESTS:
            if name in (
                "summary_only_verification_returns_count_and_tip_without_events",
                "verify_chain_uses_one_record_snapshot",
                "writer_recovery_verifies_once_and_recovers_exact_count_and_tip",
                "verification_and_replay_preserve_late_error_parity_and_apply_no_prefix",
            ):
                # These belong to the ledger's PR #57 coverage, not this note.
                continue
            with self.subTest(name=name):
                self.assertIn(f"`{name}`", self.text)

    def test_future_tests_no_longer_lists_landed_names(self) -> None:
        future_section = self.text.split("These remain future tests for later PRs")[1]
        for landed_name in (
            "external_sources_same_origin_group_do_not_amplify",
            "distinct_admitted_origin_groups_are_only_corroboration_separation",
            "external_source_corroboration_never_settles",
            "legacy_raw_refuted_with_successful_deterministic_verification_is_supported",
            "inherited_governed_refuted_with_successful_deterministic_verification_stays_refuted",
            "conflicting_origin_bindings_admit_zero_groups",
            "bare_deadbolt_label_is_not_admitted_verifier_context",
        ):
            with self.subTest(name=landed_name):
                self.assertNotIn(landed_name, future_section)

    def test_reviewer_checklist_references_ticket_0055(self) -> None:
        self.assertIn("Ticket 0055 reconciliation changed documentation", self.text)

    def test_never_claims_source_independence(self) -> None:
        self.assertNotIn("proves statistical independence", self.text)
        self.assertIn(
            "Distinct admitted origin groups are not proof of statistical independence.",
            self.text,
        )


class Ticket0054ContentTests(unittest.TestCase):
    def setUp(self) -> None:
        self.text = read(TICKET_0054)

    def test_section_12_records_final_pre_publication_candidate(self) -> None:
        self.assertIn("Observed on the final pre-publication candidate:", self.text)
        self.assertNotIn("Observed on the uncommitted candidate:", self.text)

    def test_section_14_records_fresh_checker_b_pass(self) -> None:
        self.assertIn(
            "an entirely fresh Checker B\n(`gpt-5.6-sol`, xHigh, read-only blind closure packet "
            "containing the amended\nraw diff and deterministic validation output) reviewed "
            "the final candidate\nand returned `PASS` with no findings.",
            self.text,
        )

    def test_section_16_renamed_and_records_publication(self) -> None:
        self.assertIn("## 16. Publication authority and record", self.text)
        self.assertNotIn("## 16. Publication authority\n", self.text)

    def test_section_16_publication_block_has_exact_fields(self) -> None:
        block = extract_fenced_block_after(self.text, "Published implementation:")
        self.assertIn(f"head:         {TICKET_0054_HEAD}", block)
        self.assertIn("PR:           #68", block)
        self.assertIn("merged by:    the user", block)
        self.assertIn(f"merge commit: {PR68_MERGE_COMMIT}", block)
        self.assertIn("merged:       2026-07-23", block)

    def test_section_16_preserves_pre_publication_history_and_authority(self) -> None:
        self.assertIn(
            "Pre-publication state (historical, at the validation checkpoint above): the\n"
            "implementation remained uncommitted and unpushed.",
            self.text,
        )
        self.assertIn(
            "the user\nremains the sole commit, push, and merge authority.", self.text
        )


class Ticket0055ContentTests(unittest.TestCase):
    def setUp(self) -> None:
        self.text = read(TICKET_0055)

    def test_all_twelve_numbered_sections_present_in_order(self) -> None:
        headings = re.findall(r"^## (\d+)\. ", self.text, re.MULTILINE)
        self.assertEqual(headings, [str(n) for n in range(1, 13)])

    def test_base_commit_matches_readme_and_pr68_merge_commit(self) -> None:
        block = extract_fenced_block_after(self.text, "## 1. Exact base and authority")
        self.assertIn(PR68_MERGE_COMMIT, block)
        self.assertTrue(FULL_SHA_RE.match(PR68_MERGE_COMMIT))

    def test_authority_block_lists_pr56_pr57_and_pr68(self) -> None:
        self.assertIn(f"merged PR #56 (merge commit {PR56_MERGE_COMMIT})", self.text)
        self.assertIn(f"merged PR #57 (merge commit {PR57_MERGE_COMMIT})", self.text)
        self.assertIn(
            f"merged PR #68 (head {TICKET_0054_HEAD},\n"
            f"               merge commit {PR68_MERGE_COMMIT})",
            self.text,
        )

    def test_allowlist_matches_exactly_the_pr_changed_paths(self) -> None:
        block = extract_fenced_block_after(self.text, "## 3. Exact changed-path allowlist")
        listed = [line.strip() for line in block.strip().splitlines() if line.strip()]
        self.assertEqual(listed, EXPECTED_ALLOWLIST)

    def test_classification_marks_runtime_standing_and_policy_unchanged(self) -> None:
        block = extract_fenced_block_after(self.text, "## 2. Classification")
        for key in ("runtime:", "standing:", "policy:", "canonical bytes:", "authority:"):
            with self.subTest(key=key):
                self.assertIn(key, block)
        self.assertIn("runtime:\nunchanged", block)
        self.assertIn("standing:\nunchanged", block)

    def test_reconciliation_table_has_expected_row_count_and_surfaces(self) -> None:
        section = extract_section(self.text, "## 5. Reconciliation table")
        table_lines = [
            line
            for line in section.splitlines()
            if line.startswith("| ") and "Surface" not in line and "---" not in line
        ]
        # One row for each of the five reconciled surfaces plus the trailing
        # "remaining future work" row.
        self.assertEqual(len(table_lines), 6)
        surfaces = [line.split("|")[1].strip() for line in table_lines]
        self.assertEqual(
            surfaces,
            [
                "Standing policy v3 / conservative aggregation",
                "PR #56 regression coverage",
                "PR #57 verification memory",
                "Ticket 0054 publication state",
                "Ticket 0054 checker closure",
                "Remaining future work",
            ],
        )

    def test_stop_conditions_forbid_overclaiming_pr57_and_settled(self) -> None:
        self.assertIn("claiming PR #57\nremoved raw-record materialisation", self.text)
        self.assertIn("claiming v3 produces `Settled`", self.text)
        self.assertIn("claiming v3 proves source independence", self.text)

    def test_future_work_sequence_excludes_aggregation_as_already_implemented(
        self,
    ) -> None:
        block = extract_section(self.text, "## 10. Future work")
        # Aggregation itself (the v3 corroboration rule) is done; only the
        # strictly later phases are listed as future here.
        self.assertNotIn("standing policy v3", block.lower())
        self.assertIn("direct-refutation contract", block)
        self.assertIn("librarian integration.", block)

    def test_reviewer_checklist_has_ten_items(self) -> None:
        block = self.text.split("## 12. Suggested reviewer checklist")[1]
        items = re.findall(r"^\d+\. ", block, re.MULTILINE)
        self.assertEqual(len(items), 10)


class CrossReferencedRustTestsExistTests(unittest.TestCase):
    """Every exact Rust test-function name the changed docs cite must exist,
    in the exact file the docs claim it lives in.

    This is the highest-value regression check for a documentation-only PR:
    it fails loudly if the prose drifts from the underlying code (a renamed
    test, a moved module, or a hallucinated reference).
    """

    def test_every_documented_test_name_exists_in_its_claimed_file(self) -> None:
        for test_name, relative_path in DOC_REFERENCED_TESTS.items():
            with self.subTest(test_name=test_name, path=relative_path):
                source_path = REPO_ROOT / relative_path
                self.assertTrue(
                    source_path.is_file(), f"missing source file: {source_path}"
                )
                source_text = read(source_path)
                self.assertRegex(
                    source_text,
                    r"\bfn\s+" + re.escape(test_name) + r"\s*\(",
                    f"{test_name!r} not found in {relative_path}",
                )

    def test_referenced_test_names_are_unique(self) -> None:
        names = list(DOC_REFERENCED_TESTS)
        self.assertEqual(len(names), len(set(names)))


class CommitHashCrossConsistencyTests(unittest.TestCase):
    """The same commit/PR facts are repeated across README, ticket 0054, and
    ticket 0055; they must agree exactly, and every hash must be a
    well-formed 40-character lowercase hex string.
    """

    def test_hash_constants_are_well_formed_sha1_shape(self) -> None:
        for value in (
            PR68_MERGE_COMMIT,
            TICKET_0054_HEAD,
            PR56_MERGE_COMMIT,
            PR57_MERGE_COMMIT,
        ):
            with self.subTest(value=value):
                self.assertRegex(value, FULL_SHA_RE)

    def test_pr68_merge_commit_appears_in_all_three_publication_surfaces(self) -> None:
        for path in (README, TICKET_0054, TICKET_0055):
            with self.subTest(path=path):
                self.assertIn(PR68_MERGE_COMMIT, read(path))

    def test_ticket_0054_head_appears_in_readme_ticket_0054_and_ticket_0055(self) -> None:
        for path in (README, TICKET_0054, TICKET_0055):
            with self.subTest(path=path):
                self.assertIn(TICKET_0054_HEAD, read(path))

    def test_pr56_and_pr57_merge_commits_appear_in_ledger_and_ticket_0055(self) -> None:
        for path in (LEDGER, TICKET_0055):
            with self.subTest(path=path):
                text = read(path)
                self.assertIn(PR56_MERGE_COMMIT, text)
                self.assertIn(PR57_MERGE_COMMIT, text)

    def test_pr56_and_pr57_commits_also_referenced_by_aggregation_note(self) -> None:
        text = read(AGG_NOTE)
        self.assertIn(PR56_MERGE_COMMIT, text)


if __name__ == "__main__":
    unittest.main()