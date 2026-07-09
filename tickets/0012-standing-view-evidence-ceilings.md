# Ticket 0012: StandingView evidence ceilings

Superseded doctrine note: ticket 0019 corrects the HumanRatification rule.
Humans settle nothing epistemically; HumanRatification is governance/judgment
evidence and must not yield `Settled`.

## Goal

Define the ADR-0002 evidence ceiling law for future `StandingView` semantics.

This is a docs-only rule-freezing ticket. It defines how each evidence kind may
contribute to future standing without implementing promotion, settlement,
debt, invalidation, supersession, ratification, `EpistemicGate`, or writer
surfaces.

## Scope

- Create `docs/design/standing-view-evidence-ceilings.md`.
- Define the ceiling for every ADR-0002 evidence kind.
- Define edge interaction rules for `supports`, `ratifies`, `derived_from`,
  `contradicts`, `invalidates`, and `supersedes`.
- Define claim-domain distinctions needed by future `StandingView` and
  `EpistemicGate` policy.
- Preserve the `SegmentAnchored` occurrence/inclusion boundary.
- List future tests and rollout order.

## Non-goals

- No Rust changes.
- No tests.
- No `docs/FORMAT.md` changes.
- No `Payload` changes.
- No canonical encoding changes.
- No golden vector changes.
- No Python verifier changes.
- No `StandingView` implementation changes.
- No support promotion.
- No settlement implementation.
- No contradiction debt.
- No invalidation or supersession semantics.
- No ratification semantics.
- No `EpistemicGate`.
- No writer-facing surfaces.
- No MCP or CLI write surfaces.
- No lens ingestion.
- No Deadbolt changes.
- No scope inheritance.
- No probabilistic fusion.
- No Dung/JTMS/ATMS/AGM machinery.

## Files changed

- `docs/design/standing-view-evidence-ceilings.md`
- `tickets/0012-standing-view-evidence-ceilings.md`

## Ceiling rules

Evidence ceilings are deterministic caps, not probabilities and not automatic
promotion.

- `DeterministicVerification`: may reach `Settled` for exact
  machine-checkable predicates.
- `HumanRatification`: may reach at most `Supported` as governance/judgment
  evidence under explicit human authority, exact scope, and the required
  `EpistemicGate` admission rule.
- `DeadboltAnchor`: may reach `Settled` for occurrence/inclusion claims only
  with successful verifier context and at most `Supported` for interpretation
  claims.
- `ExecutionEvidence`: may reach `Supported`.
- `BehavioralEvaluation`: may reach `Supported`.
- `ExternalSource`: may reach `Supported`, but may remain `Conjectured`
  depending on future policy, corroboration, and freshness.
- `ModelSelfReport`: may reach only `Conjectured`.
- `LensReadout`: may reach only `Conjectured` in v1.

## Edge interactions

- `supports`: may contribute positive standing only up to both source standing
  and the source evidence ceiling; requires source and target existence plus
  exact compatible scope.
- `ratifies`: may support governance/judgment claims only under
  `HumanRatification` or future explicit authority admitted by
  `EpistemicGate`; ordinary agents cannot ratify settlement, and human
  ratification does not settle truth.
- `derived_from`: preserves lineage without promotion.
- `contradicts`: future debt or settlement-blocking rule; no auto-refutation
  in this ticket.
- `invalidates`: future contribution-neutralizing rule; events are not deleted.
- `supersedes`: future currentness and lineage rule; old events are not
  rewritten or silently erased.

## Claim domains

Future `StandingView` and `EpistemicGate` policy should distinguish:

- `Occurrence/Inclusion`
- `ExactMachineCheckable`
- `OperationalObservation`
- `Interpretation`
- `ExternalReport`
- `ModelIntrospection`
- `HumanJudgment`

Claim domains are not new L0 fields in this ticket.

## Future tests

Future PRs should add tests such as:

- `model_self_report_support_does_not_promote_beyond_conjectured`
- `lens_readout_support_does_not_promote_beyond_conjectured`
- `deadbolt_anchor_settles_occurrence_not_interpretation`
- `deterministic_verification_can_settle_exact_predicate`
- `execution_evidence_supports_but_does_not_settle`
- `external_source_supports_but_does_not_settle`
- `human_ratification_requires_human_root_and_exact_scope`
- `support_requires_existing_source_and_target`
- `support_requires_exact_scope_match`
- `derived_from_preserves_lineage_without_promotion`
- `contradiction_blocks_settlement_without_auto_refuting`
- `invalidates_neutralizes_without_deleting`
- `supersedes_marks_currentness_without_deleting`
- `segment_anchor_does_not_create_evidence_or_settle_claim`

This ticket does not add tests.

## Acceptance criteria

- Docs-only PR.
- No Rust changes.
- No FORMAT changes.
- No canonical changes.
- No golden changes.
- No verifier changes.
- No writer surfaces.
- No `EpistemicGate`.
- Defines a ceiling for all eight evidence kinds.
- Defines edge interaction rules.
- Defines claim-domain distinction.
- Preserves `SegmentAnchored` occurrence-only boundary.
- States ceilings are deterministic caps, not probabilities.
- Includes future test matrix.
- Includes rollout order.

## Reviewer checklist

- Confirm only the design note and ticket changed.
- Confirm every evidence kind is covered.
- Confirm no ceiling is described as automatic promotion.
- Confirm `ModelSelfReport` and `LensReadout` cannot promote beyond
  `Conjectured`.
- Confirm `DeadboltAnchor` can settle occurrence/inclusion only with
  successful verifier context, but not interpretation truth.
- Confirm `HumanRatification` requires human authority, exact scope, and the
  required `EpistemicGate` admission rule, and still does not yield `Settled`.
- Confirm `scope_ref` remains exact-match and opaque.
- Confirm `SegmentAnchored` remains occurrence/inclusion evidence only.
- Confirm writer-facing surfaces remain blocked until `EpistemicGate`.

## Follow-up tickets

1. Add ceiling fixture/test scaffolding.
2. Implement conservative no-promotion ceiling tests.
3. Implement `supports` to `Supported` for `ExecutionEvidence` and
   `BehavioralEvaluation`.
4. Implement exact deterministic verification settlement.
5. Implement Deadbolt occurrence/inclusion settlement only with successful
   verifier context.
6. Design and implement the `EpistemicGate` admission slice required for
   HumanRoot governance/judgment ratification.
7. Implement HumanRoot ratification only through that gate, capped below
   epistemic settlement.
8. Implement contradiction debt.
9. Implement invalidation and supersession.
10. Design remaining `EpistemicGate` admission rules.
11. Add writer-facing surfaces only after `EpistemicGate`.
