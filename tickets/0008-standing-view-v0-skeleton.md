# Ticket 0008: StandingView v0 skeleton

## Goal

Implement the first code step after ADR-0002: a read-only `StandingView`
projection over the existing v0 Magpie event vocabulary.

The goal is to prove that claim standing can begin as a deterministic derived
projection over the current append-only log before any new claim-writing
vocabulary enters L0.

## Scope

- Add `StandingView` and `StandingClaim` to `magpie-claims`.
- Fold current v0 `Payload` variants only:
  `Genesis`, `ClaimAsserted`, `EvidenceRecorded`, `ClaimStatusChanged`,
  `Note`, and `SegmentAnchored`.
- Preserve the existing `ClaimsView` behavior.
- Add focused tests for deterministic replay, legacy v0 replay, and anchor
  events not creating or promoting claims.

## Non-goals

- No new event tags.
- No `ClaimAssertedV2`.
- No `EvidenceRegistered`.
- No `JustificationEdgeRecorded`.
- No canonical encoding changes.
- No golden vector regeneration.
- No verifier changes.
- No `EpistemicGate`.
- No MCP write surfaces.
- No lens ingestion.
- No scope inheritance.
- No probabilistic fusion.
- No Dung, JTMS, ATMS, or AGM machinery.
- No Deadbolt changes.
- No writer-facing surfaces.

## Files changed

- `crates/magpie-claims/src/lib.rs`
- `crates/magpie-claims/src/standing.rs`
- `tickets/0008-standing-view-v0-skeleton.md`

## Semantics

- `StandingView` is a read-only derived projection.
- It uses `BTreeMap` for deterministic serialized output.
- `ClaimAsserted` inserts a standing entry if absent.
- `asserted_initial_status` records the v0 asserted status.
- `standing` starts from the v0 asserted status for compatibility.
- `EvidenceRecorded` appends legacy evidence only when the target claim exists.
- Orphan evidence is ignored in this skeleton.
- `ClaimStatusChanged` updates legacy standing and increments
  `legacy_transitions` only when the target claim exists.
- `Genesis` and `Note` do not affect claim standing.
- `SegmentAnchored` is occurrence/inclusion evidence, not interpretation truth;
  it does not create or promote claims.

## Tests

- `standing_replay_is_deterministic`
- `legacy_v0_chains_still_replay`
- `segment_anchor_is_occurrence_evidence_not_interpretation_truth`

## Acceptance criteria

- `StandingView` exists as a read-only derived projection.
- It folds current v0 payloads only.
- It uses `BTreeMap`.
- It has explicit match arms for all current `Payload` variants.
- `SegmentAnchored` does not create or promote claims.
- Replay determinism is tested.
- Legacy v0 replay is tested.
- No new event tags are added.
- `docs/FORMAT.md` unchanged.
- Canonical encoding unchanged.
- Golden vectors unchanged.
- Verifier unchanged.
- No writer-facing surfaces added.

## Follow-up tickets

1. Add remaining ADR-0002 enforcement tests once typed events exist.
2. Implement additive tags 6-8 with FORMAT/canonical/goldens/verifier/projection
   arms.
3. Design `EpistemicGate`.
4. Add standing tour/demo after typed events exist.

## Reviewer checklist

- Confirm `StandingView` is a projection only and cannot write.
- Confirm current v0 payloads have explicit match arms.
- Confirm anchors are ignored for claim standing.
- Confirm the tests use `LogWriter`, `LogReader`, and `MemStore`.
- Confirm no log-format, golden-vector, verifier, or Deadbolt files changed.
- Confirm no writer-facing surfaces were added.
