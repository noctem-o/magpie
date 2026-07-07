# Ticket 0011: StandingView typed node tables

## Goal

Extend `StandingView` with deterministic typed replay tables for ADR-0002
claim, evidence, and justification edge events.

This is the first `StandingView` structure step after payload tags 6-8 landed.
It stores the graph material needed by later semantic folds without promoting
or settling claims.

## Scope

- Add typed claim metadata storage for `ClaimAssertedV2`.
- Add typed evidence storage for `EvidenceRegistered`.
- Add typed justification edge storage for `JustificationEdgeRecorded`.
- Preserve existing v0 standing replay behavior.
- Preserve Level A standing behavior for `ClaimAssertedV2`.
- Add deterministic replay, duplicate-ID, and no-promotion tests.
- Refresh the README ledger so typed replay structure is distinct from future
  standing semantics.

## Non-goals

- No `Payload` changes.
- No `Payload::validate` changes.
- No canonical encoding changes.
- No `docs/FORMAT.md` changes.
- No golden vector changes.
- No Python verifier changes.
- No `EpistemicGate`.
- No MCP write surfaces.
- No CLI writer surfaces.
- No ordinary agent writer APIs.
- No lens ingestion.
- No Deadbolt changes.
- No scope inheritance.
- No probabilistic fusion.
- No Dung/JTMS/ATMS/AGM machinery.
- No support promotion.
- No settled promotion.
- No contradiction debt.
- No invalidation semantics.
- No supersession semantics.
- No ratification semantics.
- No cyclic support handling.

## Files changed

- `README.md`
- `crates/magpie-claims/src/lib.rs`
- `crates/magpie-claims/src/standing.rs`
- `tickets/0011-standing-view-typed-tables.md`

## Data model

`StandingView` now has four deterministic `BTreeMap` tables:

- `claims: BTreeMap<String, StandingClaim>`
- `typed_claims: BTreeMap<String, TypedClaimNode>`
- `typed_evidence: BTreeMap<String, TypedEvidenceNode>`
- `justification_edges: BTreeMap<String, TypedJustificationEdge>`

`claims` remains the current standing surface. The three typed tables retain
ADR-0002 graph material for later fold semantics.

## Fold semantics

- v0 events keep their existing behavior.
- `SegmentAnchored` remains occurrence/inclusion evidence only and does not
  create claims or typed evidence.
- `ClaimAssertedV2` creates a `Conjectured` standing claim if absent and stores
  typed claim metadata if absent.
- `EvidenceRegistered` stores typed evidence if absent.
- `JustificationEdgeRecorded` stores typed edges if absent.
- Duplicate IDs are first-wins and do not overwrite existing typed nodes.
- Evidence and edges alone do not create claims.
- Support edges do not promote standing yet.

## Tests

- `claim_asserted_v2_retains_typed_claim_metadata`
- `duplicate_claim_asserted_v2_does_not_overwrite_typed_claim`
- `evidence_registered_is_retained_without_claim_promotion`
- `duplicate_evidence_registered_does_not_overwrite`
- `justification_edge_recorded_is_retained_without_promotion`
- `duplicate_justification_edge_recorded_does_not_overwrite`
- `typed_tables_replay_deterministically`
- `typed_evidence_and_edges_do_not_change_claim_standing_yet`
- `segment_anchor_still_does_not_create_typed_evidence`

Existing Level A tests are updated to distinguish claim-oriented emptiness from
typed table retention.

## Acceptance criteria

- `StandingView` retains typed claim metadata from `ClaimAssertedV2`.
- `StandingView` retains typed evidence from `EvidenceRegistered`.
- `StandingView` retains typed justification edges from
  `JustificationEdgeRecorded`.
- All typed maps use `BTreeMap`.
- Duplicate IDs do not overwrite existing typed nodes.
- Evidence alone does not create claims.
- Edges alone do not create claims.
- Support edges do not promote standing yet.
- `SegmentAnchored` does not create typed evidence by itself.
- Replay is deterministic.
- No `Payload` changes.
- No canonical encoding changes.
- No FORMAT changes.
- No golden vector changes.
- No Python verifier changes.
- No `EpistemicGate`.
- No writer-facing surfaces.
- No Deadbolt changes.

## Reviewer checklist

- Confirm `len()` and `is_empty()` remain claim-oriented.
- Confirm `is_structurally_empty()` checks all replay tables.
- Confirm every `Payload` arm in `StandingView` remains explicit.
- Confirm duplicate typed IDs use first-wins behavior.
- Confirm evidence and edges are stored but do not affect standing.
- Confirm README wording does not imply `EpistemicGate` or writer surfaces
  exist.
- Confirm no format, golden, or verifier files changed.

## Follow-up tickets

1. Implement evidence ceiling evaluation over typed evidence.
2. Implement `supports` semantics under deterministic ceilings.
3. Implement contradiction debt and settled-blocking.
4. Implement invalidation and supersession lineage.
5. Implement ratification semantics.
6. Design and implement `EpistemicGate`.
7. Add writer-facing surfaces only after `EpistemicGate`.
