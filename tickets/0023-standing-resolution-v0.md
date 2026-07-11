# Ticket 0023: StandingResolution v0 — fail-closed, policy-aware explanation

## Goal

Add the first canonical governed-standing explanation query without adding
promotion, aggregation, refutation, contradiction debt, invalidation,
supersession, writer surfaces, or L0 format changes.

## Scope

- Expose the fixed `magpie-claims-standing-v0` policy identifier.
- Add `StandingResolution`, deterministic trace entries, closed reason codes,
  minimal currentness, and explicit legacy raw standing.
- Parse only the target typed claim's `claim_domain` metadata, fail closed, and
  leave metadata outside chain-integrity validation.
- Report `support_ceiling` cells as candidate caps only.
- Quarantine legacy status changes from governed standing.
- Make `StandingView::resolved_standing()` delegate to the canonical governed
  resolution result.

## Acceptance Criteria

- Missing, malformed, wrong-type, unknown, or duplicate-key claim metadata
  cannot strengthen governed standing and receives a closed trace reason.
- Every justification edge targeting the claim receives one deterministic trace
  entry, whether it is a candidate or rejected.
- `HumanRatification` reports `requires_admission` and contributes nothing.
- Bare `DeadboltAnchor` and `DeterministicVerification` candidates report
  `requires_verifier_context` and do not settle.
- Candidate ceilings never become achieved standing in v0.
- Legacy `ClaimStatusChanged` remains visible as raw audit material but cannot
  settle or refute governed standing.
- Replay and independent event permutations produce identical resolutions.
- Runtime candidate lookup is tested against every support-policy matrix cell.

## Boundary

No L0 payload or canonical encoding changes, format documentation, golden
vectors, Python verifier changes, writer or MCP surfaces, EpistemicGate wrapper,
aggregation, independence groups, direct refutation, contradiction debt,
invalidation, supersession, model/lens ingestion, or Deadbolt changes.

## Validation

```text
cargo test -p magpie-claims --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --locked -p magpie-claims --all-targets -- -D warnings
git diff --check
rg -n "StandingResolution|policy_id|legacy_raw|ClaimStatusChanged|support_ceiling|refutation_ceiling|HumanRatification|DeadboltAnchor|requires_admission|requires_verifier_context|metadata_json|claim_domain" crates docs tickets
```
