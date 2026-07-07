# Ticket 0009: ADR-0002 tags 6–8 implementation plan

## Goal

Create a docs-only implementation plan for future ADR-0002 additive event tags
6-8:

- `ClaimAssertedV2`
- `EvidenceRegistered`
- `JustificationEdgeRecorded`

The plan should make the later format-expansion PR mechanical and reviewable
before any L0 payload, canonical encoding, golden vector, verifier, or
projection code changes.

## Scope

- Define proposed payload fields and canonical field order for tags 6-8.
- Define closed actor, evidence, and edge vocabularies.
- Define validation policy for the later implementation PR.
- Define scope, `metadata_json`, and `content_hash` policies.
- Define golden-vector and Python verifier requirements.
- Define `StandingView` evolution implications.
- Define test matrix, rollout order, stop conditions, and reviewer checklist.

## Non-goals

- No Rust changes.
- No `docs/FORMAT.md` changes.
- No canonical encoding changes.
- No golden vector changes or regeneration.
- No Python verifier changes.
- No event tag implementation.
- No `Payload` changes.
- No `StandingView` changes.
- No `EpistemicGate`.
- No MCP write surfaces.
- No lens ingestion.
- No scope inheritance.
- No probabilistic fusion.
- No Dung, JTMS, ATMS, or AGM machinery.
- No Deadbolt changes.
- No writer-facing surfaces.

## Files changed

- `docs/design/adr-0002-tags-6-8-implementation-plan.md`
- `tickets/0009-adr-0002-tags-6-8-plan.md`

## Proposed payloads

Tag 6, `ClaimAssertedV2`:

```text
claim_id: String
statement: String
scope_ref: String
actor_class: String
content_hash: String
metadata_json: String
```

Tag 7, `EvidenceRegistered`:

```text
evidence_id: String
evidence_kind: String
summary: String
scope_ref: String
actor_class: String
content_hash: String
metadata_json: String
```

Tag 8, `JustificationEdgeRecorded`:

```text
edge_id: String
edge_kind: String
source_id: String
target_id: String
scope_ref: String
actor_class: String
rationale: String
metadata_json: String
```

## Acceptance criteria

- This is docs-only.
- No Rust files changed.
- No `docs/FORMAT.md` changes.
- No canonical encoding changes.
- No golden vector changes.
- No verifier changes.
- No event tag implementation.
- Plan specifies tag 6, 7, and 8 fields and canonical order.
- Plan specifies validation policy.
- Plan specifies golden-vector strategy.
- Plan specifies Python verifier work.
- Plan specifies `StandingView` evolution.
- Plan specifies test matrix.
- Plan states tags 0-5 must remain unchanged.
- Plan states `SegmentAnchored` remains occurrence evidence, not
  interpretation truth.

## Tests / checks

Docs-only checks:

- `git status --short --branch`
- `git diff --stat`
- `git diff --name-only`
- `git diff --check`
- boundary check for `docs/FORMAT.md`, Rust format/event files, verifier,
  golden vectors, Cargo files, and `StandingView`
- grep sanity checks for tags 6-8, golden-vector strategy, verifier work,
  `SegmentAnchored`, tags 0-5, and no wildcard projection arms

Do not run cargo tests unless code is accidentally edited.

## Reviewer checklist

- Confirm only the design plan and ticket changed.
- Confirm no Rust, FORMAT, canonical, golden, verifier, or Cargo files changed.
- Confirm the proposed field order is explicit for each new tag.
- Confirm `metadata_json` is treated as opaque string material.
- Confirm non-empty `content_hash` validation is specified.
- Confirm `scope_ref` is opaque exact-match only.
- Confirm actor, evidence, and edge vocabularies are closed.
- Confirm the plan requires explicit projection match arms and rejects `_ => {}`.
- Confirm `SegmentAnchored` remains occurrence/inclusion evidence, not
  interpretation truth.

## Follow-up tickets

1. Implement additive tags 6-8 with `docs/FORMAT.md`, canonical encoding,
   golden vectors, independent verifier support, and explicit projection arms.
2. Extend `StandingView` for typed claims, evidence, and justification edges.
3. Design `EpistemicGate`.
4. Add standing tour/demo after typed events exist.
