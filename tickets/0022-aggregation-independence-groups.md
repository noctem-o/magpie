# Ticket 0022: Aggregation and independence groups

## Purpose

Freeze the doctrine for future achieved-standing aggregation and independence
groups before implementing that behavior.

Architectural laws for this phase:

```text
A ceiling is not achieved standing.
A contribution is not aggregation.
Aggregation is replay-derived, deterministic policy behavior.
Independence groups prevent source-count inflation.
Corroboration may help reach an allowed ceiling; it must not exceed the ceiling.
```

## Scope

- Add `docs/design/standing-aggregation-independence-groups.md`.
- Define future aggregation vocabulary:
  - candidate contribution;
  - eligible contribution;
  - support contribution;
  - refutation contribution;
  - achieved standing;
  - independence group;
  - independent corroboration;
  - same-origin evidence;
  - aggregation lane.
- Define conservative independence-group laws.
- Define external-source corroboration limits.
- Define support/refutation aggregation separation.
- Define interaction with contradiction debt, invalidation, and supersession.
- Add future implementation order and future test names.
- Add a narrow pointer from
  `docs/design/standing-view-evidence-ceilings.md`.

## Non-goals

- No Rust changes.
- No tests.
- No L0 payload variants.
- No canonical encoding changes.
- No `docs/FORMAT.md` changes.
- No golden vector changes.
- No `tools/verify_chain.py` changes.
- No writer-facing surfaces.
- No MCP write paths.
- No `EpistemicGate`.
- No achieved-standing aggregation implementation.
- No contradiction debt implementation.
- No invalidation implementation.
- No supersession implementation.
- No ratification admission semantics.
- No `StandingView::resolved_standing()` behavior change.
- No support-ceiling behavior change.
- No refutation-ceiling behavior change.
- No Deadbolt changes.
- No model integration.
- No librarian/navigator functionality.
- No network sync, remote services, or always-on behavior.
- No production trust, certification, attestation, or live-readiness claim.

## Changed files

- `docs/design/standing-aggregation-independence-groups.md`
- `docs/design/standing-view-evidence-ceilings.md`
- `tickets/0022-aggregation-independence-groups.md`

## Validation

Performed:

- `git diff --check` passed.
- `cargo test --workspace --locked` passed.
- `cargo fmt --all -- --check` passed.
- `git diff --name-only` was run. Because new files are untracked until
  staging, `git status --short` was also used to confirm the full changed file
  set.
- `rg -n "aggregation|independence|corroboration|achieved standing|support_ceiling|refutation_ceiling|ExternalSource|HumanRatification|EpistemicGate|StandingView::resolved_standing" docs tickets crates`
  was run as the broad sanity scan.

## Follow-up Phases

- aggregation policy helper code
- contradiction debt
- invalidation / supersession
- `EpistemicGate`
- optional librarian/navigator later

## Suggested Reviewer Checks

- Confirm this ticket is docs/tickets only.
- Confirm independence groups remain opaque exact policy strings.
- Confirm missing, ambiguous, conflicting, or unsupported independence metadata
  does not count as independent corroboration.
- Confirm external-source corroboration cannot yield `Settled`.
- Confirm support and refutation aggregation remain separate.
- Confirm contradiction debt, invalidation, and supersession remain future
  semantics.
- Confirm current `StandingView::resolved_standing()` behavior is unchanged.
