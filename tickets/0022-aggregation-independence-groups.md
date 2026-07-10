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
  - policy-eligible contribution;
  - admitted contribution;
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
- Describe PR #33's `StandingResolution` v0 as the canonical fail-closed
  governed-standing explanation surface.
- Make admission/verifier context a precondition for future achieved standing,
  direct refutation, and independence amplification.
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
- No runtime `StandingView::resolved_standing()` behavior change in this
  docs-only PR.
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
- `rg -n "StandingResolution|resolved_standing|claim_domain|metadata_json|Candidate|PolicyEligible|Admitted|independence_group|DeadboltAnchor|EpistemicGate|refutation_ceiling|contradiction debt|invalidation|supersession" docs tickets crates`
  was run as the broad sanity scan.

## Follow-up Phases

1. Keep or land `StandingResolution` v0 from PR #33 as the canonical
   fail-closed explanation surface.
2. Add pure admission and verifier-context predicates before aggregation,
   direct refutation, or independence amplification can affect standing.
3. Prove one narrow achieved-standing slice, preferably verified Deadbolt
   occurrence/inclusion.
4. Add deterministic lanes and traces without amplification.
5. Pin and implement one explicit support aggregation rule. Generic aggregation
   must not invent thresholds absent from policy.
6. Add conservative independence handling. Self-declared, missing, malformed,
   or unadmitted groups must not authorize or amplify themselves.
7. Add direct refutation only through admitted verifier context plus
   `refutation_ceiling`.
8. Add contradiction debt with explicit precedence against direct refutation.
9. Add invalidation and supersession.
10. Add capability-bearing `EpistemicGate` and writer surfaces.
11. Add optional librarian, navigator, model, or lens ingestion later.

## Suggested Reviewer Checks

- Confirm this ticket is docs/tickets only.
- Confirm independence groups remain opaque exact policy strings.
- Confirm missing, malformed, ambiguous, conflicting, self-declared, or
  unadmitted independence metadata does not count as independent corroboration
  or amplify standing.
- Confirm external-source corroboration cannot yield `Settled`.
- Confirm support and refutation aggregation remain separate.
- Confirm contradiction debt, invalidation, and supersession remain future
  semantics.
- Confirm `StandingResolution` v0 is first and `resolved_standing()` is described
  only as its compatibility scalar.
- Confirm admission/verifier context precedes aggregation, direct refutation,
  and independence amplification.
- Confirm a bare `DeadboltAnchor` label is not verifier authority.
- Confirm no generic aggregation implementation is allowed to invent an
  unpinned threshold.
- Confirm no Rust or runtime `StandingView` behavior changes are included.
