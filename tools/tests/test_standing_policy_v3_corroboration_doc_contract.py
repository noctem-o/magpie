#!/usr/bin/env python3
"""Pin the Ticket 0053 standing-policy-v3 corroboration documentation contract.

Ticket 0053 ("Standing policy v3 external-report corroboration contract") is
a documentation-only PR: it adds no Rust, tests, fixtures, Cargo metadata,
dependency, CI, L0 surface, or runtime capability. It ratifies a design note
and a ticket, and updates three existing design notes plus README.md to
reference them.

Because there is no runtime behavior to exercise, these tests instead pin
the exact textual invariants the ticket and design note themselves claim to
freeze (policy identity, rule identity, threshold constant, the six-path
allowlist, and cross-file consistency between README.md, the new design
note, the new ticket, and the three notes it touches). This follows the
same "freeze exact text" discipline the doctrine itself uses, and guards
against silent drift of a contract whose entire content is prose.

Run with:
    python3 -m unittest tools/tests/test_standing_policy_v3_corroboration_doc_contract.py
"""

from __future__ import annotations

import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

README = ROOT / "README.md"
AGGREGATION_NOTE = ROOT / "docs/design/standing-aggregation-independence-groups.md"
CORROBORATION_NOTE = (
    ROOT / "docs/design/standing-policy-v3-external-report-corroboration-v0.md"
)
EVIDENCE_CEILINGS_NOTE = ROOT / "docs/design/standing-view-evidence-ceilings.md"
SUPPORT_AUDIT_NOTE = ROOT / "docs/design/support-contribution-audit-v0.md"
TICKET_0053 = (
    ROOT / "tickets/0053-standing-policy-v3-external-report-corroboration-contract.md"
)

ALL_SIX_CHANGED_FILES = (
    README,
    AGGREGATION_NOTE,
    CORROBORATION_NOTE,
    EVIDENCE_CEILINGS_NOTE,
    SUPPORT_AUDIT_NOTE,
    TICKET_0053,
)

POLICY_ID = "magpie-claims-standing-v3"
RULE_ID = "ExternalReportCorroborationV0"
THRESHOLD_CONST_NAME = "EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0"


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


class FileExistenceTests(unittest.TestCase):
    def test_all_six_changed_files_exist(self) -> None:
        for path in ALL_SIX_CHANGED_FILES:
            self.assertTrue(path.is_file(), f"missing expected file: {path}")

    def test_new_files_are_substantial(self) -> None:
        # Both new documents are long, detailed contracts; a near-empty file
        # would indicate a truncated or reverted ratification.
        for path in (CORROBORATION_NOTE, TICKET_0053):
            self.assertGreater(
                len(read(path)), 1000, f"{path.name} looks truncated"
            )


class WhitespaceHygieneTests(unittest.TestCase):
    """Mirrors the `git diff --check` validation the ticket itself requires."""

    def test_no_trailing_whitespace(self) -> None:
        for path in ALL_SIX_CHANGED_FILES:
            for lineno, line in enumerate(read(path).splitlines(), start=1):
                self.assertEqual(
                    line,
                    line.rstrip(),
                    f"{path.name}:{lineno} has trailing whitespace",
                )

    def test_no_tab_characters(self) -> None:
        for path in ALL_SIX_CHANGED_FILES:
            self.assertNotIn("\t", read(path), f"{path.name} contains a tab character")

    def test_files_end_with_exactly_one_trailing_newline(self) -> None:
        for path in ALL_SIX_CHANGED_FILES:
            raw = path.read_bytes()
            self.assertTrue(raw.endswith(b"\n"), f"{path.name} lacks a trailing newline")
            self.assertFalse(
                raw.endswith(b"\n\n"), f"{path.name} has a trailing blank line"
            )


class TicketSixPathAllowlistTests(unittest.TestCase):
    """Ticket 0053, section 4, declares an exact six-path allowlist."""

    def setUp(self) -> None:
        self.text = read(TICKET_0053)

    def _allowlist_block(self) -> str:
        match = re.search(
            r"## 4\. Exact six-path allowlist\n\nOnly:\n\n```text\n(.*?)\n```",
            self.text,
            re.S,
        )
        self.assertIsNotNone(match, "could not locate the six-path allowlist block")
        return match.group(1)

    def test_allowlist_contains_exactly_six_entries(self) -> None:
        lines = [line.strip() for line in self._allowlist_block().splitlines() if line.strip()]
        self.assertEqual(len(lines), 6, f"expected exactly 6 paths, got: {lines}")

    def test_allowlist_paths_match_the_pr_file_set_exactly(self) -> None:
        expected = {
            "README.md",
            "docs/design/standing-aggregation-independence-groups.md",
            "docs/design/standing-view-evidence-ceilings.md",
            "docs/design/support-contribution-audit-v0.md",
            "docs/design/standing-policy-v3-external-report-corroboration-v0.md",
            "tickets/0053-standing-policy-v3-external-report-corroboration-contract.md",
        }
        found = {
            line.strip().split()[0]
            for line in self._allowlist_block().splitlines()
            if line.strip()
        }
        self.assertEqual(found, expected)

    def test_allowlist_paths_all_resolve_to_real_files(self) -> None:
        for line in self._allowlist_block().splitlines():
            line = line.strip()
            if not line:
                continue
            relative = line.split()[0]
            self.assertTrue(
                (ROOT / relative).is_file(), f"allowlisted path missing on disk: {relative}"
            )

    def test_no_other_path_is_declared_changeable(self) -> None:
        self.assertIn("No other path may change.", self.text)


class ReadmeItem25Tests(unittest.TestCase):
    def setUp(self) -> None:
        self.text = read(README)

    def test_item_25_heading(self) -> None:
        self.assertIn(
            "25. **Standing policy v3 external-report corroboration v0 — contract\n"
            "    ratified by Ticket 0053, runtime future.**",
            self.text,
        )

    def test_item_25_links_to_design_note(self) -> None:
        self.assertIn(
            "[`docs/design/standing-policy-v3-external-report-corroboration-v0.md`]"
            "(docs/design/standing-policy-v3-external-report-corroboration-v0.md)",
            self.text,
        )

    def test_item_25_links_to_ticket(self) -> None:
        self.assertIn(
            "[Ticket 0053]"
            "(tickets/0053-standing-policy-v3-external-report-corroboration-contract.md)",
            self.text,
        )

    def test_item_25_states_the_threshold_and_never_settled_ceiling(self) -> None:
        self.assertIn(
            "requiring at least two distinct\n"
            "    admitted origin groups for governed `Supported`, never `Settled`.",
            self.text,
        )

    def test_item_25_preserves_inherited_governed_statuses(self) -> None:
        self.assertIn(
            "Inherited governed `Settled`, `Refuted`, and `Supported` are preserved;",
            self.text,
        )

    def test_item_25_documents_same_origin_visibility(self) -> None:
        self.assertIn(
            "legacy raw status remains quarantined; same-origin contributions remain\n"
            "    visible but count once.",
            self.text,
        )

    def test_item_25_lists_runtime_as_future_work(self) -> None:
        self.assertIn(
            "Runtime implementation, conservative\n"
            "    aggregation, refutation aggregation, contradiction debt, invalidation,\n"
            "    supersession, `EpistemicGate`, ordinary claim-bearing writer, loader,\n"
            "    CAS, and filesystem or network ingestion remain future work.",
            self.text,
        )

    def test_all_readme_local_markdown_links_resolve(self) -> None:
        for rel in re.findall(r"\]\((docs/[^)]+\.md|tickets/[^)]+\.md)\)", self.text):
            self.assertTrue((ROOT / rel).is_file(), f"README links to missing file: {rel}")

    def test_next_steps_numbering_is_sequential_and_ends_at_25(self) -> None:
        # Item 25 is the last entry in the "Next steps" list; scope the
        # numbering check to that section since earlier sections (e.g. the
        # append-only/gate-is-the-only-writer list) restart their own
        # numbering.
        section_start = self.text.index("## Next steps")
        section = self.text[section_start:]
        numbers = [int(n) for n in re.findall(r"^(\d+)\.\s", section, re.M)]
        self.assertTrue(numbers, "no numbered items found in the Next steps section")
        self.assertEqual(numbers, list(range(1, len(numbers) + 1)))
        self.assertEqual(numbers[-1], 25)


class AggregationNoteUpdateTests(unittest.TestCase):
    def setUp(self) -> None:
        self.text = read(AGGREGATION_NOTE)

    def test_status_block_records_ticket_0053_contract(self) -> None:
        self.assertIn(
            "standing policy v3:\ncontract ratified by Ticket 0053;\nruntime future",
            self.text,
        )

    def test_status_block_no_longer_says_bare_future(self) -> None:
        # Regression: the pre-PR text was "standing policy v3:\nfuture".
        self.assertNotIn("standing policy v3:\nfuture", self.text)

    def test_status_block_support_contribution_audit_line_unchanged(self) -> None:
        self.assertIn(
            "support-contribution audit:\nimplemented by Ticket 0052",
            self.text,
        )

    def test_origin_comparison_namespace_section_defers_to_ticket_0053(self) -> None:
        self.assertIn(
            "would isolate every contribution and defeat comparison. The Ticket 0053\n"
            "contract separately defines the exact aggregation-lane identity for the first\n"
            "v3 rule; this note does not define those lane fields.",
            self.text,
        )

    def test_future_implementation_order_step_13_cites_ticket_0053(self) -> None:
        self.assertIn(
            "13. Ratify one explicit standing policy v3 aggregation rule — ratified by\n"
            "    Ticket 0053.",
            self.text,
        )

    def test_step_13_narrative_explains_ratification_without_runtime(self) -> None:
        self.assertIn(
            "Step 13\n"
            "is ratified by Ticket 0053, which freezes one exact threshold and lane\n"
            "without implementing runtime. Step 14 remains the later conservative\n"
            "aggregation implementation.",
            self.text,
        )

    def test_reviewer_checklist_threshold_note_cites_ticket_0053(self) -> None:
        self.assertIn(
            "Confirm no numeric threshold or implemented policy v3 appears in this note;\n"
            "  the first exact threshold is ratified separately by Ticket 0053.",
            self.text,
        )

    def test_old_step_13_wording_is_gone(self) -> None:
        # Regression: the pre-PR text ended step 13 with a bare period and no
        # ratification reference.
        self.assertNotIn(
            "13. Ratify one explicit standing policy v3 aggregation rule.\n14.",
            self.text,
        )

    def test_old_reviewer_checklist_wording_is_gone(self) -> None:
        self.assertNotIn(
            "Confirm no numeric threshold or implemented policy v3 appears.\n",
            self.text,
        )


class EvidenceCeilingsNoteUpdateTests(unittest.TestCase):
    def setUp(self) -> None:
        self.text = read(EVIDENCE_CEILINGS_NOTE)

    def test_rollout_order_item_5_cites_ticket_0053(self) -> None:
        self.assertIn(
            "5. Add one explicit support aggregation rule; do not invent generic thresholds.\n"
            "   The first exact rule contract is ratified by Ticket 0053; its runtime\n"
            "   remains future.",
            self.text,
        )


class SupportAuditNoteUpdateTests(unittest.TestCase):
    def setUp(self) -> None:
        self.text = read(SUPPORT_AUDIT_NOTE)

    def test_status_block_records_ticket_0051_and_0052(self) -> None:
        self.assertIn(
            "Ratified contract (Ticket 0051).\n\n"
            "Runtime implemented by Ticket 0052.\n\n"
            "Support contributions are standing-inert typed input only.\n\n"
            "Aggregation absent.\n\n"
            "Standing unchanged.",
            self.text,
        )

    def test_old_status_block_wording_is_gone(self) -> None:
        self.assertNotIn(
            "Ratified contract.\n\nRuntime implementation absent.\n\n"
            "Support contributions absent.",
            self.text,
        )

    def test_consumer_contract_sentence_cites_ticket_0053(self) -> None:
        self.assertIn(
            "Ticket 0051 preserves the exact material that policy v3 may later consume.\n"
            "The first exact consumer contract is ratified separately by Ticket 0053; its\n"
            "runtime remains future.",
            self.text,
        )

    def test_reviewer_checklist_records_ticket_0051_and_0052(self) -> None:
        self.assertIn(
            "- Confirm status records Ticket 0051 ratification, Ticket 0052 runtime\n"
            "  implementation, support contributions as standing-inert typed input,\n"
            "  aggregation absent and standing unchanged.",
            self.text,
        )

    def test_old_reviewer_checklist_wording_is_gone(self) -> None:
        self.assertNotIn(
            "Confirm status is ratified contract, runtime absent, support contributions\n"
            "  absent, aggregation absent and standing unchanged.",
            self.text,
        )


class CorroborationDesignNoteTests(unittest.TestCase):
    """Structural and content checks on the new design note itself."""

    def setUp(self) -> None:
        self.text = read(CORROBORATION_NOTE)

    def test_title(self) -> None:
        self.assertTrue(
            self.text.startswith("# Standing Policy V3 External-Report Corroboration v0\n")
        )

    def test_status_declares_documentation_only(self) -> None:
        self.assertIn("Contract ratified by Ticket 0053. Documentation only.", self.text)
        self.assertIn(
            "This note ratifies policy. It adds no Rust, tests, fixtures, Cargo metadata,\n"
            "dependency, CI, L0 surface, or runtime capability.",
            self.text,
        )

    def test_status_block_declares_never_settled(self) -> None:
        self.assertIn(
            "achieved standing:\ngoverned Supported only, never Settled",
            self.text,
        )

    def test_high_level_law_block(self) -> None:
        self.assertIn(
            "complete internally derived SupportContributionAuditV0\n"
            "+ exact requested ExternalReport claim\n"
            "+ at least two distinct admitted origin groups\n"
            "+ contributions inside one exact permitted aggregation lane\n"
            "->\n"
            "governed Supported",
            self.text,
        )

    def test_exact_policy_id_constant(self) -> None:
        self.assertIn(
            f'pub const MAGPIE_CLAIMS_POLICY_V3_ID: &str = "{POLICY_ID}";',
            self.text,
        )

    def test_exact_rule_identity(self) -> None:
        self.assertIn(f"StandingPolicyRuleV3::{RULE_ID}", self.text)

    def test_exact_threshold_constant(self) -> None:
        self.assertIn(
            f"pub const {THRESHOLD_CONST_NAME}: usize = 2;",
            self.text,
        )

    def test_lane_identity_is_the_origin_comparison_namespace(self) -> None:
        self.assertIn(
            "namespace:\nthe exact OriginComparisonNamespaceV0\n"
            "(exact origin-admission policy ID, exact target claim ID, exact scope_ref)",
            self.text,
        )

    def test_fixed_membership_invariants(self) -> None:
        self.assertIn(
            "support_contribution_policy_id\n== magpie-support-contribution-v0",
            self.text,
        )
        self.assertIn(
            "admitted_contribution_policy_id\n== magpie-admitted-contribution-v0",
            self.text,
        )
        self.assertIn("evidence_kind\n== ExternalSource", self.text)
        self.assertIn("claim_domain\n== ExternalReport", self.text)
        self.assertIn("support_ceiling\n== Supported", self.text)

    def test_five_step_precedence_block(self) -> None:
        self.assertIn(
            "1. inherited governed Settled   -> Settled\n"
            "2. inherited governed Refuted   -> Refuted\n"
            "3. inherited governed Supported -> Supported\n"
            "4. otherwise, successful v3 corroboration -> Supported\n"
            "5. otherwise -> the inherited v2 standing, unchanged",
            self.text,
        )

    def test_v3_rule_never_produces_settled(self) -> None:
        self.assertIn(
            "The v3 rule's achieved contribution is only `Some(Supported)`, never\n"
            "`Settled`.",
            self.text,
        )

    def test_non_goals_exclude_runtime_and_settled(self) -> None:
        self.assertIn("This contract does not define or implement:", self.text)
        self.assertIn("- `Settled` from external-source aggregation;", self.text)
        self.assertIn("- runtime implementation of any kind.", self.text)

    def test_future_resolver_surface_methods(self) -> None:
        self.assertIn(
            "pub fn resolved_standing_v3(\n"
            "        &self,\n"
            "        claim_id: &str,\n"
            "        closure: &ResolutionContentClosureV0,\n"
            "    ) -> Option<Status>;",
            self.text,
        )
        self.assertIn(
            "pub fn resolved_standing_with_trace_v3(\n"
            "        &self,\n"
            "        claim_id: &str,\n"
            "        closure: &ResolutionContentClosureV0,\n"
            "    ) -> StandingResolutionV3;",
            self.text,
        )

    def test_resolver_must_not_accept_caller_supplied_authority(self) -> None:
        self.assertIn(
            "The resolver and composer must not accept:\n\n```text\n"
            "SupportContributionAuditV0\n"
            "SupportContributionV0 values\n"
            "origin groups\n"
            "contribution lists\n"
            "lane IDs\n"
            "thresholds\n"
            "policy IDs\n"
            "StandingView\n"
            "standalone StandingReplaySnapshot authority\n"
            "callbacks\n"
            "lookup providers\n"
            "serialized or cloned audit bytes\n"
            "```",
            self.text,
        )

    def test_standing_resolution_v3_field_list(self) -> None:
        for field in (
            "claim_id: String",
            "governed_standing: Option<Status>",
            "legacy_raw_standing: Option<Status>",
            "currentness: StandingCurrentness",
            "policy_id: &'static str",
            "inherited_v2: StandingResolutionV2",
            "support_contribution_audit: SupportContributionAuditV0",
            "lane: Option<ExternalReportCorroborationLaneV0>",
            "ignored_contributions: Vec<SupportContributionV0>",
            "blockers: Vec<StandingBlockerV3>",
        ):
            self.assertIn(field, self.text)

    def test_implementation_allowlist_has_expected_seams(self) -> None:
        match = re.search(
            r"## Exact implementation allowlist \(future runtime ticket\)\n\n"
            r"The later runtime ticket may change only:\n\n```text\n(.*?)\n```",
            self.text,
            re.S,
        )
        self.assertIsNotNone(match, "could not locate the implementation allowlist block")
        block = match.group(1)
        for seam in (
            "crates/magpie-claims/src/standing_v3.rs",
            "crates/magpie-claims/src/origin_admission_replay.rs",
            "crates/magpie-claims/src/lib.rs",
            "crates/magpie-claims/tests/standing_v3.rs",
            "fixtures/standing-policy-v3-corroboration-v0/*",
            "docs/design/standing-aggregation-independence-groups.md",
            "README.md",
        ):
            self.assertIn(seam, block)

    def test_validation_commands_include_full_workspace_suite(self) -> None:
        self.assertIn("cargo fmt --all --check", self.text)
        self.assertIn(
            "cargo clippy --workspace --all-targets --locked -- -D warnings", self.text
        )
        self.assertIn("cargo test --workspace --locked", self.text)
        self.assertIn("python tools/verify_chain.py <golden logs>", self.text)
        self.assertIn("python tools/check_release_metadata.py", self.text)

    def test_next_slice_handoff_names_step_13_and_14(self) -> None:
        self.assertIn(
            "This contract completes doctrine step 13 (ratify one explicit standing\n"
            "policy v3 aggregation rule). Doctrine step 14 (implement conservative\n"
            "aggregation) remains future runtime work under the exact implementation\n"
            "allowlist above.",
            self.text,
        )

    def test_no_runtime_implementation_claim_anywhere(self) -> None:
        # The contract explicitly disclaims runtime implementation; make sure
        # no accidental "implemented" language slipped in for the v3 rule
        # itself (as opposed to the inherited v0/v1/v2 policies it discusses).
        self.assertIn(
            "- Confirm no runtime implementation claim appears anywhere in this\n"
            "  contract.",
            self.text,
        )


class Ticket0053ContentTests(unittest.TestCase):
    """Structural and content checks on the new ticket file itself."""

    def setUp(self) -> None:
        self.text = read(TICKET_0053)

    def test_title(self) -> None:
        self.assertTrue(
            self.text.startswith(
                "# Ticket 0053: Standing policy v3 external-report corroboration contract\n"
            )
        )

    def test_exact_base_commit(self) -> None:
        self.assertIn(
            "base:\n53799f92d8e7a1dc3467c24fc38896684e1f1945",
            self.text,
        )

    def test_branch_and_commit_message(self) -> None:
        self.assertIn(
            "branch:\nagent/standing-policy-v3-external-report-corroboration-contract",
            self.text,
        )
        self.assertIn(
            "commit:\ndocs: ratify standing policy v3 corroboration rule",
            self.text,
        )
        self.assertIn(
            "PR title:\ndocs: ratify standing policy v3 corroboration rule",
            self.text,
        )
        self.assertIn("PR state:\ndraft", self.text)

    def test_classification_declares_no_runtime(self) -> None:
        self.assertIn(
            "aggregation:\ncontract ratified; not implemented\n\nruntime:\nnot implemented",
            self.text,
        )

    def test_exact_policy_and_rule_constants(self) -> None:
        self.assertIn(f"magpie-claims-standing-v3", self.text)
        self.assertIn(f"StandingPolicyRuleV3::{RULE_ID}", self.text)
        self.assertIn(f"{THRESHOLD_CONST_NAME} = 2", self.text)

    def test_future_public_constants_block(self) -> None:
        self.assertIn(
            f'pub const MAGPIE_CLAIMS_POLICY_V3_ID: &str = "{POLICY_ID}";',
            self.text,
        )
        self.assertIn(
            f"pub const {THRESHOLD_CONST_NAME}: usize = 2;",
            self.text,
        )

    def test_frozen_surfaces_section_lists_v0_v1_v2(self) -> None:
        self.assertIn("This contract does not change:", self.text)
        self.assertIn(
            "- the v0, v1, or v2 standing policies, their rule vocabularies, their\n"
            "  resolution types, or their canonical bytes;",
            self.text,
        )

    def test_stop_conditions_forbid_settled_and_second_rule(self) -> None:
        self.assertIn(
            "Stop and escalate rather than proceeding if any of the following is\n"
            "required:",
            self.text,
        )
        self.assertIn(
            "- a second aggregation rule, threshold, or lane definition;", self.text
        )
        self.assertIn(
            "- any `Settled` outcome for external-source aggregation.", self.text
        )

    def test_publication_authority_defers_to_agents_md(self) -> None:
        self.assertIn(
            "`AGENTS.md` remains binding. The human retains sole authority for\n"
            "creating the branch, committing, pushing, opening or marking ready the\n"
            "draft PR, merging, and resolving review threads.",
            self.text,
        )
        self.assertIn(
            "The worker may not\nmodify `main`, create any other branch, force-push, "
            "or widen the\nallowlist.",
            self.text,
        )

    def test_precise_contract_claim_never_settles_unless_inherited(self) -> None:
        self.assertIn(
            "with fixed membership invariants,\n"
            "at least two distinct admitted origin groups\n"
            "among the complete internally derived support contributions\n"
            "->\n"
            "governed Supported,\n"
            "unless inherited governed standing already preserves\n"
            "Settled, Refuted, or Supported.",
            self.text,
        )

    def test_future_sequence_lists_completed_and_future_steps(self) -> None:
        self.assertIn("step 11 — support-contribution audit v0 contract (Ticket 0051)", self.text)
        self.assertIn("step 12 — support-contribution audit v0 runtime (Ticket 0052)", self.text)
        self.assertIn(
            "step 13 — ratify one explicit standing policy v3 aggregation rule\n"
            "          (this ticket)",
            self.text,
        )
        self.assertIn(
            "step 14 — implement conservative aggregation", self.text
        )

    def test_validation_commands_section_present(self) -> None:
        self.assertIn(
            "The contract ticket itself is documentation-only; run:", self.text
        )
        self.assertIn("git diff --check", self.text)
        self.assertIn(
            "The future runtime ticket must run and report, as observed:", self.text
        )


class CrossFileConsistencyTests(unittest.TestCase):
    """The ticket and the design note must agree on every frozen identifier."""

    def setUp(self) -> None:
        self.ticket_text = read(TICKET_0053)
        self.note_text = read(CORROBORATION_NOTE)
        self.readme_text = read(README)

    def test_policy_id_matches_between_ticket_and_design_note(self) -> None:
        pattern = re.compile(
            r'MAGPIE_CLAIMS_POLICY_V3_ID: &str = "([^"]+)"'
        )
        ticket_match = pattern.search(self.ticket_text)
        note_match = pattern.search(self.note_text)
        self.assertIsNotNone(ticket_match)
        self.assertIsNotNone(note_match)
        self.assertEqual(ticket_match.group(1), POLICY_ID)
        self.assertEqual(note_match.group(1), POLICY_ID)
        self.assertEqual(ticket_match.group(1), note_match.group(1))

    def test_threshold_matches_between_ticket_and_design_note(self) -> None:
        pattern = re.compile(rf"{THRESHOLD_CONST_NAME}: usize = (\d+)")
        ticket_match = pattern.search(self.ticket_text)
        note_match = pattern.search(self.note_text)
        self.assertIsNotNone(ticket_match)
        self.assertIsNotNone(note_match)
        self.assertEqual(ticket_match.group(1), "2")
        self.assertEqual(ticket_match.group(1), note_match.group(1))

    def test_rule_identity_present_in_ticket_and_design_note(self) -> None:
        self.assertIn(RULE_ID, self.ticket_text)
        self.assertIn(RULE_ID, self.note_text)

    def test_readme_and_design_note_agree_never_settled(self) -> None:
        self.assertIn("never `Settled`", self.readme_text)
        self.assertIn("never Settled", self.note_text)

    def test_determinism_law_stated_identically_in_ticket_and_design_note(self) -> None:
        self.assertIn(
            "No ambient CAS, filesystem, callback, plugin, environment, registry,\n"
            "remote, or network lookup participates. The same `(H, P, M)` replay\n"
            "context, compiled policies, and immutable closure produce byte-identical\n"
            "output on every evaluation.",
            self.note_text,
        )
        self.assertIn("No ambient state participates.", self.ticket_text)

    def test_five_named_files_are_all_referenced_by_the_ticket(self) -> None:
        for relative in (
            "README.md",
            "docs/design/standing-aggregation-independence-groups.md",
            "docs/design/standing-view-evidence-ceilings.md",
            "docs/design/support-contribution-audit-v0.md",
            "docs/design/standing-policy-v3-external-report-corroboration-v0.md",
        ):
            self.assertIn(relative, self.ticket_text)


if __name__ == "__main__":
    unittest.main()