# Ticket 0020: Policy enums and support ceilings

## Purpose

Turn the corrected ADR-0002 standing doctrine into small, explicit policy code:
closed vocabularies for evidence kinds and claim domains, plus an exhaustive
support ceiling matrix.

Architectural law for this phase:

```text
A ceiling is the maximum positive contribution policy permits.
A ceiling is not automatic promotion.
```

## Scope

- Add `crates/magpie-claims/src/policy.rs`.
- Define closed Rust enums for:
  - `EvidenceKind`
  - `ClaimDomain`
- Add exact fallible parsing from ADR-0002 strings.
- Add `support_ceiling(evidence_kind, claim_domain) -> Option<Status>`.
- Encode every documented matrix cell explicitly, including `None` cells.
- Add focused tests for parsing, unknown values, the full matrix, and the
  doctrine-sensitive HumanRatification / model-report / DeadboltAnchor cells.
- Export the policy module from `magpie-claims`.
- Correct stale ticket 0013 wording if still present.
- Add a narrow design-note pointer to the new helper.

## Non-goals

- No new L0 payload variants.
- No canonical encoding changes.
- No `docs/FORMAT.md` changes.
- No golden vector changes.
- No `tools/verify_chain.py` changes.
- No writer-facing surfaces.
- No MCP write paths.
- No `EpistemicGate`.
- No full achieved-standing aggregation.
- No refutation ceilings.
- No contradiction debt.
- No invalidation or supersession semantics.
- No ratification admission semantics.
- No `StandingView::resolved_standing()` behavior change.
- No Deadbolt change.
- No model integration.
- No librarian/navigator functionality.
- No network sync, remote services, or always-on behavior.
- No production trust, certification, attestation, or live-readiness claim.

## Changed files

- `crates/magpie-claims/src/lib.rs`
- `crates/magpie-claims/src/policy.rs`
- `docs/design/standing-view-evidence-ceilings.md`
- `tickets/0013-ceilings-evidence-kind-x-claim-domain-matrix.md`
- `tickets/0020-policy-enums-and-support-ceilings.md`

## Validation

Performed:

- `git diff --check` passed.
- `cargo test -p magpie-claims --locked` passed.
- `cargo test --workspace --locked` passed.
- `cargo fmt --all -- --check` passed.
- `cargo clippy --locked -p magpie-claims --all-targets -- -D warnings`
  passed.
- `git diff --name-only` was run. Because new files are untracked until
  staging, `git status --short` was also used to confirm the full changed file
  set.
- `rg -n "HumanRatification|support_ceiling|EvidenceKind|ClaimDomain|EpistemicGate|Settled" crates/magpie-claims docs tickets`
  was run as the broad sanity scan.
- A targeted stale-wording scan found no remaining old HumanRatification-only
  wording.

## Follow-up phases

- Refutation ceiling.
- Aggregation / independence groups.
- Contradiction debt.
- Invalidation / supersession.
- `EpistemicGate`.
- Optional librarian/navigator later.

## Suggested reviewer checks

- Confirm the policy matrix has no wildcard catch-all arm.
- Confirm `HumanRatification` never returns `Settled`.
- Confirm `HumanRatification` supports `HumanJudgment`, scoped
  `Interpretation`, and bounded `OperationalObservation` only.
- Confirm model self-reports and lens readouts never exceed `Conjectured`.
- Confirm `DeadboltAnchor` can settle occurrence/inclusion but only support
  interpretation.
- Confirm this phase does not change runtime standing behavior.
