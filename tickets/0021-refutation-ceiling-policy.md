# Ticket 0021: Refutation ceiling policy

## Purpose

Add the negative ceiling mirror for ADR-0002 standing policy.

Architectural law for this phase:

```text
A refutation ceiling is the maximum negative standing contribution policy permits.
A refutation ceiling is not automatic refutation.
```

This phase records which evidence-kind/domain pairs may directly contribute
`Refuted` in a future fold. It does not implement the fold behavior that applies
those ceilings.

## Scope

- Add `refutation_ceiling(kind, domain) -> Option<Status>` to
  `crates/magpie-claims/src/policy.rs`.
- Encode every 8 x 7 refutation matrix cell explicitly, including `None`
  cells.
- Return only `Some(Status::Refuted)` or `None`.
- Allow direct refutation only for:
  - `DeterministicVerification` over `Occurrence/Inclusion`.
  - `DeterministicVerification` over `ExactMachineCheckable`.
  - `DeadboltAnchor` over `Occurrence/Inclusion`.
- Add focused tests for the full matrix and doctrine-sensitive cells.
- Add a narrow design-note section pointing to
  `magpie_claims::policy::refutation_ceiling`.

## Non-goals

- No L0 payload variants.
- No canonical encoding changes.
- No `docs/FORMAT.md` changes.
- No golden vector changes.
- No `tools/verify_chain.py` changes.
- No writer-facing surfaces.
- No MCP write paths.
- No `EpistemicGate`.
- No achieved-standing aggregation.
- No contradiction debt.
- No invalidation.
- No supersession.
- No ratification admission semantics.
- No `StandingView::resolved_standing()` behavior change.
- No support-ceiling behavior change.
- No Deadbolt change.
- No model integration.
- No librarian/navigator functionality.
- No network sync, remote services, or always-on behavior.
- No production trust, certification, attestation, or live-readiness claim.

## Changed files

- `crates/magpie-claims/src/policy.rs`
- `docs/design/standing-view-evidence-ceilings.md`
- `tickets/0021-refutation-ceiling-policy.md`

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
- `rg -n "refutation_ceiling|support_ceiling|Refuted|contradiction debt|HumanRatification|ExternalSource|DeadboltAnchor|EpistemicGate" crates/magpie-claims docs tickets`
  was run as the broad sanity scan.
- `rg -n "_ =>|, _\\)|\\(_," crates/magpie-claims/src/policy.rs` was run to
  check for accidental wildcard catch-all use in the policy matrix; wildcard
  matches remain only in parser error branches and test tuple destructuring.

## Follow-up phases

- Aggregation / independence groups
- Contradiction debt
- Invalidation / supersession
- `EpistemicGate`
- Optional librarian/navigator later

## Suggested reviewer checks

- Confirm the refutation matrix has no wildcard catch-all arm.
- Confirm only `DeterministicVerification` and `DeadboltAnchor` can return
  `Refuted`, and only in the documented exact domains.
- Confirm `HumanRatification`, `ExternalSource`, `ModelSelfReport`,
  `LensReadout`, `ExecutionEvidence`, and `BehavioralEvaluation` never directly
  refute.
- Confirm support ceilings and refutation ceilings remain separate policy
  surfaces.
- Confirm this phase does not change runtime `StandingView` behavior.
