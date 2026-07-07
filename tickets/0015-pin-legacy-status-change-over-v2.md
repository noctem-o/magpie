# Ticket 0015: Characterize legacy ClaimStatusChanged over a typed V2 claim

## Goal

Pin, with one adversarial characterization test, the current behavior where a
legacy `ClaimStatusChanged` (tag 3) still drives a typed `ClaimAssertedV2`
(tag 6) claim to any status — including `Settled` — bypassing the ADR-0002
evidence ceilings entirely.

This is a **known compatibility gap**, not desired behavior. The test documents
it so the future `EpistemicGate` PR must change the test *deliberately* when it
closes the path, rather than discovering the hole later.

## Why

`StandingView`'s `ClaimStatusChanged` fold arm applies to any claim in the
`claims` table, regardless of whether it was created by v0 `ClaimAsserted` or by
typed `ClaimAssertedV2`. Every existing `ClaimStatusChanged` test uses a v0
claim, so the v2 interaction is unpinned. Today an ordinary `AgentProposer` can
assert a typed claim (which alone only reaches `Conjectured`) and a legacy manual
status change can then settle it — the exact overpromotion the ceiling law
exists to forbid. ADR-0002's own rule is that the fold must be pinned by
adversarial tests before new write surfaces are trusted; this closes the last
obvious blind spot before fold semantics are implemented.

## Scope

- Add one `#[test]` to `crates/magpie-claims/src/standing.rs` characterizing:
  - `ClaimAssertedV2` (as `AgentProposer`) creates a `Conjectured` typed claim;
  - a subsequent legacy `ClaimStatusChanged` to `Settled` drives `standing` to
    `Settled` and increments `legacy_transitions`;
  - the typed side table (`typed_claim`) is untouched by the legacy event.
- Name it to flag the gap, e.g.
  `legacy_status_change_still_settles_typed_v2_claim_pending_gate`, with a comment
  citing the ceiling doc's "Compatibility with v0 events" note and marking it as a
  characterization the `EpistemicGate` must consciously break.

## Non-goals / Constraints

- Test-only. Do NOT change the `StandingView` fold, `Payload`, or any production
  code. The behavior is pinned as-is, not fixed.
- Do NOT touch the format-freeze surface: `docs/FORMAT.md`,
  `crates/magpie-log/src/event.rs`, `crates/magpie-log/src/canonical.rs`,
  `crates/magpie-log/tests/golden.rs`,
  `crates/magpie-log/testdata/golden-v1.jsonl`, `tools/verify_chain.py`,
  `Cargo.toml`, `Cargo.lock`.
- No new dependencies. Reuse the existing test helpers (`writer`, `provenance`,
  `claim_asserted_v2`, `replay`).
- Do not add redundant no-promotion tests; that behavior is already covered
  (`agent_proposer_claim_asserted_v2_does_not_settle`,
  `typed_evidence_and_edges_do_not_change_claim_standing_yet`, etc.).

## Tests

- `cargo test -p magpie-claims` — the new test passes.
- `cargo test --workspace --locked` — stays green.

## Acceptance criteria

- Exactly one new test function added to `standing.rs`; no production-code diff in
  the crate.
- The test asserts `standing == Settled` and `legacy_transitions == 1` after the
  legacy status change, and that the typed claim metadata is retained.
- The test name and comment mark it as a known gap pending `EpistemicGate`.
- Diff touches only `crates/magpie-claims/src/standing.rs` and this ticket.
- Boundary check over the format-freeze files produces no output.

## Suggested reviewer checks

- Confirm the fold (`apply`) was not modified — only the test module grew.
- Confirm the test would have to change when the gate closes the legacy path, so
  it functions as a tripwire rather than an endorsement.
