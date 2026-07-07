# Ticket 0016: First fold slice — resolved_standing() over supports edges

## Goal

Add the first behavioral fold increment for ADR-0002: a pure, read-only
`StandingView::resolved_standing(claim_id)` that derives a claim's governed
belief from admissible `supports` edges under conservative, settlement-blind
ceilings. It proves the projection can **refuse overpromotion before it can
settle**.

## Architecture (ratified)

- **Accumulate, then derive.** `apply()` stays a structural accumulator; it must
  NOT be changed and must NOT mutate `StandingClaim.standing`. `resolved_standing`
  is a pure function over the final `BTreeMap` tables, so it is order-independent
  and regenerable by construction. Raw `standing` (asserted + legacy
  `ClaimStatusChanged`) is untouched and remains the existing surface.
- `resolved_standing` is the derived belief surface layered on top of raw
  standing.

## Slice behavior

Implement **evidence→claim `supports` only**. A support edge contributes iff:

- `edge_kind == "supports"`;
- the target typed claim exists (`target_id` in `typed_claims`);
- the source typed evidence exists (`source_id` in `typed_evidence`);
- exact **triple scope match**: `evidence.scope_ref == edge.scope_ref ==
  target_typed_claim.scope_ref`.

Conservative, settlement-blind support-slice ceilings (max derivable per kind):

- `ModelSelfReport`, `LensReadout` → at most `Conjectured` (admitted but capped);
- `DeterministicVerification`, `HumanRatification`, `DeadboltAnchor`,
  `ExecutionEvidence`, `BehavioralEvaluation`, `ExternalSource` → at most
  `Supported`;
- unrecognized kind → no contribution.

Resolution rule for a claim with raw standing `R`:

- `Refuted` is off the positive ladder → `resolved_standing` stays `Refuted`.
- `Settled` (only reachable via the legacy path here) stays `Settled`; support
  never derives `Settled`.
- Otherwise resolved = the max on the positive ladder (`Open < Conjectured <
  Supported < Settled`) of `R` and every admissible edge's ceiling. No support
  edge derives `Settled` in this slice (all ceilings ≤ `Supported`).

## Non-goals / Constraints

- Do NOT modify `apply()`, `Payload`, or any production code outside the new
  `resolved_standing` method and its private helpers in `standing.rs`.
- Do NOT touch the format-freeze surface (`docs/FORMAT.md`, `event.rs`,
  `canonical.rs`, `golden.rs`, golden fixture, `verify_chain.py`, Cargo files).
- Do NOT implement: recursive claim→claim support, ratification, contradiction
  debt, invalidation, supersession, settlement, edge-targeting objections,
  `EpistemicGate`, writer surfaces, or Deadbolt changes.
- Do NOT parse `metadata_json` or `claim_domain` (1-D kind ceilings only).
- No new dependencies. `resolved_standing` is not added to `canonical_bytes`
  (derived, not stored); the regenerable invariant is preserved.

## Tests (all new, additive)

- order-independence: same three events in different orders → same
  `resolved_standing`;
- Supported-tier promotion: each of DV / HR / DeadboltAnchor / ExecutionEvidence /
  BehavioralEvaluation / ExternalSource promotes a Conjectured typed claim to
  `Supported`;
- weak-evidence no-promotion: `ModelSelfReport` / `LensReadout` stay `Conjectured`;
- no settlement: no support path yields `Settled`;
- scope matching: any mismatch in the evidence/edge/claim triple → no promotion;
- missing endpoints: missing source evidence → no promotion; missing target claim
  → `None`; v0-only (non-typed) target → no promotion;
- no mutation of raw standing: `get(claim).standing` unchanged while
  `resolved_standing` differs;
- refuted not promoted: raw `Refuted` claim + support → `Refuted`;
- settled preserved: raw `Settled` claim + support → `Settled`.

## Acceptance criteria

- New public method `resolved_standing(&self, &str) -> Option<Status>` plus
  private ladder/ceiling helpers; `apply()` byte-for-byte unchanged.
- `cargo test -p magpie-claims` and `cargo test --workspace --locked` green;
  `cargo clippy -p magpie-claims --tests -- -D warnings` clean.
- Diff touches only `crates/magpie-claims/src/standing.rs` and this ticket;
  boundary check over the format-freeze files produces no output.

## Suggested reviewer checks

- Confirm `apply()` and `canonical_bytes()` are unchanged (regenerable invariant
  intact; resolved standing is a pure query, not stored).
- Confirm no path derives `Settled`; confirm `Refuted`/`Settled` raw are
  preserved.
- Confirm the triple scope match and endpoint-existence guards each have a
  negative test.
