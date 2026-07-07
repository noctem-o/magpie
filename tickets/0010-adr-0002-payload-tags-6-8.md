# Ticket 0010: ADR-0002 payload tags 6-8

## Goal

Implement the ADR-0002 additive payload vocabulary across the full
`magpie-core-v1` format surface:

- tag 6: `ClaimAssertedV2`
- tag 7: `EvidenceRegistered`
- tag 8: `JustificationEdgeRecorded`

This is the first intentional format-expansion implementation after ADR-0002,
the `StandingView` v0 skeleton, the boundary tests, and the tags 6-8
implementation plan.

## Scope

- Add tags 6-8 to `Payload`.
- Add syntactic/domain-closed validation for tags 6-8.
- Add canonical encoding arms in fixed field order.
- Update `docs/FORMAT.md`.
- Append golden fixture coverage for seqs 6-8.
- Preserve existing seqs 0-5 golden hashes and signatures.
- Update the independent Python verifier.
- Add explicit projection arms in `ClaimsView`, `StandingView`, and other
  projections that match `Payload`.
- Add validation and projection-boundary tests.
- Refresh README architecture notes to describe the landed ADR-0002 vocabulary.

## Non-goals

- No `EpistemicGate`.
- No MCP write surfaces.
- No CLI writer surfaces.
- No lens ingestion.
- No Deadbolt changes.
- No scope inheritance.
- No probabilistic fusion.
- No Dung/JTMS/ATMS/AGM machinery.
- No full contradiction/debt/supersession/ratification engine.
- No runtime authority gate.
- No writer-facing API for ordinary agents.

## Files changed

- `README.md`
- `docs/FORMAT.md`
- `crates/magpie-log/src/event.rs`
- `crates/magpie-log/src/canonical.rs`
- `crates/magpie-log/tests/golden.rs`
- `crates/magpie-log/testdata/golden-v1.jsonl`
- `crates/magpie-log/examples/regen_golden.rs`
- `crates/magpie-claims/src/lib.rs`
- `crates/magpie-claims/src/standing.rs`
- `crates/magpie-episodic/src/lib.rs`
- `crates/magpie-episodic/tests/episodic.rs`
- `tools/verify_chain.py`
- `tickets/0010-adr-0002-payload-tags-6-8.md`

## Payloads added

`ClaimAssertedV2` fields, canonical order:

```text
claim_id
statement
scope_ref
actor_class
content_hash
metadata_json
```

`EvidenceRegistered` fields, canonical order:

```text
evidence_id
evidence_kind
summary
scope_ref
actor_class
content_hash
metadata_json
```

`JustificationEdgeRecorded` fields, canonical order:

```text
edge_id
edge_kind
source_id
target_id
scope_ref
actor_class
rationale
metadata_json
```

## Validation

- Required strings must be non-empty: claim/evidence/edge IDs, source/target
  IDs, statement, summary, rationale, and `scope_ref` where present.
- `actor_class` must be one of `HumanRoot`, `AgentProposer`,
  `AutomatedVerifier`, `DeadboltAnchorer`, `LensWitness`, or `SourceImporter`.
- `evidence_kind` must be one of `DeterministicVerification`,
  `HumanRatification`, `DeadboltAnchor`, `ExecutionEvidence`,
  `BehavioralEvaluation`, `ExternalSource`, `ModelSelfReport`, or
  `LensReadout`.
- `edge_kind` must be one of `supports`, `derived_from`, `contradicts`,
  `supersedes`, `invalidates`, or `ratifies`.
- `content_hash` is empty or 64 lowercase hex characters.
- `metadata_json` is opaque string material and is not parsed by L0.

## Canonical / FORMAT changes

- Tags 6-8 are appended after tag 5.
- Tags 0-5 remain unchanged.
- New variants encode all fields as length-prefixed UTF-8 strings.
- `content_hash` is encoded as `str`, not raw `hash32`.
- `metadata_json` is encoded as opaque `str`, not parsed or canonicalized JSON.
- `scope_ref` is a non-empty opaque exact-match string with no inheritance,
  prefix, wildcard, lattice, or ontology semantics.

## Golden vectors

- `golden-v1.jsonl` contains seqs 0-8.
- Existing seqs 0-5 hashes and signatures remain unchanged.
- Seq 6 covers `ClaimAssertedV2`.
- Seq 7 covers `EvidenceRegistered`.
- Seq 8 covers `JustificationEdgeRecorded`.
- Old vectors must not be regenerated to make tests pass.

## Python verifier

- `tools/verify_chain.py` handles tags 6-8 in the same field order as Rust.
- It validates the same closed vocabularies and optional lowercase-hex
  `content_hash` rule.
- It continues to verify old tags 0-5 unchanged.

## Projection behavior

- `ClaimsView` has explicit compatibility arms for tags 6-8.
- `StandingView` has explicit arms for tags 6-8.
- `ClaimAssertedV2` creates a `Conjectured` standing claim if absent.
- `EvidenceRegistered` alone does not create or settle claims.
- `JustificationEdgeRecorded` alone does not create or settle claims.
- `SegmentAnchored` remains occurrence/inclusion evidence only and does not
  decide interpretation truth.

## Tests

- Golden fixture verifies and matches pinned values for seqs 0-8.
- Existing seqs 0-5 pinned hashes and signatures are asserted unchanged.
- Regeneration from code reproduces the committed fixture exactly.
- Tags 6-8 round-trip through the golden fixture.
- Invalid actor/evidence/edge kinds are rejected.
- Invalid non-empty `content_hash` is rejected; empty `content_hash` is accepted.
- Non-empty `scope_ref` and required IDs are enforced.
- `StandingView` tests cover typed assertion, agent proposal non-settlement,
  inert typed evidence/edges, and anchor occurrence-only behavior.

## Acceptance criteria

- `Payload` has tags 6-8.
- `docs/FORMAT.md` documents tags 6-8.
- Rust canonical encoding handles tags 6-8 in fixed field order.
- `Payload::validate` enforces closed vocabularies and required strings.
- `content_hash` is empty or 64 lowercase hex.
- `metadata_json` is opaque string material.
- Golden fixture includes seqs 0-8.
- Existing seqs 0-5 hashes/signatures unchanged.
- Python verifier handles tags 6-8.
- `ClaimsView` has explicit arms.
- `StandingView` has explicit arms.
- `SegmentAnchored` remains occurrence evidence only.
- No writer-facing surfaces.
- No `EpistemicGate`.
- No Deadbolt changes.

## Reviewer checklist

- Confirm no existing tag number or field order changed.
- Confirm seqs 0-5 hashes/signatures match the previous fixture.
- Confirm Rust and Python verifier field orders match.
- Confirm no wildcard payload/projection arms were introduced.
- Confirm the README does not claim `EpistemicGate` or writer surfaces exist.
- Confirm `SegmentAnchored` remains occurrence/inclusion evidence only.

## Follow-up tickets

1. Extend `StandingView` with typed evidence and edge side tables.
2. Add staged support/ceiling/debt semantics for ADR-0002 edges.
3. Design and implement `EpistemicGate`.
4. Add governed writer surfaces only after `EpistemicGate`.
5. Add standing tour/demo after typed events and gate exist.
